use core::ffi::c_void;
use core::ptr;

/// Raw window handle for Win32.
///
/// ## Construction
/// ```
/// # use raw_window_handle::Win32Handle;
/// let mut handle = Win32Handle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Win32Handle {
    /// A Win32 `HWND` handle.
    pub hwnd: *mut c_void,
    /// The `HINSTANCE` associated with this type's `HWND`.
    pub hinstance: *mut c_void,
}

impl Win32Handle {
    pub fn empty() -> Self {
        Self {
            hwnd: ptr::null_mut(),
            hinstance: ptr::null_mut(),
        }
    }
}

impl From<new::Win32WindowHandle> for Win32Handle {
    fn from(handle: new::Win32WindowHandle) -> Self {
        Self {
            hwnd: handle.hwnd,
            hinstance: handle.hinstance,
            ..Self::empty()
        }
    }
}

/// Raw window handle for WinRT.
///
/// ## Construction
/// ```
/// # use raw_window_handle::WinRtHandle;
/// let mut handle = WinRtHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WinRtHandle {
    /// A WinRT `CoreWindow` handle.
    pub core_window: *mut c_void,
}

impl WinRtHandle {
    pub fn empty() -> Self {
        Self {
            core_window: ptr::null_mut(),
        }
    }
}

impl From<new::WinRtWindowHandle> for WinRtHandle {
    fn from(handle: new::WinRtWindowHandle) -> Self {
        Self {
            core_window: handle.core_window,
            ..Self::empty()
        }
    }
}
