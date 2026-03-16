use core::marker::PhantomData;

use super::DisplayHandle;

/// Raw display handle for OpenHarmony.
///
/// ## Thread-Safety
///
/// OpenHarmony [expects] that UI primitives will only be called from one
/// thread. Therefore, all OHOS objects are `!Send` and `!Sync`. This means
/// that this type cannot be sent to or used from other threads.
///
/// Note that this type does not contain any OHOS objects. However, it is kept
/// `!Send` and `!Sync` for the event that OHOS objects are added to this
/// type.
///
/// [expects]: https://ai6s.net/6921b48882fbe0098cade00f.html
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OhosDisplayHandle {
    _thread_unsafe: PhantomData<*mut ()>,
}

impl OhosDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::OhosDisplayHandle;
    /// let handle = OhosDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self {
            _thread_unsafe: PhantomData,
        }
    }
}

impl DisplayHandle<'static> {
    /// Create an OpenHarmony-based display handle.
    ///
    /// As no data is borrowed by this handle, it is completely safe to create. This function
    /// may be useful to windowing framework implementations that want to avoid unsafe code.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::ohos();
    /// do_something(handle);
    /// ```
    pub fn ohos() -> Self {
        // SAFETY: No data is borrowed.
        unsafe { Self::borrow_raw(OhosDisplayHandle::new().into()) }
    }
}
