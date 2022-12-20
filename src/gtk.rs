//! Handles to GTK windows and displays.

use core::ffi::c_void;
use core::ptr;

/// Raw display handle for the GTK Tool Kit version 3.
///
/// ## Construction
///
/// ```
/// # use raw_window_handle::Gtk3DisplayHandle;
/// let display_handle = Gtk3DisplayHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gtk3DisplayHandle {
    /// A pointer to a `GtkApplication`.
    pub application: *mut c_void,
}

/// Raw window handle for the GTK Tool Kit version 3.
///
/// ## Construction
///
/// ```
/// # use raw_window_handle::Gtk3WindowHandle;
/// let window_handle = Gtk3WindowHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gtk3WindowHandle {
    /// A pointer to a `GtkWidget`.
    pub widget: *mut c_void,
}

/// Raw display handle for the GTK Tool Kit version 4.
///
/// ## Construction
///
/// ```
/// # use raw_window_handle::Gtk4DisplayHandle;
/// let display_handle = Gtk4DisplayHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gtk4DisplayHandle {
    /// A pointer to a `GtkApplication`.
    pub application: *mut c_void,
}

/// Raw window handle for the GTK Tool Kit version 4.
///
/// ## Construction
///
/// ```
/// # use raw_window_handle::Gtk4WindowHandle;
/// let window_handle = Gtk4WindowHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gtk4WindowHandle {
    /// A pointer to a `GtkWidget`.
    pub widget: *mut c_void,
}

impl Gtk3DisplayHandle {
    pub fn empty() -> Self {
        Self {
            application: ptr::null_mut(),
        }
    }
}

impl Gtk3WindowHandle {
    pub fn empty() -> Self {
        Self {
            widget: ptr::null_mut(),
        }
    }
}

impl Gtk4DisplayHandle {
    pub fn empty() -> Self {
        Self {
            application: ptr::null_mut(),
        }
    }
}

impl Gtk4WindowHandle {
    pub fn empty() -> Self {
        Self {
            widget: ptr::null_mut(),
        }
    }
}
