use libc::c_void;

/// Window that wraps around an iOS window handle.
pub trait HasIOSHandle {
    fn ui_window(&self) -> *mut c_void;
    fn ui_view(&self) -> *mut c_void;
    fn ui_view_controller(&self) -> *mut c_void;
}
