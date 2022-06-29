use codectrl_protobuf_bindings::{
    data::Log,
    logs_service::{Empty, LogServerTrait, LogService as LogServer},
};
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Code, Request, Response, Status};

static LOGS: Lazy<RwLock<VecDeque<Log>>> = Lazy::new(|| RwLock::new(VecDeque::new()));

#[derive(Debug)]
pub struct Service;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let logs_service = Service;
    let service = LogServer::new(logs_service);
    let gprc_addr = "127.0.0.1:3002".parse()?;

    println!("Starting gPRC server on {gprc_addr}..");

    Server::builder()
        .add_service(service)
        .serve(gprc_addr)
        .await?;

    Ok(())
}
