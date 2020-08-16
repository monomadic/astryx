{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  name = "rust-env";

  buildInputs = with pkgs; [
    rustc
    cargo
    clang
    openssl
    pkgconfig
  ];

  RUST_SRC_PATH = "${pkgs.rustc}/bin/rustc";

  shellHook = ''
  '';
}
