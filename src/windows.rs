use core::ptr;
use libc::c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowsHandle {
    pub hwnd: *mut c_void,
    #[doc(hidden)]
    #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
    pub _non_exhaustive_do_not_use: crate::seal::Seal,
}

impl WindowsHandle {
    pub fn empty() -> WindowsHandle {
        WindowsHandle {
            hwnd: ptr::null_mut(),
            _non_exhaustive_do_not_use: crate::seal::Seal,
        }
    }
}
