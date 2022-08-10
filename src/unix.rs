use core::ffi::c_void;
use core::ptr;

use cty::c_ulong;

/// Raw window handle for Xlib.
///
/// ## Construction
/// ```
/// # use raw_window_handle::XlibHandle;
/// let handle = XlibHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XlibHandle {
    /// An Xlib `Window`.
    pub window: c_ulong,
    /// A pointer to an Xlib `Display`.
    pub display: *mut c_void,
    /// An Xlib visual ID, or 0 if unknown.
    pub visual_id: c_ulong,
}

/// Raw window handle for Xcb.
///
/// ## Construction
/// ```
/// # use raw_window_handle::XcbHandle;
/// let handle = XcbHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XcbHandle {
    /// An X11 `xcb_window_t`.
    pub window: u32, // Based on xproto.h
    /// A pointer to an X server `xcb_connection_t`.
    pub connection: *mut c_void,
    /// An X11 `xcb_visualid_t`, or 0 if unknown.
    pub visual_id: u32,
}

/// Raw window handle for Wayland.
///
/// ## Construction
/// ```
/// # use raw_window_handle::WaylandHandle;
/// let handle = WaylandHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaylandHandle {
    /// A pointer to a `wl_surface`.
    pub surface: *mut c_void,
    /// A pointer to a `wl_display`.
    pub display: *mut c_void,
}

impl XlibHandle {
    pub fn empty() -> Self {
        Self {
            window: 0,
            display: ptr::null_mut(),
            visual_id: 0,
        }
    }
}

impl XcbHandle {
    pub fn empty() -> Self {
        Self {
            window: 0,
            connection: ptr::null_mut(),
            visual_id: 0,
        }
    }
}

impl WaylandHandle {
    pub fn empty() -> Self {
        Self {
            surface: ptr::null_mut(),
            display: ptr::null_mut(),
        }
    }
}

impl From<(new::XlibWindowHandle, new::XlibDisplayHandle)> for XlibHandle {
    fn from(handle: (new::XlibWindowHandle, new::XlibDisplayHandle)) -> Self {
        Self {
            window: handle.0.window,
            display: handle.1.display,
            visual_id: handle.0.visual_id,
            ..Self::empty()
        }
    }
}

impl From<(new::XcbWindowHandle, new::XcbDisplayHandle)> for XcbHandle {
    fn from(handle: (new::XcbWindowHandle, new::XcbDisplayHandle)) -> Self {
        Self {
            window: handle.0.window,
            connection: handle.1.connection,
            visual_id: handle.0.visual_id,
            ..Self::empty()
        }
    }
}

impl From<(new::WaylandWindowHandle, new::WaylandDisplayHandle)> for WaylandHandle {
    fn from(handle: (new::WaylandWindowHandle, new::WaylandDisplayHandle)) -> Self {
        Self {
            surface: handle.0.surface,
            display: handle.1.display,
            ..Self::empty()
        }
    }
}
