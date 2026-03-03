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

macro_rules! noop_window_handle {
    ($e:expr) => {{
        struct Noop;

        impl AsWindowHandle for Noop {
            #[inline(always)]
            fn window_handle(&self) -> Result<BorrowedWindowHandle<'_>, HandleError> {
                $e
            }
        }

        WindowHandle::from_static(&Noop)
    }};
}

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

    /// Create a `WindowHandle` from a static window handle reference.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{WindowHandle, XcbWindowHandle, BorrowedWindowHandle, AsWindowHandle, HandleError};
    ///
    /// struct MyX11Window(u32);
    ///
    /// impl AsWindowHandle for MyX11Window {
    ///     #[inline]
    ///     fn window_handle(&self) -> Result<BorrowedWindowHandle<'_>, HandleError> {
    ///         Ok(unsafe { BorrowedWindowHandle::borrow_raw(XcbWindowHandle::new(self.0.try_into().unwrap()).into()) })
    ///     }
    /// }
    ///
    /// static WINDOW: MyX11Window = MyX11Window(67);
    /// let handle = WindowHandle::from_static(&WINDOW);
    /// match handle.window_handle().unwrap().as_raw() {
    ///     raw_window_handle::RawWindowHandle::Xcb(xcb) => assert_eq!(xcb.window.get(), 67),
    ///     _ => unreachable!()
    /// }
    /// ```
    #[inline]
    pub fn from_static<T: AsWindowHandle + Sync + 'static>(value: &'static T) -> Self {
        struct Static<T>(T);

        impl<T: AsWindowHandle + Sync + 'static> Static<T> {
            const VTABLE: WindowVtable = WindowVtable::new(Self::retain, Self::release, Self::get);

            #[inline(always)]
            unsafe fn retain(value: NonNull<()>) -> Result<NonNull<()>, HandleError> {
                Ok(value)
            }
            #[inline(always)]
            unsafe fn release(_: NonNull<()>) {}
            #[inline(always)]
            unsafe fn get(value: NonNull<()>) -> Result<RawWindowHandle, HandleError> {
                let value: &'static T = unsafe { value.cast().as_ref() };
                value.window_handle().map(|h| h.as_raw())
            }
        }

        unsafe {
            WindowHandle::from_raw_parts(
                NonNull::new_unchecked(value as *const T as *const () as *mut ()),
                &Static::<T>::VTABLE,
            )
        }
    }

    /// Create a [`WindowHandle`] that always returns `HandleError::Unavailable`.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{WindowHandle, HandleError, AsWindowHandle};
    ///
    /// let handle = WindowHandle::unavailable();
    /// assert!(matches!(handle.window_handle().unwrap_err(), HandleError::Unavailable));
    /// ```
    #[inline]
    pub fn unavailable() -> Self {
        noop_window_handle!(Err(HandleError::Unavailable))
    }

    /// Create a [`WindowHandle`] that always returns `HandleError::NotSupported`.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{WindowHandle, HandleError, AsWindowHandle};
    ///
    /// let handle = WindowHandle::not_supported();
    /// assert!(matches!(handle.window_handle().unwrap_err(), HandleError::NotSupported));
    /// ```
    #[inline]
    pub fn not_supported() -> Self {
        noop_window_handle!(Err(HandleError::NotSupported))
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

#[cfg(feature = "alloc")]
impl<T: AsWindowHandle + Send + Sync + 'static> From<alloc::sync::Arc<T>> for WindowHandle {
    /// Create a `SyncWindowHandle` from a reference counted underlying handle.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{WindowHandle, XcbWindowHandle, BorrowedWindowHandle, AsWindowHandle, HandleError};
    /// use std::sync::Arc;
    ///
    /// struct MyX11Window(u32);
    ///
    /// impl AsWindowHandle for MyX11Window {
    ///     #[inline]
    ///     fn window_handle(&self) -> Result<BorrowedWindowHandle<'_>, HandleError> {
    ///         Ok(unsafe { BorrowedWindowHandle::borrow_raw(XcbWindowHandle::new(self.0.try_into().unwrap()).into()) })
    ///     }
    /// }
    ///
    /// let window = Arc::new(MyX11Window(67));
    /// let handle = WindowHandle::from(window);
    /// match handle.window_handle().unwrap().as_raw() {
    ///     raw_window_handle::RawWindowHandle::Xcb(xcb) => assert_eq!(xcb.window.get(), 67),
    ///     _ => unreachable!()
    /// }
    /// ```
    #[inline]
    fn from(value: alloc::sync::Arc<T>) -> Self {
        struct Arc<T>(T);

        impl<T: AsWindowHandle + Send + Sync + 'static> Arc<T> {
            const VTABLE: WindowVtable = WindowVtable::new(Self::retain, Self::release, Self::get);

            #[inline(always)]
            unsafe fn retain(ptr: NonNull<()>) -> Result<NonNull<()>, HandleError> {
                // Increment the refcount.
                unsafe {
                    alloc::sync::Arc::increment_strong_count(ptr.as_ptr().cast::<T>());
                }

                Ok(ptr)
            }

            #[inline(always)]
            unsafe fn release(ptr: NonNull<()>) {
                // Decrement the refcount.
                unsafe {
                    alloc::sync::Arc::decrement_strong_count(ptr.as_ptr().cast::<T>());
                }
            }

            #[inline(always)]
            unsafe fn get(ptr: NonNull<()>) -> Result<RawWindowHandle, HandleError> {
                let value: &T = unsafe { ptr.cast().as_ref() };
                value.window_handle().map(|h| h.as_raw())
            }
        }

        // SAFETY: Above VTable is valid for this Rc.
        unsafe {
            WindowHandle::from_raw_parts(
                NonNull::new_unchecked(alloc::sync::Arc::into_raw(value).cast::<()>() as *mut ()),
                &Arc::<T>::VTABLE,
            )
        }
    }
}

/// Virtual function table for [`WindowHandle`].
pub struct WindowVtable {
    /// Increment the refcount of the window handle.
    retain: unsafe fn(NonNull<()>) -> Result<NonNull<()>, HandleError>,

    /// Decrement the refcount of the window handle.
    ///
    /// Errors in the underlying release are expected to be ignored.
    release: unsafe fn(NonNull<()>),

    /// Get the underlying window handle.
    get: unsafe fn(NonNull<()>) -> Result<RawWindowHandle, HandleError>,
}

impl WindowVtable {
    /// Create a new [`WindowVtable`] from the required function pointers.
    #[inline]
    pub const fn new(
        retain: unsafe fn(NonNull<()>) -> Result<NonNull<()>, HandleError>,
        release: unsafe fn(NonNull<()>),
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

macro_rules! noop_display_handle {
    ($e:expr) => {{
        struct Noop;

        impl AsDisplayHandle for Noop {
            #[inline(always)]
            fn display_handle(&self) -> Result<BorrowedDisplayHandle<'_>, HandleError> {
                $e
            }
        }

        DisplayHandle::from_static(&Noop)
    }};
}

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

    /// Create a `DisplayHandle` from a static Display handle reference.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{DisplayHandle, BorrowedDisplayHandle, AsDisplayHandle, HandleError};
    ///
    /// struct MyDisplay;
    ///
    /// impl AsDisplayHandle for MyDisplay {
    ///     #[inline]
    ///     fn display_handle(&self) -> Result<BorrowedDisplayHandle<'_>, HandleError> {
    ///         Ok(BorrowedDisplayHandle::windows())
    ///     }
    /// }
    ///
    /// static DISPLAY: MyDisplay = MyDisplay;
    /// let handle = DisplayHandle::from_static(&DISPLAY);
    /// match handle.display_handle().unwrap().as_raw() {
    ///     raw_window_handle::RawDisplayHandle::Windows(_) => {}
    ///     _ => unreachable!()
    /// }
    /// ```
    #[inline]
    pub fn from_static<T: AsDisplayHandle + Sync + 'static>(value: &'static T) -> Self {
        struct Static<T>(T);

        impl<T: AsDisplayHandle + Sync + 'static> Static<T> {
            const VTABLE: DisplayVtable =
                DisplayVtable::new(Self::retain, Self::release, Self::get);

            #[inline(always)]
            unsafe fn retain(value: NonNull<()>) -> Result<NonNull<()>, HandleError> {
                Ok(value)
            }
            #[inline(always)]
            unsafe fn release(_: NonNull<()>) {}
            #[inline(always)]
            unsafe fn get(value: NonNull<()>) -> Result<RawDisplayHandle, HandleError> {
                let value: &'static T = unsafe { value.cast().as_ref() };
                value.display_handle().map(|h| h.as_raw())
            }
        }

        unsafe {
            DisplayHandle::from_raw_parts(
                NonNull::new_unchecked(value as *const T as *const () as *mut ()),
                &Static::<T>::VTABLE,
            )
        }
    }

    /// Create a [`DisplayHandle`] that always returns `HandleError::Unavailable`.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{DisplayHandle, HandleError, AsDisplayHandle};
    ///
    /// let handle = DisplayHandle::unavailable();
    /// assert!(matches!(handle.display_handle().unwrap_err(), HandleError::Unavailable));
    /// ```
    #[inline]
    pub fn unavailable() -> Self {
        noop_display_handle!(Err(HandleError::Unavailable))
    }

    /// Create a [`DisplayHandle`] that always returns `HandleError::NotSupported`.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{DisplayHandle, HandleError, AsDisplayHandle};
    ///
    /// let handle = DisplayHandle::not_supported();
    /// assert!(matches!(handle.display_handle().unwrap_err(), HandleError::NotSupported));
    /// ```
    #[inline]
    pub fn not_supported() -> Self {
        noop_display_handle!(Err(HandleError::NotSupported))
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

#[cfg(feature = "alloc")]
impl<T: AsDisplayHandle + Send + Sync + 'static> From<alloc::sync::Arc<T>> for DisplayHandle {
    /// Create a `DisplayHandle` from a reference counted underlying handle.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{DisplayHandle, BorrowedDisplayHandle, AsDisplayHandle, HandleError};
    /// use std::sync::Arc;
    ///
    /// struct MyDisplay;
    ///
    /// impl AsDisplayHandle for MyDisplay {
    ///     #[inline]
    ///     fn display_handle(&self) -> Result<BorrowedDisplayHandle<'_>, HandleError> {
    ///         Ok(BorrowedDisplayHandle::windows())
    ///     }
    /// }
    ///
    /// let display = Arc::new(MyDisplay);
    /// let handle = DisplayHandle::from(display);
    /// match handle.display_handle().unwrap().as_raw() {
    ///     raw_window_handle::RawDisplayHandle::Windows(_) => {},
    ///     _ => unreachable!()
    /// }
    /// ```
    #[inline]
    fn from(value: alloc::sync::Arc<T>) -> Self {
        struct Arc<T>(T);

        impl<T: AsDisplayHandle + Send + Sync + 'static> Arc<T> {
            const VTABLE: DisplayVtable =
                DisplayVtable::new(Self::retain, Self::release, Self::get);

            #[inline(always)]
            unsafe fn retain(ptr: NonNull<()>) -> Result<NonNull<()>, HandleError> {
                // Increment the refcount.
                unsafe {
                    alloc::sync::Arc::increment_strong_count(ptr.as_ptr().cast::<T>());
                }

                Ok(ptr)
            }

            #[inline(always)]
            unsafe fn release(ptr: NonNull<()>) {
                // Decrement the refcount.
                unsafe {
                    alloc::sync::Arc::decrement_strong_count(ptr.as_ptr().cast::<T>());
                }
            }

            #[inline(always)]
            unsafe fn get(ptr: NonNull<()>) -> Result<RawDisplayHandle, HandleError> {
                let value: &T = unsafe { ptr.cast().as_ref() };
                value.display_handle().map(|h| h.as_raw())
            }
        }

        // SAFETY: Above VTable is valid for this Rc.
        unsafe {
            DisplayHandle::from_raw_parts(
                NonNull::new_unchecked(alloc::sync::Arc::into_raw(value).cast::<()>() as *mut ()),
                &Arc::<T>::VTABLE,
            )
        }
    }
}

/// Virtual function table for [`DisplayHandle`].
pub struct DisplayVtable {
    /// Increment the refcount of the Display handle.
    retain: unsafe fn(NonNull<()>) -> Result<NonNull<()>, HandleError>,

    /// Decrement the refcount of the Display handle.
    ///
    /// Errors in the underlying release are expected to be ignored.
    release: unsafe fn(NonNull<()>),

    /// Get the underlying Display handle.
    get: unsafe fn(NonNull<()>) -> Result<RawDisplayHandle, HandleError>,
}

impl DisplayVtable {
    /// Create a new [`DisplayVtable`] from the required function pointers.
    #[inline]
    pub const fn new(
        retain: unsafe fn(NonNull<()>) -> Result<NonNull<()>, HandleError>,
        release: unsafe fn(NonNull<()>),
        get: unsafe fn(NonNull<()>) -> Result<RawDisplayHandle, HandleError>,
    ) -> Self {
        Self {
            retain,
            release,
            get,
        }
    }
}
