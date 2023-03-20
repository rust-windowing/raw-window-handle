//! Borrowable window handles based on the ones in this crate.
//!
//! These should be 100% safe to pass around and use, no possibility of dangling or invalidity.

use core::fmt;
use core::marker::PhantomData;

use crate::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};

/// The application is currently active.
///
/// This structure is a token that indicates that the application is not presently suspended. It is used
/// to ensure that the window handle is only used when the application is active.
///
/// It is safe to create this token on platforms where the application is guaranteed to be active, such as
/// on desktop platforms. On Android platforms, the application may be suspended, so this token must be
/// either created with `unsafe` or derived from a `HasDisplayHandle` type.
///
/// # Motivating Use Case
///
/// On Android, there is an [Activity]-global [`ANativeWindow`] object that is used for drawing. This
/// handle is used [within the `RawWindowHandle` type] for Android NDK, since it is necessary for GFX
/// APIs to draw to the screen.
///
/// However, the [`ANativeWindow`] type can be arbitrarily invalidated by the underlying Android runtime.
/// The reasoning for this is complicated, but this idea is exposed to native code through the
/// [`onNativeWindowCreated`] and [`onNativeWindowDestroyed`] callbacks. To save you a click, the
/// conditions associated with these callbacks are:
///
/// - [`onNativeWindowCreated`] provides a valid [`ANativeWindow`] pointer that can be used for drawing.
/// - [`onNativeWindowDestroyed`] indicates that the previous [`ANativeWindow`] pointer is no longer
///   valid. The documentation clarifies that, *once the function returns*, the [`ANativeWindow`] pointer
///   can no longer be used for drawing without resulting in undefined behavior.
///
/// In [`winit`], these are exposed via the [`Resumed`] and [`Suspended`] events, respectively. Therefore,
/// between the last [`Suspended`] event and the next [`Resumed`] event, it is undefined behavior to use
/// the raw window handle. This condition makes it tricky to define an API that safely wraps the raw
/// window handles, since an existing window handle can be made invalid at any time.
///
/// The `Active` struct works around this by providing a borrow-checker enforced guarantee that the
/// application is not suspended. This takes advantage of the fact that he current callback-based event
/// loop setup for [`winit`], [`sdl2`] and other event loop systems to my knowledge requires an
/// `&mut self` reference to the event loop. This means that it is not possible to borrow the event loop
/// and poll for events at the same.
///
/// Therefore, the `Active` token acts as a guarantee that "for the current event loop iteration, the
/// event loop is not suspended". Since we know that the event loop isn't suspended, we can successfully
/// borrow the display/window handle and use it for drawing.
///
/// Note that, while the window handle on non-Android platforms can be invalidated, they can't be
/// invalidated in a way that causes undefined behavior. Therefore, it is safe to create an `Active` token
/// in all contexts using the [`Active::new`] method no matter what.
///
/// Another possible way of enforcing these guarantees would be to have an `is_valid` method on
/// [`HasWindowHandle`] to test if its window is currently valid. However, this would require the user
/// to check if the window is valid every time they want to use it, which may create an unnecessary
/// performance hit. The `Active` token is a more ergonomic solution to this problem.
///
/// [Activity]: https://developer.android.com/reference/android/app/Activity
/// [`ANativeWindow`]: https://developer.android.com/ndk/reference/group/a-native-window
/// [within the `RawWindowHandle` type]: struct.AndroidNdkWindowHandle.html#structfield.a_native_window
/// [`onNativeWindowCreated`]: https://developer.android.com/ndk/reference/struct/a-native-activity-callbacks#onnativewindowcreated
/// [`onNativeWindowDestroyed`]: https://developer.android.com/ndk/reference/struct/a-native-activity-callbacks#onnativewindowdestroyed
/// [`winit`]: https://crates.io/crates/winit
/// [`Resumed`]: https://docs.rs/winit/latest/winit/event/enum.Event.html#variant.Resumed
/// [`Suspended`]: https://docs.rs/winit/latest/winit/event/enum.Event.html#variant.Suspended
/// [`sdl2`]: https://crates.io/crates/sdl2
/// [`Active::new`]: struct.Active.html#method.new
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
    /// On Android platforms, the native window handle must be available. It is unsound to call this
    /// function between the [`onNativeWindowDestroyed`] and [`onNativeWindowCreated`] callbacks.
    /// Otherwise, a dangling reference to the [`ANativeWindow`] will be created.
    ///
    /// On other platforms, this function is safe to call. The safe [`Active::new`] function should be
    /// used instead.
    ///
    /// [`onNativeWindowCreated`]: https://developer.android.com/ndk/reference/struct/a-native-activity-callbacks#onnativewindowcreated
    /// [`onNativeWindowDestroyed`]: https://developer.android.com/ndk/reference/struct/a-native-activity-callbacks#onnativewindowdestroyed
    /// [`ANativeWindow`]: https://developer.android.com/ndk/reference/group/a-native-window
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

    /// Create a new active token on a system where the application is guaranteed to be active.
    ///
    /// On most platforms, there is no event where the application is suspended, so there are no cases
    /// where this function is unsafe.
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
/// The safety requirements of [`HasRawDisplayHandle`] apply here as  well. To reiterate, the
/// [`DisplayHandle`] must contain a valid window handle for its lifetime.
///
/// In addition, the `active` function must only return an [`Active`] token if the application is active.
/// This also implies that the [`Active`] token must be invalidated beween events. See the documentation
/// on the [`Active`] type for more information about these safety requirements.
///
/// Note that these requirements are not enforced on `HasDisplayHandle`, rather, they are enforced on the
/// constructors of [`Active`] and [`DisplayHandle`]. This is because the `HasDisplayHandle` trait is
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
    /// The `RawDisplayHandle` must be valid for the lifetime and the application must be [`Active`]. See
    /// the requirements on the [`HasDisplayHandle`] trait and [`Active`] type for more information.
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
        // SAFETY: The fact that this handle was created means that the application is active.
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
/// In addition, the window handle must not be invalidated for the duration of the [`Active`] token.
///
/// Note that these requirements are not enforced on `HasWindowHandle`, rather, they are enforced on the
/// constructors of [`WindowHandle`]. This is because the `HasWindowHandle` trait is safe to implement.
pub trait HasWindowHandle {
    /// Get a handle to the window.
    fn window_handle<'this, 'active>(
        &'this self,
        active: &'active Active<'_>,
    ) -> WindowHandle<'this>
    where
        'active: 'this;
}

/// The handle to a window.
///
/// This is the primary return type of the [`HasWindowHandle`] trait. All *pointers* within this type
/// are guaranteed to be valid and not dangling for the lifetime of the handle. This excludes window IDs
/// like XIDs, `HWND`s, and the window ID for web platforms. See the documentation on the
/// [`HasWindowHandle`] trait for more information about these safety requirements.
///
/// This handle is guaranteed to be safe and valid. Get the underlying raw window handle with the
/// [`HasRawWindowHandle`] trait.
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
    /// Borrow a `WindowHandle` from a [`RawWindowHandle`].
    ///
    /// # Safety
    ///
    /// The [`RawWindowHandle`] must be valid for the lifetime and the application must be `Active`.
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
