use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw display handle for UIKit.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UiKitDisplayHandle {}

impl UiKitDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::UiKitDisplayHandle;
    /// let handle = UiKitDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

/// Raw window handle for UIKit.
///
/// Note that while this is `Send + Sync`, it is only usable from the main
/// thread. Any usage of `UIView` and `UIViewController` outside the main
/// thread may be undefined behaviour, unless explicitly documented
/// otherwise.
///
/// You must check whether the thread is the main thread before accessing
/// this, and if it is not, you should execute your code to access the view
/// and view controller on the main thread instead using `libdispatch`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UiKitWindowHandle {
    /// A pointer to an `UIView` object.
    pub ui_view: NonNull<c_void>,
    /// A pointer to an `UIViewController` object, if the view has one.
    pub ui_view_controller: Option<NonNull<c_void>>,
}

// SAFETY: Only accessible from the main thread.
//
// Acts as-if the view is wrapped in `MainThreadBound<T>`.
unsafe impl Send for UiKitWindowHandle {}
unsafe impl Sync for UiKitWindowHandle {}

impl UiKitWindowHandle {
    /// Create a new handle to a view.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::UiKitWindowHandle;
    /// # type UIView = ();
    /// #
    /// let view: &UIView;
    /// # view = &();
    /// let mut handle = UiKitWindowHandle::new(NonNull::from(view).cast());
    /// // Optionally set the view controller.
    /// handle.ui_view_controller = None;
    /// ```
    pub fn new(ui_view: NonNull<c_void>) -> Self {
        Self {
            ui_view,
            ui_view_controller: None,
        }
    }
}
