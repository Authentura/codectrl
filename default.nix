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
    cmake
    gtk3
    pkg-config
    rustPlatform.rust.cargo
    rustPlatform.rust.rustc
  ];

  shellHook = ''
    nix-channel --add https://github.com/guibou/nixGL/archive/main.tar.gz nixgl && nix-channel --update
    nix-env -iA nixgl.auto.nixGLDefault

    alias cargo="nixGL cargo"

    echo "Entered Nix-Shell environment..."
  '';
}
