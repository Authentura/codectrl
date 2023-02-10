#![warn(clippy::pedantic)]
#![allow(
    clippy::blocks_in_if_conditions,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    clippy::missing_panics_doc,
    incomplete_features
)]

mod app;
mod components;
mod consts;
mod data;
mod login;
mod widgets;
mod wrapper;

#[cfg(target_arch = "wasm32")]
use crate::app::App;

#[cfg(not(target_arch = "wasm32"))]
use clap::{crate_authors, crate_name, crate_version, Arg, Command};
use codectrl_protobuf_bindings::logs_service::log_server_client::LogServerClient as Client;
#[cfg(not(target_arch = "wasm32"))]
use codectrl_server::run_server;
#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::JsValue;
#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};
#[cfg(target_arch = "wasm32")]
use eframe::web::AppRunnerRef;
#[cfg(not(target_arch = "wasm32"))]
use egui_toast::Toasts;
#[cfg(target_arch = "wasm32")]
use grpc_web_client::Client as WasmClient;
#[cfg(not(target_arch = "wasm32"))]
use once_cell::unsync::OnceCell;
#[cfg(not(target_arch = "wasm32"))]
use rfd::MessageDialog;
#[cfg(not(target_arch = "wasm32"))]
use std::cell::RefCell;
#[cfg(not(target_arch = "wasm32"))]
use std::{collections::HashMap, env, path::Path};
#[cfg(not(target_arch = "wasm32"))]
use tokio::runtime::Handle;
#[cfg(not(target_arch = "wasm32"))]
use tonic::transport::Channel;
#[cfg(not(target_arch = "wasm32"))]
use wrapper::Wrapper;

#[cfg(not(target_arch = "wasm32"))]
pub static mut TOASTS: OnceCell<RefCell<Toasts>> = OnceCell::new();

#[cfg(not(target_arch = "wasm32"))]
type GrpcClient = Client<Channel>;
#[cfg(target_arch = "wasm32")]
type GrpcClient = Client<WasmClient>;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WebHandle {
    handle: AppRunnerRef,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebHandle {
    #[wasm_bindgen]
    pub fn stop_web(&self) -> Result<(), wasm_bindgen::JsValue> {
        let mut app = self.handle.lock();
        app.destroy()
    }

    #[wasm_bindgen]
    pub fn set_some_content_from_javasript(&mut self, _some_data: &str) {
        let _app = self.handle.lock().app_mut::<App>();
        // _app.data = some_data;
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn run(
    host: &'static str,
    port: &'static str,
    canvas_id: Option<&str>,
) -> Result<WebHandle, JsValue> {
    use eframe::WebOptions;

    let canvas_id = canvas_id.unwrap_or("codectrl-root");

    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let grpc_client = GrpcClient::new(WasmClient::new(format!("http://{host}:{port}")));

    eframe::start_web(
        canvas_id,
        WebOptions::default(),
        Box::new(move |cc| {
            Box::new(App::new(&cc.egui_ctx, None, grpc_client, host, port))
        }),
    )
    .await
    .map(|handle| WebHandle { handle })
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
pub async fn run() {
    #[cfg(debug_assertions)]
    console_subscriber::init();
    env_logger::try_init().ok();

    let binding = unsafe { &TOASTS };
    binding.get_or_init(|| {
        RefCell::new(
            Toasts::new()
                .anchor((0.0, 0.0))
                .direction(egui::Direction::TopDown),
        )
    });

    let command_line = env::vars().collect::<HashMap<String, String>>();

    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!(", "))
        .arg(
            Arg::new("port")
                .takes_value(true)
                .short('p')
                .long("port")
                .help(
                    "Specifies the port for the server to run on, can also be specified \
                     by the PORT environment variable",
                ),
        )
        .arg(
            Arg::new("host")
                .takes_value(true)
                .short('H')
                .long("host")
                .help(
                    "Specifies the IP address for the server to run on. Can also be \
                     specified with the HOST environment variable. Defaults to 0.0.0.0.",
                ),
        )
        .arg(
            Arg::new("PROJECT")
                .takes_value(true)
                .index(1)
                .help("The project file to load (optional)."),
        )
        .arg(
            Arg::new("server_only")
                .long("server-only")
                .help("Only runs the back-end server, not the GUI."),
        )
        .get_matches();

    let has_port = matches.is_present("port");

    let port = if has_port {
        matches.value_of("port").unwrap()
    } else if command_line.contains_key("PORT") {
        command_line.get("PORT").unwrap()
    } else {
        "3002"
    }
    .parse::<u32>()
    .expect("Port was not a valid value: needs to be an integer value.");

    let has_project = matches.is_present("PROJECT");

    let project_file = if has_project {
        Some(matches.value_of("PROJECT").unwrap())
    } else {
        None
    };

    let has_host = matches.is_present("host");

    let host = if has_host {
        matches.value_of("host").unwrap().to_owned()
    } else if command_line.contains_key("HOST") {
        command_line.get("HOST").unwrap().clone()
    } else {
        "127.0.0.1".to_owned()
    };

    let server_only = matches.is_present("server_only");

    let spawn = async move {
        if let Err(error) = run_server(Some(host), Some(port), None, None).await {
            if MessageDialog::new()
                .set_title("Could not start CodeCtrl server")
                .set_level(rfd::MessageLevel::Error)
                .set_description(&format!("{error:#?}"))
                .set_buttons(rfd::MessageButtons::Ok)
                .show()
            {
                std::process::exit(1);
            }
        }
    };

    let handle = Handle::current();

    if server_only {
        spawn.await;
    } else {
        handle.spawn(spawn);

        let file_path = if let Some(project_file) = project_file {
            let file_path = match Path::new(project_file).canonicalize() {
                Ok(file_path) => file_path,
                Err(error) =>
                    panic!("Could not cannonicalise PROJECT file path: {error}"),
            };

            file_path
        } else {
            Path::new("").to_path_buf()
        };

        let options = eframe::NativeOptions {
            drag_and_drop_support: true,
            ..eframe::NativeOptions::default()
        };

        eframe::run_native(
            "CodeCTRL",
            options,
            Box::new(move |_| Box::new(Wrapper::new(handle, file_path))),
        );
    }
}
