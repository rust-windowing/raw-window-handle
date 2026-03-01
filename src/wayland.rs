use core::ffi::c_void;
use core::ptr::NonNull;
use core::{fmt, hash};

/// Display handle for Wayland.
///
/// See [`WaylandWindowHandle`] for discussion about the design of this.
pub struct WaylandDisplayHandle {
    info: *const (),
    increment_strong_count: unsafe fn(info: *const ()),
    decrement_strong_count: unsafe fn(info: *const ()),
    get_display: unsafe fn(info: *const ()) -> NonNull<c_void>,
}

impl Clone for WaylandDisplayHandle {
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: The info pointer is valid.
        unsafe { (self.increment_strong_count)(self.info) };
        Self {
            info: self.info,
            increment_strong_count: self.increment_strong_count,
            decrement_strong_count: self.decrement_strong_count,
            get_display: self.get_display,
        }
    }
}

impl Drop for WaylandDisplayHandle {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: The info pointer is valid.
        unsafe { (self.decrement_strong_count)(self.info) };
    }
}

impl WaylandDisplayHandle {
    /// Create a new display handle.
    ///
    /// # Safety
    ///
    /// Similar to [`WaylandWindowHandle::new`].
    ///
    /// # Example
    ///
    /// ```
    /// todo!()
    /// ```
    #[inline]
    pub unsafe fn new(
        info: *const (),
        increment_strong_count: unsafe fn(info: *const ()),
        decrement_strong_count: unsafe fn(info: *const ()),
        get_display: unsafe fn(info: *const ()) -> NonNull<c_void>,
    ) -> Self {
        Self {
            info,
            increment_strong_count,
            decrement_strong_count,
            get_display,
        }
    }

    /// Create an unretained handle to a display.
    ///
    /// # Safety
    ///
    /// Similar to [`WaylandWindowHandle::new_unchecked`].
    #[inline]
    pub unsafe fn new_unchecked(wl_display: NonNull<c_void>) -> Self {
        let info = wl_display.cast().as_ptr();

        // These are intentionally no-ops, the caller ensures that the surface is alive.
        fn increment_strong_count(_info: *const ()) {}
        fn decrement_strong_count(_info: *const ()) {}

        fn get_display(info: *const ()) -> NonNull<c_void> {
            NonNull::new(info.cast_mut().cast()).unwrap()
        }

        Self {
            info,
            increment_strong_count,
            decrement_strong_count,
            get_display,
        }
    }

    /// A pointer to a `wl_display`.
    ///
    /// The pointer is guaranteed to be valid for at least as long as `self`.
    ///
    /// # Example
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn display(&self) -> NonNull<c_void> {
        // SAFETY: The info pointer is valid.
        unsafe { (self.get_display)(self.info) }
    }
}

impl PartialEq for WaylandDisplayHandle {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.display() == other.display()
    }
}

impl Eq for WaylandDisplayHandle {}

impl hash::Hash for WaylandDisplayHandle {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.display().hash(state);
    }
}

impl fmt::Debug for WaylandDisplayHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WaylandDisplayHandle")
            .field("display", &self.display())
            .finish()
    }
}

/// Window handle for Wayland.
///
/// libwayland proxies, unlike most other platforms' handles, are neither reference-counted nor an
/// ID. This is problematic for us, because we'd really prefer [`WindowHandle`][crate::WindowHandle]
/// to not contain a lifetime parameter or generic (this makes usage much simpler for the end user).
///
/// To solve this, we provide two options for constructing this handle:
/// - [`WaylandWindowHandle::new`], which requires that you reference-count the handle and delay
///   destruction of the underlying `wl_proxy` until the last reference to the `WaylandWindowHandle`
///   is dropped. You can think of this as storing `Arc<WlSurface>`.
/// - [`WaylandWindowHandle::new_unchecked`], which still requires that you keep the surface alive
///   for as long as any `WaylandWindowHandle` exists, but since it's done without
///   reference-counting, you must unsafely assert this. This option can only be safely used if you
///   know exactly how the handle will be used.
pub struct WaylandWindowHandle {
    info: *const (),
    increment_strong_count: unsafe fn(info: *const ()),
    decrement_strong_count: unsafe fn(info: *const ()),
    // TODO: Should we store a function to get the surface pointer, or just the surface pointer
    // itself? The user is required to always return the same pointer, so either option is valid.
    get_surface: unsafe fn(info: *const ()) -> NonNull<c_void>,
}

impl Clone for WaylandWindowHandle {
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: The info pointer is valid.
        unsafe { (self.increment_strong_count)(self.info) };
        Self {
            info: self.info,
            increment_strong_count: self.increment_strong_count,
            decrement_strong_count: self.decrement_strong_count,
            get_surface: self.get_surface,
        }
    }
}

impl Drop for WaylandWindowHandle {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: The info pointer is valid.
        unsafe { (self.decrement_strong_count)(self.info) };
    }
}

impl WaylandWindowHandle {
    /// Create a new handle to a surface.
    ///
    /// # Safety
    ///
    /// `info` must be a valid pointer to a something that can be reference-counted with the given
    /// functions, and the `get_surface` function must return a `wl_surface` pointer that is valid
    /// for as long as the `info` pointer is alive.
    ///
    /// # Example
    ///
    /// ```
    /// use std::ptr::NonNull;
    /// use std::ffi::c_void;
    /// use std::sync::Arc;
    /// use raw_window_handle::WaylandWindowHandle;
    ///
    /// // wl_surface pointer gotten from somewhere.
    /// let wl_surface: NonNull<c_void>;
    /// # wl_surface = NonNull::dangling();
    ///
    /// todo!();
    /// ```
    #[inline]
    pub unsafe fn new(
        info: *const (),
        increment_strong_count: unsafe fn(info: *const ()),
        decrement_strong_count: unsafe fn(info: *const ()),
        get_surface: unsafe fn(info: *const ()) -> NonNull<c_void>,
    ) -> Self {
        Self {
            info,
            increment_strong_count,
            decrement_strong_count,
            get_surface,
        }
    }

    /// Create an unretained handle to a surface.
    ///
    /// This is intended as an "escape hook" for users that want to use `raw-window-handle`, but do
    /// not want (or cannot) add reference-counting to their handles.
    ///
    /// It is strongly recommended to use [`WaylandWindowHandle::new`] instead.
    ///
    /// # Safety
    ///
    /// The given `wl_surface` must be valid for the entire duration of `WaylandWindowHandle`, **and
    /// any of its clones**.
    ///
    /// This is impossible to ensure in general (since this type does not contain a lifetime
    /// parameter), so as a library author, you must not expose the returned instance safely to your
    /// users.
    ///
    /// # Example
    ///
    /// ```
    /// todo!();
    /// ```
    #[inline]
    pub unsafe fn new_unchecked(wl_surface: NonNull<c_void>) -> Self {
        let info = wl_surface.cast().as_ptr();

        // These are intentionally no-ops, the caller ensures that the surface is alive.
        fn increment_strong_count(_info: *const ()) {}
        fn decrement_strong_count(_info: *const ()) {}

        fn get_surface(info: *const ()) -> NonNull<c_void> {
            NonNull::new(info.cast_mut().cast()).unwrap()
        }

        Self {
            info,
            increment_strong_count,
            decrement_strong_count,
            get_surface,
        }
    }

    // TODO: pub unsafe fn from_arc(Arc<dyn Fn() -> NonNull<c_void>>) ?

    /// A pointer to a `wl_surface`.
    ///
    /// The pointer is guaranteed to be valid for at least as long as this type is alive.
    ///
    /// # Example
    ///
    /// ```
    /// todo!()
    /// ```
    #[inline]
    pub fn surface(&self) -> NonNull<c_void> {
        // SAFETY: The info pointer is valid.
        unsafe { (self.get_surface)(self.info) }
    }
}

impl PartialEq for WaylandWindowHandle {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.surface() == other.surface()
    }
}

impl Eq for WaylandWindowHandle {}

impl hash::Hash for WaylandWindowHandle {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.surface().hash(state);
    }
}

impl fmt::Debug for WaylandWindowHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WaylandWindowHandle")
            .field("surface", &self.surface())
            .finish()
    }
}
