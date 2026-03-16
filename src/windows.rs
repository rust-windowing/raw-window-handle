use super::DisplayHandle;

/// Raw display handle for Windows.
///
/// It can be used regardless of Windows window backend.
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
