use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::NonNull;

use super::DisplayHandle;

/// Raw display handle for Haiku.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HaikuDisplayHandle(());

impl HaikuDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::HaikuDisplayHandle;
    /// let handle = HaikuDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self(())
    }
}

impl DisplayHandle<'static> {
    /// Create an Haiku-based display handle.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::haiku();
    /// do_something(handle);
    /// ```
    pub fn haiku() -> Self {
        HaikuDisplayHandle::new().into()
    }
}

/// Raw window handle for Haiku.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HaikuWindowHandle<'window> {
    b_window: NonNull<c_void>,
    b_direct_window: Option<NonNull<c_void>>,
    _marker: PhantomData<&'window ()>,
}

impl HaikuWindowHandle<'_> {
    /// Create a new handle to a window.
    ///
    /// # Safety
    ///
    /// `b_window` must be a valid pointer to a `BWindow`, and must remain valid for the lifetime of
    /// this type.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::HaikuWindowHandle;
    /// # type BWindow = ();
    /// #
    /// let b_window: NonNull<BWindow>;
    /// # b_window = NonNull::from(&());
    /// let handle = unsafe { HaikuWindowHandle::new(b_window.cast()) };
    /// ```
    pub unsafe fn new(b_window: NonNull<c_void>) -> Self {
        Self {
            b_window,
            b_direct_window: None,
            _marker: PhantomData,
        }
    }

    /// Create a new window handle together with a `BDirectWindow`.
    ///
    /// # Safety
    ///
    /// `b_window` must be a valid pointer to a `BWindow`, and `b_direct_window` must be a valid
    /// pointer to `BDirectWindow`, and both pointers must remain valid for the lifetime of this
    /// type.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::HaikuWindowHandle;
    /// # type BWindow = ();
    /// # type BDirectWindow = ();
    /// #
    /// let b_window: NonNull<BWindow>;
    /// let b_direct_window: NonNull<BDirectWindow>;
    /// # b_window = NonNull::dangling();
    /// # b_direct_window = NonNull::dangling();
    /// let handle = unsafe { HaikuWindowHandle::with_window(b_window.cast(), b_direct_window.cast()) };
    /// ```
    pub unsafe fn with_window(b_window: NonNull<c_void>, b_direct_window: NonNull<c_void>) -> Self {
        Self {
            b_window,
            b_direct_window: Some(b_direct_window),
            _marker: PhantomData,
        }
    }

    /// A pointer to a BWindow object.
    ///
    /// The pointer is guaranteed to be valid for at least as long as `self`.
    pub fn b_window(&self) -> NonNull<c_void> {
        self.b_window
    }

    /// A pointer to a BDirectWindow object that might be null.
    ///
    /// If `Some`, the pointer is guaranteed to be valid for at least as long as `self`.
    pub fn b_direct_window(&self) -> Option<NonNull<c_void>> {
        self.b_direct_window
    }
}
