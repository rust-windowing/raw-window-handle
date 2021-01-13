use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw window handle for Windows.
///
/// ## Construction
/// ```
/// # use raw_window_handle::windows::WindowsHandle;
/// let handle = WindowsHandle {
///     /* fields */
///     ..WindowsHandle::empty()
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowsHandle {
    /// A Win32 HWND handle.
    pub hwnd: Option<NonNull<c_void>>,
    /// The HINSTANCE associated with this type's HWND.
    pub hinstance: Option<NonNull<c_void>>,
    #[doc(hidden)]
    #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
    pub _non_exhaustive_do_not_use: crate::seal::Seal,
}

impl WindowsHandle {
    pub fn empty() -> WindowsHandle {
        #[allow(deprecated)]
        WindowsHandle {
            hwnd: None,
            hinstance: None,
            _non_exhaustive_do_not_use: crate::seal::Seal,
        }
    }
}
