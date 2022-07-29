#!/usr/bin/env bash


source /etc/os-release

set -xe

echo -e "\nInstalling dependencies for your distro...\n"

case $ID in
    "debian" | "ubuntu" | "elementary")
        sudo apt install build-essential gcc clang -y
        sudo apt install libglib2.0-dev libpango1.0-dev libgdk-pixbuf-2.0-dev libatk1.0-dev libgtk-3-dev libxcb-shape0-dev libxcb-xfixes0-dev curl -y
    ;;
    "fedora")
        sudo dnf groupinstall "Development Tools" -y
        sudo dnf install gobject-introspection-devel cairo-devel atk-devel pango-devel gdk-pixbuf2-devel gtk3-devel clang curl cmake -y
    ;;
esac

if [[ ! -z $(which rustup 2>/dev/null ) ]]; then
    echo -e "\nInstalling wasm32-unknown-unknown Rust target..."
    rustup target install wasm32-unknown-unknown
fi
