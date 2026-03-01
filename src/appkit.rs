use core::ffi::c_void;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;
use core::{fmt, hash};

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
/// # fn main() {
/// #![cfg(target_os = "macos")]
/// use objc2::MainThreadMarker;
/// use objc2::rc::Retained;
/// use objc2_app_kit::NSView;
/// use raw_window_handle::WindowHandle;
///
/// let handle: WindowHandle<'_>; // Get the window handle from somewhere
/// # handle = unimplemented!();
/// match handle {
///     WindowHandle::AppKit(handle) => {
///         assert!(MainThreadMarker::new().is_some(), "can only access AppKit handles on the main thread");
///         let ns_view = handle.into_ns_view().cast::<NSView>().as_ptr();
///         // SAFETY: The pointer is valid, and has +1 retain count from above.
///         // Unwrap is fine, since the pointer came from `NonNull`.
///         let ns_view = unsafe { Retained::from_raw(ns_view) }.unwrap();
///         // Do something with the NSView here, like getting the `NSWindow`
///         let ns_window = ns_view.window().expect("view was not installed in a window");
///     }
///     handle => unreachable!("unknown handle {handle:?} for platform"),
/// }
/// # }
/// #
/// # #[cfg(not(target_os = "macos"))]
/// # fn main() {}
/// ```
pub struct AppKitWindowHandle {
    ns_view: NonNull<c_void>,
    // objc_retain
    retain: unsafe extern "C-unwind" fn(ns_view: *mut c_void) -> *mut c_void,
    // objc_release
    release: unsafe extern "C-unwind" fn(ns_view: *mut c_void),
}

impl Clone for AppKitWindowHandle {
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: The view pointer is guaranteed to be valid.
        let ns_view = unsafe { (self.retain)(self.ns_view.as_ptr()) };
        Self {
            ns_view: NonNull::new(ns_view).expect("retain returned NULL pointer"),
            retain: self.retain,
            release: self.release,
        }
    }
}

impl Drop for AppKitWindowHandle {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: The view pointer is guaranteed to be valid.
        unsafe { (self.release)(self.ns_view.as_ptr()) }
    }
}

impl AppKitWindowHandle {
    /// Create a new handle to a view.
    ///
    /// # Safety
    ///
    /// `ns_view` must be a valid pointer to a `NSView` with +1 retain count, and the function
    /// pointers must correctly increase / decrease the retain count of the view.
    ///
    /// # Example
    ///
    /// Create a handle from the content view of a `NSWindow` using `objc2`.
    ///
    /// ```
    /// # fn main() {
    /// #![cfg(target_os = "macos")]
    /// use std::ptr::NonNull;
    /// use std::ffi::c_void;
    /// use objc2::rc::Retained;
    /// use objc2_app_kit::{NSWindow, NSView};
    /// use raw_window_handle::AppKitWindowHandle;
    ///
    /// // NSWindow gotten from somewhere.
    /// let window: Retained<NSWindow>;
    /// # window = unsafe { objc2_app_kit::NSWindow::new(objc2::MainThreadMarker::new().unwrap()) };
    ///
    /// // Use the window's content view.
    /// let ns_view = window.contentView().unwrap();
    ///
    /// // Helper functions to retain/release the view.
    /// unsafe extern "C-unwind" fn retain(ns_view: *mut c_void) -> *mut c_void {
    ///     // SAFETY: Upheld by the caller that the pointer is a `NSView`.
    ///     // Unwrapping is fine, the pointer should be non-null.
    ///     let ns_view = unsafe { Retained::retain(ns_view.cast::<NSView>()) }.unwrap();
    ///     Retained::into_raw(ns_view).cast::<c_void>()
    /// }
    /// unsafe extern "C-unwind" fn release(ns_view: *mut c_void) {
    ///     // SAFETY: Upheld by the caller that the pointer is a `NSView`.
    ///     // Unwrapping is fine, the pointer should be non-null.
    ///     let _ = unsafe { Retained::from_raw(ns_view.cast::<NSView>()) }.unwrap();
    /// }
    ///
    /// // Pass +1 retain count.
    /// let ns_view: NonNull<c_void> = NonNull::new(Retained::into_raw(ns_view)).unwrap().cast();
    ///
    /// // SAFETY: The view is valid and has +1 retain count, and the function pointers are correct.
    /// let handle = unsafe { AppKitWindowHandle::new(ns_view, retain, release) };
    ///
    /// // Handle can be cloned, which bumps the reference-count.
    /// let handle2 = handle.clone();
    /// # }
    /// #
    /// # #[cfg(not(target_os = "macos"))]
    /// # fn main() {}
    /// ```
    ///
    /// Create a handle from an unretained `NSWindow` pointer you have from somewhere else, without
    /// dependencies.
    ///
    /// ```
    /// # fn main() {
    /// #![cfg(target_os = "macos")]
    /// use std::ptr::NonNull;
    /// use std::ffi::c_void;
    /// use raw_window_handle::AppKitWindowHandle;
    ///
    /// // Link directly to the Objective-C runtime functions.
    /// #[link(name = "objc", kind = "dylib")]
    /// unsafe extern "C-unwind" {
    ///     fn objc_retain(obj: *mut c_void) -> *mut c_void;
    ///     fn objc_release(obj: *mut c_void);
    /// }
    ///
    /// // NSView pointer gotten from somewhere.
    /// let ns_view: NonNull<c_void>;
    /// # let view = unsafe { objc2_app_kit::NSView::new(objc2::MainThreadMarker::new().unwrap()) };
    /// # ns_view = NonNull::from(&*view).cast();
    ///
    /// // Increase the reference-count of the view.
    /// let ns_view = NonNull::new(unsafe { objc_retain(ns_view.as_ptr()) }).unwrap();
    ///
    /// // SAFETY: The view is valid and has +1 retain count, and the function pointers are correct.
    /// let handle = unsafe { AppKitWindowHandle::new(ns_view, objc_retain, objc_release) };
    ///
    /// // Handle can be cloned, which bumps the reference-count.
    /// let handle2 = handle.clone();
    /// # }
    /// #
    /// # #[cfg(not(target_os = "macos"))]
    /// # fn main() {}
    /// ```
    #[inline]
    pub unsafe fn new(
        ns_view: NonNull<c_void>,
        retain: unsafe extern "C-unwind" fn(ns_view: *mut c_void) -> *mut c_void,
        release: unsafe extern "C-unwind" fn(ns_view: *mut c_void),
    ) -> Self {
        Self {
            ns_view,
            retain,
            release,
        }
    }

    /// TODO: Should we expose platform-specific methods like this?
    #[cfg(target_vendor = "apple")]
    pub unsafe fn new2(ns_view: NonNull<c_void>) -> Self {
        #[link(name = "objc", kind = "dylib")]
        unsafe extern "C-unwind" {
            fn objc_retain(obj: *mut c_void) -> *mut c_void;
            fn objc_release(obj: *mut c_void);
        }

        unsafe { Self::new(ns_view, objc_retain, objc_release) }
    }

    /// A pointer to an `NSView` object.
    ///
    /// The pointer is guaranteed to be valid for at least as long as `self`.
    #[inline]
    pub fn ns_view(&self) -> NonNull<c_void> {
        self.ns_view
    }

    /// A retained pointer to an `NSView` object.
    ///
    /// The pointer has +1 retain count, and should be released by the caller.
    #[inline]
    pub fn into_ns_view(self) -> NonNull<c_void> {
        // Pass +1 retain count to the caller.
        ManuallyDrop::new(self).ns_view
    }
}

impl PartialEq for AppKitWindowHandle {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ns_view == other.ns_view
    }
}

impl Eq for AppKitWindowHandle {}

impl hash::Hash for AppKitWindowHandle {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.ns_view.hash(state);
    }
}

impl fmt::Debug for AppKitWindowHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppKitWindowHandle")
            .field("ns_view", &self.ns_view)
            .finish()
    }
}
