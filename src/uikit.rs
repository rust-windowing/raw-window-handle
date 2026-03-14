use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for UIKit.
///
/// ## Thread Safety
///
/// This type has the same thread safety guarantees as [`UiKitWindowHandle`].
///
/// Note that this type does not contain any UiKit objects. However,
/// it is kept `!Send` and `!Sync` for the event that UiKit objects are
/// added to this type.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UiKitDisplayHandle {
    _thread_unsafe: PhantomData<*mut ()>,
}

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
        Self {
            _thread_unsafe: PhantomData,
        }
    }
}

impl DisplayHandle<'static> {
    /// Create a UiKit-based display handle.
    ///
    /// As no data is borrowed by this handle, it is completely safe to create. This function
    /// may be useful to windowing framework implementations that want to avoid unsafe code.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::uikit();
    /// do_something(handle);
    /// ```
    pub fn uikit() -> Self {
        // SAFETY: No data is borrowed.
        unsafe { Self::borrow_raw(UiKitDisplayHandle::new().into()) }
    }
}

/// Raw window handle for UIKit.
///
/// # Example
///
/// Getting the view from a [`WindowHandle`][crate::WindowHandle].
///
/// ```no_run
/// # #[cfg(not(all(target_vendor = "apple", not(target_os = "macos"))))]
/// # fn main() {}
/// # fn main() {
/// #![cfg(all(target_vendor = "apple", not(target_os = "macos")))]
/// use objc2::MainThreadMarker;
/// use objc2::rc::Retained;
/// use objc2_ui_kit::UIView;
/// use raw_window_handle::{WindowHandle, RawWindowHandle};
///
/// let handle: WindowHandle<'_>; // Get the window handle from somewhere else
/// # handle = unimplemented!();
/// match handle.as_raw() {
///     RawWindowHandle::UiKit(handle) => {
///         assert!(MainThreadMarker::new().is_some(), "can only access UIKit handles on the main thread");
///         let ui_view = handle.ui_view.as_ptr();
///         // SAFETY: The pointer came from `WindowHandle`, which ensures
///         // that the `UiKitWindowHandle` contains a valid pointer to an
///         // `UIView`.
///         // Unwrap is fine, since the pointer came from `NonNull`.
///         let ui_view: Retained<UIView> = unsafe { Retained::retain(ui_view.cast()) }.unwrap();
///         // Do something with the UIView here.
///     }
///     handle => unreachable!("unknown handle {handle:?} for platform"),
/// }
/// # }
/// ```
///
/// Get a pointer to an `UIViewController` object by traversing the `UIView`'s responder chain:
///
/// ```
/// # #[cfg(not(all(target_vendor = "apple", not(target_os = "macos"))))]
/// # fn main() {}
/// # fn main() {
/// #![cfg(all(target_vendor = "apple", not(target_os = "macos")))]
/// use objc2::rc::Retained;
/// use objc2_ui_kit::{UIResponder, UIView, UIViewController};
///
/// // View gotten from somewhere (e.g. as in the example above).
/// let view: Retained<UIView>;
/// # view = unsafe { objc2_ui_kit::UIView::new(objc2::MainThreadMarker::new().unwrap()) };
///
/// let mut current_responder: Retained<UIResponder> = view.into_super();
/// let mut found_controller = None;
/// while let Some(responder) = unsafe { current_responder.nextResponder() } {
///     match responder.downcast::<UIViewController>() {
///         Ok(controller) => {
///             found_controller = Some(controller);
///             break;
///         }
///         // Search next.
///         Err(responder) => current_responder = responder,
///     }
/// }
///
/// // Use found_controller here.
/// # }
/// ```
///
/// ## Thread Safety
///
/// Handles to UiKit objects can only be safely used from the main thread.
/// Therefore, all UiKit objects are `!Send` and `!Sync`.
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
pub struct UiKitWindowHandle {
    /// A pointer to an `UIView` object.
    pub ui_view: NonNull<c_void>,
}

impl UiKitWindowHandle {
    /// Create a new handle to a view.
    ///
    ///
    /// # Example
    ///
    /// Create a handle from a `UIView`.
    ///
    /// ```
    /// # #[cfg(not(all(target_vendor = "apple", not(target_os = "macos"))))]
    /// # fn main() {}
    /// # fn main() {
    /// #![cfg(all(target_vendor = "apple", not(target_os = "macos")))]
    /// use std::ptr::NonNull;
    /// use objc2::rc::Retained;
    /// use objc2_ui_kit::UIView;
    /// use raw_window_handle::UiKitWindowHandle;
    ///
    /// // UIView gotten from somewhere.
    /// let ui_view: Retained<UIView>;
    /// # ui_view = unsafe { objc2_ui_kit::UIView::new(objc2::MainThreadMarker::new().unwrap()) };
    ///
    /// // Pass it to raw-window-handle.
    /// let ui_view: NonNull<UIView> = NonNull::from(&*ui_view);
    /// let handle = UiKitWindowHandle::new(ui_view.cast());
    /// # }
    /// ```
    pub fn new(ui_view: NonNull<c_void>) -> Self {
        Self { ui_view }
    }
}
