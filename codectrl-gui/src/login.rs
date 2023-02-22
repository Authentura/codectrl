#![cfg(not(target_arch = "wasm32"))]

// region: imports

use crate::{widgets::CopyableLabel, wrapper::WrapperMsg};
use authentura_egui_styling::{application_style, fonts, FontSizes};
use codectrl_protobuf_bindings::{
    auth_service::{authentication_client::AuthenticationClient, LoginUrl},
    logs_service::{log_server_client::LogServerClient, Connection, ServerDetails},
};
use eframe::{App, Frame};
use egui::{
    Button, CentralPanel, Color32, Context, Grid, Pos2, Response, TopBottomPanel, Ui,
    Vec2, Window,
};
use once_cell::race::OnceBool;
use poll_promise::Promise;
use std::{
    cell::RefCell,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{runtime::Handle, task::JoinHandle};
use tonic::transport::Channel;

// endregion

static GITHUB_BUTTON_HAS_BEEN_CLICKED: OnceBool = OnceBool::new();

#[derive(Default)]
pub struct Login {
    token: String,
    wrapper_msg: Arc<RefCell<WrapperMsg>>,
    host: String,
    port: String,
    is_local: bool,
    handle: Option<Arc<Handle>>,
    connection_promise: Option<(
        Promise<(LogServerClient<Channel>, AuthenticationClient<Channel>)>,
        JoinHandle<()>,
    )>,
    server_details_promise: Option<Promise<ServerDetails>>,
    registration_promise: Option<Promise<Connection>>,
    github_login_url_promise: Option<Promise<LoginUrl>>,
    connection_promise_initialised: Option<Instant>,
    reset_connection: bool,
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
            server_details_promise: None,
            registration_promise: None,
            connection_promise_initialised: None,
            github_login_url_promise: None,
            reset_connection: false,
        }
    }

    fn register(&mut self, mut channel: LogServerClient<Channel>) {
        self.registration_promise.get_or_insert_with(|| {
            let (sender, promise) = Promise::new();

            if let Some(handle) = self.handle.as_deref() {
                handle.spawn(async move {
                    if let Ok(registered_client) = channel.register_client(()).await {
                        sender.send(registered_client.into_inner());
                    }
                });
            }

            promise
        });
    }

    fn draw_token_window(
        &mut self,
        ctx: &Context,
        frame: &mut Frame,
        (channel, auth_channel): &(
            LogServerClient<Channel>,
            AuthenticationClient<Channel>,
        ),
    ) -> Response {
        let window_size = frame.info().window_info.size;
        let mut auth_channel = auth_channel.clone();

        Window::new("token_input")
            .title_bar(false)
            .auto_sized()
            .fixed_pos(Pos2::new(
                (window_size.x / 2.0) - 125.0,
                (window_size.y / 2.0) - 100.0,
            ))
            .show(ctx, |ui| {
                ui.heading("Please login with your GitHub");

                if ui.button("GitHub Login").clicked()
                    || matches!(GITHUB_BUTTON_HAS_BEEN_CLICKED.get(), Some(true))
                {
                    GITHUB_BUTTON_HAS_BEEN_CLICKED.get_or_init(|| true);

                    if self.github_login_url_promise.is_none() {
                        let (sender, promise) = Promise::new();

                        if let Some(handle) = self.handle.as_deref() {
                            handle.spawn(async move {
                                if let Ok(result) = auth_channel.github_login(()).await {
                                    sender.send(result.into_inner());
                                }
                            });
                        }

                        self.github_login_url_promise = Some(promise);
                    }

                    if let Some(login_url_promise) = &self.github_login_url_promise {
                        match login_url_promise.ready() {
                            Some(login_url) => {
                                let _res = open::that(&login_url.url);
                                ui.spinner()
                            },
                            None => ui.spinner(),
                        };
                    }

                    // open::that(path);
                }

                ui.add_space(5.0);

                if ui
                    .add_enabled(!self.token.is_empty(), Button::new("Login"))
                    .clicked()
                {
                    self.register(channel.clone());
                }

                if let Some(registration_promise) = &self.registration_promise {
                    match registration_promise.ready() {
                        Some(registered_client) => {
                            if let Ok(mut wrapper_msg) = self.wrapper_msg.try_borrow_mut()
                            {
                                *wrapper_msg = WrapperMsg::Main {
                                    grpc_client: channel.clone(),
                                    grpc_client_connection: registered_client.clone(),
                                };
                            }

                            ui.add(CopyableLabel::new(format!(
                                "Registered client: {}",
                                registered_client.uuid
                            )))
                        },
                        None => ui.spinner(),
                    }
                } else {
                    ui.label("")
                }
            })
            .unwrap()
            .response
    }
}

fn responsive_row(ctx: &Context, ui: &mut Ui, text: &str, data: &mut String) {
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
}

impl App for Login {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        TopBottomPanel::top("top_bar")
            .resizable(false)
            .default_height(200.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);

                ui.menu_button("File", |ui| {
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("Quit").clicked() {
                            frame.close();
                        }
                    });
                });
            });

        CentralPanel::default().show(ctx, |ui| {
            Grid::new("login_form")
                .min_col_width(ui.available_width() / 2.0)
                .spacing(Vec2::new(10.0, 10.0))
                .show(ui, |ui| {
                    ui.checkbox(&mut self.is_local, "Is local?");
                    ui.end_row();

                    if !self.is_local {
                        responsive_row(ctx, ui, "Host", &mut self.host);
                    }

                    responsive_row(ctx, ui, "Port", &mut self.port);
                });

            ui.add_space(5.0);

            if ui
                .button(if self.is_local { "Start" } else { "Login" })
                .clicked()
            {
                {
                    let fun = || {
                        let (sender, promise) = Promise::new();

                        let host = self.host.clone();
                        let port = self.port.clone();

                        let promise_handle = if let Some(handle) = self.handle.as_deref()
                        {
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

                                let auth_client = loop {
                                    let res = AuthenticationClient::connect(format!(
                                        "http://{host}:{port}"
                                    ))
                                    .await;

                                    if let Ok(res) = res {
                                        break res;
                                    }
                                };

                                sender.send((grpc_client, auth_client));
                            })
                        } else {
                            panic!("No tokio runtime!")
                        };

                        (promise, promise_handle)
                    };

                    if self.reset_connection {
                        self.connection_promise.replace(fun());
                        self.reset_connection = false;
                    } else {
                        self.connection_promise.get_or_insert_with(fun);
                    }

                    self.connection_promise_initialised = Some(Instant::now());
                }
            }

            if let Some(connection_promise) = &mut self.connection_promise {
                match connection_promise.0.ready_mut() {
                    None => {
                        if let Some(promise_initialised) =
                            self.connection_promise_initialised
                        {
                            if promise_initialised.elapsed() > Duration::new(10, 0) {
                                connection_promise.1.abort();
                                self.reset_connection = true;
                                ui.colored_label(
                                    Color32::RED,
                                    "Could not connect to gRPC server: timed out after \
                                     10s",
                                )
                            } else {
                                ui.spinner()
                            }
                        } else {
                            ui.spinner()
                        }
                    },
                    Some(channel) => {
                        let mut channel_clone = channel.clone();
                        self.server_details_promise.get_or_insert_with(|| {
                            let (sender, promise) = Promise::new();

                            if let Some(handle) = self.handle.as_deref() {
                                handle.spawn(async move {
                                    if let Ok(server_details) =
                                        channel_clone.0.get_server_details(()).await
                                    {
                                        sender.send(server_details.into_inner());
                                    }
                                });
                            }

                            promise
                        });

                        let channel_clone = channel.clone();
                        if let Some(server_details_promise) = &self.server_details_promise
                        {
                            match server_details_promise.ready() {
                                Some(server_details) =>
                                    if server_details.requires_authentication {
                                        self.draw_token_window(ctx, frame, &channel_clone)
                                    } else {
                                        self.register(channel_clone.clone().0);
                                        ui.label("")
                                    },
                                None => ui.colored_label(
                                    Color32::LIGHT_RED,
                                    "Could not fetch server details!",
                                ),
                            }
                        } else {
                            ui.spinner()
                        };

                        if let Some(registration_promise) = &self.registration_promise {
                            match registration_promise.ready() {
                                Some(registered_client) => {
                                    if let Ok(mut wrapper_msg) =
                                        self.wrapper_msg.try_borrow_mut()
                                    {
                                        *wrapper_msg = WrapperMsg::Main {
                                            grpc_client: channel_clone.0,
                                            grpc_client_connection: registered_client
                                                .clone(),
                                        };
                                    }

                                    ui.add(CopyableLabel::new(format!(
                                        "Registered client: {}",
                                        registered_client.uuid
                                    )))
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
