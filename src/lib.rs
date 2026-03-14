#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::new_without_default)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Interoperability library for Rust Windowing applications.
//!
//! This library provides standard types for accessing a window's platform-specific raw window
//! handle and platforms display handle. This does not provide any utilities for creating and
//! managing windows; instead, it provides a common interface that window creation libraries (e.g.
//! Winit, SDL) can use to easily talk with graphics libraries (e.g. gfx-hal).
//!
//! ## Safety guarantees
//!
//! Please see the docs of [`HasWindowHandle`] and [`HasDisplayHandle`].
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
//!
//! ## Dependencies
//!
//! This library is intentionally dependency-free, to:
//! 1. Allow it to better interoperate with arbitrary versions of different crates in the ecosystem.
//! 2. Reduce compile-times, especially since it allows compiling consumer and producer crates in
//!    parallel.

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod borrowed;
mod raw_display;
mod raw_window;

pub use self::borrowed::{DisplayHandle, HasDisplayHandle, HasWindowHandle, WindowHandle};
pub use self::raw_display::RawDisplayHandle;
pub use self::raw_window::RawWindowHandle;

use core::fmt;

/// An error that can occur while fetching a display or window handle.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum HandleError {
    /// The underlying handle cannot be represented using the types in this crate.
    ///
    /// This may be returned if the underlying window system does not support any of the
    /// representative C window handles in this crate. For instance, if you were using a pure Rust
    /// library to set up X11 (like [`x11rb`]), you would not be able to use any of the
    /// [`RawWindowHandle`] variants, as they all represent C types.
    ///
    /// Another example would be a system that isn't supported by `raw-window-handle` yet,
    /// like some game consoles.
    ///
    /// In the event that this error is returned, you should try to use the underlying window
    /// system's native API to get the handle you need.
    ///
    /// [`x11rb`]: https://crates.io/crates/x11rb
    NotSupported,

    /// The underlying handle is not available.
    ///
    /// In some cases the underlying window handle may become temporarily unusable. For example, on
    /// Android, the native window pointer can arbitrarily be replaced or removed by the system. In
    /// this case, returning a window handle would be disingenuous, as it would not be usable. A
    /// similar situation can occur on Wayland for the layer shell windows.
    ///
    /// In the event that this error is returned, you should wait until the handle becomes available
    /// again.
    Unavailable,
}

impl fmt::Display for HandleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSupported => write!(
                f,
                "the underlying handle cannot be represented using the types in this crate"
            ),
            Self::Unavailable => write!(f, "the underlying handle is not available"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HandleError {}

#[cfg(test)]
mod tests {
    use core::panic::{RefUnwindSafe, UnwindSafe};
    use static_assertions::{assert_impl_all, assert_not_impl_any};

    use super::*;

    #[test]
    fn auto_traits() {
        assert_impl_all!(RawDisplayHandle: UnwindSafe, RefUnwindSafe, Unpin);
        assert_not_impl_any!(RawDisplayHandle: Send, Sync);
        assert_impl_all!(DisplayHandle<'_>: UnwindSafe, RefUnwindSafe, Unpin);
        assert_not_impl_any!(DisplayHandle<'_>: Send, Sync);
        assert_impl_all!(RawWindowHandle: UnwindSafe, RefUnwindSafe, Unpin);
        assert_not_impl_any!(RawWindowHandle: Send, Sync);
        assert_impl_all!(WindowHandle<'_>: UnwindSafe, RefUnwindSafe, Unpin);
        assert_not_impl_any!(WindowHandle<'_>: Send, Sync);
        assert_impl_all!(HandleError: Send, Sync, UnwindSafe, RefUnwindSafe, Unpin);
    }

    #[allow(unused)]
    fn assert_object_safe(_: &dyn HasWindowHandle, _: &dyn HasDisplayHandle) {}
}
