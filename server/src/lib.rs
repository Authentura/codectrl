use codectrl_logger::Log;
use log::{error, warn};
use simple_logger::SimpleLogger;
use std::{
    error::Error,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc,
    },
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpSocket,
    runtime::Runtime,
};

#[derive(Clone)]
pub struct Server {
    sender: Arc<SyncSender<Log<String>>>,
    port: String,
}

impl Server {
    pub fn new(port: &str) -> (Self, Receiver<Log<String>>) {
        let (sender, receiver) = sync_channel(2);

        (
            Self {
                sender: Arc::new(sender),
                port: port.into(),
            },
            receiver,
        )
    }

    pub fn run_server(&mut self) -> Result<(), Box<dyn Error>> {
        let rt = Runtime::new()?;

        let mut ret = Ok(());

        rt.block_on(async {
            ret = self._run_server().await;
        });

        ret
    }

    async fn _run_server(&mut self) -> Result<(), Box<dyn Error>> {
        SimpleLogger::new().init()?;

        let socket = TcpSocket::new_v4()?;
        socket.set_reuseaddr(true)?;
        #[cfg(unix)]
        socket.set_reuseport(true)?; // If we're using a *NIX system, allow for multiple
                                     // instances of codeCTRL to use the same port.
                                     // *However*, this will cause some instances to
                                     // receive POST data but some others will not.

        socket.bind(format!("127.0.0.1:{}", self.port).parse().unwrap())?;

        let listener = socket.listen(1024)?;

        loop {
            let (mut socket, peer_address) = listener.accept().await?;

            let mut buf = Vec::with_capacity(2048);

            loop {
                let n = match socket.read_to_end(&mut buf).await {
                    Ok(n) if n == 0 => break,
                    Ok(n) => n,
                    Err(e) => {
                        error!(target: "log_server", "Failed to read from socket: {}", e);
                        break;
                    },
                };

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    error!(target: "log_server", "Failed to write response to socket: {:?}", e);
                    break;
                }

                let mut data: Log<String> = match serde_cbor::from_reader(&buf[..n]) {
                    Ok(data) => data,
                    Err(e) => {
                        error!(target: "log_server", "{}", e);
                        break;
                    },
                };

                if data.message.len() > 1000 {
                    warn!(target: "log_server",
                        "Log message is quite long: max recommended characters is 1000, \
                         log had {}",
                        data.message.len()
                    );

                    data.warnings.push("Message exceeds 1000 characters".into());
                }

                if data.message.is_empty() {
                    data.warnings.push("No message was given".into());
                    data.message = "<None>".into();
                }

                if data.message_type.is_empty() {
                    data.warnings.push("Message type was not supplied".into());
                }

                if data.stack.is_empty() {
                    data.warnings.push("Stacktrace is empty".into());
                }

                if data.file_name.is_empty() {
                    data.warnings.push("No file name found".into());
                    data.file_name = "<None>".into();
                }

                data.address = peer_address.to_string().split(':').collect::<Vec<_>>()[0]
                    .to_string();

                if let Err(e) = self.sender.send(data) {
                    error!(target: "log_server", "Failed to send data to main channel: {}", e);
                    break;
                }
            }
        }
    }
}
