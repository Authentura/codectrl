# codeCTRL

## Implementing a logger for a language

By default, these are the officially supported loggers:

- [Rust](https://github.com/Authentura/codectrl-rust-logger)
- [C++](https://github.com/Authentura/codectrl-cxx-logger)
- [Python](https://github.com/Authentura/codectrl-python-logger)
- [PHP](https://github.com/Authentura/codectrl-php-logger)
- JavaScript - [Node](https://github.com/Authentura/codectrl-nodejs-logger), Browser (not yet, possibly not possible<sup>?</sup>)

If you require a logger in a language where one does not yet exist, please feel free to create one, **_making sure to be consistent with the offical [schema and API](https://github.com/Authentura/codectrl/blob/main/loggers/SCHEMA.md)_**, and submit a PR to add it to the list of unofficial language loggers below:

- None yet (remove me if/when one is added).

## Requirements

The MSRV (minimum supported Rust version): 1.62 (on the nightly toolchain).

Below you will find the requirements to build on each platform. The supported platform(s)
are:

- [Linux](#Linux)
- [Web](#Web)

Planned support:

- Windows
- MacOS (M1 and legacy Intel systems)

### Linux

The current _officially_ supported Linux distributions are the following:

- [Fedora](#Fedora)
- [Ubuntu and Debian 11](#Debian-based)

Support is planned for the following:

- Arch (and it's derivatives)
- RHEL 7+ (and compatible distros, i.e. Rocky Linux)

#### Fedora

Minimum supported Fedora version: 34.

You will need to install the "Development Tools" group. You can do this by running:
`sudo dnf groupinstall "Development Tools" -y`.

##### Dependencies

- `gobject-introspection-devel`
- `cairo-devel`
- `atk-devel`
- `pango-devel`
- `gdk-pixbuf2-devel`
- `gtk3-devel`

#### Debian-based

There is confirmed support for Ubuntu 21.10, 21.04, and 20.04 and Debian 11 and 10.

##### Dependencies

- A C/C++ compiler. For example `gcc` or `clang`.
- `libglib2.0-dev`
- `libpango1.0-dev`
- `libgdk-pixbuf-2.0-dev`
- `libatk1.0-dev`
- `libgtk-3-dev`
- `libxcb-shape0-dev`
- `libxcb-xfixes0-dev`

### Web

The main GUI is now able to run in the browser using `trunk`. You can install `trunk` with `cargo install trunk`. You will need the `wasm32-unknown-unknown` target installed through `rustup`. You
can install that with `rustup target add wasm32-unknown-unknown`.

Then, you can run a local server with `trunk serve --release`.
