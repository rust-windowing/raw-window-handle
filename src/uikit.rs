use core::ffi::c_void;
use core::ptr::NonNull;

use super::DisplayHandle;

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
/// Note that `UIView` can only be accessed from the main thread of the
/// application. This struct is `!Send` and `!Sync` to help with ensuring
/// that.
///
/// # Example
///
/// Getting the view from a [`WindowHandle`][crate::WindowHandle].
///
/// ```no_run
/// # fn inner() {
/// #![cfg(any(target_os = "ios", target_os = "tvos", target_os = "watchos", target_os = "xros"))]
/// # #[cfg(requires_objc2)]
/// use objc2::MainThreadMarker;
/// # #[cfg(requires_objc2)]
/// use objc2::rc::Retained;
/// # #[cfg(requires_objc2)]
/// use objc2_ui_kit::UIView;
/// use raw_window_handle::{WindowHandle, RawWindowHandle};
///
/// let handle: WindowHandle<'_>; // Get the window handle from somewhere else
/// # handle = unimplemented!();
/// match handle.as_raw() {
///     # #[cfg(requires_objc2)]
///     RawWindowHandle::UIKit(handle) => {
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
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UiKitWindowHandle {
    /// A pointer to an `UIView` object.
    pub ui_view: NonNull<c_void>,
    /// A pointer to an `UIViewController` object, if the view has one.
    ///
    /// This is deprecated, the controller should be retrieved by traversing the `UIView`'s\
    /// responder chain instead:
    ///
    /// ```ignore
    /// use objc2::rc::Retained;
    /// use objc2_ui_kit::{UIResponder, UIView, UIViewController};
    ///
    /// let view: Retained<UIView> = ...;
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
    /// ```
    #[deprecated = "retrieve the view controller from the UIView's responder chain instead"]
    pub ui_view_controller: Option<NonNull<c_void>>,
}

impl UiKitWindowHandle {
    /// Create a new handle to a view.
    ///
    ///
    /// # Example
    ///
    /// Create a handle from a `UIView`.
    ///
    /// ```ignore
    /// use std::ptr::NonNull;
    /// use objc2::rc::Retained;
    /// use objc2_ui_kit::UIView;
    /// use raw_window_handle::UiKitWindowHandle;
    ///
    /// let ui_view: Retained<UIView> = ...;
    /// let ui_view: NonNull<UIView> = NonNull::from(&*ui_view);
    /// let handle = UiKitWindowHandle::new(ui_view.cast());
    /// ```
    pub fn new(ui_view: NonNull<c_void>) -> Self {
        #[allow(deprecated)]
        Self {
            ui_view,
            ui_view_controller: None,
        }
    }
}
