# CodeCTRL

[![Formatting](https://github.com/Authentura/codectrl/actions/workflows/reformat.yml/badge.svg)](https://github.com/Authentura/codectrl/actions/workflows/reformat.yml)
[![Clippy](https://github.com/Authentura/codectrl/actions/workflows/clippy.yml/badge.svg)](https://github.com/Authentura/codectrl/actions/workflows/clippy.yml)
[![Build & Packaging](https://github.com/Authentura/codectrl/actions/workflows/build-and-package.yml/badge.svg)](https://github.com/Authentura/codectrl/actions/workflows/build-and-package.yml)

## Implementing a logger for a language

By default, these are the officially supported loggers:

| Language | Updated to gRPC? | Link                                                         |
| :------- | :--------------- | :----------------------------------------------------------- |
| Rust     | Yes              | [Here](https://github.com/Authentura/codectrl-rust-logger)   |
| C++      | No               | [Here](https://github.com/Authentura/codectrl-cxx-logger)    |
| Python   | No               | [Here](https://github.com/Authentura/codectrl-python-logger) |
| PHP      | No               | [Here](https://github.com/Authentura/codectrl-php-logger)    |
| NodeJS   | No               | [Here](https://github.com/Authentura/codectrl-nodejs-logger) |

All language loggers now need to use gRPC in order to implement the API schema.
The protobuf files are available
[here](https://github.com/Authentura/codectrl-protobuf-specifications).

Unofficial language loggers:

- None yet (remove me if/when one is added).

## Build requirements

The MSRV (minimum supported Rust version): 1.62.

Below you will find the requirements to build on each platform. The supported platform(s)
are:

- [Linux](#Linux) - Supported: Ubuntu 22.04, Ubuntu 20.04, Fedora 36, Fedora Rawhide,
  Debian 11, Debian 10 and Debian Sid.
- [Web](#Web)
- [Windows](#Windows)

Planned support:

- MacOS (M1 and legacy Intel systems)

Packages for the supported distributions listed above can be found
[here](https://github.com/Authentura/codectrl/actions/workflows/build-and-package.yml)
underneath each of the **_completed_** CI jobs.

### Linux

The current _officially_ supported Linux distributions are the following:

- [Fedora (36, Rawhide)](#Fedora)
- [Ubuntu (22.04, 20.04) and Debian (11, 10, Sid)](#Debian-based)

**_NOTE:_** You can use the `./bootstrap-build.sh` or the
`./bootstrap-action.sh` scripts to automatically install the dependencies for
the supported distributions.

Support is planned for the following:

- Arch (and it's derivatives)
- RHEL 7+ (and compatible distros, i.e. Rocky Linux) (should work already but
  haven't gotten around to confirming)

#### Fedora

Minimum supported Fedora version: 36.

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

There is support for Ubuntu 22.04, Ubuntu 20.04, Debian 11, Debian 10 and
Debian Sid.

##### Dependencies

- A C/C++ compiler. For example `gcc` or `clang`.
- `libglib2.0-dev`
- `libpango1.0-dev`
- `libgdk-pixbuf-2.0-dev` (or `libgdk-pixbuf2.0-dev` on 20.04).
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

### Windows

You can build CodeCTRL for Windows simply by installing `rustup` via the normal
channel: [here](https://rustup.rs), and issuing a `cargo build --release` at
the root of this project.

A MSI for Windows is automatically generated on every commit of the CodeCTRL
`main` branch and can be found in one of the completed workflow runs
[here](https://github.com/Authentura/codectrl/actions/workflows/build-and-package.yml).
