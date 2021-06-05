use core::ffi::c_void;
use core::ptr;

/// Raw window handle for UIKit.
///
/// ## Construction
/// ```
/// # use raw_window_handle::UIKitHandle;
/// let mut handle = UIKitHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UIKitHandle {
    /// A pointer to an `UIWindow` object.
    pub ui_window: *mut c_void,
    /// A pointer to an `UIView` object.
    pub ui_view: *mut c_void,
    /// A pointer to an `UIViewController` object.
    pub ui_view_controller: *mut c_void,
}

impl UIKitHandle {
    pub fn empty() -> UIKitHandle {
        UIKitHandle {
            ui_window: ptr::null_mut(),
            ui_view: ptr::null_mut(),
            ui_view_controller: ptr::null_mut(),
        }
    }
}
