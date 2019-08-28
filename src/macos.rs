use libc::c_void;

/// Window that wraps around a MacOS window handle.
pub trait HasMacOSHandle {
    fn ns_window(&self) -> *mut c_void;
    fn ns_view(&self) -> *mut c_void;

    // TODO: WHAT ABOUT ns_window_controller and ns_view_controller?
}
