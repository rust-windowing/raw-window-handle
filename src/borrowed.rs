//! Traits.

use crate::{DisplayHandle, HandleError, WindowHandle};

/// A display that acts as a wrapper around a display handle.
///
/// Objects that implement this trait should be able to return a [`DisplayHandle`] for the display
/// that they are associated with. This handle should last for the lifetime of the object, and should
/// return an error if the application is inactive.
///
/// Implementors of this trait will be windowing systems, like [`winit`] and [`sdl2`]. These windowing
/// systems should implement this trait on types that represent the top-level display server. It
/// should be implemented by tying the lifetime of the [`DisplayHandle`] to the lifetime of the
/// display object.
///
/// Users of this trait will include graphics libraries, like [`wgpu`] and [`glutin`]. These APIs
/// should be generic over a type that implements `HasDisplayHandle`, and should use the
/// [`DisplayHandle`] type to access the display handle.
///
/// Note that these requirements are not enforced on `HasDisplayHandle`, rather, they are enforced on the
/// constructors of [`DisplayHandle`]. This is because the `HasDisplayHandle` trait is safe to implement.
///
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

impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for &mut H {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        (**self).display_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::boxed::Box<H> {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        (**self).display_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::rc::Rc<H> {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        (**self).display_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<H: HasDisplayHandle + ?Sized> HasDisplayHandle for alloc::sync::Arc<H> {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        (**self).display_handle()
    }
}

impl<'a> HasDisplayHandle for DisplayHandle<'a> {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Ok(*self)
    }
}

/// A handle to a window.
///
/// Objects that implement this trait should be able to return a [`WindowHandle`] for the window
/// that they are associated with. This handle should last for the lifetime of the object, and should
/// return an error if the application is inactive.
///
/// Implementors of this trait will be windowing systems, like [`winit`] and [`sdl2`]. These windowing
/// systems should implement this trait on types that represent windows.
///
/// Users of this trait will include graphics libraries, like [`wgpu`] and [`glutin`]. These APIs
/// should be generic over a type that implements `HasWindowHandle`, and should use the
/// [`WindowHandle`] type to access the window handle. The window handle should be acquired and held
/// while the window is being used, in order to ensure that the window is not deleted while it is in
/// use.
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

impl<H: HasWindowHandle + ?Sized> HasWindowHandle for &mut H {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        (**self).window_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<H: HasWindowHandle + ?Sized> HasWindowHandle for alloc::boxed::Box<H> {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        (**self).window_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<H: HasWindowHandle + ?Sized> HasWindowHandle for alloc::rc::Rc<H> {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        (**self).window_handle()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl<H: HasWindowHandle + ?Sized> HasWindowHandle for alloc::sync::Arc<H> {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        (**self).window_handle()
    }
}

impl HasWindowHandle for WindowHandle<'_> {
    fn window_handle(&self) -> Result<Self, HandleError> {
        Ok(*self)
    }
}
