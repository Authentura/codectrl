#!/bin/sh -l

set -xe

/bootstrap-action.sh

echo "Installing rustup..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- --default-toolchain nightly -y
source $HOME/.cargo/env

cargo install cargo-generate-rpm

$1
