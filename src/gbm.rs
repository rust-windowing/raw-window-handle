use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw display handle for the Linux Generic Buffer Manager.
///
/// ## Thread-Safety
///
/// GBM devices are not bound to a single thread; however, they are not
/// internally secured by mutexes and cannot be used by multiple threads at
/// once. Therefore this type is `Send` but not `Sync`. This means it can be
/// sent to other threads but not used from other threads.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GbmDisplayHandle {
    /// The gbm device.
    pub gbm_device: NonNull<c_void>,
}

unsafe impl Send for GbmDisplayHandle {}

impl GbmDisplayHandle {
    /// Create a new handle to a device.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::GbmDisplayHandle;
    /// #
    /// let ptr: NonNull<c_void>;
    /// # ptr = NonNull::from(&()).cast();
    /// let handle = GbmDisplayHandle::new(ptr);
    /// ```
    pub fn new(gbm_device: NonNull<c_void>) -> Self {
        Self { gbm_device }
    }
}
