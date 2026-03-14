//! Owned, persistent window handles.

use crate::borrowed::{
    AsDisplayHandle, AsWindowHandle, BorrowedDisplayHandle, BorrowedWindowHandle,
};
use crate::{HandleError, RawDisplayHandle, RawWindowHandle};

use core::fmt;
use core::ptr::NonNull;

/// An owned window handle.
pub struct WindowHandle {
    /// Underlying data representative of the window handle.
    ptr: NonNull<()>,

    /// Virtual function table implementing window handle functionality.
    vtable: &'static WindowVtable,
}

/// A thread-safe owned window handle.
#[derive(Debug, Clone)]
pub struct SyncWindowHandle {
    /// Inner window handle.
    inner: WindowHandle,
}

unsafe impl Send for SyncWindowHandle {}
unsafe impl Sync for SyncWindowHandle {}

macro_rules! noop_window_handle {
    ($e:expr) => {{
        struct Noop;

        impl AsWindowHandle for Noop {
            #[inline(always)]
            fn window_handle(&self) -> Result<BorrowedWindowHandle<'_>, HandleError> {
                $e
            }
        }

        SyncWindowHandle::from_static(&Noop)
    }};
}

impl fmt::Debug for WindowHandle {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.window_handle() {
            Ok(handle) => f.debug_tuple("WindowHandle").field(&handle).finish(),
            Err(_) => f
                .debug_tuple("WindowHandle")
                .field(&format_args!("<error>"))
                .finish(),
        }
    }
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
    ///
    /// Note that this API expects the underlying handle to be retained by
    /// default. So if the `retain` callback is called `N` times, `release`
    /// will be called `N+1` times.
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
    pub fn from_static<T: AsWindowHandle + 'static>(value: &'static T) -> Self {
        struct Static<T>(T);

        impl<T: AsWindowHandle + 'static> Static<T> {
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
        SyncWindowHandle::unavailable().into()
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
        SyncWindowHandle::not_supported().into()
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
impl<T: AsWindowHandle + 'static> From<alloc::rc::Rc<T>> for WindowHandle {
    /// Create a `WindowHandle` from a reference counted underlying handle.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{WindowHandle, XcbWindowHandle, BorrowedWindowHandle, AsWindowHandle, HandleError};
    /// use std::rc::Rc;
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
    /// let window = Rc::new(MyX11Window(67));
    /// let handle = WindowHandle::from(window);
    /// match handle.window_handle().unwrap().as_raw() {
    ///     raw_window_handle::RawWindowHandle::Xcb(xcb) => assert_eq!(xcb.window.get(), 67),
    ///     _ => unreachable!()
    /// }
    /// ```
    #[inline]
    fn from(value: alloc::rc::Rc<T>) -> Self {
        struct Rc<T>(T);

        impl<T: AsWindowHandle + 'static> Rc<T> {
            const VTABLE: WindowVtable = WindowVtable::new(Self::retain, Self::release, Self::get);

            #[inline(always)]
            unsafe fn retain(ptr: NonNull<()>) -> Result<NonNull<()>, HandleError> {
                // Increment the refcount.
                unsafe {
                    alloc::rc::Rc::increment_strong_count(ptr.as_ptr().cast::<T>());
                }

                Ok(ptr)
            }

            #[inline(always)]
            unsafe fn release(ptr: NonNull<()>) {
                // Decrement the refcount.
                unsafe {
                    alloc::rc::Rc::decrement_strong_count(ptr.as_ptr().cast::<T>());
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
                NonNull::new_unchecked(alloc::rc::Rc::into_raw(value).cast::<()>() as *mut ()),
                &Rc::<T>::VTABLE,
            )
        }
    }
}

#[cfg(feature = "alloc")]
impl<T: AsWindowHandle + Send + Sync + 'static> From<alloc::sync::Arc<T>> for WindowHandle {
    /// Create a `WindowHandle` from a reference counted underlying handle.
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
        SyncWindowHandle::from(value).into()
    }
}

impl SyncWindowHandle {
    /// Create a new [`SyncWindowHandle`] from a [`WindowHandle`].
    ///
    /// # Safety
    ///
    /// In addition to the other safety requirements of [`WindowHandle`], it is
    /// also required that the `retain` and `release` functions are thread safe.
    /// As in, the internal refcounting mechanism for the window handle must be
    /// atomic.
    #[inline]
    pub const unsafe fn new(handle: WindowHandle) -> Self {
        Self { inner: handle }
    }

    /// Clone this [`SyncWindowHandle`] fallibly.
    #[inline]
    pub fn try_clone(&self) -> Result<Self, HandleError> {
        self.inner.try_clone().map(|x| unsafe { Self::new(x) })
    }

    /// Create a `SyncWindowHandle` from a static window handle reference.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{SyncWindowHandle, XcbWindowHandle, BorrowedWindowHandle, AsWindowHandle, HandleError};
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
    /// let handle = SyncWindowHandle::from_static(&WINDOW);
    /// match handle.window_handle().unwrap().as_raw() {
    ///     raw_window_handle::RawWindowHandle::Xcb(xcb) => assert_eq!(xcb.window.get(), 67),
    ///     _ => unreachable!()
    /// }
    /// ```
    #[inline]
    pub fn from_static<T: AsWindowHandle + Sync + 'static>(window: &'static T) -> Self {
        let inner = WindowHandle::from_static(window);

        // SAFETY: Since `T` is `Sync`, `&T` is `Send + Sync`, making it possible
        // to send this window handle reference across threads.
        unsafe { Self::new(inner) }
    }

    /// Create a [`SyncWindowHandle`] that always returns `HandleError::Unavailable`.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{SyncWindowHandle, HandleError, AsWindowHandle};
    ///
    /// let handle = SyncWindowHandle::unavailable();
    /// assert!(matches!(handle.window_handle().unwrap_err(), HandleError::Unavailable));
    /// ```
    #[inline]
    pub fn unavailable() -> Self {
        noop_window_handle!(Err(HandleError::Unavailable))
    }

    /// Create a [`SyncWindowHandle`] that always returns `HandleError::NotSupported`.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{SyncWindowHandle, HandleError, AsWindowHandle};
    ///
    /// let handle = SyncWindowHandle::not_supported();
    /// assert!(matches!(handle.window_handle().unwrap_err(), HandleError::NotSupported));
    /// ```
    #[inline]
    pub fn not_supported() -> Self {
        noop_window_handle!(Err(HandleError::NotSupported))
    }
}

impl AsWindowHandle for SyncWindowHandle {
    #[inline]
    fn window_handle(&self) -> Result<BorrowedWindowHandle<'_>, HandleError> {
        self.inner.window_handle()
    }
}

impl From<SyncWindowHandle> for WindowHandle {
    #[inline]
    fn from(value: SyncWindowHandle) -> Self {
        value.inner
    }
}

#[cfg(feature = "alloc")]
impl<T: AsWindowHandle + Send + Sync + 'static> From<alloc::sync::Arc<T>> for SyncWindowHandle {
    /// Create a `SyncWindowHandle` from a reference counted underlying handle.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{SyncWindowHandle, XcbWindowHandle, BorrowedWindowHandle, AsWindowHandle, HandleError};
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
    /// let handle = SyncWindowHandle::from(window);
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
            SyncWindowHandle::new(WindowHandle::from_raw_parts(
                NonNull::new_unchecked(alloc::sync::Arc::into_raw(value).cast::<()>() as *mut ()),
                &Arc::<T>::VTABLE,
            ))
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

/// An owned display handle.
pub struct DisplayHandle {
    /// Underlying data representative of the Display handle.
    ptr: NonNull<()>,

    /// Virtual function table implementing Display handle functionality.
    vtable: &'static DisplayVtable,
}

/// A thread-safe owned display handle.
#[derive(Clone)]
pub struct SyncDisplayHandle {
    /// Inner Display handle.
    inner: DisplayHandle,
}

unsafe impl Send for SyncDisplayHandle {}
unsafe impl Sync for SyncDisplayHandle {}

macro_rules! noop_display_handle {
    ($e:expr) => {{
        struct Noop;

        impl AsDisplayHandle for Noop {
            #[inline(always)]
            fn display_handle(&self) -> Result<BorrowedDisplayHandle<'_>, HandleError> {
                $e
            }
        }

        SyncDisplayHandle::from_static(&Noop)
    }};
}

impl fmt::Debug for DisplayHandle {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.display_handle() {
            Ok(handle) => f.debug_tuple("DisplayHandle").field(&handle).finish(),
            Err(_) => f
                .debug_tuple("DisplayHandle")
                .field(&format_args!("<error>"))
                .finish(),
        }
    }
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
    ///
    /// Note that this API expects the underlying handle to be retained by
    /// default. So if the `retain` callback is called `N` times, `release`
    /// will be called `N+1` times.
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
        SyncDisplayHandle::unavailable().into()
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
        SyncDisplayHandle::not_supported().into()
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
impl<T: AsDisplayHandle + 'static> From<alloc::rc::Rc<T>> for DisplayHandle {
    /// Create a `DisplayHandle` from a reference counted underlying handle.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{DisplayHandle, BorrowedDisplayHandle, AsDisplayHandle, HandleError};
    /// use std::rc::Rc;
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
    /// let display = Rc::new(MyDisplay);
    /// let handle = DisplayHandle::from(display);
    /// match handle.display_handle().unwrap().as_raw() {
    ///     raw_window_handle::RawDisplayHandle::Windows(_) => {},
    ///     _ => unreachable!()
    /// }
    /// ```
    #[inline]
    fn from(value: alloc::rc::Rc<T>) -> Self {
        struct Rc<T>(T);

        impl<T: AsDisplayHandle + 'static> Rc<T> {
            const VTABLE: DisplayVtable =
                DisplayVtable::new(Self::retain, Self::release, Self::get);

            #[inline(always)]
            unsafe fn retain(ptr: NonNull<()>) -> Result<NonNull<()>, HandleError> {
                // Increment the refcount.
                unsafe {
                    alloc::rc::Rc::increment_strong_count(ptr.as_ptr().cast::<T>());
                }

                Ok(ptr)
            }

            #[inline(always)]
            unsafe fn release(ptr: NonNull<()>) {
                // Decrement the refcount.
                unsafe {
                    alloc::rc::Rc::decrement_strong_count(ptr.as_ptr().cast::<T>());
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
                NonNull::new_unchecked(alloc::rc::Rc::into_raw(value).cast::<()>() as *mut ()),
                &Rc::<T>::VTABLE,
            )
        }
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
        SyncDisplayHandle::from(value).into()
    }
}

impl SyncDisplayHandle {
    /// Create a new [`SyncDisplayHandle`] from a [`DisplayHandle`].
    ///
    /// # Safety
    ///
    /// In addition to the other safety requirements of [`DisplayHandle`], it is
    /// also required that the `retain` and `release` functions are thread safe.
    /// As in, the internal refcounting mechanism for the Display handle must be
    /// atomic.
    #[inline]
    pub const unsafe fn new(handle: DisplayHandle) -> Self {
        Self { inner: handle }
    }

    /// Clone this [`SyncDisplayHandle`] fallibly.
    #[inline]
    pub fn try_clone(&self) -> Result<Self, HandleError> {
        self.inner.try_clone().map(|x| unsafe { Self::new(x) })
    }

    /// Create a `SyncDisplayHandle` from a static display handle reference.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{SyncDisplayHandle, XcbDisplayHandle, BorrowedDisplayHandle, AsDisplayHandle, HandleError};
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
    /// let handle = SyncDisplayHandle::from_static(&DISPLAY);
    /// match handle.display_handle().unwrap().as_raw() {
    ///     raw_window_handle::RawDisplayHandle::Windows(_) => {},
    ///     _ => unreachable!()
    /// }
    /// ```
    #[inline]
    pub fn from_static<T: AsDisplayHandle + Sync + 'static>(display: &'static T) -> Self {
        let inner = DisplayHandle::from_static(display);

        // SAFETY: Since `T` is `Sync`, `&T` is `Send + Sync`, making it possible
        // to send this Display handle reference across threads.
        unsafe { Self::new(inner) }
    }

    /// Create a [`SyncDisplayHandle`] that always returns `HandleError::Unavailable`.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{SyncDisplayHandle, HandleError, AsDisplayHandle};
    ///
    /// let handle = SyncDisplayHandle::unavailable();
    /// assert!(matches!(handle.display_handle().unwrap_err(), HandleError::Unavailable));
    /// ```
    #[inline]
    pub fn unavailable() -> Self {
        noop_display_handle!(Err(HandleError::Unavailable))
    }

    /// Create a [`SyncDisplayHandle`] that always returns `HandleError::NotSupported`.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{SyncDisplayHandle, HandleError, AsDisplayHandle};
    ///
    /// let handle = SyncDisplayHandle::not_supported();
    /// assert!(matches!(handle.display_handle().unwrap_err(), HandleError::NotSupported));
    /// ```
    #[inline]
    pub fn not_supported() -> Self {
        noop_display_handle!(Err(HandleError::NotSupported))
    }
}

impl AsDisplayHandle for SyncDisplayHandle {
    #[inline]
    fn display_handle(&self) -> Result<BorrowedDisplayHandle<'_>, HandleError> {
        self.inner.display_handle()
    }
}

impl From<SyncDisplayHandle> for DisplayHandle {
    #[inline]
    fn from(value: SyncDisplayHandle) -> Self {
        value.inner
    }
}

#[cfg(feature = "alloc")]
impl<T: AsDisplayHandle + Send + Sync + 'static> From<alloc::sync::Arc<T>> for SyncDisplayHandle {
    /// Create a `SyncDisplayHandle` from a reference counted underlying handle.
    ///
    /// ## Example
    ///
    /// ```
    /// use raw_window_handle::{SyncDisplayHandle, BorrowedDisplayHandle, AsDisplayHandle, HandleError};
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
    /// let handle = SyncDisplayHandle::from(display);
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
            SyncDisplayHandle::new(DisplayHandle::from_raw_parts(
                NonNull::new_unchecked(alloc::sync::Arc::into_raw(value).cast::<()>() as *mut ()),
                &Arc::<T>::VTABLE,
            ))
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
