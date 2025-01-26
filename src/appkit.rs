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
        unsafe { Self::borrow_raw(AppKitDisplayHandle::new().into()) }
    }
}

/// Raw window handle for AppKit.
///
/// Note that `NSView` can only be accessed from the main thread of the
/// application. This struct is `!Send` and `!Sync` to help with ensuring
/// that.
///
/// # Example
///
/// Getting the view from a [`WindowHandle`][crate::WindowHandle].
///
/// ```no_run
/// # fn inner() {
/// #![cfg(target_os = "macos")]
/// # #[cfg(requires_objc2)]
/// use objc2::MainThreadMarker;
/// # #[cfg(requires_objc2)]
/// use objc2::rc::Retained;
/// # #[cfg(requires_objc2)]
/// use objc2_app_kit::NSView;
/// use raw_window_handle::{WindowHandle, RawWindowHandle};
///
/// let handle: WindowHandle<'_>; // Get the window handle from somewhere else
/// # handle = unimplemented!();
/// match handle.as_raw() {
///     # #[cfg(requires_objc2)]
///     RawWindowHandle::AppKit(handle) => {
///         assert!(MainThreadMarker::new().is_some(), "can only access AppKit handles on the main thread");
///         let ns_view = handle.ns_view.as_ptr();
///         // SAFETY: The pointer came from `WindowHandle`, which ensures
///         // that the `AppKitWindowHandle` contains a valid pointer to an
///         // `NSView`.
///         // Unwrap is fine, since the pointer came from `NonNull`.
///         let ns_view: Retained<NSView> = unsafe { Retained::retain(ns_view.cast()) }.unwrap();
///         // Do something with the NSView here, like getting the `NSWindow`
///         let ns_window = ns_view.window().expect("view was not installed in a window");
///     }
///     handle => unreachable!("unknown handle {handle:?} for platform"),
/// }
/// # }
/// ```
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
    /// Create a handle from the content view of a `NSWindow`.
    ///
    /// ```ignore
    /// use std::ptr::NonNull;
    /// use objc2::rc::Retained;
    /// use objc2_app_kit::{NSWindow, NSView};
    /// use raw_window_handle::AppKitWindowHandle;
    ///
    /// let ns_window: Retained<NSWindow> = ...;
    /// let ns_view: Retained<NSView> = window.contentView();
    /// let ns_view: NonNull<NSView> = NonNull::from(&*ns_view);
    /// let handle = AppKitWindowHandle::new(ns_view.cast());
    /// ```
    pub fn new(ns_view: NonNull<c_void>) -> Self {
        Self { ns_view }
    }
}
