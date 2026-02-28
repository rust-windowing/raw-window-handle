use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for AppKit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppKitDisplayHandle(());

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
        Self(())
    }
}

impl DisplayHandle<'static> {
    /// Create an AppKit-based display handle.
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
        AppKitDisplayHandle::new().into()
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
/// use raw_window_handle::WindowHandle;
///
/// let handle: WindowHandle<'_>; // Get the window handle from somewhere else
/// # handle = unimplemented!();
/// match handle {
///     # #[cfg(requires_objc2)]
///     WindowHandle::AppKit(handle) => {
///         assert!(MainThreadMarker::new().is_some(), "can only access AppKit handles on the main thread");
///         let ns_view = handle.ns_view().as_ptr();
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppKitWindowHandle<'window> {
    ns_view: NonNull<c_void>,
    _marker: PhantomData<&'window ()>,
}

impl AppKitWindowHandle<'_> {
    /// Create a new handle to a view.
    ///
    /// # Safety
    ///
    /// `ns_view` must be a valid pointer to a `NSView`, and must remain valid for the lifetime of
    /// this type.
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
    /// let handle = unsafe { AppKitWindowHandle::new(ns_view.cast()) };
    /// ```
    pub unsafe fn new(ns_view: NonNull<c_void>) -> Self {
        Self {
            ns_view,
            _marker: PhantomData,
        }
    }

    /// A pointer to an `NSView` object.
    ///
    /// The pointer is guaranteed to be valid for at least as long as `self`.
    pub fn ns_view(&self) -> NonNull<c_void> {
        self.ns_view
    }
}
