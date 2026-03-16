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
