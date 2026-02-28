use core::marker::PhantomData;

/// Raw display handle for the Linux Kernel Mode Set/Direct Rendering Manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrmDisplayHandle<'display> {
    // TODO: Use `std::os::fd::RawFd`? Or `BorrowedFd`?
    fd: i32,
    _marker: PhantomData<&'display ()>,
}

impl DrmDisplayHandle<'_> {
    /// Create a new handle to a file descriptor.
    ///
    /// # Safety
    ///
    /// TODO.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::DrmDisplayHandle;
    /// #
    /// let fd: i32;
    /// # fd = 0;
    /// let handle = unsafe { DrmDisplayHandle::new(fd) };
    /// ```
    pub unsafe fn new(fd: i32) -> Self {
        Self {
            fd,
            _marker: PhantomData,
        }
    }

    /// The drm file descriptor.
    pub fn fd(&self) -> i32 {
        self.fd
    }
}

/// Raw window handle for the Linux Kernel Mode Set/Direct Rendering Manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrmWindowHandle {
    plane: u32,
}

impl DrmWindowHandle {
    /// Create a new handle to a plane.
    ///
    /// # Safety
    ///
    /// TODO.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::DrmWindowHandle;
    /// #
    /// let plane: u32;
    /// # plane = 0;
    /// let handle = unsafe { DrmWindowHandle::new(plane) };
    /// ```
    pub unsafe fn new(plane: u32) -> Self {
        Self { plane }
    }

    /// The primary drm plane handle.
    pub fn plane(&self) -> u32 {
        self.plane
    }
}
