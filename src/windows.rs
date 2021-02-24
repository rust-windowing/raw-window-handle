use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw window handle for Windows.
///
/// ## Construction
/// ```
/// # use raw_window_handle::windows::WindowsHandle;
/// let mut handle = WindowsHandle::empty();
/// /* set fields */
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

/// Raw window handle for WinRT.
///
/// ## Construction
/// ```
/// # use raw_window_handle::windows::WinRTHandle;
/// let mut handle = WinRTHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WinRTHandle {
    /// A WinRT CoreWindow handle.
    pub core_window: Option<NonNull<c_void>>,
}

impl WinRTHandle {
    pub fn empty() -> WinRTHandle {
        WinRTHandle { core_window: None }
    }
}
