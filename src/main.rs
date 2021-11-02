extern crate clap;

use clap::{crate_authors, crate_version, App, Arg};
use server::{server, Mode};

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

    server(running_mode).await;
}
