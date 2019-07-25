use core::ptr;
use libc::{c_ulong, c_void};

impl crate::RawWindowHandle {
    pub fn new_x11(handle: X11Handle) -> Self {
        Self {
            handle: RawWindowHandle {
                handle: UnixHandle::X11(handle),
            },
        }
    }

    pub fn new_wayland(handle: WaylandHandle) -> Self {
        Self {
            handle: RawWindowHandle {
                handle: UnixHandle::Wayland(handle),
            },
        }
    }

    pub fn x11_handle(&self) -> Option<X11Handle> {
        match self.handle.handle {
            UnixHandle::X11(handle) => Some(handle),
            UnixHandle::Wayland(_) => None,
        }
    }

    pub fn wayland_handle(&self) -> Option<WaylandHandle> {
        match self.handle.handle {
            UnixHandle::X11(_) => None,
            UnixHandle::Wayland(handle) => Some(handle),
        }
    }
}

pub(crate) struct RawWindowHandle {
    handle: UnixHandle,
}

enum UnixHandle {
    X11(X11Handle),
    Wayland(WaylandHandle),
}

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
    pub fn empty() -> Self {
        Self {
            window: 0,
            display: ptr::null_mut(),
            _non_exhaustive: (),
        }
    }
}

impl WaylandHandle {
    pub fn empty() -> Self {
        Self {
            surface: ptr::null_mut(),
            display: ptr::null_mut(),
            _non_exhaustive: (),
        }
    }
}
