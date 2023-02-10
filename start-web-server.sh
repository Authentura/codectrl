#!/usr/bin/env bash

set -xe

PORT=8080

source ./build-wasm.sh

cargo install basic-http-server
basic-http-server --addr 127.0.0.1:$PORT ${WASM_PATH}