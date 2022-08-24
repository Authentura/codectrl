use crate::wrapper::WrapperMsg;

use authentura_egui_styling::{application_style, fonts, FontSizes};
use codectrl_protobuf_bindings::logs_service::{
    log_server_client::LogServerClient, Connection,
};
use eframe::{App, Frame};
use egui::{Color32, Context, Vec2};
use poll_promise::Promise;
#[cfg(not(target_arch = "wasm32"))]
use std::{cell::RefCell, sync::Arc};
#[cfg(not(target_arch = "wasm32"))]
use tokio::runtime::Handle;
#[cfg(not(target_arch = "wasm32"))]
use tonic::transport::Channel;

#[derive(Default)]
pub struct Login {
    token: String,
    wrapper_msg: Arc<RefCell<WrapperMsg>>,
    host: String,
    port: String,
    is_local: bool,
    handle: Option<Arc<Handle>>,
    connection_promise: Option<Promise<LogServerClient<Channel>>>,
    registration_promise: Option<Promise<Connection>>,
}

impl Login {
    pub fn new(
        ctx: &Context,
        wrapper_msg: Arc<RefCell<WrapperMsg>>,
        handle: Arc<Handle>,
    ) -> Self {
        ctx.set_fonts(fonts());
        ctx.set_style(application_style(FontSizes::default()));

        Self {
            token: String::new(),
            wrapper_msg,
            host: String::from("127.0.0.1"),
            port: String::from("3002"),
            is_local: true,
            handle: Some(handle),
            connection_promise: None,
            registration_promise: None,
        }
    }
}

impl App for Login {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        egui::TopBottomPanel::top("top_bar")
            .resizable(false)
            .default_height(200.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);

                ui.menu_button("File", |ui| {
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Quit").clicked() {
                            frame.quit();
                        }
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("login_form")
                .min_col_width(ui.available_width() / 2.0)
                .spacing(Vec2::new(10.0, 10.0))
                .show(ui, |ui| {
                    ui.checkbox(&mut self.is_local, "Is local?");
                    ui.end_row();

                    let responsive_row =
                        |ui: &mut egui::Ui, text: &str, data: &mut String| {
                            ui.colored_label(
                                if data.is_empty() {
                                    Color32::LIGHT_RED
                                } else {
                                    ctx.style().visuals.text_color()
                                },
                                text,
                            );

                            ui.text_edit_singleline(data);
                            ui.end_row();
                        };

                    if !self.is_local {
                        responsive_row(ui, "Token", &mut self.token);
                        responsive_row(ui, "Host", &mut self.host);
                    }

                    responsive_row(ui, "Port", &mut self.port);
                });

            ui.add_space(5.0);

            if ui
                .button(if self.is_local { "Start" } else { "Login" })
                .clicked()
            {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    self.connection_promise.get_or_insert_with(|| {
                        let (sender, promise) = Promise::new();

                        let host = self.host.clone();
                        let port = self.port.clone();

                        if let Some(handle) = self.handle.as_deref() {
                            handle.spawn(async move {
                                let grpc_client = loop {
                                    let res = LogServerClient::connect(format!(
                                        "http://{host}:{port}"
                                    ))
                                    .await;

                                    if let Ok(res) = res {
                                        break res;
                                    }
                                };

                                sender.send(grpc_client);
                            });
                        }

                        promise
                    });
                }
            }

            if let Some(connection_promise) = &mut self.connection_promise {
                match connection_promise.ready_mut() {
                    None => ui.spinner(),
                    Some(channel) => {
                        let mut channel_clone = channel.clone();
                        self.registration_promise.get_or_insert_with(|| {
                            let (sender, promise) = Promise::new();

                            if let Some(handle) = self.handle.as_deref() {
                                handle.spawn(async move {
                                    if let Ok(registered_client) =
                                        channel_clone.register_client(()).await
                                    {
                                        sender.send(registered_client.into_inner());
                                    }
                                });
                            }

                            promise
                        });

                        if let Some(registration_promise) = &self.registration_promise {
                            match registration_promise.ready() {
                                Some(registered_client) => {
                                    if let Ok(mut wrapper_msg) =
                                        self.wrapper_msg.try_borrow_mut()
                                    {
                                        *wrapper_msg = WrapperMsg::Main {
                                            grpc_client: channel.clone(),
                                            grpc_client_connection: registered_client
                                                .clone(),
                                        };
                                    }

                                    ui.label(format!(
                                        "Registered client: {}",
                                        registered_client.uuid
                                    ))
                                },
                                None => ui.spinner(),
                            }
                        } else {
                            ui.spinner()
                        }
                    },
                };
            }
        });
    }
}
