use core::ptr;
use libc::{c_ulong, c_void};

/// Raw window handle for X11.
///
/// ## Construction
/// ```
/// # use raw_window_handle::unix::X11Handle;
/// let handle = X11Handle {
///     /* fields */
///     ..X11Handle::empty()
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X11Handle {
    pub window: c_ulong,
    pub display: *mut c_void,
    #[doc(hidden)]
    #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
    pub _non_exhaustive_do_not_use: crate::seal::Seal,
}

/// Raw window handle for Wayland.
///
/// ## Construction
/// ```
/// # use raw_window_handle::unix::WaylandHandle;
/// let handle = WaylandHandle {
///     /* fields */
///     ..WaylandHandle::empty()
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaylandHandle {
    pub surface: *mut c_void,
    pub display: *mut c_void,
    #[doc(hidden)]
    #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
    pub _non_exhaustive_do_not_use: crate::seal::Seal,
}

impl X11Handle {
    pub fn empty() -> X11Handle {
        #[allow(deprecated)]
        X11Handle {
            window: 0,
            display: ptr::null_mut(),
            _non_exhaustive_do_not_use: crate::seal::Seal,
        }
    }
}

impl WaylandHandle {
    pub fn empty() -> WaylandHandle {
        #[allow(deprecated)]
        WaylandHandle {
            surface: ptr::null_mut(),
            display: ptr::null_mut(),
            _non_exhaustive_do_not_use: crate::seal::Seal,
        }
    }
}
