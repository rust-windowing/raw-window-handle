use core::ptr;
use libc::c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacOSHandle {
    pub ns_window: *mut c_void,
    pub ns_view: *mut c_void,
    // TODO: WHAT ABOUT ns_window_controller and ns_view_controller?
    _non_exhaustive: (),
}

impl MacOSHandle {
    pub fn empty() -> MacOSHandle {
        MacOSHandle {
            ns_window: ptr::null_mut(),
            ns_view: ptr::null_mut(),
            _non_exhaustive: (),
        }
    }
}
