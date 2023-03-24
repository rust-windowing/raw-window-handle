//! Borrowable window handles based on the ones in this crate.
//!
//! These should be 100% safe to pass around and use, no possibility of dangling or invalidity.

#[cfg(all(not(feature = "std"), target_os = "android"))]
compile_error!("Using borrowed handles on Android requires the `std` feature to be enabled.");

use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

use crate::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};

/// Keeps track of whether the application is currently active.
pub struct Active(imp::Active);

impl fmt::Debug for Active {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Active { .. }")
    }
}

/// Represents a live window handle.
#[derive(Clone)]
pub struct ActiveHandle<'a>(imp::ActiveHandle<'a>);

impl<'a> fmt::Debug for ActiveHandle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ActiveHandle { .. }")
    }
}

impl Active {
    /// Create a new `Active` tracker.
    pub const fn new() -> Self {
        Self(imp::Active::new())
    }

    /// Get a live window handle.
    pub fn handle(&self) -> Option<ActiveHandle<'_>> {
        self.0.handle().map(ActiveHandle)
    }

    /// Set the application to be inactive.
    ///
    /// This function may block until there are no more active handles.
    ///
    /// # Safety
    ///
    /// The application must actually be inactive.
    pub unsafe fn set_inactive(&self) {
        self.0.set_inactive()
    }

    /// Set the application to be active.
    ///
    /// # Safety
    ///
    /// The application must actually be active.
    pub unsafe fn set_active(&self) {
        self.0.set_active()
    }
}

impl ActiveHandle<'_> {
    /// Create a new freestanding active handle.
    ///
    /// # Safety
    ///
    /// The application must actually be active.
    pub unsafe fn new_unchecked() -> Self {
        Self(imp::ActiveHandle::new_unchecked())
    }
}

/// A display that acts as a wrapper around a display handle.
///
/// # Safety
///
/// The safety requirements of [`HasRawDisplayHandle`] apply here as  well. To reiterate, the
/// [`DisplayHandle`] must contain a valid window handle for its lifetime.
///
/// It is not possible to invalidate a [`DisplayHandle`] on any platform without additional unsafe code.
///
/// Note that these requirements are not enforced on `HasDisplayHandle`, rather, they are enforced on the
/// constructors of [`DisplayHandle`]. This is because the `HasDisplayHandle` trait is safe to implement.
///
/// [`HasRawDisplayHandle`]: crate::HasRawDisplayHandle
pub trait HasDisplayHandle {
    /// Get a handle to the display controller of the windowing system.
    fn display_handle(&self) -> DisplayHandle<'_>;
}

impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for &H {
    fn display_handle(&self) -> DisplayHandle<'_> {
        (**self).display_handle()
    }
}

#[cfg(feature = "alloc")]
impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::boxed::Box<H> {
    fn display_handle(&self) -> DisplayHandle<'_> {
        (**self).display_handle()
    }
}

#[cfg(feature = "alloc")]
impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::rc::Rc<H> {
    fn display_handle(&self) -> DisplayHandle<'_> {
        (**self).display_handle()
    }
}

#[cfg(feature = "alloc")]
impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::sync::Arc<H> {
    fn display_handle(&self) -> DisplayHandle<'_> {
        (**self).display_handle()
    }
}

/// The handle to the display controller of the windowing system.
///
/// This is the primary return type of the [`HasDisplayHandle`] trait. It is guaranteed to contain
/// a valid platform-specific display handle for its lifetime.
///
/// Get the underlying raw display handle with the [`HasRawDisplayHandle`] trait.
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
    /// Borrow a `DisplayHandle` from a [`RawDisplayHandle`].
    ///
    /// # Safety
    ///
    /// The `RawDisplayHandle` must be valid for the lifetime.
    pub unsafe fn borrow_raw(raw: RawDisplayHandle) -> Self {
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
    fn display_handle(&self) -> DisplayHandle<'_> {
        *self
    }
}

/// A handle to a window.
///
/// # Safety
///
/// All pointers within the resulting [`WindowHandle`] must be valid and not dangling for the lifetime of
/// the handle.
///
/// Note that this guarantee only applies to *pointers*, and not any window ID types in the handle.
/// This includes Window IDs (XIDs) from X11, `HWND`s from Win32, and the window ID for web platforms.
/// There is no way for Rust to enforce any kind of invariant on these types, since:
///
/// - For all three listed platforms, it is possible for safe code in the same process to delete
///   the window.
/// - For X11 and Win32, it is possible for code in a different process to delete the window.
/// - For X11, it is possible for code on a different *machine* to delete the window.
///
/// It is *also* possible for the window to be replaced with another, valid-but-different window. User
/// code should be aware of this possibility, and should be ready to soundly handle the possible error
/// conditions that can arise from this.
///
/// In addition, the window handle must not be invalidated for the duration of the [`ActiveHandle`] token.
///
/// Note that these requirements are not enforced on `HasWindowHandle`, rather, they are enforced on the
/// constructors of [`WindowHandle`]. This is because the `HasWindowHandle` trait is safe to implement.
pub trait HasWindowHandle {
    /// Get a handle to the window.
    fn window_handle(&self) -> Result<WindowHandle<'_>, WindowHandleError>;
}

impl<H: HasWindowHandle + ?Sized> HasWindowHandle for &H {
    fn window_handle(&self) -> Result<WindowHandle<'_>, WindowHandleError> {
        (**self).window_handle()
    }
}

#[cfg(feature = "alloc")]
impl<H: HasWindowHandle + ?Sized> HasWindowHandle for alloc::boxed::Box<H> {
    fn window_handle(&self) -> Result<WindowHandle<'_>, WindowHandleError> {
        (**self).window_handle()
    }
}

#[cfg(feature = "alloc")]
impl<H: HasWindowHandle + ?Sized> HasWindowHandle for alloc::rc::Rc<H> {
    fn window_handle(&self) -> Result<WindowHandle<'_>, WindowHandleError> {
        (**self).window_handle()
    }
}

#[cfg(feature = "alloc")]
impl<H: HasWindowHandle + ?Sized> HasWindowHandle for alloc::sync::Arc<H> {
    fn window_handle(&self) -> Result<WindowHandle<'_>, WindowHandleError> {
        (**self).window_handle()
    }
}

/// The error type returned when a window handle cannot be obtained.
#[derive(Debug)]
#[non_exhaustive]
pub enum WindowHandleError {
    /// The window is not currently active.
    Inactive,
}

impl fmt::Display for WindowHandleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inactive => write!(f, "the window is not currently active"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WindowHandleError {}

/// The handle to a window.
///
/// This is the primary return type of the [`HasWindowHandle`] trait. All *pointers* within this type
/// are guaranteed to be valid and not dangling for the lifetime of the handle. This excludes window IDs
/// like XIDs, `HWND`s, and the window ID for web platforms. See the documentation on the
/// [`HasWindowHandle`] trait for more information about these safety requirements.
///
/// This handle is guaranteed to be safe and valid. Get the underlying raw window handle with the
/// [`HasRawWindowHandle`] trait.
#[derive(Clone)]
pub struct WindowHandle<'a> {
    raw: RawWindowHandle,
    _active: ActiveHandle<'a>,
    _marker: PhantomData<&'a *const ()>,
}

impl fmt::Debug for WindowHandle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("WindowHandle").field(&self.raw).finish()
    }
}

impl PartialEq for WindowHandle<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl Eq for WindowHandle<'_> {}

impl Hash for WindowHandle<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<'a> WindowHandle<'a> {
    /// Borrow a `WindowHandle` from a [`RawWindowHandle`].
    ///
    /// # Safety
    ///
    /// The [`RawWindowHandle`] must be valid for the lifetime and the application must be `Active`.
    pub unsafe fn borrow_raw(raw: RawWindowHandle, active: ActiveHandle<'a>) -> Self {
        Self {
            raw,
            _active: active,
            _marker: PhantomData,
        }
    }
}

unsafe impl HasRawWindowHandle for WindowHandle<'_> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.raw
    }
}

impl HasWindowHandle for WindowHandle<'_> {
    fn window_handle(&self) -> Result<Self, WindowHandleError> {
        Ok(self.clone())
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

#[cfg(not(any(target_os = "android", raw_window_handle_force_refcount)))]
#[cfg_attr(docsrs, doc(cfg(not(target_os = "android"))))]
mod imp {
    //! We don't need to refcount the handles, so we can just use no-ops.

    use core::marker::PhantomData;

    pub(super) struct Active;

    #[derive(Clone)]
    pub(super) struct ActiveHandle<'a> {
        _marker: PhantomData<&'a ()>,
    }

    impl Active {
        pub(super) const fn new() -> Self {
            Self
        }

        pub(super) fn handle(&self) -> Option<ActiveHandle<'_>> {
            // SAFETY: The handle is always active.
            Some(unsafe { ActiveHandle::new_unchecked() })
        }

        pub(super) unsafe fn set_active(&self) {}

        pub(super) unsafe fn set_inactive(&self) {}
    }

    impl ActiveHandle<'_> {
        pub(super) unsafe fn new_unchecked() -> Self {
            Self {
                _marker: PhantomData,
            }
        }
    }

    impl super::ActiveHandle<'_> {
        /// Create a new `ActiveHandle`.
        ///
        /// This is safe because the handle is always active.
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            // SAFETY: The handle is always active.
            unsafe { super::ActiveHandle::new_unchecked() }
        }
    }
}

#[cfg(any(target_os = "android", raw_window_handle_force_refcount))]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "android"))))]
mod imp {
    //! We need to refcount the handles, so we use an `RwLock` to do so.

    use std::sync::{RwLock, RwLockReadGuard};

    pub(super) struct Active {
        active: RwLock<bool>,
    }

    pub(super) struct ActiveHandle<'a> {
        inner: Option<Inner<'a>>,
    }

    struct Inner<'a> {
        _read_guard: RwLockReadGuard<'a, bool>,
        active: &'a Active,
    }

    impl Clone for ActiveHandle<'_> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.as_ref().map(|inner| Inner {
                    _read_guard: inner.active.active.read().unwrap(),
                    active: inner.active,
                }),
            }
        }
    }

    impl Active {
        pub(super) const fn new() -> Self {
            Self {
                active: RwLock::new(false),
            }
        }

        pub(super) fn handle(&self) -> Option<ActiveHandle<'_>> {
            let active = self.active.read().ok()?;
            if !*active {
                return None;
            }

            Some(ActiveHandle {
                inner: Some(Inner {
                    _read_guard: active,
                    active: self,
                }),
            })
        }

        pub(super) unsafe fn set_active(&self) {
            *self.active.write().unwrap() = true;
        }

        pub(super) unsafe fn set_inactive(&self) {
            *self.active.write().unwrap() = false;
        }
    }

    impl ActiveHandle<'_> {
        pub(super) unsafe fn new_unchecked() -> Self {
            Self { inner: None }
        }
    }
}
