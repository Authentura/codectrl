{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  name = "dev";
  buildInputs = [
    pkgs.rustup
    pkgs.cargo
    pkgs.cmake
  ];
  shellHook = ''
    rustup update
  '';
}
