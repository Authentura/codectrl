#![feature(async_closure)]

extern crate clap;

use clap::{crate_authors, crate_version, App, Arg};
use server::{server, Mode};
use std::thread;

#[tokio::main]
async fn main() {
    let matches = App::new("codeCTRL")
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

    let server_thread = thread::spawn(async move || {
        server(running_mode).await;
    });

    let _ = server_thread.join().expect("Could not join server thread");
}
