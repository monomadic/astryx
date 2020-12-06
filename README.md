<p align="center"><img src="https://raw.githubusercontent.com/monomadic/astryx/master/assets/logo.svg" /></p>

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.42.0+-green.svg)](#rust-version-requirements)

Astryx is a declarative, pure, expressive static web content tool. It walks the line
between being a simple language and a compiler environment, taking static generators
to a different place.

I personally developed the language over the 2020 lockdown period, over a year of many
rewrites and refactors.

__NOTE: pre-production state, not even alpha yet. pushing updates regularly, check the kanban board for progress__

## Features
* declarative, type-checked, intuitive ui language
* web based primitive objects (html elements, styles, etc)
* single binary (makes CI/CD into github/gitlab pages very simple)
* zero boilerplate
* zero orphan files
* smaller static sites than any other library.

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
