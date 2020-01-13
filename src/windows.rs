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
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowsHandle {
    /// A Win32 HWND handle.
    pub hwnd: Option<NonNull<c_void>>,
    /// The HINSTANCE associated with this type's HWND.
    pub hinstance: Option<NonNull<c_void>>,
}

impl WindowsHandle {
    pub fn empty() -> WindowsHandle {
        WindowsHandle {
            hwnd: None,
            hinstance: None,
        }
    }
}
