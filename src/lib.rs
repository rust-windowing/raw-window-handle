#![no_std]

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

pub mod android;
pub mod ios;
pub mod macos;
pub mod redox;
pub mod unix;
pub mod web;
pub mod windows;

/// Window that wraps around a raw window handle.
///
/// # Safety guarantees
///
/// Users can safely assume that non-`null`/`0` fields are valid handles, and it is up to the
/// implementer of this trait to ensure that condition is upheld.
///
/// Despite that qualification, implementers should still make a best-effort attempt to fill in all
/// available fields. If an implementation doesn't, and a downstream user needs the field, it should
/// try to derive the field from other fields the implementer *does* provide via whatever methods the
/// platform provides.
///
/// The exact handles returned by `raw_window_handle` must remain consistent between multiple calls
/// to `raw_window_handle` as long as not indicated otherwise by platform specific events.
pub unsafe trait HasRawWindowHandle {
    fn raw_window_handle(&self) -> RawWindowHandle;
}

/// An enum to simply combine the different possible raw window handle variants.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawWindowHandle {
    IOS(ios::IOSHandle),

    MacOS(macos::MacOSHandle),

    Redox(redox::RedoxHandle),

    Xlib(unix::XlibHandle),

    Xcb(unix::XcbHandle),

    Wayland(unix::WaylandHandle),

    Windows(windows::WindowsHandle),
  
    WinRT(windows::WinRTHandle),
  
    Web(web::WebHandle),

    Android(android::AndroidHandle),
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
    /// Assert that the [`RawWindowHandle`] value can be trusted.
    ///
    /// ## Safety
    /// If the value violates any of the safety outlines given in the
    /// [`HasRawWindowHandle`] trait this can lead to UB.
    pub const unsafe fn new(raw: RawWindowHandle) -> Self {
        Self { raw }
    }

    /// Read from a [`HasRawWindowHandle`] into being a trusted value.
    pub fn from_has_raw_window_handle<H: HasRawWindowHandle>(fr: &H) -> Self {
        // Safety: Because `HasRawWindowHandle` is an unsafe trait, we can trust
        // that it gives a correct handle. If not, the fault lies with the trait
        // implementation, not this function.
        Self {
            raw: fr.raw_window_handle(),
        }
    }
}
unsafe impl HasRawWindowHandle for TrustedWindowHandle {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.raw
    }
}
