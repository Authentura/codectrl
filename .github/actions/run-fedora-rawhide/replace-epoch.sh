#!/bin/bash

set -xe

sed -i "s/<>/$1/g" "$2/Cargo.toml"
