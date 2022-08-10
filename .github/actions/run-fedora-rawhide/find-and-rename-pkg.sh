#!/usr/bin/env bash

source /etc/os-release

set -xe

cat Cargo.toml

SUFFIX=""
DIR=""
ROOT="."
FILE_EXTENSION=""

if [[ ! -z "$1" ]]; then
  ROOT="$1"
fi

case $ID in
  "debian" | "ubuntu" | "elementary")
    FILE_EXTENSION=".deb"
    SUFFIX="-$ID-$VERSION_CODENAME-$VERSION_ID"
    DIR="debian"
  ;;
  "fedora")
    FILE_EXTENSION=".rpm"
    SUFFIX=".$ID-$(rpm -E %fedora)"
    DIR="generate-rpm"
  ;;
esac

package_file=$(find "$ROOT"/target/x86_64-unknown-linux-musl/"$DIR" | egrep '*\.(deb|rpm)' | head -n1)
file_name=$(basename -s "$FILE_EXTENSION" $package_file)
output_file="$(dirname $package_file)/$file_name$SUFFIX$FILE_EXTENSION"

mv "$package_file" "$output_file"
