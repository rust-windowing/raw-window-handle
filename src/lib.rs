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

pub mod android;
pub mod ios;
pub mod macos;
pub mod unix;
pub mod web;
pub mod windows;

mod platform {
    pub use crate::android::*;
    pub use crate::ios::*;
    pub use crate::macos::*;
    pub use crate::unix::*;
    pub use crate::web::*;
    pub use crate::windows::*;
}

/// Window that wraps around a raw window handle.
///
/// # Safety guarantees
///
/// Users can safely assume that non-`null`/`0` fields are valid handles, and it is up to the
/// implementer of this trait to ensure that condition is upheld. However, It is entirely valid
/// behavior for fields within each platform-specific `RawWindowHandle` variant to be `null` or
/// `0`, and appropriate checking should be done before the handle is used.
///
/// Despite that qualification, implementers should still make a best-effort attempt to fill in all
/// available fields. If an implementation doesn't, and a downstream user needs the field, it should
/// try to derive the field from other fields the implementer *does* provide via whatever methods the
/// platform provides.
///
/// The exact handles returned by `raw_window_handle` must remain consistent between multiple calls
/// to `raw_window_handle`, and must be valid for at least the lifetime of the `HasRawWindowHandle`
/// implementer.
pub unsafe trait HasRawWindowHandle {
    fn raw_window_handle(&self) -> RawWindowHandle;
}

/// A "raw" handle to some specific window library or system.
///
/// # Variant Availability
///
/// Note that all variants are present on all targets (none are disabled behind
/// `#[cfg]`s), but see the "Availability Hints" section on each variant for
/// some hints on where this variant might be expected.
///
/// Note that these "Availability Hints" are not normative. That is to say, a
/// [`HasRawWindowHandle`] implementor is completely allowed to return something
/// unexpected. (For example, it's legal for someone to return a
/// [`RawWindowHandle::Xlib`] on macOS, it would just be weird, and probably
/// requires something like XQuartz be used).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawWindowHandle {
    /// A raw window handle for UIKit (Apple's non-macOS windowing library).
    ///
    /// # Availability Hints
    /// This variant is likely to be used on iOS, tvOS, (in theory) watchOS, and
    /// Mac Catalyst (`$arch-apple-ios-macabi` targets, which can notably use
    /// UIKit *or* AppKit), as these are the targets that (currently) support
    /// UIKit.
    IOS(ios::IOSHandle),
    /// A raw window handle for AppKit.
    ///
    /// # Availability Hints
    /// This variant is likely to be used on macOS, although Mac Catalyst
    /// (`$arch-apple-ios-macabi` targets, which can notably use UIKit *or*
    /// AppKit) can also use it despite being `target_os = "ios"`.
    MacOS(macos::MacOSHandle),
    /// A raw window handle for Xlib.
    ///
    /// # Availability Hints
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that Xlib can be built for, which is to say, most (but not all)
    /// Unix systems.
    Xlib(unix::XlibHandle),
    /// A raw window handle for Xcb.
    ///
    /// # Availability Hints
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that XCB can be built for, which is to say, most (but not all)
    /// Unix systems.
    Xcb(unix::XcbHandle),
    /// A raw window handle for Wayland.
    ///
    /// # Availability Hints
    /// This variant should be expected anywhere Wayland works, which is
    /// currently some subset of unix systems.
    Wayland(unix::WaylandHandle),
    /// A raw window handle for Windows.
    ///
    /// # Availibility Hints
    /// This variant is used on windows systems.
    Windows(windows::WindowsHandle),
    /// A raw window handle for the web.
    ///
    /// # Availibility Hints
    /// This variant is used on wasm or asmjs targets when targeting the web/html5.
    Web(web::WebHandle),
    /// A raw window handle for Android.
    ///
    /// # Availibility Hints
    /// This variant is used on android targets.
    Android(android::AndroidHandle),

    #[doc(hidden)]
    #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
    __NonExhaustiveDoNotUse(seal::Seal),
}

mod seal {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Seal;
}

/// This wraps a [`RawWindowHandle`] to give it a [`HasRawWindowHandle`] impl.
///
/// The `HasRawWindowHandle` trait must be an `unsafe` trait because *other*
/// unsafe code is going to rely on it to provide accurate window handle info.
/// Since `RawWindowHandle` is an enum and enum fields are public, anyone could
/// make any random `RawWindowHandle` value in safe code.
///
/// The solution is that you assert that you're trusting a particular handle
/// value by (unsafely) placing it within this wrapper struct.
pub struct TrustedWindowHandle {
    raw: RawWindowHandle,
}
impl TrustedWindowHandle {
    /// Assert that the `RawWindowHandle` value can be trusted.
    ///
    /// ## Safety
    /// If the value violates any of the safety outlines given in the
    /// [`HasRawWindowHandle`] trait this can lead to UB.
    pub const unsafe fn new(raw: RawWindowHandle) -> Self {
        Self { raw }
    }
}
unsafe impl HasRawWindowHandle for TrustedWindowHandle {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.raw
    }
}
