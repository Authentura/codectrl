#![warn(clippy::pedantic)]

use codectrl_protobuf_bindings::{
    data::Log,
    logs_service::{
        Empty, LogClientService, LogClientTrait, LogServerService, LogServerTrait,
        Received, ReceivedResult,
    },
};
use futures::StreamExt;
use once_cell::sync::Lazy;
use std::{collections::VecDeque, net::SocketAddr};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    metadata::MetadataMap, transport::Server, Code, Request, Response, Status, Streaming,
};

// TODO: convert to Arc<_> in `run_server`
static LOGS: Lazy<RwLock<VecDeque<Log>>> = Lazy::new(|| RwLock::new(VecDeque::new()));

#[derive(Debug, Copy, Clone)]
pub struct Service;

impl Service {
    #[allow(clippy::missing_panics_doc)]
    pub fn verify_log(
        log: &mut Log,
        remote_addr: Option<SocketAddr>,
        metadata: &MetadataMap,
    ) {
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
                    log.address = host.to_string();
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
}

#[tonic::async_trait]
impl LogServerTrait for Service {
    async fn get_log(&self, _: Request<Empty>) -> Result<Response<Log>, Status> {
        if let Some(log) = LOGS.write().await.pop_front() {
            return Ok(Response::new(log));
        }

        Err(Status::new(Code::ResourceExhausted, "No more logs"))
    }

    type GetLogsStream = ReceiverStream<Result<Log, Status>>;

    async fn get_logs(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<Self::GetLogsStream>, Status> {
        let (tx, rx) = mpsc::channel(1024);

        tokio::spawn(async move {
            while let Some(log) = LOGS.write().await.pop_front() {
                if let Err(e) = tx.send(Ok(log.clone())).await {
                    if cfg!(debug_assertions) {
                        eprintln!("[ERROR] Occurred when writing to channel: {e:?}");
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tonic::async_trait]
impl LogClientTrait for Service {
    async fn send_log(
        &self,
        request: Request<Log>,
    ) -> Result<Response<ReceivedResult>, Status> {
        let remote_addr = request.remote_addr();
        let metadata = request.metadata().clone();
        let mut log = request.into_inner();

        Self::verify_log(&mut log, remote_addr, &metadata);

        LOGS.write().await.push_back(log);

        Ok(Response::new(ReceivedResult {
            message: "Log added!".into(),
            status: Received::Confirmed.into(),
        }))
    }

    async fn send_logs(
        &self,
        request: Request<Streaming<Log>>,
    ) -> Result<Response<ReceivedResult>, Status> {
        let remote_addr = request.remote_addr();
        let metadata = request.metadata().clone();
        let mut stream = request.into_inner();

        let mut lock = LOGS.write().await;

        let mut amount = 0;
        while let Some(log) = stream.next().await {
            let mut log = log?;

            Self::verify_log(&mut log, remote_addr, &metadata);
            lock.push_back(log);

            amount += 1;
        }

        Ok(Response::new(ReceivedResult {
            message: format!("{amount} logs added!"),
            status: Received::Confirmed.into(),
        }))
    }
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
    host: Option<&str>,
    port: Option<u32>,
) -> anyhow::Result<()> {
    // TODO: Add the legacy server thread and manage it through the gPRC server.
    let _run_legacy_server = if run_legacy_server.is_some() {
        run_legacy_server.unwrap()
    } else {
        false
    };
    let host = if host.is_some() {
        host.unwrap()
    } else {
        "127.0.0.1"
    };
    let port = if port.is_some() { port.unwrap() } else { 3002 };

    let logs_service = Service;
    let server_service = LogServerService::new(logs_service);
    let client_service = LogClientService::new(logs_service);

    let gprc_addr = format!("{host}:{port}").parse()?;

    println!("Starting gPRC server on {gprc_addr}...");

    Server::builder()
        .add_service(server_service)
        .add_service(client_service)
        .serve(gprc_addr)
        .await?;

    Ok(())
}
