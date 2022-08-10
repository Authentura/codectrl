#!/usr/bin/env bash

for d in *; do
  if [[ -d "$d" ]]; then
    cd "$d"
    podman build -t "authentura/codectrl-$d":latest .
    cd ..
  fi
done
