use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw window handle for macOS.
///
/// ## Construction
/// ```
/// # use raw_window_handle::macos::MacOSHandle;
/// let mut handle = MacOSHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MacOSHandle {
    pub ns_window: Option<NonNull<c_void>>,
    pub ns_view: Option<NonNull<c_void>>,
    // TODO: WHAT ABOUT ns_window_controller and ns_view_controller?
}

impl MacOSHandle {
    pub fn empty() -> MacOSHandle {
        MacOSHandle {
            ns_window: None,
            ns_view: None,
        }
    }
}
