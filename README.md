# Resonance-Audio-Sys: low level Rust bindings to Resonance Audio

This repository contains low-level bindings for [Resonance Audio](https://developers.google.com/resonance-audio/)

## Documentation

Use `cargo doc --open`. This is not on crates.io yet, since this is highly experimental, and doesn't work yet due to problems  binding to C++.

## Requirements

* Llatest nightly Rust.
* The Resonance Audio library statically built for linking. See the [Resonance Audio repo](https://github.com/resonance-audio/resonance-audio) for build instructions. For rustc to find it, either set the `RESONANCE_LIB_DIR` environment variable to the directory containing the library, or place it in the lib subfolder. The latter option is provided as a development convenience.

## Installation

Clone from GitHub to obtain the latest development version, then `cargo build`.

## Generating the resonance-audio-sys bindings with bindgen

The optional feature "use-bindgen" generates the low level bindings using the included headers, which requires [bindgen](https://github.com/rust-lang-nursery/rust-bindgen) to be installed. Generating this shouldn't be necessary in most cases, as the bindings have already been bundled with this crate.

[crates]: https://crates.io/