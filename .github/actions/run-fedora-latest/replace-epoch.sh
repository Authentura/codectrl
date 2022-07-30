#!/bin/bash

set -xe

sed "s/<>/$1/g" "$2/Cargo.toml" > "$2/Cargo.toml.new"
mv "$2/Cargo.toml" "$2/Cargo.toml.old"
mv "$2/Cargo.toml.new" "$2/Cargo.toml"
