use core::ffi::c_void;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;
use core::{fmt, hash};

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
/// # fn main() {
/// #![cfg(all(target_vendor = "apple", not(target_os = "macos")))]
/// use objc2::MainThreadMarker;
/// use objc2::rc::Retained;
/// use objc2_ui_kit::UIView;
/// use raw_window_handle::WindowHandle;
///
/// let handle: WindowHandle<'_>; // Get the window handle from somewhere else
/// # handle = unimplemented!();
/// match handle {
///     WindowHandle::UiKit(handle) => {
///         assert!(MainThreadMarker::new().is_some(), "can only access UIKit handles on the main thread");
///         let ui_view = handle.into_ui_view().cast::<UIView>().as_ptr();
///         // SAFETY: The pointer is valid, and has +1 retain count from above.
///         // Unwrap is fine, since the pointer came from `NonNull`.
///         let ui_view = unsafe { Retained::from_raw(ui_view) }.unwrap();
///         // Do something with the UIView here.
///     }
///     handle => unreachable!("unknown handle {handle:?} for platform"),
/// }
/// # }
/// #
/// # #[cfg(not(all(target_vendor = "apple", not(target_os = "macos"))))]
/// # fn main() {}
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
pub struct UiKitWindowHandle {
    ui_view: NonNull<c_void>,
    // objc_retain
    retain: unsafe extern "C-unwind" fn(ui_view: *mut c_void) -> *mut c_void,
    // objc_release
    release: unsafe extern "C-unwind" fn(ui_view: *mut c_void),
}

impl Clone for UiKitWindowHandle {
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: The view pointer is guaranteed to be valid.
        let ui_view = unsafe { (self.retain)(self.ui_view.as_ptr()) };
        Self {
            ui_view: NonNull::new(ui_view).expect("retain returned NULL pointer"),
            retain: self.retain,
            release: self.release,
        }
    }
}

impl Drop for UiKitWindowHandle {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: The view pointer is guaranteed to be valid.
        unsafe { (self.release)(self.ui_view.as_ptr()) }
    }
}

impl UiKitWindowHandle {
    /// Create a new handle to a view.
    ///
    /// # Safety
    ///
    /// `ui_view` must be a valid pointer to a `UIView` with +1 retain count, and the function
    /// pointers must correctly increase / decrease the retain count of the view.
    ///
    /// # Example
    ///
    /// Create a handle from a `UIView` using `objc2`.
    ///
    /// ```
    /// # fn main() {
    /// #![cfg(all(target_vendor = "apple", not(target_os = "macos")))]
    /// use std::ptr::NonNull;
    /// use std::ffi::c_void;
    /// use objc2::rc::Retained;
    /// use objc2_ui_kit::UIView;
    /// use raw_window_handle::UiKitWindowHandle;
    ///
    /// // UIView gotten from somewhere.
    /// let view: Retained<UIView>;
    /// # view = unsafe { objc2_ui_kit::UIView::new(objc2::MainThreadMarker::new().unwrap()) };
    ///
    /// // Helper functions to retain/release the view.
    /// unsafe extern "C-unwind" fn retain(ui_view: *mut c_void) -> *mut c_void {
    ///     // SAFETY: Upheld by the caller that the pointer is a `UIView`.
    ///     // Unwrapping is fine, the pointer should be non-null.
    ///     let ui_view = unsafe { Retained::retain(ui_view.cast::<UIView>()) }.unwrap();
    ///     Retained::into_raw(ui_view).cast::<c_void>()
    /// }
    /// unsafe extern "C-unwind" fn release(ui_view: *mut c_void) {
    ///     // SAFETY: Upheld by the caller that the pointer is a `UIView`.
    ///     // Unwrapping is fine, the pointer should be non-null.
    ///     let _ = unsafe { Retained::from_raw(ui_view.cast::<UIView>()) }.unwrap();
    /// }
    ///
    /// // Pass +1 retain count.
    /// let ui_view: NonNull<c_void> = NonNull::new(Retained::into_raw(view)).unwrap().cast();
    ///
    /// // SAFETY: The view is valid and has +1 retain count, and the function pointers are correct.
    /// let handle = unsafe { UiKitWindowHandle::new(ui_view, retain, release) };
    ///
    /// // Handle can be cloned, which bumps the reference-count.
    /// let handle2 = handle.clone();
    /// # }
    /// #
    /// # #[cfg(not(all(target_vendor = "apple", not(target_os = "macos"))))]
    /// # fn main() {}
    /// ```
    ///
    /// Create a handle from an unretained `UIWindow` pointer you have from somewhere else, without
    /// dependencies.
    ///
    /// ```
    /// # fn main() {
    /// #![cfg(all(target_vendor = "apple", not(target_os = "macos")))]
    /// use std::ptr::NonNull;
    /// use std::ffi::c_void;
    /// use raw_window_handle::UiKitWindowHandle;
    ///
    /// // Link directly to the Objective-C runtime functions.
    /// #[link(name = "objc", kind = "dylib")]
    /// unsafe extern "C-unwind" {
    ///     fn objc_retain(obj: *mut c_void) -> *mut c_void;
    ///     fn objc_release(obj: *mut c_void);
    /// }
    ///
    /// // UIView pointer gotten from somewhere.
    /// let ui_view: NonNull<c_void>;
    /// # let view = unsafe { objc2_ui_kit::UIView::new(objc2::MainThreadMarker::new().unwrap()) };
    /// # ui_view = NonNull::from(&*view).cast();
    ///
    /// // Increase the reference-count of the view.
    /// let ui_view = NonNull::new(unsafe { objc_retain(ui_view.as_ptr()) }).unwrap();
    ///
    /// // SAFETY: The view is valid and has +1 retain count, and the function pointers are correct.
    /// let handle = unsafe { UiKitWindowHandle::new(ui_view, objc_retain, objc_release) };
    ///
    /// // Handle can be cloned, which bumps the reference-count.
    /// let handle2 = handle.clone();
    /// # }
    /// #
    /// # #[cfg(not(all(target_vendor = "apple", not(target_os = "macos"))))]
    /// # fn main() {}
    /// ```
    #[inline]
    pub unsafe fn new(
        ui_view: NonNull<c_void>,
        retain: unsafe extern "C-unwind" fn(ui_view: *mut c_void) -> *mut c_void,
        release: unsafe extern "C-unwind" fn(ui_view: *mut c_void),
    ) -> Self {
        Self {
            ui_view,
            retain,
            release,
        }
    }

    /// A pointer to an `UIView` object.
    ///
    /// The pointer is guaranteed to be valid for at least as long as `self`.
    #[inline]
    pub fn ui_view(&self) -> NonNull<c_void> {
        self.ui_view
    }

    /// A retained pointer to an `UIView` object.
    ///
    /// The pointer has +1 retain count, and should be released by the caller.
    #[inline]
    pub fn into_ui_view(self) -> NonNull<c_void> {
        // Pass +1 retain count to the caller.
        ManuallyDrop::new(self).ui_view
    }
}

impl PartialEq for UiKitWindowHandle {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ui_view == other.ui_view
    }
}

impl Eq for UiKitWindowHandle {}

impl hash::Hash for UiKitWindowHandle {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.ui_view.hash(state);
    }
}

impl fmt::Debug for UiKitWindowHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UiKitWindowHandle")
            .field("ui_view", &self.ui_view)
            .finish()
    }
}
