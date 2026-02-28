use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::NonNull;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AndroidNdkWindowHandle<'window> {
    a_native_window: NonNull<c_void>,
    _marker: PhantomData<&'window ()>,
}

impl AndroidNdkWindowHandle<'_> {
    /// Create a new handle to an `ANativeWindow`.
    ///
    /// # Safety
    ///
    /// `a_native_window` must be a valid pointer to a `ANativeWindow`, and must remain valid for
    /// the lifetime of this type.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::AndroidNdkWindowHandle;
    /// # type ANativeWindow = ();
    /// #
    /// let ptr: NonNull<ANativeWindow>;
    /// # ptr = NonNull::from(&());
    /// let handle = unsafe { AndroidNdkWindowHandle::new(ptr.cast()) };
    /// ```
    pub unsafe fn new(a_native_window: NonNull<c_void>) -> Self {
        Self {
            a_native_window,
            _marker: PhantomData,
        }
    }

    /// A pointer to an `ANativeWindow`.
    ///
    /// The pointer is guaranteed to be valid for at least as long as `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::AndroidNdkWindowHandle;
    /// # type ANativeWindow = ();
    /// #
    /// # let handle = unsafe { AndroidNdkWindowHandle::new(NonNull::dangling()) };
    /// let ptr = handle.a_native_window();
    /// let ptr = ptr.cast::<ANativeWindow>();
    /// ```
    pub fn a_native_window(&self) -> NonNull<c_void> {
        self.a_native_window
    }
}
