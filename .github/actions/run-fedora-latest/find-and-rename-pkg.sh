#!/usr/bin/env bash

source /etc/os-release

set -xe

SUFFIX=""
DIR=""
ROOT="."

if [[ ! -z "$1" ]]; then
  ROOT="$1"
fi

case $ID in
  "debian" | "ubuntu" | "elementary")
    SUFFIX="-$ID-$VERSION_CODENAME-$VERSION_ID.deb"
    DIR="debian"
  ;;
  "fedora")
    SUFFIX="-$ID-$(rpm -E %fedora).rpm"
    DIR="generate-rpm"
  ;;
esac

package_file=$(find "$ROOT"/target/"$DIR" | egrep '*\.(deb|rpm)' | head -n1)
file_name=$(basename $package_file)
output_file="$(dirname $package_file)/$file_name$SUFFIX"

mv "$package_file" "$output_file"
