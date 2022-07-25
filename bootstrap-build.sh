#!/usr/bin/env bash

source /etc/os-release

echo -e "\nInstalling dependencies for your distro...\n"

case $ID in
    "debian" | "ubuntu" | "elementary")
        sudo apt install build-essential gcc clang -y
        sudo apt install libglib2.0-dev libpango1.0-dev libgdk-pixbuf-2.0-dev libatk1.0-dev libgtk-3-dev libxcb-shape0-dev libxcb-xfixes0-dev -y
    ;;
    "fedora")
        sudo dnf groupinstall "Development Tools" -y
        sudo dnf install gobject-introspection-devel cairo-devel atk-devel pango-devel gdk-pixbuf2-devel gtk3-devel clang -y
    ;;
    "arch")
        # TODO: Add Arch Linux packages
    ;;
    *)
        echo -e "Unknown distribution, please manually find and install the relevant packages for your distro.\n"
    ;;
esac

if [[ ! -z $(which rustup 2>/dev/null) ]]; then
    echo "Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- --default-toolchain nightly -y
    source $HOME/.cargo/env
fi

echo -e "\nInstalling wasm32-unknown-unknown Rust target..."
rustup target install wasm32-unknown-unknown

echo -e "\nInstalling trunk..."
cargo install trunk

echo -e "\nIf you're inside a toolbox or distrobox, you need to install the necessary display drivers for your GPU *inside* the container otherwise you will run into OpenGL issues."
