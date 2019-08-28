//! Interoperability library for Rust Windowing applications.
//!
//! This library provides standard types for accessing a window's platform-specific raw window
//! handle. This does not provide any utilities for creating and managing windows; instead, it
//! provides a common interface that window creation libraries (e.g. Winit, SDL) can use to easily
//! talk with graphics libraries (e.g. gfx-hal).
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

pub use _trait::HasRawWindowHandle;

#[cfg(feature = "nightly-docs")]
mod _trait {
    /// Window that wraps around a raw window handle.
    ///
    /// It is entirely valid behavior for fields within each platform-specific `RawWindowHandle` variant
    /// to be `null` or `0`, and appropriate checking should be done before the handle is used. However,
    /// users can safely assume that non-`null`/`0` fields are valid handles, and it is up to the
    /// implementor of this trait to ensure that condition is upheld.
    pub unsafe trait HasRawWindowHandle {}
}

#[cfg(all(not(feature = "nightly-docs"), target_os = "macos"))]
mod _trait {
    pub unsafe trait HasRawWindowHandle: macos::HasMacOSHandle {}

    unsafe impl<T> HasRawWindowHandle for T where T: crate::unix::HasMacOSHandle {}
}

#[cfg(all(
    not(feature = "nightly-docs"),
    any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    )
))]
mod _trait {
    pub unsafe trait HasRawWindowHandle: crate::unix::HasUnixHandle {}

    unsafe impl<T> HasRawWindowHandle for T where T: crate::unix::HasUnixHandle {}
}

#[cfg(all(not(feature = "nightly-docs"), target_os = "windows"))]
mod _trait {
    pub unsafe trait HasRawWindowHandle: crate::windows::HasWindowsHandle {}

    unsafe impl<T> HasRawWindowHandle for T where T: crate::windows::HasWindowsHandle {}
}

#[cfg(all(not(feature = "nightly-docs"), target_os = "ios"))]
mod _trait {
    pub unsafe trait HasRawWindowHandle: crate::ios::HasIOSHandle {}

    unsafe impl<T> HasRawWindowHandle for T where T: crate::ios::HasIOSHandle {}
}
