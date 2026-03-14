use super::DisplayHandle;

/// Raw display handle for Haiku.
///
/// ## Thread Safety
///
/// Haiku objects are protected by a [global lock]. They are `Send` and `Sync`
/// as long as producers/downstream consumers take this lock before the `BLooper`
/// or `BWindow` are used outside of their origin threads.
///
/// Note that this type does not currently contain any Haiku objects. However,
/// it is kept `Send` and `Sync` for the event that Haiku objects are added to
/// this type.
///
/// [global lock]: https://grok.nikisoft.one/opengrok/xref/haiku/src/kits/app/Looper.cpp?r=b47e8b0cadeb9a9d985d7f72d2e9a099cbcb8f90#591-627
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HaikuDisplayHandle {}

impl HaikuDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::HaikuDisplayHandle;
    /// let handle = HaikuDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

impl DisplayHandle<'static> {
    /// Create an Haiku-based display handle.
    ///
    /// As no data is borrowed by this handle, it is completely safe to create. This function
    /// may be useful to windowing framework implementations that want to avoid unsafe code.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::haiku();
    /// do_something(handle);
    /// ```
    pub fn haiku() -> Self {
        // SAFETY: No data is borrowed.
        unsafe { Self::borrow_raw(HaikuDisplayHandle::new().into()) }
    }
}
