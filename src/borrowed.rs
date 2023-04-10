//! Borrowable window handles based on the ones in this crate.
//!
//! These should be 100% safe to pass around and use, no possibility of dangling or invalidity.

use core::cell::UnsafeCell;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

use crate::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};

/// Keeps track of whether the application is currently active.
///
/// On Android, it was previously believed that the application could enter the suspended state
/// and immediately invalidate all window handles. However, it was later discovered that the
/// handle actually remains valid, but the window does not produce any more GPU buffers. This
/// type is a no-op and will be removed at the next major release.
#[deprecated = "Will be removed at next major release, use ActiveHandle::new() for now"]
pub struct Active(());

#[allow(deprecated)]
impl fmt::Debug for Active {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Active { .. }")
    }
}

/// Represents a live window handle.
///
/// On Android, it was previously believed that the application could enter the suspended state
/// and immediately invalidate all window handles. However, it was later discovered that the
/// handle actually remains valid, but the window does not produce any more GPU buffers. This
/// type is a no-op and will be removed at the next major release.
#[derive(Clone)]
pub struct ActiveHandle<'a>(PhantomData<&'a UnsafeCell<()>>);

impl<'a> fmt::Debug for ActiveHandle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ActiveHandle { .. }")
    }
}

#[allow(deprecated)]
impl Active {
    /// Create a new `Active` tracker.
    ///
    /// Only one of these should exist per display connection.
    ///
    /// # Example
    ///
    /// ```
    /// use raw_window_handle::Active;
    /// let active = Active::new();
    /// ```
    pub const fn new() -> Self {
        Self(())
    }

    /// Get a live window handle.
    ///
    /// This function returns an active handle if the application is active, and `None` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use raw_window_handle::Active;
    ///
    /// // Set the application to be active.
    /// let active = Active::new();
    /// unsafe { active.set_active() };
    ///
    /// // Get a live window handle.
    /// let handle = active.handle();
    ///
    /// // Drop it and set the application to be inactive.
    /// drop(handle);
    /// active.set_inactive();
    /// ```
    pub fn handle(&self) -> Option<ActiveHandle<'_>> {
        Some(ActiveHandle(PhantomData))
    }

    /// Set the application to be inactive.
    ///
    /// This function may block until there are no more active handles.
    ///
    /// # Example
    ///
    /// ```
    /// use raw_window_handle::Active;
    ///
    /// // Set the application to be active.
    /// let active = Active::new();
    /// unsafe { active.set_active() };
    ///
    /// // Set the application to be inactive.
    /// active.set_inactive();
    /// ```
    pub fn set_inactive(&self) {}

    /// Set the application to be active.
    ///
    /// # Safety
    ///
    /// The application must actually be active. Setting to active when the application is not active
    /// will result in undefined behavior.
    ///
    /// # Example
    ///
    /// ```
    /// use raw_window_handle::Active;
    ///
    /// // Set the application to be active.
    /// let active = Active::new();
    /// unsafe { active.set_active() };
    ///
    /// // Set the application to be inactive.
    /// active.set_inactive();
    /// ```
    pub unsafe fn set_active(&self) {}
}

impl ActiveHandle<'_> {
    /// Create a new freestanding active handle.
    ///
    /// This function acts as an "escape hatch" to allow the user to create a live window handle
    /// without having to go through the [`Active`] type. This is useful if the user *knows* that the
    /// application is active, and wants to create a live window handle without having to go through
    /// the [`Active`] type.
    ///
    /// # Safety
    ///
    /// The application must actually be active.
    ///
    /// # Example
    ///
    /// ```
    /// use raw_window_handle::ActiveHandle;
    ///
    /// // Create a freestanding active handle.
    /// // SAFETY: The application must actually be active.
    /// let handle = unsafe { ActiveHandle::new_unchecked() };
    /// ```
    #[deprecated = "Will be removed at next major release, use ActiveHandle::new() for now"]
    pub unsafe fn new_unchecked() -> Self {
        Self(PhantomData)
    }

    /// Create a new `ActiveHandle`.
    ///
    /// This is safe because the handle is always active.
    ///
    /// # Example
    ///
    /// ```
    /// use raw_window_handle::ActiveHandle;
    /// let handle = ActiveHandle::new();
    /// ```
    #[allow(clippy::new_without_default, deprecated)]
    pub fn new() -> Self {
        // SAFETY: The handle is always active.
        unsafe { super::ActiveHandle::new_unchecked() }
    }
}

/// A display that acts as a wrapper around a display handle.
///
/// Objects that implement this trait should be able to return a [`DisplayHandle`] for the display
/// that they are associated with. This handle should last for the lifetime of the object, and should
/// return an error if the application is inactive.
///
/// Implementors of this trait will be windowing systems, like [`winit`] and [`sdl2`]. These windowing
/// systems should implement this trait on types that already implement [`HasRawDisplayHandle`]. It
/// should be implemented by tying the lifetime of the [`DisplayHandle`] to the lifetime of the
/// display object.
///
/// Users of this trait will include graphics libraries, like [`wgpu`] and [`glutin`]. These APIs
/// should be generic over a type that implements `HasDisplayHandle`, and should use the
/// [`DisplayHandle`] type to access the display handle.
///
/// # Safety
///
/// The safety requirements of [`HasRawDisplayHandle`] apply here as well. To reiterate, the
/// [`DisplayHandle`] must contain a valid window handle for its lifetime.
///
/// It is not possible to invalidate a [`DisplayHandle`] on any platform without additional unsafe code.
///
/// Note that these requirements are not enforced on `HasDisplayHandle`, rather, they are enforced on the
/// constructors of [`DisplayHandle`]. This is because the `HasDisplayHandle` trait is safe to implement.
///
/// [`HasRawDisplayHandle`]: crate::HasRawDisplayHandle
/// [`winit`]: https://crates.io/crates/winit
/// [`sdl2`]: https://crates.io/crates/sdl2
/// [`wgpu`]: https://crates.io/crates/wgpu
/// [`glutin`]: https://crates.io/crates/glutin
pub trait HasDisplayHandle {
    /// Get a handle to the display controller of the windowing system.
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError>;
}

impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for &H {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        (**self).display_handle()
    }
}

#[cfg(feature = "alloc")]
impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::boxed::Box<H> {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        (**self).display_handle()
    }
}

#[cfg(feature = "alloc")]
impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::rc::Rc<H> {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        (**self).display_handle()
    }
}

#[cfg(feature = "alloc")]
impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::sync::Arc<H> {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
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

impl<'a> Clone for DisplayHandle<'a> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw,
            _marker: PhantomData,
        }
    }
}

impl<'a> DisplayHandle<'a> {
    /// Create a `DisplayHandle` from a [`RawDisplayHandle`].
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
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(self.clone())
    }
}

/// A handle to a window.
///
/// Objects that implement this trait should be able to return a [`WindowHandle`] for the window
/// that they are associated with. This handle should last for the lifetime of the object, and should
/// return an error if the application is inactive.
///
/// Implementors of this trait will be windowing systems, like [`winit`] and [`sdl2`]. These windowing
/// systems should implement this trait on types that already implement [`HasRawWindowHandle`]. First,
/// it should be made sure that the display type contains a unique [`Active`] ref-counted handle.
/// To create a [`WindowHandle`], the [`Active`] should be used to create an [`ActiveHandle`] that is
/// then used to create a [`WindowHandle`]. Finally, the raw window handle should be retrieved from
/// the type and used to create a [`WindowHandle`].
///
/// Users of this trait will include graphics libraries, like [`wgpu`] and [`glutin`]. These APIs
/// should be generic over a type that implements `HasWindowHandle`, and should use the
/// [`WindowHandle`] type to access the window handle. The window handle should be acquired and held
/// while the window is being used, in order to ensure that the window is not deleted while it is in
/// use.
///
/// # Safety
///
/// All pointers within the resulting [`WindowHandle`] must be valid and not dangling for the lifetime of
/// the handle.
///
/// Note that this guarantee only applies to *pointers*, and not any window ID types in the handle.
/// This includes Window IDs (XIDs) from X11 and the window ID for web platforms. There is no way for
/// Rust to enforce any kind of invariant on these types, since:
///
/// - For all three listed platforms, it is possible for safe code in the same process to delete
///   the window.
/// - For X11, it is possible for code in a different process to delete the window. In fact, it is
///   possible for code on a different *machine* to delete the window.
///
/// It is *also* possible for the window to be replaced with another, valid-but-different window. User
/// code should be aware of this possibility, and should be ready to soundly handle the possible error
/// conditions that can arise from this.
///
/// In addition, the window handle must not be invalidated for the duration of the [`ActiveHandle`] token.
///
/// Note that these requirements are not enforced on `HasWindowHandle`, rather, they are enforced on the
/// constructors of [`WindowHandle`]. This is because the `HasWindowHandle` trait is safe to implement.
///
/// [`winit`]: https://crates.io/crates/winit
/// [`sdl2`]: https://crates.io/crates/sdl2
/// [`wgpu`]: https://crates.io/crates/wgpu
/// [`glutin`]: https://crates.io/crates/glutin
pub trait HasWindowHandle {
    /// Get a handle to the window.
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError>;
}

impl<H: HasWindowHandle + ?Sized> HasWindowHandle for &H {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        (**self).window_handle()
    }
}

#[cfg(feature = "alloc")]
impl<H: HasWindowHandle + ?Sized> HasWindowHandle for alloc::boxed::Box<H> {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        (**self).window_handle()
    }
}

#[cfg(feature = "alloc")]
impl<H: HasWindowHandle + ?Sized> HasWindowHandle for alloc::rc::Rc<H> {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        (**self).window_handle()
    }
}

#[cfg(feature = "alloc")]
impl<H: HasWindowHandle + ?Sized> HasWindowHandle for alloc::sync::Arc<H> {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        (**self).window_handle()
    }
}

/// The handle to a window.
///
/// This is the primary return type of the [`HasWindowHandle`] trait. All *pointers* within this type
/// are guaranteed to be valid and not dangling for the lifetime of the handle. This excludes window IDs
/// like XIDs and the window ID for web platforms. See the documentation on the [`HasWindowHandle`]
/// trait for more information about these safety requirements.
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
    /// The [`RawWindowHandle`] must be valid for the lifetime and the application must not be
    /// suspended. The [`Active`] object that the [`ActiveHandle`] was created from must be
    /// associated directly with the display that the window handle is associated with.
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
    fn window_handle(&self) -> Result<Self, HandleError> {
        Ok(self.clone())
    }
}

/// The error type returned when a handle cannot be obtained.
#[derive(Debug)]
#[non_exhaustive]
pub enum HandleError {
    /// The handle is not currently active.
    ///
    /// See documentation on [`Active`] for more information.
    Inactive,
}

impl fmt::Display for HandleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inactive => write!(f, "the handle is not currently active"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HandleError {}

/// ```compile_fail
/// use raw_window_handle::{Active, DisplayHandle, WindowHandle};
/// fn _assert<T: Send + Sync>() {}
/// _assert::<Active<'static>>();
/// _assert::<DisplayHandle<'static>>();
/// _assert::<WindowHandle<'static>>();
/// ```
fn _not_send_or_sync() {}
