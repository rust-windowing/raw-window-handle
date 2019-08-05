use core::ptr;
use libc::c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IOSHandle {
    pub ui_window: *mut c_void,
    pub ui_view: *mut c_void,
    pub ui_view_controller: *mut c_void,
    _non_exhaustive: (),
}

impl IOSHandle {
    pub fn empty() -> IOSHandle {
        IOSHandle {
            ui_window: ptr::null_mut(),
            ui_view: ptr::null_mut(),
            ui_view_controller: ptr::null_mut(),
            _non_exhaustive: (),
        }
    }
}
