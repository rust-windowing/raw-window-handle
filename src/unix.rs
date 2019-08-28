use libc::{c_ulong, c_void};

/// Window that wraps around a Unix window handle.
pub trait HasUnixHandle {
    /// Try to cast this handle into an X11 handle.
    fn x11(&self) -> Option<&dyn HasX11Handle>;

    /// Try to cast this handle into a Wayland handle.
    fn wayland(&self) -> Option<&dyn HasWaylandHandle>;
}

/// An X11 window.
pub trait HasX11Handle {
    fn window(&self) -> c_ulong;
    fn display(&self) -> *mut c_void;
}

/// A Wayland window.
pub trait HasWaylandHandle {
    fn surface(&self) -> *mut c_void;
    fn display(&self) -> *mut c_void;
}
