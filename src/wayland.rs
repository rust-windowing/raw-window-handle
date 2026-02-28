use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::NonNull;

/// Raw display handle for Wayland.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaylandDisplayHandle<'display> {
    display: NonNull<c_void>,
    _marker: PhantomData<&'display ()>,
}

impl WaylandDisplayHandle<'_> {
    /// Create a new display handle.
    ///
    /// # Safety
    ///
    /// `display` must be a valid pointer to a `wl_display` and must remain valid for the lifetime
    /// of this type.
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
    /// let handle = unsafe { WaylandDisplayHandle::new(display) };
    /// ```
    pub unsafe fn new(display: NonNull<c_void>) -> Self {
        Self {
            display,
            _marker: PhantomData,
        }
    }

    /// A pointer to a `wl_display`.
    ///
    /// The pointer is guaranteed to be valid for at least as long as `self`.
    pub fn display(&self) -> NonNull<c_void> {
        self.display
    }
}

/// Raw window handle for Wayland.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaylandWindowHandle<'window> {
    surface: NonNull<c_void>,
    _marker: PhantomData<&'window ()>,
}

impl WaylandWindowHandle<'_> {
    /// Create a new handle to a surface.
    ///
    /// # Safety
    ///
    /// `display` must be a valid pointer to a `wl_surface` and must remain valid for the lifetime
    /// of this type.
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
    /// let handle = unsafe { WaylandWindowHandle::new(surface) };
    /// ```
    pub unsafe fn new(surface: NonNull<c_void>) -> Self {
        Self {
            surface,
            _marker: PhantomData,
        }
    }

    /// A pointer to a `wl_surface`.
    ///
    /// The pointer is guaranteed to be valid for at least as long as `self`.
    pub fn surface(&self) -> NonNull<c_void> {
        self.surface
    }
}
