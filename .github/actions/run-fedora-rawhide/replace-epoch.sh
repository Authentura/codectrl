#!/bin/bash

set -xe

sed "s/<>/$1/g" "$2/Cargo.toml" > "$2/Cargo.toml"
