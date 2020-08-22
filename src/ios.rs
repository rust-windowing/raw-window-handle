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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IOSHandle {
    pub ui_window: Option<NonNull<c_void>>,
    pub ui_view: Option<NonNull<c_void>>,
    pub ui_view_controller: Option<NonNull<c_void>>,
    #[doc(hidden)]
    #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
    pub _non_exhaustive_do_not_use: crate::seal::Seal,
}

impl IOSHandle {
    pub fn empty() -> IOSHandle {
        #[allow(deprecated)]
        IOSHandle {
            ui_window: None,
            ui_view: None,
            ui_view_controller: None,
            _non_exhaustive_do_not_use: crate::seal::Seal,
        }
    }
}
