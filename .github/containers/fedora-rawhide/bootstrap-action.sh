#!/usr/bin/env bash


source /etc/os-release

set -xe

echo -e "\nInstalling dependencies for your distro...\n"

case $ID in
    "debian" | "ubuntu" | "elementary")
        export DEBIAN_FRONTEND=noninteractive

        apt update -y
        pixbuf_dev=""
        libc_static=""
        case "$VERSION_CODENAME" in
            "focal" | "buster")
                pixbuf_dev="libgdk-pixbuf2.0-dev"
            ;;
            *)
                pixbuf_dev="libgdk-pixbuf-2.0-dev"
            ;;
        esac
        apt install build-essential clang cmake libglib2.0-dev libpango1.0-dev "$pixbuf_dev" libatk1.0-dev libgtk-3-dev libxcb-shape0-dev libxcb-xfixes0-dev curl libc6-dev libsqlite3-dev git -y
    ;;
    "fedora" | "rocky" | "centos")
        packages=(gobject-introspection-devel cairo-devel atk-devel pango-devel gdk-pixbuf2-devel gtk3-devel clang curl cmake git)

        dnf update -y
        dnf groupinstall "Development Tools" -y

        if [[ "$ID" == "rocky" || "$ID" == "centos" ]]; then
            dnf --enablerepo=crb install ${packages[@]}
        else
            dnf install ${packages[@]}
        fi
    ;;
esac

if [[ ! -z $(which rustup 2>/dev/null ) ]]; then
    echo -e "\nInstalling wasm32-unknown-unknown Rust target..."
    rustup target install wasm32-unknown-unknown
fi
