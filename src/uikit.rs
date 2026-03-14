use core::marker::PhantomData;

use super::DisplayHandle;

/// Raw display handle for UIKit.
///
/// ## Thread Safety
///
/// This type has the same thread safety guarantees as [`RawWindowHandle::UiKit`].
///
/// Note that this type does not contain any UiKit objects. However,
/// it is kept `!Send` and `!Sync` for the event that UiKit objects are
/// added to this type.
///
/// [`RawWindowHandle::UiKit`]: crate::RawWindowHandle::UiKit
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
