#![feature(async_closure)]
#![feature(thread_spawn_unchecked)]
#![warn(clippy::pedantic)]

mod gui_app;
mod text_app;

extern crate clap;

use crate::text_app::TextApp;
use clap::{crate_authors, crate_version, App as ClapApp, Arg};
use gui_app::GuiApp;
use server::{Mode, Server};
use std::thread;

static NAME: &str = "codeCTRL";

#[tokio::main]
async fn main() {
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
        .get_matches();

    let is_headless = matches.is_present("headless");
    let mut running_mode = Mode::Full;

    if is_headless {
        running_mode = Mode::Headless;
    }

    let (server, receiver) = Server::new(running_mode);

    thread::spawn(move || {
        server.run_server();
    });

    match running_mode {
        Mode::Full => {
            let app = GuiApp::new(NAME, receiver);

            let options = egui_glow::NativeOptions {
                transparent: true,
                drag_and_drop_support: true,
                ..egui_glow::NativeOptions::default()
            };

            eframe::run_native(Box::new(app), options);
        },
        Mode::Headless => {
            let mut app = TextApp::new(NAME, receiver)
                .expect("Could not create headless application");

            app.draw().unwrap();
        },
    }
}
