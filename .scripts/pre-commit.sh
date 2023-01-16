#!/usr/bin/env bash

if ! cargo +nightly fmt --all -- --unstable-features --error-on-unformatted --check; then
  touch .rustfmt-triggered
fi
