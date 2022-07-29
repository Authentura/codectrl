#!/bin/bash

set -xe

sed -i.bak "s/<>/$1/g" $2/Cargo.toml
