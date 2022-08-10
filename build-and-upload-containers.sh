#!/usr/bin/env bash

if [[ -z "$1" ]]; then
  echo "Requires username as input"
  exit 1
fi

if [[ -z "$CR_PAT" ]]; then
  echo "Requires the CR_PAT environment variable to set to a valid GitHub PAT with the relevant permissions."
  exit 1
fi

echo $CR_PAT | podman login ghcr.io -u "$1" --password-stdin

cd .github/actions/

./build-all.sh

podman push ghcr.io/authentura/codectrl-fedora-latest:latest
podman push ghcr.io/authentura/codectrl-fedora-rawhide:latest
podman push ghcr.io/authentura/codectrl-ubuntu-latest:latest
podman push ghcr.io/authentura/codectrl-ubuntu-22-04:latest
