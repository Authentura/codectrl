#![warn(clippy::pedantic)]

use codectrl_protobuf_bindings::{
    auth_service::{
        // authentication_client::AuthenticationClient,
        authentication_server::{Authentication, AuthenticationServer},
        GenerateTokenRequest,
        GenerateTokenRequestResult,
        RevokeTokenRequestResult,
        Token,
        VerifyTokenRequest,
        VerifyTokenRequestResult,
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
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ConnectionTrait, Database, DatabaseConnection,
    EntityTrait, Schema, Set,
};
use serde::{Deserialize, Serialize};
use std::{
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

mod entity;

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

        if let Some(sent_log_ids) = &connection.sent_logs {
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
}

fn generate_token() -> String {
    let mut rng = thread_rng();
    let secret = Alphanumeric.sample_string(&mut rng, 50);

    info!("Auto-generated token secret (DO NOT SHARE!): {secret}");

    secret
}

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
    run_legacy_server: Option<bool>,
    host: Option<String>,
    port: Option<u32>,
    requires_authentication: Option<bool>,
) -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::try_init().ok();

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

    // TODO: Add the legacy server thread and manage it through the gPRC server.
    let run_legacy_server = if run_legacy_server.is_some() {
        run_legacy_server.unwrap()
    } else {
        false
    };
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

    if run_legacy_server {
        info!("Legacy server compatiblity not yet implemented");
    }

    let server_service = LogServerService::new(logs_service.clone());
    let client_service = LogClientService::new(logs_service.clone());
    let auth_service = AuthenticationServer::new(logs_service);

    let grpc_addr = format!("{host}:{port}").parse()?;

    info!("Starting gPRC server on {grpc_addr}...");

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(server_service))
        .add_service(tonic_web::enable(client_service))
        .add_service(tonic_web::enable(auth_service))
        .serve(grpc_addr)
        .await?;

    Ok(())
}
