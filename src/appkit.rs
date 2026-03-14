use core::marker::PhantomData;

use super::DisplayHandle;

/// Raw display handle for AppKit.
///
/// ## Thread Safety
///
/// This type has the safe thread safety guarantees as [`RawWindowHandle::AppKit`].
///
/// Note that this type does not contain any Appkit objects. However,
/// it is kept `!Send` and `!Sync` for the event that Appkit objects are
/// added to this type.
///
/// [`RawWindowHandle::AppKit`]: crate::RawWindowHandle::AppKit
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
