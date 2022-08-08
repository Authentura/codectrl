{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  name = "dev";
  buildInputs = [
    pkgs.rustup
    pkgs.cmake
  ];

  shellHook = ''
    rustup update
  '';
}
