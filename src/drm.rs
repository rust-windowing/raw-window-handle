/// Raw display handle for the Linux Kernel Mode Set/Direct Rendering Manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrmDisplayHandle {
    // TODO: Use `std::os::fd::RawFd`?
    fd: i32,
}

impl DrmDisplayHandle {
    /// Create a new handle to a file descriptor.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::DrmDisplayHandle;
    /// #
    /// let fd: i32;
    /// # fd = 0;
    /// let handle = DrmDisplayHandle::new(fd);
    /// ```
    pub fn new(fd: i32) -> Self {
        Self { fd }
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
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::DrmWindowHandle;
    /// #
    /// let plane: u32;
    /// # plane = 0;
    /// let handle = DrmWindowHandle::new(plane);
    /// ```
    pub fn new(plane: u32) -> Self {
        Self { plane }
    }

    /// The primary drm plane handle.
    pub fn plane(&self) -> u32 {
        self.plane
    }
}
