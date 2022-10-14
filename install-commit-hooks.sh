#!/usr/bin/env bash

if ! command -v pre-commit >/dev/null; then
  echo "Please install pre-commit: pip install pre-commit"
  exit 1;
fi

pre-commit install
