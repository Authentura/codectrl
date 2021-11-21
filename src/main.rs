#![feature(async_closure, thread_spawn_unchecked)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::struct_excessive_bools)]

mod app;
mod components;

extern crate clap;

use app::App;
use clap::{crate_authors, crate_version, App as ClapApp, Arg};
use server::Server;
use std::{collections::HashMap, env, thread};

static NAME: &str = "codeCTRL";

#[tokio::main]
async fn main() {
    let command_line = env::vars().collect::<HashMap<String, String>>();

    let matches = ClapApp::new(NAME)
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
        .get_matches();

    let has_port = matches.is_present("port");
    let port = if has_port {
        matches.value_of("port").unwrap()
    } else if command_line.contains_key("PORT") {
        command_line.get("PORT").unwrap()
    } else {
        "3001"
    };

    let socket_address = format!("127.0.0.1:{}", port);

    let (mut server, receiver) = Server::new(port);

    thread::spawn(move || {
        server.run_server().unwrap();
    });

    let app = App::new(NAME, receiver, socket_address);

    let options = egui_glow::NativeOptions {
        transparent: true,
        drag_and_drop_support: true,
        ..egui_glow::NativeOptions::default()
    };

    eframe::run_native(Box::new(app), options);
}
