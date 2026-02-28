use core::ffi::c_void;
use core::marker::PhantomData;
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
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::windows();
    /// do_something(handle);
    /// ```
    pub fn windows() -> Self {
        WindowsDisplayHandle::new().into()
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
    /// let handle = unsafe { Win32WindowHandle::new(NonNull::new(window.0).unwrap()) };
    /// ```
    pub unsafe fn new(hwnd: NonNull<c_void>) -> Self {
        Self {
            hwnd,
            hinstance: None,
        }
    }

    /// Create a new window handle to a window together with its `GWLP_HINSTANCE`.
    ///
    /// # Safety
    ///
    /// Same as in [`Win32WindowHandle::new`].
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
    /// let handle = unsafe { Win32WindowHandle::with_hinstance(NonNull::new(window.0).unwrap(), hinstance) };
    /// ```
    pub unsafe fn with_hinstance(hwnd: NonNull<c_void>, hinstance: NonNull<c_void>) -> Self {
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
pub struct WinRtWindowHandle<'window> {
    core_window: NonNull<c_void>,
    _marker: PhantomData<&'window ()>,
}

impl WinRtWindowHandle<'_> {
    /// Create a new handle to a window.
    ///
    /// # Safety
    ///
    /// `core_window` must be a valid pointer to a WinRT `CoreWindow` and must remain valid for the
    /// lifetime of this type.
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
    /// let handle = unsafe { WinRtWindowHandle::new(window.cast()) };
    /// ```
    pub unsafe fn new(core_window: NonNull<c_void>) -> Self {
        Self {
            core_window,
            _marker: PhantomData,
        }
    }

    /// A WinRT `CoreWindow` handle.
    ///
    /// The pointer is guaranteed to be valid for at least as long as `self`.
    pub fn core_window(&self) -> NonNull<c_void> {
        self.core_window
    }
}
