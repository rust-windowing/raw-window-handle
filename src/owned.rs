//! Owned, persistent window handles.

use crate::borrowed::{
    AsDisplayHandle, AsWindowHandle, BorrowedDisplayHandle, BorrowedWindowHandle,
};
use crate::{HandleError, RawDisplayHandle, RawWindowHandle};
use core::ptr::NonNull;

/// An owned window handle.
pub struct WindowHandle {
    /// Underlying data representative of the window handle.
    ptr: NonNull<()>,

    /// Virtual function table implementing window handle functionality.
    vtable: &'static WindowVtable,
}

unsafe impl Send for WindowHandle {}
unsafe impl Sync for WindowHandle {}

impl WindowHandle {
    /// Create a new window handle from its raw parts.
    ///
    /// # Safety
    ///
    /// - `ptr` must be a valid window object.
    /// - `retain` must increment its refcount and `release` must decrement is
    ///   refcount.
    /// - While a single `retain`ed window handle still exists, the window must
    ///   remain valid (i.e. not freed).
    /// - The window handle returned by `get` must be valid for the lifetime of
    ///   this window handle.
    /// - `retain` and `release` must be atomic. This is necessary to ensure that
    ///   `WindowHandle` is `Send` and `Sync`.
    #[inline]
    pub const unsafe fn from_raw_parts(ptr: NonNull<()>, vtable: &'static WindowVtable) -> Self {
        Self { ptr, vtable }
    }

    /// Try to retain this [`WindowHandle`] in a fallible way.
    #[inline]
    pub fn try_clone(&self) -> Result<Self, HandleError> {
        // SAFETY: `from_raw_parts` constructor guarantees that `retain` properly imcrements the refcount.
        Ok(unsafe {
            let new_ptr = (self.vtable.retain)(self.ptr)?;
            Self::from_raw_parts(new_ptr, self.vtable)
        })
    }
}

impl Drop for WindowHandle {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: `from_raw_parts` constructor guarantees that `release` properly decrements the refcount.
        unsafe {
            (self.vtable.release)(self.ptr);
        }
    }
}

impl Clone for WindowHandle {
    #[inline]
    fn clone(&self) -> Self {
        self.try_clone()
            .expect("failed to increment refcount of `WindowHandle`")
    }
}

impl AsWindowHandle for WindowHandle {
    #[inline]
    fn window_handle(&self) -> Result<BorrowedWindowHandle<'_>, HandleError> {
        // SAFETY: Window handle is guaranteed to be valid while held.
        Ok(unsafe {
            let handle = (self.vtable.get)(self.ptr)?;
            BorrowedWindowHandle::borrow_raw(handle)
        })
    }
}

/// Virtual function table for [`WindowHandle`].
pub struct WindowVtable {
    /// Increment the refcount of the window handle.
    retain: unsafe fn(NonNull<()>) -> Result<NonNull<()>, HandleError>,

    /// Decrement the refcount of the window handle.
    ///
    /// Errors in the underlying release are expected to be ignored.
    release: unsafe fn(NonNull<()>) -> NonNull<()>,

    /// Get the underlying window handle.
    get: unsafe fn(NonNull<()>) -> Result<RawWindowHandle, HandleError>,
}

impl WindowVtable {
    /// Create a new [`WindowVtable`] from the required function pointers.
    #[inline]
    pub const fn new(
        retain: unsafe fn(NonNull<()>) -> Result<NonNull<()>, HandleError>,
        release: unsafe fn(NonNull<()>) -> NonNull<()>,
        get: unsafe fn(NonNull<()>) -> Result<RawWindowHandle, HandleError>,
    ) -> Self {
        Self {
            retain,
            release,
            get,
        }
    }
}

/// An owned Display handle.
pub struct DisplayHandle {
    /// Underlying data representative of the Display handle.
    ptr: NonNull<()>,

    /// Virtual function table implementing Display handle functionality.
    vtable: &'static DisplayVtable,
}

unsafe impl Send for DisplayHandle {}
unsafe impl Sync for DisplayHandle {}

impl DisplayHandle {
    /// Create a new Display handle from its raw parts.
    ///
    /// # Safety
    ///
    /// - `ptr` must be a valid Display object.
    /// - `retain` must increment its refcount and `release` must decrement is
    ///   refcount.
    /// - While a single `retain`ed Display handle still exists, the Display must
    ///   remain valid (i.e. not freed).
    /// - The Display handle returned by `get` must be valid for the lifetime of
    ///   this Display handle.
    /// - `retain` and `release` must be atomic. This is necessary to ensure that
    ///   `WindowHandle` is `Send` and `Sync`.
    #[inline]
    pub const unsafe fn from_raw_parts(ptr: NonNull<()>, vtable: &'static DisplayVtable) -> Self {
        Self { ptr, vtable }
    }

    /// Try to retain this [`DisplayHandle`] in a fallible way.
    #[inline]
    pub fn try_clone(&self) -> Result<Self, HandleError> {
        // SAFETY: `from_raw_parts` constructor guarantees that `retain` properly imcrements the refcount.
        Ok(unsafe {
            let new_ptr = (self.vtable.retain)(self.ptr)?;
            Self::from_raw_parts(new_ptr, self.vtable)
        })
    }
}

impl Drop for DisplayHandle {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: `from_raw_parts` constructor guarantees that `release` properly decrements the refcount.
        unsafe {
            (self.vtable.release)(self.ptr);
        }
    }
}

impl Clone for DisplayHandle {
    #[inline]
    fn clone(&self) -> Self {
        self.try_clone()
            .expect("failed to increment refcount of `DisplayHandle`")
    }
}

impl AsDisplayHandle for DisplayHandle {
    #[inline]
    fn display_handle(&self) -> Result<BorrowedDisplayHandle<'_>, HandleError> {
        // SAFETY: Display handle is guaranteed to be valid while held.
        Ok(unsafe {
            let handle = (self.vtable.get)(self.ptr)?;
            BorrowedDisplayHandle::borrow_raw(handle)
        })
    }
}

/// Virtual function table for [`DisplayHandle`].
pub struct DisplayVtable {
    /// Increment the refcount of the Display handle.
    retain: unsafe fn(NonNull<()>) -> Result<NonNull<()>, HandleError>,

    /// Decrement the refcount of the Display handle.
    ///
    /// Errors in the underlying release are expected to be ignored.
    release: unsafe fn(NonNull<()>) -> NonNull<()>,

    /// Get the underlying Display handle.
    get: unsafe fn(NonNull<()>) -> Result<RawDisplayHandle, HandleError>,
}

impl DisplayVtable {
    /// Create a new [`DisplayVtable`] from the required function pointers.
    #[inline]
    pub const fn new(
        retain: unsafe fn(NonNull<()>) -> Result<NonNull<()>, HandleError>,
        release: unsafe fn(NonNull<()>) -> NonNull<()>,
        get: unsafe fn(NonNull<()>) -> Result<RawDisplayHandle, HandleError>,
    ) -> Self {
        Self {
            retain,
            release,
            get,
        }
    }
}
