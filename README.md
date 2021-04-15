<p align="center"><img src="https://raw.githubusercontent.com/monomadic/astryx/master/assets/logo.svg" /></p>

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.42.0+-green.svg)](#rust-version-requirements)

_Note: Astryx is still in a very alpha state._

Astryx is a declarative, safe, expressive language and compiler for generating static sites and single page web applications. It draws heavy inspiration from the projects svelte, deno, and flutter, but aims to be leaner, much faster, and far simpler to use and master.

- It is similar to **svelte** in that it acts as a complete compiler, not a framework or library. Output is as lean as possible.

- It is similar to **deno** in that it deploys as a single, portable rust binary, with minimal allocations during compilation and is extremely fast. If you're coming from node or a javascript compiler, the difference is brain-rocking.

- It is similar to **flutter** through use of simple declarative code and the redux data flow architecture (soon).

- It is similar to **rust** in its use of highly correct type safety, attempting to capture as many errors as possible during compile time, so they don't make it to runtime.

It is totally unlike and far more powerful than simple templating languages and static content generators, which tend to focus on themes, blogs, etc. Astryx could build a simple blog program like hugo in a few lines of code, or an image gallery constructor, a cryptocurrency tracker, etc.

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
