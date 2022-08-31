#[cfg(target_arch = "wasm32")]
use crate::data::Received;
use crate::{
    components::{about_view, details_view, main_view, main_view_empty, settings_view},
    data::{AppState, Filter},
    wrapper::WrapperMsg,
};
#[cfg(not(target_arch = "wasm32"))]
use log::info;
#[cfg(target_arch = "wasm32")]
use tracing::info;

#[cfg(not(target_arch = "wasm32"))]
use authentura_egui_styling::FontSizes;
use authentura_egui_styling::{application_style, fonts};
use chrono::{DateTime, Local};
use ciborium::de as ciborium_de;
#[cfg(not(target_arch = "wasm32"))]
use ciborium::ser as ciborium_ser;
use codectrl_protobuf_bindings::{
    data::Log,
    logs_service::{
        log_server_client::LogServerClient as Client, Connection, ServerDetails,
    },
};
use eframe::{Frame, Storage};
use egui::{Context, Vec2};
#[cfg(not(target_arch = "wasm32"))]
use egui::{Event, InputState, Key};
use flate2::bufread;
#[cfg(not(target_arch = "wasm32"))]
use flate2::{write, Compression};
#[cfg(target_arch = "wasm32")]
use futures_channel::oneshot::{channel, Receiver};
#[cfg(target_arch = "wasm32")]
use grpc_web_client::Client as WasmClient;
#[cfg(not(target_arch = "wasm32"))]
use poll_promise::Promise;
#[cfg(target_arch = "wasm32")]
use rfd::{AsyncFileDialog as FileDialog, FileHandle, MessageDialog};
#[cfg(not(target_arch = "wasm32"))]
use rfd::{FileDialog, MessageDialog};
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use std::sync::Mutex;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
use std::{
    cell::RefCell,
    collections::{BTreeSet, VecDeque},
    error::Error,
    io::{BufReader, Error as IOError, ErrorKind},
    sync::Arc,
};
#[cfg(not(target_arch = "wasm32"))]
use std::{fs::File, io::Write, path::Path};
#[cfg(not(target_arch = "wasm32"))]
use tokio::runtime::Handle;
#[cfg(not(target_arch = "wasm32"))]
use tonic::transport::Channel;
#[cfg(not(target_arch = "wasm32"))]
use tonic::{Response, Status};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;
#[cfg(target_arch = "wasm32")]
use wasm_rs_async_executor::single_threaded as executor;

type Decoder<T> = bufread::DeflateDecoder<T>;
#[cfg(not(target_arch = "wasm32"))]
type Encoder<T> = write::DeflateEncoder<T>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub session_timestamp: String,
    pub received: VecDeque<(Log, DateTime<Local>)>,
    pub message_alerts: BTreeSet<String>,
}

#[cfg(not(target_arch = "wasm32"))]
type GrpcClient = Client<Channel>;
#[cfg(target_arch = "wasm32")]
type GrpcClient = Client<WasmClient>;

#[cfg(target_arch = "wasm32")]
fn yield_loop() {
    let task = executor::spawn(async move {
        loop {
            executor::yield_animation_frame().await;
        }
    });

    executor::run(Some(task.task()));
}

#[cfg(target_arch = "wasm32")]
fn register_client(mut grpc_client: GrpcClient) -> Receiver<Connection> {
    let (tx, rx) = channel();

    let task = executor::spawn(async move {
        spawn_local(async move {
            info!("Registering client...");
            let res = grpc_client.register_client(()).await;
            if let Ok(connection) = res {
                let connection = connection.into_inner();

                if tx.send(connection).is_err() {}
            }
        });
    });

    executor::run(Some(task.task()));

    rx
}

#[cfg(target_arch = "wasm32")]
fn get_server_logs(
    mut grpc_client: GrpcClient,
    grpc_client_connection: Connection,
    received: Received,
) {
    let task = executor::spawn(async move {
        info!("Starting logs loop...");
        loop {
            if let Ok(res) = grpc_client.get_logs(grpc_client_connection.clone()).await {
                let mut response = res.into_inner();

                while let Ok(Some(message)) = response.message().await {
                    received
                        .write()
                        .unwrap()
                        .push_front((message.clone(), Local::now()));
                    executor::yield_animation_frame().await;
                }
            }
        }
    });

    executor::run(Some(task.task()));
}

#[derive(Default, Deserialize, Serialize)]
pub struct App {
    state: AppState,
    title: &'static str,
    #[serde(skip)]
    grpc_client: Option<GrpcClient>,
    #[cfg(not(target_arch = "wasm32"))]
    #[serde(skip)]
    promise: Option<Promise<Result<Response<ServerDetails>, Status>>>,
    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    client_connection_channel: Option<Receiver<Connection>>,
    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    started_logs_loop: bool,
    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    server_host: &'static str,
    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    server_port: &'static str,
    #[serde(skip)]
    wrapper_msg: Option<Arc<RefCell<WrapperMsg>>>,
}

impl App {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(
        ctx: &egui::Context,
        storage: Option<&dyn Storage>,
        grpc_client: GrpcClient,
        grpc_client_connection: Connection,
        wrapper_msg: Arc<RefCell<WrapperMsg>>,
        handle: &Handle,
    ) -> Self {
        let mut app = Self {
            state: AppState::default(),
            title: "CodeCTRL",
            grpc_client: Some(grpc_client),
            promise: None,
            wrapper_msg: Some(wrapper_msg),
        };

        ctx.set_fonts(fonts());
        ctx.set_style(application_style(app.state.application_settings.font_sizes));

        if let Some(storage) = storage {
            let data: AppState =
                eframe::get_value(storage, &format!("{}-appstate", eframe::APP_KEY))
                    .unwrap_or_default();

            if data.preserve_session {
                app.state = data;
            } else {
                app.state = AppState::default();
                app.state.preserve_session = false;
            }
        }

        let received = Arc::clone(&app.state.received);

        ctx.set_visuals(app.state.current_theme.clone());

        let mut grpc_client = app.grpc_client.clone().unwrap();
        let grpc_client_connection =
            if let Some(client) = app.state.grpc_client_connection.as_ref() {
                client.clone()
            } else {
                grpc_client_connection
            };

        handle.spawn(async move {
            loop {
                if let Ok(res) =
                    grpc_client.get_logs(grpc_client_connection.clone()).await
                {
                    let mut response = res.into_inner();

                    while let Ok(Some(message)) = response.message().await {
                        received
                            .write()
                            .unwrap()
                            .push_front((message.clone(), Local::now()));
                    }
                }
            }
        });

        app
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(
        ctx: &egui::Context,
        storage: Option<&dyn Storage>,
        grpc_client: GrpcClient,
        server_host: &'static str,
        server_port: &'static str,
    ) -> Self {
        let mut app = Self {
            state: AppState::default(),
            title: "CodeCTRL",
            grpc_client: Some(grpc_client.clone()),
            client_connection_channel: None,
            started_logs_loop: false,
            server_host,
            server_port,
        };

        if let Some(storage) = storage {
            let state: AppState =
                eframe::get_value(storage, &format!("{}-appstate", eframe::APP_KEY))
                    .unwrap_or_default();

            if state.preserve_session {
                app.state = state;
            } else {
                app.state = AppState::default();
                app.state.preserve_session = false;
            }
        }

        yield_loop();

        app.client_connection_channel = Some(register_client(grpc_client));

        ctx.set_fonts(fonts());
        ctx.set_style(application_style(app.state.application_settings.font_sizes));

        app
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn handle_key_inputs(&mut self, input_state: &InputState) {
        for event in &input_state.events {
            match event {
                // zoom bindings
                Event::Key {
                    key,
                    pressed,
                    modifiers,
                } if *pressed
                    && *key == Key::PageUp
                    && (modifiers.ctrl || modifiers.mac_cmd) =>
                {
                    self.state.application_settings.font_sizes.scale(1.0);
                },
                Event::Key {
                    key,
                    pressed,
                    modifiers,
                } if *pressed
                    && *key == Key::PageDown
                    && (modifiers.ctrl || modifiers.mac_cmd) =>
                    self.state.application_settings.font_sizes.scale(-1.0),
                Event::Key {
                    key,
                    pressed,
                    modifiers,
                } if *pressed
                    && *key == Key::Num0
                    && (modifiers.ctrl || modifiers.mac_cmd) =>
                {
                    self.state.application_settings.font_sizes = FontSizes::default();
                },
                Event::Zoom(zoom_delta) =>
                    if *zoom_delta > 1.0 {
                        self.state.application_settings.font_sizes.scale(1.0);
                    } else if *zoom_delta < 1.0 {
                        self.state.application_settings.font_sizes.scale(-1.0);
                    },

                // open/load bindings
                Event::Key {
                    key,
                    pressed,
                    modifiers,
                } if *pressed
                    && *key == Key::O
                    && (modifiers.ctrl || modifiers.mac_cmd) =>
                    self.load_file_dialog(),
                Event::Key {
                    key,
                    pressed,
                    modifiers,
                } if *pressed
                    && *key == Key::S
                    && (modifiers.ctrl || modifiers.mac_cmd) =>
                    self.save_file_dialog(),

                _ => (),
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save_file_dialog(&mut self) {
        self.state.session_timestamp =
            Local::now().format(&self.state.filename_format).to_string();

        let file_path = if let Some(file_path) = FileDialog::new()
            .set_file_name(&format!(
                "{file_name}.cdctrl",
                file_name = self.state.session_timestamp
            ))
            .add_filter("CodeCTRL Session", &["cdctrl"])
            .save_file()
        {
            file_path
        } else {
            return;
        };

        let AppState {
            session_timestamp,
            message_alerts,
            ..
        } = self.state.clone();

        let session = Session {
            session_timestamp,
            received: self.state.received.read().unwrap().clone(),
            message_alerts,
        };

        let mut data = vec![];

        ciborium_ser::into_writer(&session, &mut data).expect("Could not serialise logs");

        let mut gzip = Encoder::new(Vec::new(), Compression::default());

        let compression_error_dialog = |error| {
            MessageDialog::new()
                .set_title("Could not save file")
                .set_description(&format!("Could not compress logs: {error}"))
                .show()
        };

        if let Err(error) = gzip.write_all(&data) {
            compression_error_dialog(error);
            return;
        }

        let data = match gzip.finish() {
            Ok(data) => data,
            Err(error) => {
                compression_error_dialog(error);
                return;
            },
        };

        let mut file = match File::create(&file_path) {
            Ok(file_path) => file_path,
            Err(error) => {
                MessageDialog::new()
                    .set_title("Could not save file")
                    .set_description(&format!(
                        "Could not save file \"{file_path}\": {error}",
                        file_path = file_path.to_string_lossy(),
                    ))
                    .show();

                return;
            },
        };

        if let Err(error) = file.write_all(data.as_slice()) {
            MessageDialog::new()
                .set_title("Could not write to file")
                .set_description(&format!(
                    "Could not write to file \"{file_path}\": {error}",
                    file_path = file_path.to_string_lossy(),
                ))
                .show();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_file_dialog(&mut self) {
        let file_path = if let Some(file_path) = FileDialog::new()
            .add_filter("CodeCTRL Session", &["cdctrl"])
            .pick_file()
        {
            file_path
        } else {
            return;
        };

        match Self::load_from_file(&file_path, self) {
            Ok(_) => MessageDialog::new()
                .set_title("Successfully loaded file data")
                .set_description("Successfully loaded file data"),
            Err(error) => MessageDialog::new()
                .set_title("Could not parse log data")
                .set_description(&format!("{error}")),
        }
        .show();
    }

    #[cfg(target_arch = "wasm32")]
    fn load_file_dialog(&mut self) {
        let file_path = Arc::new(Mutex::new(FileHandle::wrap(
            web_sys::File::new_with_str_sequence(
                &wasm_bindgen::JsValue::from_serde(&vec![""]).unwrap(),
                "",
            )
            .unwrap(),
        )));
        let app = Arc::new(Mutex::new(unsafe {
            std::mem::transmute::<_, &'static mut Self>(self)
        }));

        let file_path_clone = Arc::clone(&file_path);
        let app_clone = Arc::clone(&app);

        spawn_local(async move {
            *file_path_clone.as_ref().lock().unwrap() = if let Some(file_path) =
                FileDialog::new()
                    .add_filter("CodeCTRL Session", &["cdctrl"])
                    .pick_file()
                    .await
            {
                file_path
            } else {
                return;
            };

            match Self::load_from_file(&file_path, &app_clone).await {
                Ok(_) => MessageDialog::new().set_title("Successfully loaded file data"),
                Err(error) => MessageDialog::new().set_title(&format!("{error}")),
            }
            .show();
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_from_file(
        file_path: &Path,
        app: &mut Self,
    ) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;

        let mut reader = BufReader::new(file);
        let reader = Decoder::new(&mut reader);

        let session: Session = match ciborium_de::from_reader(reader) {
            Ok(data) => data,
            Err(error) =>
                return Err(Box::new(IOError::new(
                    ErrorKind::Other,
                    format!(
                        "Could not parse log data from file \"{file_path}\": {error}",
                        file_path = file_path.to_string_lossy()
                    ),
                ))),
        };

        let AppState {
            received,
            session_timestamp,
            message_alerts,
            ..
        } = &mut app.state;

        *received.write().unwrap() = session.received;
        *session_timestamp = session.session_timestamp;
        *message_alerts = session.message_alerts;

        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn load_from_file(
        file_path: &Arc<Mutex<FileHandle>>,
        app: &Arc<Mutex<&mut Self>>,
    ) -> Result<(), Box<dyn Error>> {
        let data = file_path.as_ref().lock().unwrap().read().await;

        let mut reader = BufReader::new(data.as_slice());
        let reader = Decoder::new(&mut reader);

        let file_name = file_path.as_ref().lock().unwrap().file_name();

        let session: Session = match ciborium_de::from_reader(reader) {
            Ok(data) => data,
            Err(error) =>
                return Err(Box::new(IOError::new(
                    ErrorKind::Other,
                    format!(
                        "Could not parse log data from file \"{file_name}\": {error}",
                    ),
                ))),
        };

        let AppState {
            received,
            session_timestamp,
            message_alerts,
            ..
        } = &mut app.as_ref().lock().unwrap().state;

        *received.write().unwrap() = session.received;
        *session_timestamp = session.session_timestamp;
        *message_alerts = session.message_alerts;

        Ok(())
    }
}

impl eframe::App for App {
    #[allow(clippy::used_underscore_binding)]
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        #[cfg(not(target_arch = "wasm32"))]
        self.handle_key_inputs(&ctx.input());

        #[cfg(target_arch = "wasm32")]
        if let Some(grpc_client_connection) = &self.state.grpc_client_connection {
            if !self.started_logs_loop {
                let grpc_client = self.grpc_client.as_ref().unwrap().clone();
                let grpc_client_connection = grpc_client_connection.clone();

                self.started_logs_loop = true;

                get_server_logs(
                    grpc_client,
                    grpc_client_connection,
                    Arc::clone(&self.state.received),
                );
            }
        } else {
            if let Some(client_channel) = self.client_connection_channel.as_mut() {
                if let Ok(Some(connection)) = client_channel.try_recv() {
                    self.state.grpc_client_connection = Some(connection);
                    client_channel.close();
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        if self.state.refresh_server_details {
            let mut grpc_client = self.grpc_client.clone().unwrap();

            let promise = self.promise.get_or_insert_with(|| {
                Promise::spawn_async(
                    async move { grpc_client.get_server_details(()).await },
                )
            });

            if let Some(Ok(details)) = promise.ready() {
                let details = details.get_ref().clone();
                self.state.server_details = Some(details);
                self.state.refresh_server_details = false;
            }
        } else {
            let now = Instant::now();

            if now.duration_since(self.state.time_details_last_checked)
                > Duration::from_secs(5)
            {
                let mut grpc_client = self.grpc_client.clone().unwrap();

                self.state.refresh_server_details = true;
                self.promise = Some(Promise::spawn_async(async move {
                    grpc_client.get_server_details(()).await
                }));
                self.state.time_details_last_checked = Instant::now();
            }
        }

        if self.state.is_about_open {
            about_view(&mut self.state, ctx);
        }

        if self.state.is_settings_open {
            settings_view(&mut self.state, ctx);
        }

        egui::TopBottomPanel::top("top_bar")
            .resizable(false)
            .default_height(200.0)
            .show(ctx, |ui| {
                ui.add_space(4.0);

                ui.horizontal_wrapped(|ui| {
                    ui.menu_button("File", |ui| {
                        #[cfg(not(target_arch = "wasm32"))]
                        if ui.button("Save project").clicked() {
                            self.save_file_dialog();
                        }

                        if ui.button("Open project").clicked() {
                            self.load_file_dialog();
                        }

                        ui.separator();

                        if ui.button("Settings").clicked() {
                            self.state.is_settings_open = !self.state.is_settings_open;
                        }

                        ui.separator();
                        if ui.button("Log out").clicked() {
                            if let Some(wrapper_msg) = self.wrapper_msg.as_deref() {
                                if let Ok(mut wrapper_msg) = wrapper_msg.try_borrow_mut()
                                {
                                    *wrapper_msg = WrapperMsg::LogOut;
                                }
                            }
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        ui.separator();

                        #[cfg(not(target_arch = "wasm32"))]
                        if ui.button("Quit").clicked() {
                            info!("Quitting...");
                            _frame.close();
                        }
                    });

                    ui.menu_button("Help", |ui| {
                        if ui.button("About").clicked() {
                            self.state.is_about_open = !self.state.is_about_open;
                        }
                    });

                    ui.separator();

                    ui.label("Filter: ");
                    ui.text_edit_singleline(&mut self.state.search_filter);

                    // u1f5d9 = 🗙
                    if ui.button("\u{1f5d9}").clicked() {
                        self.state.search_filter = "".into();
                    }

                    ui.label("Filter by:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{}", self.state.filter_by))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.state.filter_by,
                                Filter::Message,
                                format!("{}", Filter::Message),
                            );

                            ui.selectable_value(
                                &mut self.state.filter_by,
                                Filter::Time,
                                format!("{}", Filter::Time),
                            );

                            ui.selectable_value(
                                &mut self.state.filter_by,
                                Filter::FileName,
                                format!("{}", Filter::FileName),
                            );

                            ui.selectable_value(
                                &mut self.state.filter_by,
                                Filter::Address,
                                format!("{}", Filter::Address),
                            );

                            ui.selectable_value(
                                &mut self.state.filter_by,
                                Filter::LineNumber,
                                format!("{}", Filter::LineNumber),
                            );
                        });

                    ui.checkbox(&mut self.state.is_case_sensitive, "Case sensitive");
                    ui.checkbox(&mut self.state.is_using_regex, "Regex");
                    ui.checkbox(
                        &mut self.state.do_scroll_to_selected_log,
                        "Scroll to selected log",
                    );

                    if ui
                        .button(
                            if self.state.is_newest_first {
                                "\u{2b07} Newest first" // u2b07 = ⬇
                            } else {
                                "\u{2b06} Newest last" // u2b06 = ⬆
                            },
                        )
                        .clicked()
                    {
                        self.state.is_newest_first = !self.state.is_newest_first;
                    }

                    // u1f5d1 = ��
                    if ui.button("\u{1f5d1} Clear logs").clicked() {
                        if let Ok(mut received) = self.state.received.write() {
                            received.clear();
                            self.state.clicked_item = None;
                        }
                    }

                    ui.separator();

                    if self.state.server_details.is_some() {
                        let ServerDetails { host, port, .. } =
                            self.state.server_details.as_ref().unwrap();

                        ui.label(format!("Listening on: {host}:{port}"));
                    }
                });

                ui.add_space(2.0);
            });

        let is_empty = {
            let received = Arc::clone(&self.state.received);

            let x = if let Ok(received) = received.read() {
                received.is_empty()
            } else {
                false
            };

            x
        };

        if is_empty {
            #[cfg(not(target_arch = "wasm32"))]
            match self.state.server_details.as_ref() {
                Some(ServerDetails { host, port, .. }) =>
                    main_view_empty(ctx, &format!("{host}:{port}")),
                None => main_view_empty(ctx, "Fetching server details..."),
            }

            #[cfg(target_arch = "wasm32")]
            main_view_empty(ctx, &format!("{}:{}", self.server_host, self.server_port));
        } else {
            main_view(&mut self.state, ctx);
        }

        if self.state.clicked_item.is_some() {
            details_view(&mut self.state, ctx);
        } else {
            self.state.preview_height = 0.0;
        }

        ctx.request_repaint();
    }

    fn max_size_points(&self) -> egui::Vec2 {
        Vec2 {
            x: f32::INFINITY,
            y: f32::INFINITY,
        }
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(
            storage,
            &format!("{}-appstate", eframe::APP_KEY),
            &self.state,
        );
    }
}
