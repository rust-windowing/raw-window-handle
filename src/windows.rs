use core::ffi::c_void;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for Windows.
///
/// It can be used regardless of Windows window backend.
///
/// ## Thread-Safety
///
/// Overall, even though Win32 windows have [thread affinity], the overall
/// Win32 user API is thread-safe. Therefore this type is `Send` and `Sync`.
/// This means it can be sent to or used from other threads.
///
/// [thread affinity]: https://devblogs.microsoft.com/oldnewthing/20051010-09/?p=33843
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowsDisplayHandle {}

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
        Self {}
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
///
/// ## Thread-Safety
///
/// Window handles have [thread affinity]. This means that they are `!Send`, as
/// they must be dropped on the same thread that created them. However, some
/// functions on the window can be called from other threads. This means that
/// the window is `Sync`.
///
/// Note that not all functions of the Win32 handle are thread-safe (modifying
/// functions especially), so care should be taken to not call these functions
/// from other threads. When in doubt, only run the function on the main thread.
///
/// [thread affinity]: https://devblogs.microsoft.com/oldnewthing/20051010-09/?p=33843
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Win32WindowHandle {
    /// A Win32 `HWND` handle.
    pub hwnd: NonNull<c_void>,
    /// The `GWLP_HINSTANCE` associated with this type's `HWND`.
    pub hinstance: Option<NonNull<c_void>>,
}

unsafe impl Sync for Win32WindowHandle {}

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
    /// let mut handle = Win32WindowHandle::new(NonNull::new(window.0).unwrap());
    /// // Optionally set the GWLP_HINSTANCE.
    /// # #[cfg(only_for_showcase)]
    /// let hinstance = NonNull::new(unsafe { GetWindowLongPtrW(window, GWLP_HINSTANCE) }).unwrap();
    /// # let hinstance = None;
    /// handle.hinstance = hinstance;
    /// ```
    pub fn new(hwnd: NonNull<c_void>) -> Self {
        Self {
            hwnd,
            hinstance: None,
        }
    }
}

/// Raw window handle for WinRT.
///
/// ## Thread-Safety
///
/// Window handles have [thread affinity]. This means that they are `!Send`, as
/// they must be dropped on the same thread that created them. However, some
/// functions on the window can be called from other threads. This means that
/// the window is `Sync`.
///
/// Note that not all functions of the Win32 handle are thread-safe (modifying
/// functions especially), so care should be taken to not call these functions
/// from other threads. When in doubt, only run the function on the main thread.
///
/// [thread affinity]: https://devblogs.microsoft.com/oldnewthing/20051010-09/?p=33843
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WinRtWindowHandle {
    /// A WinRT `CoreWindow` handle.
    pub core_window: NonNull<c_void>,
}

unsafe impl Sync for WinRtWindowHandle {}

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
}
