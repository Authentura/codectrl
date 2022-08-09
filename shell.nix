{
  mozillaOverlay ? import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz),
  pkgs ? import <nixpkgs> {
      overlays = [ mozillaOverlay ];
  },
  rust ? (pkgs.rustChannelOf { channel = "stable"; }).rust,
  rustPlatform ? pkgs.makeRustPlatform {
      cargo = rust;
      rustc = rust;
  },
}:


pkgs.mkShell {
  name = "build-shell";
  buildInputs = [
    rustPlatform.rust.cargo
    rustPlatform.rust.rustc
    pkgs.cmake
    pkgs.gtk3
  ];

  shellHook = ''
    export RUSTFLAGS="-Ctarget-feature=+crt-static"
    echo "Entered Nix-Shell environment..."
  '';
}
