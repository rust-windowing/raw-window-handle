use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw display handle for the Linux Generic Buffer Manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GbmDisplayHandle {
    gbm_device: NonNull<c_void>,
}

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

    /// The gbm device.
    pub fn gbm_device(&self) -> NonNull<c_void> {
        self.gbm_device
    }
}

/// Raw window handle for the Linux Generic Buffer Manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GbmWindowHandle {
    gbm_surface: NonNull<c_void>,
}

impl GbmWindowHandle {
    /// Create a new handle to a surface.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::GbmWindowHandle;
    /// #
    /// let ptr: NonNull<c_void>;
    /// # ptr = NonNull::from(&()).cast();
    /// let handle = GbmWindowHandle::new(ptr);
    /// ```
    pub fn new(gbm_surface: NonNull<c_void>) -> Self {
        Self { gbm_surface }
    }

    /// The gbm surface.
    pub fn gbm_surface(&self) -> NonNull<c_void> {
        self.gbm_surface
    }
}
