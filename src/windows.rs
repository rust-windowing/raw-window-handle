use core::ptr;
use libc::c_void;

impl crate::RawWindowHandle {
    pub fn new(handle: WindowsHandle) -> Self {
        Self {
            handle: RawWindowHandle { handle },
        }
    }

    pub fn windows_handle(&self) -> WindowsHandle {
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
    pub fn empty() -> Self {
        Self {
            hwnd: ptr::null_mut(),
            _non_exhaustive: (),
        }
    }
}
