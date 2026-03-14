use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for AppKit.
///
/// ## Thread Safety
///
/// This type has the safe thread safety guarantees as [`AppKitWindowHandle`].
///
/// Note that this type does not contain any Appkit objects. However,
/// it is kept `!Send` and `!Sync` for the event that Appkit objects are
/// added to this type.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppKitDisplayHandle {
    _thread_unsafe: PhantomData<*mut ()>,
}

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
        Self {
            _thread_unsafe: PhantomData,
        }
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
/// # Example
///
/// Getting the view from a [`WindowHandle`][crate::WindowHandle].
///
/// ```no_run
/// # #[cfg(not(target_os = "macos"))]
/// # fn main() {}
/// # fn main() {
/// #![cfg(target_os = "macos")]
/// use objc2::MainThreadMarker;
/// use objc2::rc::Retained;
/// use objc2_app_kit::NSView;
/// use raw_window_handle::{WindowHandle, RawWindowHandle};
///
/// let handle: WindowHandle<'_>; // Get the window handle from somewhere
/// # handle = unimplemented!();
/// match handle.as_raw() {
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
///
/// ## Thread Safety
///
/// Handles to AppKit objects can only be safely used from the main thread.
/// Therefore, all Appkit objects are `!Send` and `!Sync`.
/// This means that this type cannot be sent to or used from other threads.
///
/// In addition, it is also expected that the consumer will take precautions to
/// ensure that this object is only used on the main thread.
/// It is recommended to use [`objc2::MainThreadMarker`] as a strategy for
/// ensuring this.
///
/// [`objc2::MainThreadMarker`]: https://docs.rs/objc2/latest/objc2/struct.MainThreadMarker.html
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
    /// ```
    /// # #[cfg(not(target_os = "macos"))]
    /// # fn main() {}
    /// # fn main() {
    /// #![cfg(target_os = "macos")]
    /// use std::ptr::NonNull;
    /// use objc2::rc::Retained;
    /// use objc2_app_kit::{NSWindow, NSView};
    /// use raw_window_handle::AppKitWindowHandle;
    ///
    /// // NSWindow gotten from somewhere.
    /// let ns_window: Retained<NSWindow>;
    /// # ns_window = unsafe { objc2_app_kit::NSWindow::new(objc2::MainThreadMarker::new().unwrap()) };
    ///
    /// // Use the window's content view.
    /// let ns_view: Retained<NSView> = ns_window.contentView().unwrap();
    /// let ns_view: NonNull<NSView> = NonNull::from(&*ns_view);
    /// let handle = AppKitWindowHandle::new(ns_view.cast());
    /// # }
    /// ```
    pub fn new(ns_view: NonNull<c_void>) -> Self {
        Self { ns_view }
    }
}
