use core::ptr;
use libc::c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowsHandle {
    pub hwnd: *mut c_void,
    _non_exhaustive: (),
}

impl WindowsHandle {
    pub fn empty() -> WindowsHandle {
        WindowsHandle {
            hwnd: ptr::null_mut(),
            _non_exhaustive: (),
        }
    }
}
