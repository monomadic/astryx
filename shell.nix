{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    cmake
    pkgconfig
    rustc
    rustfmt
    python36
  ];
}
