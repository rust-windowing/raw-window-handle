use core::ffi::c_void;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for the Redox operating system.
///
/// ## Thread Safety
///
/// The underlying window is a [file descriptor], and most calls on the window
/// correspond directly to non-mutating file descriptor reads and writes.
/// Therefore this type is `Send` and `Sync`. This means that this type can be
/// sent to and used from other threads.
///
/// Note that this type does not currently contain any Orbital file descriptors.
/// This type is kept as `Send` and `Sync` in preparation for file descriptors
/// to be added to this type.
///
/// [file descriptor]: https://github.com/redox-os/orbclient/blob/77c28e88fcb180c750175f2dcf5c7342d357ab26/src/sys/orbital.rs#L64-L65
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrbitalDisplayHandle {}

impl OrbitalDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::OrbitalDisplayHandle;
    /// let handle = OrbitalDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

impl DisplayHandle<'static> {
    /// Create an Orbital-based display handle.
    ///
    /// As no data is borrowed by this handle, it is completely safe to create. This function
    /// may be useful to windowing framework implementations that want to avoid unsafe code.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::orbital();
    /// do_something(handle);
    /// ```
    pub fn orbital() -> Self {
        // SAFETY: No data is borrowed.
        unsafe { Self::borrow_raw(OrbitalDisplayHandle::new().into()) }
    }
}

/// Raw window handle for the Redox operating system.
///
/// ## Thread Safety
///
/// The underlying window is a [file descriptor], and most calls on the window
/// correspond directly to non-mutating file descriptor reads and writes.
/// Therefore this type is `Send` and `Sync`. This means that this type can be
/// sent to and used from other threads.
///
/// [file descriptor]: https://github.com/redox-os/orbclient/blob/77c28e88fcb180c750175f2dcf5c7342d357ab26/src/sys/orbital.rs#L64-L65
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrbitalWindowHandle {
    /// A pointer to an orbclient window.
    // TODO(madsmtm): I think this is a file descriptor, so perhaps it should
    // actually use `std::os::fd::RawFd`, or some sort of integer instead?
    pub window: NonNull<c_void>,
}

unsafe impl Send for OrbitalWindowHandle {}
unsafe impl Sync for OrbitalWindowHandle {}

impl OrbitalWindowHandle {
    /// Create a new handle to a window.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::OrbitalWindowHandle;
    /// # type Window = ();
    /// #
    /// let window: NonNull<Window>;
    /// # window = NonNull::from(&());
    /// let mut handle = OrbitalWindowHandle::new(window.cast());
    /// ```
    pub fn new(window: NonNull<c_void>) -> Self {
        Self { window }
    }
}
