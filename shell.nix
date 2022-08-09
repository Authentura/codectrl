{
  mozillaOverlay ? import (
    builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
  ),
  pkgs ? import <nixpkgs> {
    overlays = [ mozillaOverlay ];
  },
  rust ? (pkgs.rustChannelOf { channel = "stable"; }).rust,
  rustPlatform ? pkgs.makeRustPlatform {
    cargo = rust;
    rustc = rust;
  },
  lib ? pkgs.lib
}:

pkgs.stdenv.mkDerivation rec {
  name = "build-shell";

  buildInputs = with pkgs; [
    # clang
    # mold
    atk
    cairo
    cmake
    gcc
    gdk-pixbuf
    glib
    glibc
    glibc.static
    gobject-introspection
    gtk3
    libxkbcommon
    pango
    pkg-config
    rustPlatform.rust.cargo
    rustPlatform.rust.rustc
    wayland
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];

  LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
  # RUSTFLAGS = "-Ctarget-feature=+crt-static -Clinker=clang -Clink-arg=-fuse-ld=${pkgs.mold}/bin/mold";
  RUSTFLAGS = "-Ctarget-feature=+crt-static";

  shellHook = ''
    echo "Entered Nix-Shell environment..."
  '';
}
