use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for the Redox operating system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrbitalDisplayHandle(());

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
        Self(())
    }
}

impl DisplayHandle<'static> {
    /// Create an Orbital-based display handle.
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
        OrbitalDisplayHandle::new().into()
    }
}

/// Raw window handle for the Redox operating system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrbitalWindowHandle<'window> {
    // TODO(madsmtm): I think this is a file descriptor, so perhaps it should
    // actually use `std::os::fd::RawFd`, or some sort of integer instead?
    window: NonNull<c_void>,
    _marker: PhantomData<&'window ()>,
}

impl OrbitalWindowHandle<'_> {
    /// Create a new handle to a window.
    ///
    /// # Safety
    ///
    /// `window` must be a valid pointer to an orbclient window, and must remain valid for the
    /// lifetime of this type.
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
    /// let handle = unsafe { OrbitalWindowHandle::new(window.cast()) };
    /// ```
    pub unsafe fn new(window: NonNull<c_void>) -> Self {
        Self {
            window,
            _marker: PhantomData,
        }
    }

    /// A pointer to an orbclient window.
    ///
    /// The pointer is guaranteed to be valid for at least as long as `self`.
    pub fn window(&self) -> NonNull<c_void> {
        self.window
    }
}
