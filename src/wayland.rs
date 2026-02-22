use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw display handle for Wayland.
///
/// ## Thread Safety
///
/// `libwayland-client` is thread safe, therefore this type is `Send` and `Sync`.
/// This means that this type can be sent to and from other threads.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaylandDisplayHandle {
    /// A pointer to a `wl_display`.
    pub display: NonNull<c_void>,
}

unsafe impl Send for WaylandDisplayHandle {}
unsafe impl Sync for WaylandDisplayHandle {}

impl WaylandDisplayHandle {
    /// Create a new display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::WaylandDisplayHandle;
    /// #
    /// let display: NonNull<c_void>;
    /// # display = NonNull::from(&()).cast();
    /// let handle = WaylandDisplayHandle::new(display);
    /// ```
    pub fn new(display: NonNull<c_void>) -> Self {
        Self { display }
    }
}

/// Raw window handle for Wayland.
///
/// ## Thread Safety
///
/// `libwayland-client` is thread safe, therefore this type is `Send` and `Sync`.
/// This means that this type can be sent to and from other threads.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaylandWindowHandle {
    /// A pointer to a `wl_surface`.
    pub surface: NonNull<c_void>,
}

unsafe impl Send for WaylandWindowHandle {}
unsafe impl Sync for WaylandWindowHandle {}

impl WaylandWindowHandle {
    /// Create a new handle to a surface.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::WaylandWindowHandle;
    /// #
    /// let surface: NonNull<c_void>;
    /// # surface = NonNull::from(&()).cast();
    /// let handle = WaylandWindowHandle::new(surface);
    /// ```
    pub fn new(surface: NonNull<c_void>) -> Self {
        Self { surface }
    }
}
