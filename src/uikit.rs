use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for UIKit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UiKitDisplayHandle(());

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
        Self(())
    }
}

impl DisplayHandle<'static> {
    /// Create a UiKit-based display handle.
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
        UiKitDisplayHandle::new().into()
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
/// use raw_window_handle::WindowHandle;
///
/// let handle: WindowHandle<'_>; // Get the window handle from somewhere else
/// # handle = unimplemented!();
/// match handle {
///     # #[cfg(requires_objc2)]
///     WindowHandle::UIKit(handle) => {
///         assert!(MainThreadMarker::new().is_some(), "can only access UIKit handles on the main thread");
///         let ui_view = handle.ui_view().as_ptr();
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UiKitWindowHandle<'window> {
    ui_view: NonNull<c_void>,
    _marker: PhantomData<&'window ()>,
}

impl UiKitWindowHandle<'_> {
    /// Create a new handle to a view.
    ///
    /// # Safety
    ///
    /// `ui_view` must be a valid pointer to a `UIView`, and must remain valid for the lifetime of
    /// this type.
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
    /// let handle = unsafe { UiKitWindowHandle::new(ui_view.cast()) };
    /// ```
    pub unsafe fn new(ui_view: NonNull<c_void>) -> Self {
        Self {
            ui_view,
            _marker: PhantomData,
        }
    }

    /// A pointer to an `UIView` object.
    ///
    /// The pointer is guaranteed to be valid for at least as long as `self`.
    pub fn ui_view(&self) -> NonNull<c_void> {
        self.ui_view
    }
}
