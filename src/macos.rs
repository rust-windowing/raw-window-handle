use core::ptr;
use libc::c_void;

pub trait RawWindowHandleEx {
    fn new_macos(handle: MacOSHandle) -> Self;
    fn macos_handle(&self) -> MacOSHandle;
}

impl RawWindowHandleEx for crate::RawWindowHandle {
    fn new_macos(handle: MacOSHandle) -> Self {
        Self {
            handle: RawWindowHandle { handle },
        }
    }

    fn macos_handle(&self) -> MacOSHandle {
        self.handle.handle
    }
}

pub(crate) struct RawWindowHandle {
    handle: MacOSHandle,
}

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
