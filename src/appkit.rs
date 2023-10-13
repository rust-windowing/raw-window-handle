use core::ffi::c_void;
use core::ptr::NonNull;

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

/// Raw window handle for AppKit.
///
/// Note that while this is `Send + Sync`, it is only usable from the main
/// thread. Any usage of `NSView` outside the main thread may be undefined
/// behaviour, unless explicitly documented otherwise.
///
/// You must check whether the thread is the main thread before accessing
/// this, and if it is not, you should execute your code to access the view
/// on the main thread instead using `libdispatch`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppKitWindowHandle {
    /// A pointer to an `NSView` object.
    pub ns_view: NonNull<c_void>,
}

// SAFETY: Only accessible from the main thread.
//
// Acts as-if the view is wrapped in `MainThreadBound<T>`.
unsafe impl Send for AppKitWindowHandle {}
unsafe impl Sync for AppKitWindowHandle {}

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
