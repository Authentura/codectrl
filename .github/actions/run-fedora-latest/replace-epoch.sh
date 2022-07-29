#!/bin/bash

sed -i "s/<(+)>/$1/" "$2/Cargo.toml"
