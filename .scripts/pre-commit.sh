#!/usr/bin/env bash

if cargo +nightly fmt --all -- --unstable-features --error-on-unformatted --check --color always; then
  touch .rustfmt-triggered
fi
