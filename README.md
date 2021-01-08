<p align="center"><img src="https://raw.githubusercontent.com/monomadic/astryx/master/assets/logo.svg" /></p>

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.42.0+-green.svg)](#rust-version-requirements)

Astryx is a declarative, safe, expressive DSL for generating static and semi-static web content. It is not designed to be a general purpose language. It is developed in rust as a single binary with minimal allocations during compilation and is designed to be extremely fast.

As Astryx is fast and correct (safety wherever possible) and a single binary it can easily fit into a CI/CD workflow (or github actions) and generate static content in a psuedo-dynamic fashion. For example, it could pull data from an api, process images, and generate all required output (html, css, js, etc) to static files at periodic intervals, so many sites which previously could only be dynamic now can benefit from static caching and cdn delivery.

It is totally unlike and far more powerful than other templating languages and static content generators, which tend to focus on themes, blogs, etc. Astryx could build a simple blog program in a few lines of code, or an image gallery, a cryptocurrency tracker, anything.

It's primary goals are:
- rapid, simple, intuitive content creation
  - a single astryx program can generate an entire site with many pages
  - could replicate the behavior of much more complicated projects simply and rapidly to static content
- safe domain types (html, css, markdown, etc)
- elimites invalid state (like a typed compiler but for web)
  - type checking
  - asset checking
  - api validation

__NOTE: pre-production state, not even alpha yet. pushing updates regularly, check the kanban board for progress__

## Compiling / Running (with cargo)

``` bash
cargo run -- serve examples/pages.astryx
```

Or start a server:
``` bash
cargo run -- serve examples/pages.astryx
```

### NixOS

A NixOS shell has been provided for convenience. This will be a flake very soon.

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
