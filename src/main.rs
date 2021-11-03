#![feature(async_closure)]
#![feature(thread_spawn_unchecked)]

mod app;

extern crate clap;

use app::App;
use clap::{crate_authors, crate_version, App as ClapApp, Arg};
use server::{Mode, Server};
use std::thread;

#[tokio::main]
async fn main() {
    let matches = ClapApp::new("codeCTRL")
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

    let app = App::new(receiver);

    let options = egui_glow::NativeOptions {
        transparent: true,
        drag_and_drop_support: true,
        ..egui_glow::NativeOptions::default()
    };

    eframe::run_native(Box::new(app), options);
}
