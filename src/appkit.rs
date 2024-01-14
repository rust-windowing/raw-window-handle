use core::ffi::c_void;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for AppKit.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppKitDisplayHandle {}

impl AppKitDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::AppKitDisplayHandle;
    /// let handle = AppKitDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

impl DisplayHandle<'static> {
    /// Create an AppKit-based display handle.
    /// 
    /// As no data is borrowed by this handle, it is completely safe to create. This function
    /// may be useful to windowing framework implementations that want to avoid unsafe code.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::appkit();
    /// do_something(handle);
    /// ```
    pub fn appkit() -> Self {
        // SAFETY: No data is borrowed.
        unsafe {
            Self::borrow_raw(AppKitDisplayHandle::new().into())
        }
    }
}

/// Raw window handle for AppKit.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppKitWindowHandle {
    /// A pointer to an `NSView` object.
    pub ns_view: NonNull<c_void>,
}

impl AppKitWindowHandle {
    /// Create a new handle to a view.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::AppKitWindowHandle;
    /// # type NSView = ();
    /// #
    /// let view: &NSView;
    /// # view = &();
    /// let handle = AppKitWindowHandle::new(NonNull::from(view).cast());
    /// ```
    pub fn new(ns_view: NonNull<c_void>) -> Self {
        Self { ns_view }
    }
}
