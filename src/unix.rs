use core::ptr;
use libc::{c_ulong, c_void};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X11Handle {
    pub window: c_ulong,
    pub display: *mut c_void,
    _non_exhaustive: (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaylandHandle {
    pub surface: *mut c_void,
    pub display: *mut c_void,
    _non_exhaustive: (),
}

impl X11Handle {
    pub fn empty() -> X11Handle {
        X11Handle {
            window: 0,
            display: ptr::null_mut(),
            _non_exhaustive: (),
        }
    }
}

impl WaylandHandle {
    pub fn empty() -> WaylandHandle {
        WaylandHandle {
            surface: ptr::null_mut(),
            display: ptr::null_mut(),
            _non_exhaustive: (),
        }
    }
}
