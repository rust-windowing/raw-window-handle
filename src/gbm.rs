use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::NonNull;

/// Raw display handle for the Linux Generic Buffer Manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GbmDisplayHandle<'display> {
    gbm_device: NonNull<c_void>,
    _marker: PhantomData<&'display ()>,
}

impl GbmDisplayHandle<'_> {
    /// Create a new handle to a device.
    ///
    /// # Safety
    ///
    /// `gbm_device` must be a valid pointer, and the pointer must remain valid for the lifetime of
    /// this type.
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
    /// let handle = unsafe { GbmDisplayHandle::new(ptr) };
    /// ```
    pub unsafe fn new(gbm_device: NonNull<c_void>) -> Self {
        Self {
            gbm_device,
            _marker: PhantomData,
        }
    }

    /// The gbm device.
    pub fn gbm_device(&self) -> NonNull<c_void> {
        self.gbm_device
    }
}

/// Raw window handle for the Linux Generic Buffer Manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GbmWindowHandle<'window> {
    gbm_surface: NonNull<c_void>,
    _marker: PhantomData<&'window ()>,
}

impl GbmWindowHandle<'_> {
    /// Create a new handle to a surface.
    ///
    /// # Safety
    ///
    /// `gbm_surface` must be a valid pointer and must remain valid for the lifetime of this type.
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
    /// let handle = unsafe { GbmWindowHandle::new(ptr) };
    /// ```
    pub unsafe fn new(gbm_surface: NonNull<c_void>) -> Self {
        Self {
            gbm_surface,
            _marker: PhantomData,
        }
    }

    /// The gbm surface.
    ///
    /// The pointer is guaranteed to be valid for at least as long as `self`.
    pub fn gbm_surface(&self) -> NonNull<c_void> {
        self.gbm_surface
    }
}
