#![cfg(not(target_arch = "wasm32"))]

use crate::{app::App, login::Login, GrpcClient};
use codectrl_protobuf_bindings::logs_service::Connection;
use std::{cell::RefCell, collections::HashMap, path::PathBuf, sync::Arc};
use tokio::runtime::Handle;

#[derive(Default, Debug, Clone)]
pub enum WrapperMsg {
    LogOut,
    LogIn,
    Main {
        grpc_client: GrpcClient,
        grpc_client_connection: Connection,
    },
    #[default]
    NoOp,
}

impl PartialEq for WrapperMsg {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (WrapperMsg::LogOut, WrapperMsg::LogOut)
                | (WrapperMsg::LogIn, WrapperMsg::LogIn)
                | (WrapperMsg::NoOp, WrapperMsg::NoOp)
        )
    }
}

pub struct Wrapper<'a> {
    state: HashMap<&'static str, Box<dyn eframe::App + 'a>>,
    selected_state: &'static str,
    msg: Arc<RefCell<WrapperMsg>>,
    handle: Arc<Handle>,

    preload_project: PathBuf,
}

impl<'a> Wrapper<'a> {
    pub fn new(handle: Handle, file_path: PathBuf) -> Self {
        let msg = Arc::new(RefCell::new(WrapperMsg::LogIn {}));

        Self {
            state: HashMap::new(),
            selected_state: "login",
            msg,
            handle: Arc::new(handle),
            preload_project: file_path,
        }
    }
}

impl<'a> eframe::App for Wrapper<'a> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let msg = self.msg.borrow().clone();

        match msg {
            WrapperMsg::LogOut =>
                if let Ok(mut msg) = self.msg.try_borrow_mut() {
                    self.state.clear();
                    *msg = WrapperMsg::LogIn;
                },
            WrapperMsg::LogIn => {
                self.selected_state = "login";

                if self.state.get("login").is_none() {
                    self.state.insert(
                        "login",
                        Box::new(Login::new(
                            ctx,
                            Arc::clone(&self.msg),
                            Arc::clone(&self.handle),
                        )),
                    );
                }
            },
            #[cfg(not(target_arch = "wasm32"))]
            WrapperMsg::Main {
                grpc_client,
                grpc_client_connection,
            } => {
                self.selected_state = "main";

                if self.state.get("main").is_none() {
                    let mut app = App::new(
                        ctx,
                        frame.storage(),
                        grpc_client,
                        grpc_client_connection,
                        Arc::clone(&self.msg),
                        &Arc::clone(&self.handle),
                    );

                    if self.preload_project.exists() {
                        if let Err(error) =
                            App::load_from_file(&self.preload_project, &mut app)
                        {
                            panic!("An error occurred: {error}");
                        }
                    }

                    self.state.clear();

                    self.state
                        .insert("main", Box::new(app) as Box<dyn eframe::App>);
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
