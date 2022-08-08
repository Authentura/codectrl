{
    mozillaOverlay ? import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz),
    pkgs ? import <nixpkgs> {
        crossSystem = {
          config = "x86_64-unknown-linux-musl";
        };
        overlays = [ mozillaOverlay ];
    },
    rust ? (pkgs.rustChannelOf { channel = "stable"; }).rust.override {
        targets = [ "x86_64-unknown-linux-musl" ];
    },
    rustPlatform ? pkgs.makeRustPlatform {
        cargo = rust;
        rustc = rust;
    },
}:


pkgs.mkShell {
  name = "dev";
  buildInputs = [
    rustPlatform.rust.cargo
    rustPlatform.rust.rustc
    pkgs.cmake
    pkgs.musl
    pkgs.gtk3
  ];

  shellHook = ''
  '';
}
