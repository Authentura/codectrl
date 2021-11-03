# codeCTRL

## Requirements
The MSRV (minimum supported Rust version): 1.56.1 (on the nightly toolchain).

Below you will find the requirements to build on each platform. The supported platform
(s) are:

- [Linux](#Linux)

Planned support:

- Windows
- MacOS (M1 and legacy Intel systems)

### Linux
The current *officially* supported Linux distributions are the following:

- [Fedora](#Fedora)

Support is planned for the following:

- Arch (and it's derivatives)
- Ubuntu/Debian
- RHEL 7+ (and compatible distros, i.e. Rocky Linux)

#### Fedora

Minimum supported Fedora version: 34.

You will need to install the "Development Tools" group. You can do this by running: 
`sudo dnf groupinstall "Development Tools" -y`.