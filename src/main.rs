#![feature(async_closure, thread_spawn_unchecked)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod common;
mod components;
mod gui_app;
mod text_app;

extern crate clap;

use crate::text_app::TextApp;
use clap::{crate_authors, crate_version, App as ClapApp, Arg};
use gui_app::GuiApp;
use server::Server;
use std::{collections::HashMap, env, thread};

#[derive(Copy, Clone)]
pub enum Mode {
    Full,
    Headless,
}

impl Default for Mode {
    fn default() -> Self { Self::Full }
}

static NAME: &str = "codeCTRL";

#[tokio::main]
async fn main() {
    let command_line = env::vars().collect::<HashMap<String, String>>();

    let matches = ClapApp::new(NAME)
        .version(crate_version!())
        .author(crate_authors!(", "))
        .arg(
            Arg::with_name("graphical")
                .short("g")
                .long("graphical")
                .help("Runs codeCTRL in graphical mode (default)"),
        )
        .arg(
            Arg::with_name("headless")
                .short("H")
                .long("headless")
                .help("Runs codeCTRL in text-only mode"),
        )
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

    let is_headless = matches.is_present("headless");
    let mut running_mode = Mode::Full;

    let has_port = matches.is_present("port");
    let port = if has_port {
        matches.value_of("port").unwrap()
    } else if command_line.contains_key("PORT") {
        command_line.get("PORT").unwrap()
    } else {
        "3001"
    };

    let socket_address = format!("127.0.0.1:{}", port);

    if is_headless {
        running_mode = Mode::Headless;
    }

    let (mut server, receiver) = Server::new(port);

    thread::spawn(move || {
        server.run_server().unwrap();
    });

    match running_mode {
        Mode::Full => {
            let app = GuiApp::new(NAME, receiver, socket_address);

            let options = egui_glow::NativeOptions {
                transparent: true,
                drag_and_drop_support: true,
                ..egui_glow::NativeOptions::default()
            };

            eframe::run_native(Box::new(app), options);
        },
        Mode::Headless => {
            let mut app = TextApp::new(NAME, receiver, socket_address)
                .expect("Could not create headless application");

            app.draw().unwrap();
        },
    }
}
