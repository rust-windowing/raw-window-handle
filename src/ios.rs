use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw window handle for iOS.
///
/// ## Construction
/// ```
/// # use raw_window_handle::ios::IOSHandle;
/// let handle = IOSHandle {
///     /* fields */
///     ..IOSHandle::empty()
/// };
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IOSHandle {
    pub ui_window: Option<NonNull<c_void>>,
    pub ui_view: Option<NonNull<c_void>>,
    pub ui_view_controller: Option<NonNull<c_void>>,
}

impl IOSHandle {
    pub fn empty() -> IOSHandle {
        IOSHandle {
            ui_window: None,
            ui_view: None,
            ui_view_controller: None,
        }
    }
}
