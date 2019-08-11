//! Interoperability library for Rust Windowing applications.
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

//! Window that wraps around a raw window handle.
//!
//! It is entirely valid behavior for fields within each platform-specific `RawWindowHandle` variant
//! to be `null` or `0`, and appropriate checking should be done before the handle is used. However,
//! users can safely assume that non-`null`/`0` fields are valid handles, and it is up to the
//! implementor of this trait to ensure that condition is upheld.
pub unsafe trait HasRawWindowHandle {
    fn raw_window_handle(&self) -> RawWindowHandle;
}

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
    __NonExhaustiveDoNotUse(seal::Seal),
}

mod seal {
    pub enum Seal {}
}
