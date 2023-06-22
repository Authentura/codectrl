#![warn(clippy::pedantic)]
#![allow(clippy::invalid_regex)]

mod entity;
pub mod redirect_handler;

// region: imports

use codectrl_protobuf_bindings::{
    auth_service::{
        authentication_server::{Authentication, AuthenticationServer},
        GenerateTokenRequest, GenerateTokenRequestResult, LoginUrl,
        RevokeTokenRequestResult, Token, VerifyTokenRequest, VerifyTokenRequestResult,
    },
    data::Log,
    logs_service::{
        Connection, LogClientService, LogClientTrait, LogServerService, LogServerTrait,
        RequestResult, RequestStatus, ServerDetails,
    },
};
use dashmap::{DashMap, DashSet};
use directories::ProjectDirs;
use dotenv::dotenv;
use entity::connection::{ActiveModel, Entity};
use futures::StreamExt;
use log::{error, info, trace, warn};
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenUrl,
};
use once_cell::{race::OnceBool, sync::OnceCell};
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use regex::Regex;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ConnectionTrait, Database, DatabaseConnection,
    EntityTrait, Schema, Set,
};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::VecDeque,
    env,
    fs::{self, File},
    net::SocketAddr,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    sync::{mpsc, RwLock},
    time::sleep_until,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    metadata::MetadataMap, transport::Server, Code, Request, Response, Status, Streaming,
};
use uuid::Uuid;

// endregion

static CENSOR_USERNAMES: OnceBool = OnceBool::new();
// STBoyden: I haven't measured how much of a performance impact recreating the
// Regexes each time `strip_username_from_path` is called would have overall on
// the server, but "caching" them inside a lazy initialised static should
// definitely be faster.
static USERNAME_REGEXES: OnceCell<[Result<Regex, regex::Error>; 4]> = OnceCell::new();
static REDIRECT_HANDLER_PORT: OnceCell<u16> = OnceCell::new();

// region: ConnectionState
#[derive(Debug, Clone)]
pub struct ConnectionState {
    last_update: Instant,
    sent_log_ids: DashSet<String>,
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            sent_log_ids: DashSet::new(),
        }
    }
}

impl ConnectionState {
    pub fn add_log(&mut self, log: &Log) {
        self.sent_log_ids.insert(log.uuid.clone());
        self.last_update = Instant::now();
    }
}
// endregion

// region: Service
#[derive(Debug, Clone)]
pub struct Service {
    logs: Arc<RwLock<VecDeque<Log>>>,
    connections: Arc<RwLock<DashMap<String, ConnectionState>>>,
    host: String,
    port: u32,
    uptime: Instant,
    db_connection: Arc<DatabaseConnection>,
    requires_authentication: bool,
}

impl Service {
    pub fn start_backup_thread(&self) {
        let connections = Arc::clone(&self.connections);
        let db_connection = Arc::clone(&self.db_connection);

        info!("Starting background backup thread...");

        tokio::spawn(async move {
            info!(target: "codectrl_server - background backup thread", "Running every 5 seconds");
            loop {
                sleep_until(tokio::time::Instant::now() + Duration::new(5, 0)).await;

                for mut connection in connections.write().await.iter_mut() {
                    if connection.last_update.elapsed() >= Duration::new(5, 0) {
                        let sent_logs = if let Ok(sent_log_ids) =
                            serde_json::to_string(&connection.sent_log_ids)
                        {
                            Set(Some(sent_log_ids))
                        } else {
                            Set(None)
                        };

                        let model = ActiveModel {
                            uuid: Set(connection.key().clone()),
                            sent_logs,
                        };

                        if let Err(error) = model.update(db_connection.as_ref()).await {
                            error!(target: "codectrl_server - background backup thread", "Error occurred while updating DB: {error}");
                        } else {
                            trace!(target: "codectrl_server - background backup thread", "Updated DB");
                            connection.last_update = Instant::now();
                        }
                    }
                }
            }
        });

        info!("... Done!");
    }

    fn strip_username_from_path(path: &str) -> Cow<str> {
        let path: Cow<str> = path.into();

        // We want to check for _each_ of the possible patterns where in which a
        // username may appear in a file path. Though this will not account for
        // _every single possibility_, it should account for the most common ones
        // - i.e. creating a log from a file that is in a directory under the
        // user account (depending on platform).
        let regexes = USERNAME_REGEXES.get_or_init(|| {
            [
                // To ensure compatibility, we must account for a few things:
                // 1. We need to account for [a-zA-Z] just in case of weird setups on
                // Windows.
                //
                // 2. We also account for single backslash paths for Windows in case the
                // logger already accounts for reformatting the directory paths.
                //
                // 3. We need to account for spaces in usernames on Windows systems.
                Regex::new(r"[a-zA-Z]:\\Users\\([a-zA-Z0-9\s]*)\\*"),
                Regex::new(r"[a-zA-Z]:\Users\([a-zA-Z0-9\s]*)\*"),
                Regex::new(r"/Users/([a-zA-Z0-9_\-\s]*)/*"),
                Regex::new(r"/home/([a-zA-Z0-9_\-\s]*)/.*"),
            ]
        });

        regexes
            .iter()
            .filter_map(|regex| regex.as_ref().ok())
            .filter_map(|pattern| pattern.captures(&path))
            .filter_map(|captures| captures.get(1))
            .map(|capture| path.clone().replace(capture.as_str(), "<USERNAME>"))
            .next()
            .unwrap_or(path.to_string())
            .into()
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn verify_log(
        log: &mut Log,
        remote_addr: Option<SocketAddr>,
        metadata: &MetadataMap,
    ) {
        log.uuid = Uuid::new_v4().hyphenated().to_string();

        if log.message.len() > 1000 {
            log.warnings.push("Message exceeds 1000 characters".into());
        }

        if log.message.is_empty() {
            log.warnings.push("No message was given".into());
            log.message = "<None>".into();
        }

        if log.message_type.is_empty() {
            log.warnings.push("Message type was not supplied".into());
        }

        if log.stack.is_empty() {
            log.warnings.push("Stacktrace is empty".into());
        }

        if log.file_name.is_empty() {
            log.warnings.push("No file name found".into());
            log.file_name = "<None>".into();
        }

        if let Some(censor_usernames) = CENSOR_USERNAMES.get() {
            if censor_usernames {
                log.file_name =
                    Self::strip_username_from_path(&log.file_name).to_string();

                log.stack.iter_mut().for_each(|stack| {
                    stack.file_path =
                        Self::strip_username_from_path(&stack.file_path).to_string();
                });
            }
        }

        match metadata.get("x-host") {
            Some(host) if matches!(remote_addr, Some(_)) =>
                if let Ok(host) = host.to_str() {
                    log.address = format!("{host}:{}", remote_addr.unwrap().port());
                } else {
                    log.address = remote_addr.unwrap().to_string();
                },
            Some(host) =>
                if let Ok(host) = host.to_str() {
                    log.address = host.to_string();
                },
            None if matches!(remote_addr, Some(_)) =>
                log.address = remote_addr.unwrap().to_string(),

            None => log.address = "Unknown".into(),
        }
    }

    pub fn requires_authentication(&mut self, requires_authentication: bool) {
        self.requires_authentication = requires_authentication;
    }
}

// region: server-to-client implementation

#[tonic::async_trait]
impl LogServerTrait for Service {
    async fn register_client(
        &self,
        req: Request<()>,
    ) -> Result<Response<Connection>, Status> {
        let connection = Connection::new();

        self.connections
            .write()
            .await
            .insert(connection.uuid.clone(), ConnectionState::default());

        let model = ActiveModel {
            uuid: Set(connection.uuid.clone()),
            sent_logs: NotSet,
        };

        if let Err(error) = model.insert(self.db_connection.as_ref()).await {
            return Err(Status::aborted(error.to_string()));
        };

        info!(
            "Registered new connection: {} to {}",
            &connection.uuid,
            req.remote_addr().unwrap()
        );

        Ok(Response::new(connection))
    }

    async fn register_existing_client(
        &self,
        connection: Request<Connection>,
    ) -> Result<Response<RequestResult>, Status> {
        let remote_addr = connection.remote_addr().unwrap();
        let connection = connection.into_inner();

        let connections = match Entity::find_by_id(connection.uuid)
            .all(self.db_connection.as_ref())
            .await
        {
            Ok(connections) => connections,
            Err(error) => return Err(Status::aborted(error.to_string())),
        };

        if connections.is_empty() {
            return Err(Status::not_found(
                "Given connection UUID was not found in database",
            ));
        }

        let connection = connections[0].clone();

        if let Some(sent_log_ids) = connection.sent_logs.as_ref() {
            let sent_log_ids: DashSet<String> = match serde_json::from_str(sent_log_ids) {
                Ok(ids) => ids,
                Err(error) => return Err(Status::internal(error.to_string())),
            };

            self.connections.write().await.insert(
                connection.uuid.clone(),
                ConnectionState {
                    last_update: Instant::now(),
                    sent_log_ids,
                },
            );
        }

        let req_result = RequestResult {
            message: "Re-registration succeeded!".to_string(),
            status: RequestStatus::Confirmed.into(),
            auth_status: None,
        };

        info!(
            "Re-registered a connection: {} to {}",
            &connection.uuid, remote_addr
        );

        Ok(Response::new(req_result))
    }

    async fn get_server_details(
        &self,
        req: Request<()>,
    ) -> Result<Response<ServerDetails>, Status> {
        let host = std::env::var("HOST").unwrap_or_else(|_| self.host.clone());

        let response = Response::new(ServerDetails {
            host,
            port: self.port,
            uptime: self.uptime.elapsed().as_secs(),
            requires_authentication: self.requires_authentication,
        });

        trace!("{} requested server details", req.remote_addr().unwrap());

        Ok(response)
    }

    async fn get_log(
        &self,
        connection: Request<Connection>,
    ) -> Result<Response<Log>, Status> {
        let remote_addr = connection.remote_addr().unwrap();
        let connection = connection.into_inner();

        if Uuid::try_parse(&connection.uuid).is_err() {
            return Err(Status::unauthenticated("No valid Connection was supplied."));
        }

        if !self.connections.read().await.contains_key(&connection.uuid) {
            return Err(Status::unauthenticated(
                "Invalid connection, please register.",
            ));
        }

        let mut ignore = DashSet::new();

        if self.connections.read().await.contains_key(&connection.uuid) {
            ignore = self
                .connections
                .read()
                .await
                .get(&connection.uuid)
                .unwrap()
                .clone()
                .sent_log_ids;
        }

        let logs = self.logs.read().await.clone();
        let mut logs = logs
            .iter()
            .cloned()
            .filter(|log| !ignore.contains(&log.uuid))
            .collect::<VecDeque<_>>();

        if let Some(log) = logs.pop_front() {
            if !ignore.contains(&log.uuid) {
                let key = self.connections.write().await;
                let key = key.get_mut(&connection.uuid);

                if let Some(mut key) = key {
                    key.add_log(&log);
                }

                trace!("{} requested one log and received new log", remote_addr);

                return Ok(Response::new(log));
            }
        }

        Err(Status::new(Code::ResourceExhausted, "No more logs"))
    }

    type GetLogsStream = ReceiverStream<Result<Log, Status>>;

    async fn get_logs(
        &self,
        connection: Request<Connection>,
    ) -> Result<Response<Self::GetLogsStream>, Status> {
        let remote_addr = connection.remote_addr().unwrap();
        let (tx, rx) = mpsc::channel(1024);
        let connection = connection.into_inner();

        if Uuid::try_parse(&connection.uuid).is_err() {
            return Err(Status::unauthenticated("No valid Connection was supplied."));
        }

        if !self.connections.read().await.contains_key(&connection.uuid) {
            return Err(Status::unauthenticated(
                "Invalid connection, please register.",
            ));
        }

        let connections = Arc::clone(&self.connections);

        let mut ignore = DashSet::new();

        if connections.read().await.contains_key(&connection.uuid) {
            ignore = connections
                .read()
                .await
                .get(&connection.uuid)
                .unwrap()
                .clone()
                .sent_log_ids;
        }

        let logs = self.logs.read().await.clone();
        let mut logs = logs
            .iter()
            .cloned()
            .filter(|log| !ignore.contains(&log.uuid))
            .collect::<VecDeque<_>>();

        let log_amount = logs.len();

        tokio::spawn(async move {
            let key = connections.write().await;
            let mut key = key.get_mut(&connection.uuid);

            while let Some(log) = logs.pop_front() {
                if !ignore.contains(&log.uuid) {
                    if let Err(e) = tx.send(Ok(log.clone())).await {
                        error!("Occurred when writing to channel: {e:?}");
                    } else if let Some(key) = key.as_mut() {
                        key.add_log(&log);
                    }
                }
            }
        });

        trace!(
            "{} requested log stream and will recieve new {} log(s)",
            remote_addr,
            log_amount
        );

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

// endregion
// region: client-to-server implementation

#[tonic::async_trait]
impl LogClientTrait for Service {
    async fn send_log(
        &self,
        request: Request<Log>,
    ) -> Result<Response<RequestResult>, Status> {
        let remote_addr = request.remote_addr();
        let metadata = request.metadata().clone();
        let mut log = request.into_inner();

        Self::verify_log(&mut log, remote_addr, &metadata);

        self.logs.write().await.push_back(log);

        info!("Log received from {}", remote_addr.unwrap());

        Ok(Response::new(RequestResult {
            message: "Log added!".into(),
            status: RequestStatus::Confirmed.into(),
            auth_status: None,
        }))
    }

    async fn send_logs(
        &self,
        request: Request<Streaming<Log>>,
    ) -> Result<Response<RequestResult>, Status> {
        let remote_addr = request.remote_addr();
        let metadata = request.metadata().clone();
        let mut stream = request.into_inner();

        let mut lock = self.logs.write().await;

        let mut amount = 0;
        while let Some(log) = stream.next().await {
            let mut log = log?;

            Self::verify_log(&mut log, remote_addr, &metadata);
            lock.push_back(log);

            amount += 1;
        }

        info!("{amount} log(s) received from {}", remote_addr.unwrap());

        Ok(Response::new(RequestResult {
            message: format!("{amount} logs added!"),
            status: RequestStatus::Confirmed.into(),
            auth_status: None,
        }))
    }
}

// endregion
// region: oauth implementation

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TokenClaims {
    #[serde(rename = "iat")]
    issued_at: usize,
    #[serde(rename = "iss")]
    issuer: String,
    #[serde(rename = "exp")]
    expire: usize,
    #[serde(rename = "sub")]
    subject: String,
    #[serde(rename = "nbf")]
    not_before: usize,
}

#[tonic::async_trait]
impl Authentication for Service {
    async fn verify_token(
        &self,
        _request: Request<VerifyTokenRequest>,
    ) -> Result<Response<VerifyTokenRequestResult>, Status> {
        todo!()
    }

    async fn generate_token(
        &self,
        _request: Request<GenerateTokenRequest>,
    ) -> Result<Response<GenerateTokenRequestResult>, Status> {
        todo!()
    }

    async fn revoke_token(
        &self,
        _request: Request<Token>,
    ) -> Result<Response<RevokeTokenRequestResult>, Status> {
        todo!()
    }

    async fn refresh_token(
        &self,
        _request: Request<Token>,
    ) -> Result<Response<Token>, Status> {
        todo!()
    }

    async fn github_login(&self, _: Request<()>) -> Result<Response<LoginUrl>, Status> {
        Ok(Response::new(LoginUrl {
            url: generate_github_login_url(),
        }))
    }
}

fn generate_github_login_url() -> String {
    let github_client_id = ClientId::new(
        env::var("GITHUB_CLIENT_ID")
            .expect("Missing the GITHUB_CLIENT_ID environment variable."),
    );
    let github_client_secret = ClientSecret::new(
        env::var("GITHUB_CLIENT_SECRET")
            .expect("Missing the GITHUB_CLIENT_SECRET environment variable."),
    );
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url =
        TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
            .expect("Invalid token endpoint URL");

    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new(format!(
            "https://localhost:{}",
            REDIRECT_HANDLER_PORT.get_or_init(|| 8080)
        ))
        .expect("Invalid redirect URL"),
    );

    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    authorize_url.to_string()
}

fn generate_token() -> String {
    let mut rng = thread_rng();
    let secret = Alphanumeric.sample_string(&mut rng, 50);

    info!("Auto-generated token secret (DO NOT SHARE!): {secret}");

    secret
}

// endregion

// endregion

pub type ServerResult = anyhow::Result<mpsc::UnboundedReceiver<anyhow::Error>>;

/// Runs the `gRPC` server to be used by the GUI or the standalone binary.
///
/// # Errors
///
/// This function could error under the following circumstances:
///
/// 1. Supplied host was taken or invalid.
/// 2. Supplied port was taken or invalid.
/// 3. The inner tonic server returns an error during runtime.
#[allow(clippy::missing_panics_doc)]
pub async fn run_server(
    host: Option<String>,
    port: Option<u32>,
    requires_authentication: Option<bool>,
    redirect_handler_port: Option<u16>,
) -> ServerResult {
    dotenv().ok();
    env_logger::try_init().ok();

    if let Ok(value) = env::var("CENSOR_USERNAMES") {
        if let Ok(value) = value.parse::<u8>() {
            if value == 0 {
                CENSOR_USERNAMES.get_or_init(|| false);
            }
        }
        CENSOR_USERNAMES.get_or_init(|| true);
    } else {
        CENSOR_USERNAMES.get_or_init(|| true);
    }

    let _token_secret = if let Ok(secret) = env::var("TOKEN_SECRET") {
        if secret.is_empty() {
            warn!("TOKEN_SECRET was found but was empty!");
            generate_token()
        } else {
            warn!("TOKEN_SECRET wasn't found in the environment variables!");
            secret
        }
    } else {
        generate_token()
    };

    let data_dir = if let Some(data_directory) =
        ProjectDirs::from("com", "Authentura", "codectrl-server")
    {
        data_directory.data_dir().to_owned()
    } else {
        Path::new(".codectrl-server").to_owned()
    };

    let requires_authentication =
        if let Some(requires_authentication) = requires_authentication {
            requires_authentication
        } else {
            false
        };

    let handler_port;
    if requires_authentication {
        handler_port = if let Some(port) = redirect_handler_port {
            port
        } else {
            8080
        };

        REDIRECT_HANDLER_PORT.get_or_init(|| handler_port);
    };

    info!(
        "Data directory for CodeCTRL: {}",
        data_dir.to_string_lossy()
    );

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
        info!("Created {}", data_dir.to_string_lossy());
    }

    let data_dir = data_dir.to_string_lossy().to_string();
    let db_file = format!("{data_dir}/db.sqlite");

    // If the DB file does not exist or is completely empty, then create and create
    // the necessary table.
    if !Path::new(&db_file).exists() || File::open(&db_file)?.metadata()?.len() == 0 {
        File::create(&db_file)?;

        let db_connection = Database::connect(format!("sqlite:{db_file}")).await?;

        let backend = db_connection.get_database_backend();
        let schema = Schema::new(backend);
        let statement = backend.build(&schema.create_table_from_entity(Entity));

        info!("Creating initial SQLite database");

        db_connection.execute(statement).await?;
    }

    let db_connection = Database::connect(format!("sqlite:{db_file}")).await?;

    let host = if host.is_some() {
        host.unwrap()
    } else {
        String::from("127.0.0.1")
    };
    let port = if port.is_some() { port.unwrap() } else { 3002 };

    let logs = Arc::new(RwLock::new(VecDeque::new()));

    let logs_service = Service {
        host: host.clone(),
        port,
        uptime: Instant::now(),
        logs: Arc::clone(&logs),
        connections: Arc::new(RwLock::new(DashMap::new())),
        db_connection: Arc::new(db_connection),
        requires_authentication,
    };

    logs_service.start_backup_thread();

    let server_service = LogServerService::new(logs_service.clone());
    let client_service = LogClientService::new(logs_service.clone());
    let auth_service = AuthenticationServer::new(logs_service);

    let grpc_addr = format!("{host}:{port}").parse()?;

    info!("Starting gPRC server on {grpc_addr}...");

    let (error_sender, error_receiver) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        Server::builder()
            .accept_http1(true)
            .add_service(tonic_web::enable(server_service))
            .add_service(tonic_web::enable(client_service))
            .add_service(tonic_web::enable(auth_service))
            .serve(grpc_addr)
            .await
            .map_err(|error| error_sender.send(error.into()))
    });

    Ok(error_receiver)
}
