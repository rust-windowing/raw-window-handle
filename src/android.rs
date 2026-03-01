use core::ffi::c_void;
use core::ptr::NonNull;
use core::{fmt, hash};

use super::DisplayHandle;

/// Raw display handle for Android.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AndroidDisplayHandle(());

impl AndroidDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::AndroidDisplayHandle;
    /// let handle = AndroidDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self(())
    }
}

impl DisplayHandle<'static> {
    /// Create an Android-based display handle.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::android();
    /// do_something(handle);
    /// ```
    pub fn android() -> Self {
        AndroidDisplayHandle::new().into()
    }
}

/// Raw window handle for Android NDK.
pub struct AndroidNdkWindowHandle {
    a_native_window: NonNull<c_void>,
    // ANativeWindow_acquire
    acquire: unsafe extern "C" fn(a_native_window: NonNull<c_void>),
    // ANativeWindow_release
    release: unsafe extern "C" fn(a_native_window: NonNull<c_void>),
}

impl Clone for AndroidNdkWindowHandle {
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: The window pointer is valid.
        unsafe { (self.acquire)(self.a_native_window) };
        Self {
            a_native_window: self.a_native_window,
            acquire: self.acquire,
            release: self.release,
        }
    }
}

impl Drop for AndroidNdkWindowHandle {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: The window pointer is valid.
        unsafe { (self.release)(self.a_native_window) };
    }
}

impl AndroidNdkWindowHandle {
    /// Create a new handle to an `ANativeWindow`.
    ///
    /// # Safety
    ///
    /// `a_native_window` must be a valid pointer to a `ANativeWindow`, and the given function
    /// pointers must correctly acquire / release the window.
    ///
    /// This function takes ownership of the pointer.
    ///
    /// # Example
    ///
    /// Create a handle using the `ndk` crate.
    ///
    /// ```
    /// # fn inner() {
    /// #![cfg(target_os = "android")]
    /// use std::ffi::c_void;
    /// use std::mem::{self, ManuallyDrop};
    /// use std::ptr::NonNull;
    /// use ndk::native_window::NativeWindow;
    /// use raw_window_handle::AndroidNdkWindowHandle;
    ///
    /// // Window gotten from somewhere (for example using `android-activity`).
    /// let window: NativeWindow;
    /// window = unimplemented!();
    ///
    /// // Helper functions to acquire/release the window.
    /// unsafe extern "C" fn acquire(a_native_window: NonNull<c_void>) {
    ///     // SAFETY: Upheld by the caller that the pointer is a `ANativeWindow`.
    ///     mem::forget(unsafe { NativeWindow::clone_from_ptr(a_native_window.cast()) });
    /// }
    /// unsafe extern "C" fn release(a_native_window: NonNull<c_void>) {
    ///     // SAFETY: Upheld by the caller that the pointer is a `ANativeWindow`.
    ///     let _ = unsafe { NativeWindow::from_ptr(a_native_window.cast()) };
    /// }
    ///
    /// // Pass reference count to `AndroidNdkWindowHandle`.
    /// let a_native_window: NonNull<c_void> = ManuallyDrop::new(window).ptr().cast();
    ///
    /// // SAFETY: The window is valid and the function pointers are correct.
    /// let handle = unsafe { AndroidNdkWindowHandle::new(a_native_window, acquire, release) };
    ///
    /// // Handle can be cloned, which acquires it, and releases it when dropped.
    /// let handle2 = handle.clone();
    /// # }
    /// ```
    #[inline]
    pub unsafe fn new(
        a_native_window: NonNull<c_void>,
        acquire: unsafe extern "C" fn(a_native_window: NonNull<c_void>),
        release: unsafe extern "C" fn(a_native_window: NonNull<c_void>),
    ) -> Self {
        Self {
            a_native_window,
            acquire,
            release,
        }
    }

    /// A pointer to an `ANativeWindow`.
    ///
    /// The pointer is guaranteed to be valid for at least as long as `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # fn inner() {
    /// #![cfg(target_os = "android")]
    /// use ndk::native_window::NativeWindow;
    /// use raw_window_handle::AndroidNdkWindowHandle;
    ///
    /// // Gotten from somewhere.
    /// let handle: AndroidNdkWindowHandle;
    /// # handle = unimplemented!();
    ///
    /// // SAFETY: The pointer is a valid `ANativeWindow`.
    /// let window = unsafe { NativeWindow::clone_from_ptr(handle.a_native_window().cast()) };
    ///
    /// // Do stuff with `window` here.
    /// # }
    /// ```
    #[inline]
    pub fn a_native_window(&self) -> NonNull<c_void> {
        self.a_native_window
    }
}

impl PartialEq for AndroidNdkWindowHandle {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.a_native_window == other.a_native_window
    }
}

impl Eq for AndroidNdkWindowHandle {}

impl hash::Hash for AndroidNdkWindowHandle {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.a_native_window.hash(state);
    }
}

impl fmt::Debug for AndroidNdkWindowHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AndroidNdkWindowHandle")
            .field("a_native_window", &self.a_native_window)
            .finish()
    }
}
