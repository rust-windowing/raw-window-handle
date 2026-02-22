use core::ffi::c_void;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for Android.
///
/// ## Thread Safety
///
/// Android native objects are thread-safe by default; therefore this type is
/// `Send` and `Sync`. This means that this type can be sent to or used from
/// any thread.
///
/// Note that this type does not contain any Android native objects. However,
/// it is kept `Send` and `Sync` for the event that Android native objects are
/// added to this type.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AndroidDisplayHandle {}

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
        Self {}
    }
}

impl DisplayHandle<'static> {
    /// Create an Android-based display handle.
    ///
    /// As no data is borrowed by this handle, it is completely safe to create. This function
    /// may be useful to windowing framework implementations that want to avoid unsafe code.
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
        // SAFETY: No data is borrowed.
        unsafe { Self::borrow_raw(AndroidDisplayHandle::new().into()) }
    }
}

/// Raw window handle for Android NDK.
///
/// ## Thread Safety
///
/// Android native objects are thread-safe by default; therefore this type is
/// `Send` and `Sync`. This means that this type can be sent to or used from
/// any thread.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AndroidNdkWindowHandle {
    /// A pointer to an `ANativeWindow`.
    pub a_native_window: NonNull<c_void>,
}

unsafe impl Send for AndroidNdkWindowHandle {}
unsafe impl Sync for AndroidNdkWindowHandle {}

impl AndroidNdkWindowHandle {
    /// Create a new handle to an `ANativeWindow`.
    ///
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
    /// let handle = AndroidNdkWindowHandle::new(ptr.cast());
    /// ```
    pub fn new(a_native_window: NonNull<c_void>) -> Self {
        Self { a_native_window }
    }
}
