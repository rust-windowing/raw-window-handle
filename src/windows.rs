use core::ffi::c_void;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for Windows.
///
/// It can be used regardless of Windows window backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowsDisplayHandle(());

impl WindowsDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::WindowsDisplayHandle;
    /// let handle = WindowsDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self(())
    }
}

impl DisplayHandle<'static> {
    /// Create a Windows-based display handle.
    ///
    /// As no data is borrowed by this handle, it is completely safe to create. This function
    /// may be useful to windowing framework implementations that want to avoid unsafe code.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::windows();
    /// do_something(handle);
    /// ```
    pub fn windows() -> Self {
        // SAFETY: No data is borrowed.
        unsafe { Self::borrow_raw(WindowsDisplayHandle::new().into()) }
    }
}

/// Raw window handle for Win32.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Win32WindowHandle {
    hwnd: NonNull<c_void>,
    hinstance: Option<NonNull<c_void>>,
}

impl Win32WindowHandle {
    /// Create a new handle to a window.
    ///
    /// # Safety
    ///
    /// It is assumed that the Win32 handle belongs to the current thread. This
    /// is necessary for the handle to be considered "valid" in all cases.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::Win32WindowHandle;
    /// # struct HWND(*mut c_void);
    /// #
    /// let window: HWND;
    /// # window = HWND(1 as *mut c_void);
    /// let handle = Win32WindowHandle::new(NonNull::new(window.0).unwrap());
    /// ```
    pub fn new(hwnd: NonNull<c_void>) -> Self {
        Self {
            hwnd,
            hinstance: None,
        }
    }

    /// Create a new window handle to a window together with its `GWLP_HINSTANCE`.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::Win32WindowHandle;
    /// # struct HWND(*mut c_void);
    /// #
    /// let window: HWND;
    /// # window = HWND(1 as *mut c_void);
    /// # #[cfg(only_for_showcase)]
    /// let hinstance = NonNull::new(unsafe { GetWindowLongPtrW(window, GWLP_HINSTANCE) }).unwrap();
    /// # let hinstance = NonNull::dangling();
    /// let handle = Win32WindowHandle::with_hinstance(NonNull::new(window.0).unwrap(), hinstance);
    /// ```
    pub fn with_hinstance(hwnd: NonNull<c_void>, hinstance: NonNull<c_void>) -> Self {
        Self {
            hwnd,
            hinstance: Some(hinstance),
        }
    }

    /// A Win32 `HWND` handle.
    pub fn hwnd(&self) -> NonNull<c_void> {
        self.hwnd
    }

    /// The `GWLP_HINSTANCE` associated with this type's `HWND`.
    pub fn hinstance(&self) -> Option<NonNull<c_void>> {
        self.hinstance
    }
}

/// Raw window handle for WinRT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WinRtWindowHandle {
    core_window: NonNull<c_void>,
}

impl WinRtWindowHandle {
    /// Create a new handle to a window.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::WinRtWindowHandle;
    /// # type CoreWindow = ();
    /// #
    /// let window: NonNull<CoreWindow>;
    /// # window = NonNull::from(&());
    /// let handle = WinRtWindowHandle::new(window.cast());
    /// ```
    pub fn new(core_window: NonNull<c_void>) -> Self {
        Self { core_window }
    }

    /// A WinRT `CoreWindow` handle.
    pub fn core_window(&self) -> NonNull<c_void> {
        self.core_window
    }
}
