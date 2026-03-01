use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw display handle for Wayland.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaylandDisplayHandle {
    display: NonNull<c_void>,
}

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

    /// A pointer to a `wl_display`.
    pub fn display(&self) -> NonNull<c_void> {
        self.display
    }
}

/// Raw window handle for Wayland.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaylandWindowHandle {
    surface: NonNull<c_void>,
}

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

    /// A pointer to a `wl_surface`.
    pub fn surface(&self) -> NonNull<c_void> {
        self.surface
    }
}
