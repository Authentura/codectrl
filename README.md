# CodeCTRL

[![Formatting](https://github.com/Authentura/codectrl/actions/workflows/reformat.yml/badge.svg)](https://github.com/Authentura/codectrl/actions/workflows/reformat.yml)
[![Clippy](https://github.com/Authentura/codectrl/actions/workflows/clippy.yml/badge.svg)](https://github.com/Authentura/codectrl/actions/workflows/clippy.yml)
[![Build & Packaging](https://github.com/Authentura/codectrl/actions/workflows/build-and-package.yml/badge.svg)](https://github.com/Authentura/codectrl/actions/workflows/build-and-package.yml)

## Implementing a logger for a language

By default, these are the officially supported loggers:

| Language | Updated to gRPC?                                    | Link                                                         |
| :------- | :-------------------------------------------------- | :----------------------------------------------------------- |
| Rust     | Yes (on the grpc-rewrite branch, soon to be merged) | [Here](https://github.com/Authentura/codectrl-rust-logger)   |
| C++      | No                                                  | [Here](https://github.com/Authentura/codectrl-cxx-logger)    |
| Python   | No                                                  | [Here](https://github.com/Authentura/codectrl-python-logger) |
| PHP      | No                                                  | [Here](https://github.com/Authentura/codectrl-php-logger)    |
| NodeJS   | No                                                  | [Here](https://github.com/Authentura/codectrl-nodejs-logger) |

All language loggers now need to use gRPC in order to implement the API schema.
The protobuf files are available
[here](https://github.com/Authentura/codectrl-protobuf-specifications).

Unofficial language loggers:

- None yet (remove me if/when one is added).

## Build requirements

The MSRV (minimum supported Rust version): 1.64 (on the nightly toolchain).

Below you will find the requirements to build on each platform. The supported platform(s)
are:

- [Linux](#Linux)
- [Web](#Web)

Planned support:

- Windows
- MacOS (M1 and legacy Intel systems)

Packages for Ubuntu 22.04, the latest stable Fedora and Fedora Rawhide are
built per commit and per pull-request and can be found
[here](https://github.com/Authentura/codectrl/actions/workflows/build-and-package.yml)
underneath each of the **_completed_** CI jobs.

### Linux

The current _officially_ supported Linux distributions are the following:

- [Fedora](#Fedora)
- [Ubuntu and Debian 11](#Debian-based)

**_NOTE:_** You can use the `./bootstrap-build.sh` or the
`./bootstrap-action.sh` scripts to automatically install the dependencies for
the supported distributions.

Support is planned for the following:

- Arch (and it's derivatives)
- RHEL 7+ (and compatible distros, i.e. Rocky Linux) (should work already but
  haven't gotten around to confirming)

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

There is confirmed support for Ubuntu 22.04, 21.10, 21.04, and Debian 11 and 10.

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

The main GUI is now able to run in the browser using `trunk`. You can install
`trunk` with `cargo install trunk`. You will need the `wasm32-unknown-unknown`
target installed through `rustup`. You can install that with `rustup target add wasm32-unknown-unknown`.

**_NOTE:_** Currently, `trunk` doesn't have support for manually specifying the
headers like you can see in the `Trunk.toml` in this repository. A PR is open
for it and to use the `Trunk.toml` you can issue this command to install a
version of `trunk` that _does_ support manually specifying headers: `cargo install --git https://github.com/oberien/trunk --branch headers --force trunk`

Then, you can run a local server with `trunk serve --release`.
