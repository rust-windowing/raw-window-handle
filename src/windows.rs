use libc::c_void;
use core::ptr;

pub trait RawWindowHandleEx {
    fn new(handle: WindowsHandle) -> Self;
    fn windows_handle(&self) -> WindowsHandle;
}

impl RawWindowHandleEx for crate::RawWindowHandle {
    fn new(handle: WindowsHandle) -> Self {
        Self {
            handle: RawWindowHandle {
                handle,
            }
        }
    }

    fn windows_handle(&self) -> WindowsHandle {
        self.handle.handle
    }
}

pub(crate) struct RawWindowHandle {
    handle: WindowsHandle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowsHandle {
    pub hwnd: *mut c_void,
    // TODO: WHAT ABOUT ns_window_controller and ns_view_controller?
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
