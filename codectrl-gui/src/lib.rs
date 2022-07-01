#![feature(thread_spawn_unchecked)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::blocks_in_if_conditions,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    incomplete_features
)]

mod app;
mod components;
mod consts;
mod data;

#[cfg(not(target_arch = "wasm32"))]
extern crate clap;

use app::App;
#[cfg(not(target_arch = "wasm32"))]
use clap::{crate_authors, crate_name, crate_version, Arg, Command};
#[cfg(not(target_arch = "wasm32"))]
use codectrl_log_server::Server;
#[cfg(not(target_arch = "wasm32"))]
use codectrl_server::run_server;
#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::JsValue;
#[cfg(not(target_arch = "wasm32"))]
use std::{collections::HashMap, env, path::Path, thread};
#[cfg(not(target_arch = "wasm32"))]
use tokio::runtime::Runtime;

use codectrl_protobuf_bindings::logs_service::log_server_client::LogServerClient;

#[cfg(target_arch = "wasm32")]
use tonic_web_wasm_client::Client;

#[cfg(target_arch = "wasm32")]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let grpc_client =
        LogServerClient::new(Client::new("http://127.0.0.1:3002".to_string()));

    eframe::start_web(
        "codectrl-root",
        Box::new(move |cc| Box::new(App::new(cc, grpc_client))),
    )
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
pub async fn run() {
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
        "3001"
    };

    let has_host = matches.is_present("host");

    let host = if has_host {
        matches.value_of("host").unwrap()
    } else if command_line.contains_key("HOST") {
        command_line.get("HOST").unwrap()
    } else {
        "0.0.0.0"
    };

    let has_project = matches.is_present("PROJECT");

    let project_file = if has_project {
        Some(matches.value_of("PROJECT").unwrap())
    } else {
        None
    };

    let socket_address = format!("{host}:{port}");

    let (mut server, receiver) = Server::new(host, port);

    if matches.is_present("server_only") {
        server.run_server_current_runtime().await.unwrap();
        return;
    }

    thread::spawn(move || {
        server.run_server_new_runtime().unwrap();
    });

    thread::spawn(|| {
        let rt = Runtime::new().unwrap();

        rt.spawn_blocking(|| async { run_server(None, None, None).await.unwrap() })
    });

    // let mut app = App::new(receiver, socket_address);

    println!("Waiting for gRPC server to become available...");
    let grpc_client = loop {
        let res = LogServerClient::connect("http://127.0.0.1:3002").await;

        if let Ok(res) = res {
            break res;
        }
    };

    // let grpc_client = LogServerClient::connect("http://127.0.0.1:3002")
    //     .await
    //     .expect("Could not connect to gRPC endpoint");

    let file_path = if let Some(project_file) = project_file {
        let file_path = match Path::new(project_file).canonicalize() {
            Ok(file_path) => file_path,
            Err(error) => panic!("Could not cannonicalise PROJECT file path: {error}"),
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
        "codeCtrl",
        options,
        Box::new(move |cc| {
            let mut app = App::new(cc, receiver, socket_address, grpc_client);

            if file_path.exists() {
                if let Err(error) = App::load_from_file(&file_path, &mut app) {
                    panic!("An error occurred: {error}");
                }
            }

            Box::new(app)
        }),
    );
}
