/// Raw display handle for the Linux Kernel Mode Set/Direct Rendering Manager.
///
/// ## Thread Safety
///
/// The DRM display handle is a file descriptor, and file descriptors in Unix
/// are thread-safe by default. Therefore this type is `Send` and `Sync`. This
/// means that it can be sent to or used from other threads.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrmDisplayHandle {
    /// The drm file descriptor.
    // TODO: Use `std::os::fd::RawFd`?
    pub fd: i32,
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
}

/// Raw window handle for the Linux Kernel Mode Set/Direct Rendering Manager.
///
/// ## Thread Safety
///
/// DRM "windows" are just planes, which are just numbers. Therefore this type
/// is `Send` and `Sync`. This means that it can be sent to or used from other
/// threads.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrmWindowHandle {
    /// The primary drm plane handle.
    pub plane: u32,
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
}
