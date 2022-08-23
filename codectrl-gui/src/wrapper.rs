#[cfg(not(target_arch = "wasm32"))]
use crate::{app::App, login::Login};
use codectrl_protobuf_bindings::logs_service::{
    log_server_client::LogServerClient as Client, Connection,
};
use eframe::{CreationContext, Storage};
use std::{cell::RefCell, collections::HashMap, sync::Arc};
#[cfg(not(target_arch = "wasm32"))]
use tokio::runtime::Handle;
#[cfg(not(target_arch = "wasm32"))]
use tonic::transport::Channel;

#[cfg(not(target_arch = "wasm32"))]
type GrpcClient = Client<Channel>;

#[derive(Default, Debug, Clone)]
pub enum WrapperMsg<'a> {
    Login {},

    #[cfg(not(target_arch = "wasm32"))]
    Main {
        grpc_client: GrpcClient,
        grpc_client_connection: Connection,
        runtime: &'a Handle,
    },
    #[cfg(target_arch = "wasm32")]
    Main {
        grpc_client: GrpcClient,
        server_host: &'static str,
        server_port: &'static str,
    },
    #[default]
    NoOp,
}

pub struct Wrapper<'a> {
    state: HashMap<&'static str, Box<dyn eframe::App + 'a>>,
    selected_state: &'static str,
    msg: Arc<RefCell<WrapperMsg<'a>>>,
}

impl<'a> Wrapper<'a> {
    pub fn new() -> Self {
        let msg = Arc::new(RefCell::new(WrapperMsg::Login {}));

        Self {
            state: HashMap::new(),
            selected_state: "login",
            msg,
        }
    }
}

impl<'a> eframe::App for Wrapper<'a> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match &self.msg.borrow().clone() {
            WrapperMsg::Login { .. } => {
                self.selected_state = "login";

                if self.state.get("login").is_none() {
                    self.state.insert(
                        "login",
                        Box::new(Login::new(ctx, Arc::clone(&self.msg))),
                    );
                }
            },
            #[cfg(not(target_arch = "wasm32"))]
            WrapperMsg::Main {
                grpc_client,
                grpc_client_connection,
                runtime,
            } => {
                self.selected_state = "main";

                if self.state.get("main").is_none() {
                    self.state.insert(
                        "main",
                        Box::new(App::new(
                            ctx,
                            frame.storage(),
                            grpc_client.clone(),
                            grpc_client_connection.clone(),
                            runtime,
                        )) as Box<dyn eframe::App>,
                    );
                }
            },
            #[cfg(target_arch = "wasm32")]
            WrapperMsg::Main {
                grpc_client,
                server_host,
                server_port,
            } => {
                self.selected_state = "main";

                if self.state.get("main").is_none() {
                    self.state.insert(
                        "main",
                        Box::new(App::new(
                            ctx,
                            frame.storage(),
                            grpc_client,
                            server_host,
                            server_port,
                        )) as Box<dyn eframe::App>,
                    );
                }
            },

            WrapperMsg::NoOp => (),
        }

        if let Some(app) = self.state.get_mut(self.selected_state) {
            app.update(ctx, frame);
        }
    }
}
