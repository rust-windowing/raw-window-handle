//! Borrowable window handles based on the ones in this crate.
//!
//! These should be 100% safe to pass around and use, no possibility of dangling or
//! invalidity.

use core::fmt;
use core::marker::PhantomData;

use crate::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};

/// The application is currently active.
///
/// This structure is a token that indicates that the application is
/// not presently suspended. It is used to ensure that the window handle
/// is only used when the application is active.
///
/// It is safe to create this token on platforms where the application
/// is guaranteed to be active, such as on desktop platforms. On Android
/// platforms, the application may be suspended, so this token must be
/// either created with `unsafe` or derived from a `HasDisplayHandle`
/// type.
pub struct Active<'a> {
    _marker: PhantomData<&'a *const ()>,
}

impl fmt::Debug for Active<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Active { .. }")
    }
}

impl<'a> Active<'a> {
    /// Create a new active token.
    ///
    /// # Safety
    ///
    /// The application must not be `Suspend`ed.
    ///
    /// # Examples
    ///
    /// ```
    /// use raw_window_handle::Active;
    ///
    /// // SAFETY: The application is active.
    /// let active = unsafe { Active::new_unchecked() };
    /// ```
    pub unsafe fn new_unchecked() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Create a new active token on a system where the application is
    /// guaranteed to be active.
    ///
    /// On most platforms, there is no event where the application is
    /// suspended, so there are no cases where this function is unsafe.
    ///
    /// ```
    /// use raw_window_handle::Active;
    ///
    /// let with_active = |active: Active<'_>| {
    ///     /* ... */
    /// };
    ///
    /// // Only use this code-path on non-android platforms.
    /// #[cfg(not(target_os = "android"))]
    /// {
    ///     let active = Active::new();
    ///     with_active(active);
    /// }
    ///
    /// // Only use this code-path on android platforms.
    /// #[cfg(target_os = "android")]
    /// {
    ///     if application_is_active() {
    ///         let active = unsafe { Active::new_unchecked() };
    ///         with_active(active);
    ///     }
    /// }
    /// # fn application_is_active() -> bool { false }
    /// ```       
    #[cfg(not(target_os = "android"))]
    #[cfg_attr(docsrs, doc(cfg(not(target_os = "android"))))]
    pub fn new() -> Self {
        // SAFETY: The application is guaranteed to be active.
        unsafe { Self::new_unchecked() }
    }
}

/// A display that acts as a wrapper around a display handle.
///
/// # Safety
///
/// The safety requirements of [`HasRawDisplayHandle`] apply here as
/// well. To clarify, the [`DisplayHandle`] must contain a valid window
/// handle for its lifetime. In addition, the handle must be consistent
/// between multiple calls barring platform-specific events.
///
/// In addition, the `active` function must only return an `Active`
/// token if the application is active.
///
/// Note that these requirements are not enforced on `HasDisplayHandle`,
/// rather, they are enforced on the constructors of [`Active`] and
/// [`DisplayHandle`]. This is because the `HasDisplayHandle` trait is
/// safe to implement.
///
/// [`HasRawDisplayHandle`]: crate::HasRawDisplayHandle
pub trait HasDisplayHandle {
    /// Get a token indicating whether the application is active.
    fn active(&self) -> Option<Active<'_>>;

    /// Get a handle to the display controller of the windowing system.
    fn display_handle<'this, 'active>(
        &'this self,
        active: &'active Active<'_>,
    ) -> DisplayHandle<'this>
    where
        'active: 'this;
}

impl<T: HasDisplayHandle + ?Sized> HasDisplayHandle for &T {
    fn active(&self) -> Option<Active<'_>> {
        (**self).active()
    }

    fn display_handle<'this, 'active>(
        &'this self,
        active: &'active Active<'_>,
    ) -> DisplayHandle<'this>
    where
        'active: 'this,
    {
        (**self).display_handle(active)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::boxed::Box<T> {
    fn active(&self) -> Option<Active<'_>> {
        (**self).active()
    }

    fn display_handle<'this, 'active>(
        &'this self,
        active: &'active Active<'_>,
    ) -> DisplayHandle<'this>
    where
        'active: 'this,
    {
        (**self).display_handle(active)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::rc::Rc<T> {
    fn active(&self) -> Option<Active<'_>> {
        (**self).active()
    }

    fn display_handle<'this, 'active>(
        &'this self,
        active: &'active Active<'_>,
    ) -> DisplayHandle<'this>
    where
        'active: 'this,
    {
        (**self).display_handle(active)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::sync::Arc<T> {
    fn active(&self) -> Option<Active<'_>> {
        (**self).active()
    }

    fn display_handle<'this, 'active>(
        &'this self,
        active: &'active Active<'_>,
    ) -> DisplayHandle<'this>
    where
        'active: 'this,
    {
        (**self).display_handle(active)
    }
}

/// The handle to the display controller of the windowing system.
///
/// Get the underlying raw display handle with the `HasRawDisplayHandle` trait.
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash)]
pub struct DisplayHandle<'a> {
    raw: RawDisplayHandle,
    _marker: PhantomData<&'a *const ()>,
}

impl fmt::Debug for DisplayHandle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DisplayHandle").field(&self.raw).finish()
    }
}

impl<'a> Copy for DisplayHandle<'a> {}
impl<'a> Clone for DisplayHandle<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> DisplayHandle<'a> {
    /// Borrow a `DisplayHandle` from a `RawDisplayHandle`.
    ///
    /// # Safety
    ///
    /// The `RawDisplayHandle` must be valid for the lifetime and the
    /// application must be `Active`. See the requirements on the
    /// [`HasDisplayHandle`] trait for more information.
    pub unsafe fn borrow_raw(raw: RawDisplayHandle, active: &Active<'a>) -> Self {
        let _ = active;
        Self {
            raw,
            _marker: PhantomData,
        }
    }
}

unsafe impl HasRawDisplayHandle for DisplayHandle<'_> {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        self.raw
    }
}

impl<'a> HasDisplayHandle for DisplayHandle<'a> {
    fn active(&self) -> Option<Active<'_>> {
        // SAFETY: The fact that this handle was created means that the
        // application is active.
        Some(unsafe { Active::new_unchecked() })
    }

    fn display_handle<'this, 'active>(
        &'this self,
        _active: &'active Active<'_>,
    ) -> DisplayHandle<'this>
    where
        'active: 'this,
    {
        *self
    }
}

/// A handle to a window.
///
/// # Safety
///
/// The safety requirements of [`HasRawWindowHandle`] apply here as
/// well. To clarify, the [`WindowHandle`] must contain a valid window
/// handle for its lifetime. In addition, the handle must be consistent
/// between multiple calls barring platform-specific events.
///
/// Note that these requirements are not enforced on `HasWindowHandle`,
/// rather, they are enforced on the constructors of
/// [`WindowHandle`]. This is because the `HasWindowHandle` trait is
/// safe to implement.
pub trait HasWindowHandle {
    /// Get a handle to the window.
    fn window_handle<'this, 'active>(
        &'this self,
        active: &'active Active<'_>,
    ) -> WindowHandle<'this>
    where
        'active: 'this;
}

impl<T: HasWindowHandle + ?Sized> HasWindowHandle for &T {
    fn window_handle<'this, 'active>(
        &'this self,
        active: &'active Active<'_>,
    ) -> WindowHandle<'this>
    where
        'active: 'this,
    {
        (**self).window_handle(active)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: HasWindowHandle + ?Sized> HasWindowHandle for alloc::boxed::Box<T> {
    fn window_handle<'this, 'active>(
        &'this self,
        active: &'active Active<'_>,
    ) -> WindowHandle<'this>
    where
        'active: 'this,
    {
        (**self).window_handle(active)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: HasWindowHandle + ?Sized> HasWindowHandle for alloc::rc::Rc<T> {
    fn window_handle<'this, 'active>(
        &'this self,
        active: &'active Active<'_>,
    ) -> WindowHandle<'this>
    where
        'active: 'this,
    {
        (**self).window_handle(active)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<T: HasWindowHandle + ?Sized> HasWindowHandle for alloc::sync::Arc<T> {
    fn window_handle<'this, 'active>(
        &'this self,
        active: &'active Active<'_>,
    ) -> WindowHandle<'this>
    where
        'active: 'this,
    {
        (**self).window_handle(active)
    }
}

/// The handle to a window.
///
/// This handle is guaranteed to be safe and valid. Get the underlying
/// raw window handle with the `HasRawWindowHandle` trait.
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash)]
pub struct WindowHandle<'a> {
    raw: RawWindowHandle,
    _marker: PhantomData<&'a *const ()>,
}

impl fmt::Debug for WindowHandle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("WindowHandle").field(&self.raw).finish()
    }
}

impl<'a> Copy for WindowHandle<'a> {}
impl<'a> Clone for WindowHandle<'a> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a> WindowHandle<'a> {
    /// Borrow a `WindowHandle` from a `RawWindowHandle`.
    ///
    /// # Safety
    ///
    /// The `RawWindowHandle` must be valid for the lifetime and the
    /// application must be `Active`.
    pub unsafe fn borrow_raw(raw: RawWindowHandle, active: &Active<'a>) -> Self {
        let _ = active;
        Self {
            raw,
            _marker: PhantomData,
        }
    }
}

unsafe impl HasRawWindowHandle for WindowHandle<'_> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.raw
    }
}

impl<'a> HasWindowHandle for WindowHandle<'a> {
    fn window_handle<'this, 'active>(
        &'this self,
        _active: &'active Active<'_>,
    ) -> WindowHandle<'this>
    where
        'active: 'this,
    {
        *self
    }
}

/// ```compile_fail
/// use raw_window_handle::{Active, DisplayHandle, WindowHandle};
/// fn _assert<T: Send + Sync>() {}
/// _assert::<Active<'static>>();
/// _assert::<DisplayHandle<'static>>();
/// _assert::<WindowHandle<'static>>();
/// ```
fn _not_send_or_sync() {}
