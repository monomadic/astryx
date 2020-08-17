<p align="center"><img src="https://raw.githubusercontent.com/monomadic/astryx/master/assets/logo.svg" /></p>

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.42.0+-green.svg)](#rust-version-requirements)

Astryx is a declarative, pure, expressive static web content compiler.
It draws inspiration from projects like elm-ui, haml and svelte, with the
type safety of rust and far, far easier setup and deployment.

## Features
* declarative, type-checked, intuitive ui language
* clean separation of style, layout, and content
* single binary (makes CI/CD into github/gitlab pages very simple)
* zero boilerplate
* zero orphans
* smaller static sites than any other library, period.

__NOTE: pre-production state, not even alpha yet. pushing updates regularly, check the kanban board for progress__

## Compiling / Running

``` bash
cargo run --package cli -- serve examples/general.astryx
```

### NixOS

A NixOS shell has been provided for convenience.

## Installation

### Cargo

``` bash
cargo install --path https://github.com/monomadic/astryx
# or, locally:
cargo install --path components/cli
```

### NixOS / Nixpkgs (soon...)

``` bash
nix-env --install astryx --file https://github.com/monomadic/astryx/astryx.nix
```
