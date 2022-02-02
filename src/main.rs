#![feature(async_closure, thread_spawn_unchecked)]
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

extern crate clap;

use app::App;
use clap::{crate_authors, crate_name, crate_version, App as ClapApp, Arg};
use codectrl_log_server::Server;
use std::{collections::HashMap, env, path::Path, thread};

#[tokio::main]
async fn main() {
    let command_line = env::vars().collect::<HashMap<String, String>>();

    let matches = ClapApp::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!(", "))
        .arg(
            Arg::with_name("port")
                .takes_value(true)
                .short("p")
                .long("port")
                .help(
                    "Specifies the port for the server to run on, can also be specified \
                     by the PORT environment variable",
                ),
        )
        .arg(
            Arg::with_name("PROJECT")
                .takes_value(true)
                .index(1)
                .help("The project file to load (optional)."),
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

    let has_project = matches.is_present("PROJECT");

    let project_file = if has_project {
        Some(matches.value_of("PROJECT").unwrap())
    } else {
        None
    };

    let socket_address = format!("127.0.0.1:{}", port);

    let (mut server, receiver) = Server::new(port);

    thread::spawn(move || {
        server.run_server().unwrap();
    });

    let mut app = App::new(receiver, socket_address);

    if let Some(project_file) = project_file {
        let file_path = match Path::new(project_file).canonicalize() {
            Ok(file_path) => file_path,
            Err(error) => panic!("Could not cannonicalise PROJECT file path: {error}"),
        };

        if let Err(error) = App::load_from_file(&file_path, &mut app) {
            panic!("An error occurred: {error}")
        }
    }

    let options = egui_glow::NativeOptions {
        drag_and_drop_support: true,
        ..egui_glow::NativeOptions::default()
    };

    eframe::run_native(Box::new(app), options);
}
