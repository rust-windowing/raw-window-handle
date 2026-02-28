# `raw-window-handle`: A common windowing interoperability library for Rust

[![Crates.io](https://img.shields.io/crates/v/raw-window-handle.svg?maxAge=2592000)](https://crates.io/crates/raw-window-handle)
[![Docs](https://docs.rs/raw-window-handle/badge.svg)](https://docs.rs/raw-window-handle)
[![CI Status](https://github.com/rust-windowing/raw-window-handle/workflows/CI/badge.svg)](https://github.com/rust-windowing/raw-window-handle/actions)

This crate is intended as an interoperability crate, that allows "producer"
crates to create a handle to a window surface, and for this handle to be
passed onwards to "consumer" crates for rendering to that surface.

This crate does not by itself provide any utilities for creating and managing
windows, nor for rendering into them; use Winit or SDL for the former, and
Softbuffer, Glutin or Wgpu for the latter.

## MSRV Policy

The Minimum Safe Rust Version (MSRV) of this crate as of the time of writing is
**1.64.0**. For pre-`1.0` releases of `raw-window-handle`, this version will not
be changed without a patch bump to the version of `raw-window-handle`. After
version `1.0.0` is released, changes to the MSRV will necessitate a minor
version bump.

When the `wasm-bindgen-0-2` feature is enabled, the MSRV of this crate will be
raised to the MSRV of the latest version of `wasm-bindgen`.
