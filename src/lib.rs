#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Interoperability library for Rust Windowing applications.
//!
//! This library provides standard types for accessing a window's platform-specific raw window
//! handle and platforms display handle. This does not provide any utilities for creating and
//! managing windows; instead, it provides a common interface that window creation libraries (e.g.
//! Winit, SDL) can use to easily talk with graphics libraries (e.g. gfx-hal).
//!
//! ## Safety guarantees
//!
//! Please see the docs of [`HasRawWindowHandle`] and [`HasRawDisplayHandle`].
//!
//! ## Platform handle initialization
//!
//! Each platform handle struct is purposefully non-exhaustive, so that additional fields may be
//! added without breaking backwards compatibility. Each struct provides an `empty` method that may
//! be used along with the struct update syntax to construct it. See each specific struct for
//! examples.
//!
//! ## Display Handles
//!
//! Some windowing systems use a separate display handle for some operations. The display usually
//! represents a connection to some display server, but it is not necessarily tied to a particular
//! window. See [`RawDisplayHandle`] for more details.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod android;
mod appkit;
#[cfg(any(feature = "std", not(target_os = "android")))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", not(target_os = "android")))))]
mod borrowed;
mod haiku;
mod redox;
mod uikit;
mod unix;
mod web;
mod windows;

pub use android::{AndroidDisplayHandle, AndroidNdkWindowHandle};
pub use appkit::{AppKitDisplayHandle, AppKitWindowHandle};
#[cfg(any(feature = "std", not(target_os = "android")))]
pub use borrowed::{
    Active, ActiveHandle, DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle,
    WindowHandle,
};
pub use haiku::{HaikuDisplayHandle, HaikuWindowHandle};
pub use redox::{OrbitalDisplayHandle, OrbitalWindowHandle};
pub use uikit::{UiKitDisplayHandle, UiKitWindowHandle};
pub use unix::{
    DrmDisplayHandle, DrmWindowHandle, GbmDisplayHandle, GbmWindowHandle, WaylandDisplayHandle,
    WaylandWindowHandle, XcbDisplayHandle, XcbWindowHandle, XlibDisplayHandle, XlibWindowHandle,
};
pub use web::{WebDisplayHandle, WebWindowHandle};
pub use windows::{Win32WindowHandle, WinRtWindowHandle, WindowsDisplayHandle};

/// Window that wraps around a raw window handle.
///
/// # Safety
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

unsafe impl<'a, T: HasRawWindowHandle + ?Sized> HasRawWindowHandle for &'a T {
    fn raw_window_handle(&self) -> RawWindowHandle {
        (*self).raw_window_handle()
    }
}
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
unsafe impl<T: HasRawWindowHandle + ?Sized> HasRawWindowHandle for alloc::rc::Rc<T> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        (**self).raw_window_handle()
    }
}
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
unsafe impl<T: HasRawWindowHandle + ?Sized> HasRawWindowHandle for alloc::sync::Arc<T> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        (**self).raw_window_handle()
    }
}

/// A window handle for a particular windowing system.
///
/// Each variant contains a struct with fields specific to that windowing system
/// (e.g. [`Win32WindowHandle`] will include a [HWND], [`WaylandWindowHandle`] uses [wl_surface],
/// etc.)
///
/// [HWND]: https://learn.microsoft.com/en-us/windows/win32/winmsg/about-windows#window-handle
/// [wl_surface]: https://wayland.freedesktop.org/docs/html/apa.html#protocol-spec-wl_surface
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
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawWindowHandle {
    /// A raw window handle for UIKit (Apple's non-macOS windowing library).
    ///
    /// ## Availability Hints
    /// This variant is likely to be used on iOS, tvOS, (in theory) watchOS, and
    /// Mac Catalyst (`$arch-apple-ios-macabi` targets, which can notably use
    /// UIKit *or* AppKit), as these are the targets that (currently) support
    /// UIKit.
    UiKit(UiKitWindowHandle),
    /// A raw window handle for AppKit.
    ///
    /// ## Availability Hints
    /// This variant is likely to be used on macOS, although Mac Catalyst
    /// (`$arch-apple-ios-macabi` targets, which can notably use UIKit *or*
    /// AppKit) can also use it despite being `target_os = "ios"`.
    AppKit(AppKitWindowHandle),
    /// A raw window handle for the Redox operating system.
    ///
    /// ## Availability Hints
    /// This variant is used by the Orbital Windowing System in the Redox
    /// operating system.
    Orbital(OrbitalWindowHandle),
    /// A raw window handle for Xlib.
    ///
    /// ## Availability Hints
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that Xlib can be built for, which is to say, most (but not all)
    /// Unix systems.
    Xlib(XlibWindowHandle),
    /// A raw window handle for Xcb.
    ///
    /// ## Availability Hints
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that XCB can be built for, which is to say, most (but not all)
    /// Unix systems.
    Xcb(XcbWindowHandle),
    /// A raw window handle for Wayland.
    ///
    /// ## Availability Hints
    /// This variant should be expected anywhere Wayland works, which is
    /// currently some subset of unix systems.
    Wayland(WaylandWindowHandle),
    /// A raw window handle for the Linux Kernel Mode Set/Direct Rendering Manager
    ///
    /// ## Availability Hints
    /// This variant is used on Linux when neither X nor Wayland are available
    Drm(DrmWindowHandle),
    /// A raw window handle for the Linux Generic Buffer Manager.
    ///
    /// ## Availability Hints
    /// This variant is present regardless of windowing backend and likely to be used with
    /// EGL_MESA_platfrom_gbm or EGL_KHR_platfrom_gbm.
    Gbm(GbmWindowHandle),
    /// A raw window handle for Win32.
    ///
    /// ## Availability Hints
    /// This variant is used on Windows systems.
    Win32(Win32WindowHandle),
    /// A raw window handle for WinRT.
    ///
    /// ## Availability Hints
    /// This variant is used on Windows systems.
    WinRt(WinRtWindowHandle),
    /// A raw window handle for the Web.
    ///
    /// ## Availability Hints
    /// This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
    Web(WebWindowHandle),
    /// A raw window handle for Android NDK.
    ///
    /// ## Availability Hints
    /// This variant is used on Android targets.
    AndroidNdk(AndroidNdkWindowHandle),
    /// A raw window handle for Haiku.
    ///
    /// ## Availability Hints
    /// This variant is used on HaikuOS.
    Haiku(HaikuWindowHandle),
}

/// Display that wraps around a raw display handle.
///
/// # Safety
///
/// Users can safely assume that non-`null`/`0` fields are valid handles, and it is up to the
/// implementer of this trait to ensure that condition is upheld.
///
/// Despite that qualification, implementers should still make a best-effort attempt to fill in all
/// available fields. If an implementation doesn't, and a downstream user needs the field, it should
/// try to derive the field from other fields the implementer *does* provide via whatever methods the
/// platform provides.
///
/// The exact handles returned by `raw_display_handle` must remain consistent between multiple calls
/// to `raw_display_handle` as long as not indicated otherwise by platform specific events.
pub unsafe trait HasRawDisplayHandle {
    fn raw_display_handle(&self) -> RawDisplayHandle;
}

unsafe impl<'a, T: HasRawDisplayHandle + ?Sized> HasRawDisplayHandle for &'a T {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        (*self).raw_display_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
unsafe impl<T: HasRawDisplayHandle + ?Sized> HasRawDisplayHandle for alloc::rc::Rc<T> {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        (**self).raw_display_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
unsafe impl<T: HasRawDisplayHandle + ?Sized> HasRawDisplayHandle for alloc::sync::Arc<T> {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        (**self).raw_display_handle()
    }
}

/// A display server handle for a particular windowing system.
///
/// The display usually represents a connection to some display server, but it is not necessarily
/// tied to a particular window. Some APIs can use the display handle without ever creating a window
/// handle (e.g. offscreen rendering, headless event handling).
///
/// Each variant contains a struct with fields specific to that windowing system
/// (e.g. [`XlibDisplayHandle`] contains a [Display] connection to an X Server,
/// [`WaylandDisplayHandle`] uses [wl_display] to connect to a compositor). Not all windowing
/// systems have a separate display handle (or they haven't been implemented yet) and their variants
/// contain empty structs.
///
/// [Display]: https://www.x.org/releases/current/doc/libX11/libX11/libX11.html#Display_Functions
/// [wl_display]: https://wayland.freedesktop.org/docs/html/apb.html#Client-classwl__display
///
/// # Variant Availability
///
/// Note that all variants are present on all targets (none are disabled behind
/// `#[cfg]`s), but see the "Availability Hints" section on each variant for
/// some hints on where this variant might be expected.
///
/// Note that these "Availability Hints" are not normative. That is to say, a
/// [`HasRawDisplayHandle`] implementor is completely allowed to return something
/// unexpected. (For example, it's legal for someone to return a
/// [`RawDisplayHandle::Xlib`] on macOS, it would just be weird, and probably
/// requires something like XQuartz be used).
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawDisplayHandle {
    /// A raw display handle for UIKit (Apple's non-macOS windowing library).
    ///
    /// ## Availability Hints
    /// This variant is likely to be used on iOS, tvOS, (in theory) watchOS, and
    /// Mac Catalyst (`$arch-apple-ios-macabi` targets, which can notably use
    /// UIKit *or* AppKit), as these are the targets that (currently) support
    /// UIKit.
    UiKit(UiKitDisplayHandle),
    /// A raw display handle for AppKit.
    ///
    /// ## Availability Hints
    /// This variant is likely to be used on macOS, although Mac Catalyst
    /// (`$arch-apple-ios-macabi` targets, which can notably use UIKit *or*
    /// AppKit) can also use it despite being `target_os = "ios"`.
    AppKit(AppKitDisplayHandle),
    /// A raw display handle for the Redox operating system.
    ///
    /// ## Availability Hints
    /// This variant is used by the Orbital Windowing System in the Redox
    /// operating system.
    Orbital(OrbitalDisplayHandle),
    /// A raw display handle for Xlib.
    ///
    /// ## Availability Hints
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that Xlib can be built for, which is to say, most (but not all)
    /// Unix systems.
    Xlib(XlibDisplayHandle),
    /// A raw display handle for Xcb.
    ///
    /// ## Availability Hints
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that XCB can be built for, which is to say, most (but not all)
    /// Unix systems.
    Xcb(XcbDisplayHandle),
    /// A raw display handle for Wayland.
    ///
    /// ## Availability Hints
    /// This variant should be expected anywhere Wayland works, which is
    /// currently some subset of unix systems.
    Wayland(WaylandDisplayHandle),
    /// A raw display handle for the Linux Kernel Mode Set/Direct Rendering Manager
    ///
    /// ## Availability Hints
    /// This variant is used on Linux when neither X nor Wayland are available
    Drm(DrmDisplayHandle),
    /// A raw display handle for the Linux Generic Buffer Manager.
    ///
    /// ## Availability Hints
    /// This variant is present regardless of windowing backend and likely to be used with
    /// EGL_MESA_platfrom_gbm or EGL_KHR_platfrom_gbm.
    Gbm(GbmDisplayHandle),
    /// A raw display handle for Win32.
    ///
    /// ## Availability Hints
    /// This variant is used on Windows systems.
    Windows(WindowsDisplayHandle),
    /// A raw display handle for the Web.
    ///
    /// ## Availability Hints
    /// This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
    Web(WebDisplayHandle),
    /// A raw display handle for Android NDK.
    ///
    /// ## Availability Hints
    /// This variant is used on Android targets.
    Android(AndroidDisplayHandle),
    /// A raw display handle for Haiku.
    ///
    /// ## Availability Hints
    /// This variant is used on HaikuOS.
    Haiku(HaikuDisplayHandle),
}

macro_rules! from_impl {
    ($($to:ident, $enum:ident, $from:ty)*) => ($(
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                $to::$enum(value)
            }
        }
    )*)
}

from_impl!(RawDisplayHandle, UiKit, UiKitDisplayHandle);
from_impl!(RawDisplayHandle, AppKit, AppKitDisplayHandle);
from_impl!(RawDisplayHandle, Orbital, OrbitalDisplayHandle);
from_impl!(RawDisplayHandle, Xlib, XlibDisplayHandle);
from_impl!(RawDisplayHandle, Xcb, XcbDisplayHandle);
from_impl!(RawDisplayHandle, Wayland, WaylandDisplayHandle);
from_impl!(RawDisplayHandle, Drm, DrmDisplayHandle);
from_impl!(RawDisplayHandle, Gbm, GbmDisplayHandle);
from_impl!(RawDisplayHandle, Windows, WindowsDisplayHandle);
from_impl!(RawDisplayHandle, Web, WebDisplayHandle);
from_impl!(RawDisplayHandle, Android, AndroidDisplayHandle);
from_impl!(RawDisplayHandle, Haiku, HaikuDisplayHandle);

from_impl!(RawWindowHandle, UiKit, UiKitWindowHandle);
from_impl!(RawWindowHandle, AppKit, AppKitWindowHandle);
from_impl!(RawWindowHandle, Orbital, OrbitalWindowHandle);
from_impl!(RawWindowHandle, Xlib, XlibWindowHandle);
from_impl!(RawWindowHandle, Xcb, XcbWindowHandle);
from_impl!(RawWindowHandle, Wayland, WaylandWindowHandle);
from_impl!(RawWindowHandle, Drm, DrmWindowHandle);
from_impl!(RawWindowHandle, Gbm, GbmWindowHandle);
from_impl!(RawWindowHandle, Win32, Win32WindowHandle);
from_impl!(RawWindowHandle, WinRt, WinRtWindowHandle);
from_impl!(RawWindowHandle, Web, WebWindowHandle);
from_impl!(RawWindowHandle, AndroidNdk, AndroidNdkWindowHandle);
from_impl!(RawWindowHandle, Haiku, HaikuWindowHandle);
