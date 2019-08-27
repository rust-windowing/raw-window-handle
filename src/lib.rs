//! Interoperability library for Rust Windowing applications.
//!
//! This library provides standard types for accessing a window's platform-specific raw window
//! handle. This does not provide any utilities for creating and managing windows; instead, it
//! provides a common interface that window creation libraries (e.g. Winit, SDL) can use to easily
//! talk with graphics libraries (e.g. gfx-hal).
//!
//! ## Platform handle initialization
//!
//! Each platform handle struct is purposefully non-exhaustive, so that additional fields may be
//! added without breaking backwards compatibility. Each struct provides an `empty` method that may
//! be used along with the struct update syntax to construct it. See each specific struct for
//! examples.
//!
#![cfg_attr(feature = "nightly-docs", feature(doc_cfg))]
#![no_std]

#[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "macos")))]
#[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "macos"))]
pub mod macos;
#[cfg_attr(
    feature = "nightly-docs",
    doc(cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    )))
)]
#[cfg_attr(
    not(feature = "nightly-docs"),
    cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))
)]
pub mod unix;
#[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "windows")))]
#[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "windows"))]
pub mod windows;
// pub mod android;
#[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "ios")))]
#[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "ios"))]
pub mod ios;
// pub mod wasm;

mod platform {
    #[cfg(target_os = "macos")]
    pub use crate::macos::*;
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub use crate::unix::*;
    #[cfg(target_os = "windows")]
    pub use crate::windows::*;
    // #[cfg(target_os = "android")]
    // #[path = "android/mod.rs"]
    // mod platform;
    #[cfg(target_os = "ios")]
    pub use crate::ios::*;
}

/// Window that wraps around a raw window handle.
///
/// It is entirely valid behavior for fields within each platform-specific `RawWindowHandle` variant
/// to be `null` or `0`, and appropriate checking should be done before the handle is used. However,
/// users can safely assume that non-`null`/`0` fields are valid handles, and it is up to the
/// implementor of this trait to ensure that condition is upheld.
///
/// The exact handle returned by `raw_window_handle` must not change during the lifetime of this
/// trait's implementor.
pub unsafe trait HasRawWindowHandle {
    fn raw_window_handle(&self) -> RawWindowHandle;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawWindowHandle {
    #[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "ios")))]
    #[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "ios"))]
    IOS(ios::IOSHandle),

    #[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "macos")))]
    #[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "macos"))]
    MacOS(macos::MacOSHandle),

    #[cfg_attr(
        feature = "nightly-docs",
        doc(cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        )))
    )]
    #[cfg_attr(
        not(feature = "nightly-docs"),
        cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))
    )]
    X11(unix::X11Handle),

    #[cfg_attr(
        feature = "nightly-docs",
        doc(cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        )))
    )]
    #[cfg_attr(
        not(feature = "nightly-docs"),
        cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))
    )]
    Wayland(unix::WaylandHandle),

    #[cfg_attr(feature = "nightly-docs", doc(cfg(target_os = "windows")))]
    #[cfg_attr(not(feature = "nightly-docs"), cfg(target_os = "windows"))]
    Windows(windows::WindowsHandle),

    #[doc(hidden)]
    #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
    __NonExhaustiveDoNotUse(seal::Seal),
}

mod seal {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Seal;
}
