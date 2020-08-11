<p align="center"><img src="https://raw.githubusercontent.com/monomadic/astryx/master/assets/logo.svg" /></p>

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.42.0+-green.svg)](#rust-version-requirements)

Astryx is a declarative, pure, expressive static web content compiler..

## Why did I make this?

Architecturally and practically, Astryx is quite different from other web generators. It is closer to a flutter or elm than other static tools, but much easier to use and rapidly produce content.

- single binary (makes CI/CD into github/gitlab pages very simple)
- declarative, safe, correct language
- zero boilerplate
- (optionally) a single astryx program generates an entire site of static content and files including html, css, images to a degree, and scripts.
- not specific to blogs, expressive enough for any applicable use case including SPAs

__NOTE: pre-production state, not even alpha yet. pushing updates regularly, check the kanban board for progress__

- example code

Looks too magical? Actually it tries to be extremely reasonable and logical.

## Compiling / Running

``` bash
cargo run --package astryx-cli -- serve examples/general.astryx
```

### NixOS

A NixOS shell has been provided for convenience.

## Installation

### Cargo

``` bash
cargo install --path https://github.com/monomadic/astryx
# or, locally:
cargo install --path cli
```

### NixOS / Nixpkgs (soon...)

``` bash
nix-env --install astryx --file https://github.com/monomadic/astryx/astryx.nix
```
