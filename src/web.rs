use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw display handle for the Web.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebDisplayHandle {}

impl WebDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::WebDisplayHandle;
    /// let handle = WebDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

/// Raw window handle for the Web.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebWindowHandle {
    /// An ID value inserted into the [data attributes] of the canvas element as '`raw-handle`'.
    ///
    /// When accessing from JS, the attribute will automatically be called `rawHandle`.
    ///
    /// Each canvas created by the windowing system should be assigned their own unique ID.
    ///
    /// [data attributes]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/data-*
    pub id: u32,
}

impl WebWindowHandle {
    /// Create a new handle to a canvas element.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::WebWindowHandle;
    /// #
    /// let id: u32 = 0; // canvas.rawHandle;
    /// let handle = WebWindowHandle::new(id);
    /// ```
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

/// Raw window handle for a Web canvas registered via [`wasm-bindgen`].
///
/// ## Construction
/// ```no_run
/// # use raw_window_handle::Wbg02CanvasWindowHandle;
/// # use core::{ffi::c_void, ptr::NonNull};
/// # fn get_canvas() -> NonNull<c_void> { unimplemented!() }
/// let obj: NonNull<c_void> = get_canvas();
/// let mut window_handle = Wbg02CanvasWindowHandle::new(obj);
/// /* set fields */
/// ```
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wbg02CanvasWindowHandle {
    /// The object representing the [`HtmlCanvasElement`].
    ///
    /// It is implied that this object is registered in the [`wasm-bindgen`] table and is an instance
    /// of [`HtmlCanvasElement`]. The pointer is a direct reference to a [`JsValue`].
    ///
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
    pub obj: NonNull<c_void>,
}

impl Wbg02CanvasWindowHandle {
    /// Create a new handle to an [`HtmlCanvasElement`].
    ///
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    pub fn new(obj: NonNull<c_void>) -> Self {
        Self { obj }
    }
}

#[cfg(all(target_family = "wasm", feature = "unstable_web_handles_wbg_02"))]
/// These implementations are only available when `unstable_web_handles_wbg_02` is enabled.
impl Wbg02CanvasWindowHandle {
    /// Create a new `Wbg02CanvasWindowHandle` from a [`wasm-bindgen`] object.
    ///
    /// This function is unstable. Its signature may be changed or even removed outright without a
    /// breaking version change.
    ///
    /// # Safety
    ///
    /// The [`JsValue`] must refer to an [`HtmlCanvasElement`], and the lifetime must be longer than
    /// the `Wbg02CanvasWindowHandle` lives for.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    #[cfg_attr(
        docsrs,
        doc(cfg(all(target_family = "wasm", feature = "unstable_web_handles_wbg_02")))
    )]
    pub unsafe fn from_wasm_bindgen_0_2(js_value: &wasm_bindgen::JsValue) -> Self {
        Self::new(NonNull::from(js_value).cast())
    }

    /// Convert to the underlying [`wasm-bindgen`] index.
    ///
    /// This function is unstable. Its signature may be changed or even removed outright without a
    /// breaking version change.
    ///
    /// # Safety
    ///
    /// The lifetime from the `from_wasm_bindgen_0_2` function must still be valid, and the
    /// underlying pointer must still be a [`wasm_bindgen`] object.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    #[cfg_attr(
        docsrs,
        doc(cfg(all(target_family = "wasm", feature = "unstable_web_handles_wbg_02")))
    )]
    pub unsafe fn as_wasm_bindgen_0_2(&self) -> &wasm_bindgen::JsValue {
        self.obj.cast().as_ref()
    }
}

/// Raw window handle for a Web offscreen canvas registered via [`wasm-bindgen`].
///
/// ## Construction
/// ```no_run
/// # use raw_window_handle::Wbg02OffscreenCanvasWindowHandle;
/// # use core::{ffi::c_void, ptr::NonNull};
/// # fn get_offscreen_canvas() -> NonNull<c_void> { unimplemented!() }
/// let obj: NonNull<c_void> = get_offscreen_canvas();
/// let mut window_handle = Wbg02OffscreenCanvasWindowHandle::new(obj);
/// /* set fields */
/// ```
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wbg02OffscreenCanvasWindowHandle {
    /// The object representing the [`OffscreenCanvas`].
    ///
    /// It is implied that this object is registered in the [`wasm-bindgen`] table and is an instance
    /// of [`OffscreenCanvas`]. This is a pointer to the actual [`JsValue`] object.
    ///
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
    pub obj: NonNull<c_void>,
}

impl Wbg02OffscreenCanvasWindowHandle {
    /// Create a new handle to an [`OffscreenCanvas`].
    ///
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    pub fn new(obj: NonNull<c_void>) -> Self {
        Self { obj }
    }
}

#[cfg(all(target_family = "wasm", feature = "unstable_web_handles_wbg_02"))]
/// These implementations are only available when `unstable_web_handles_wbg_02` is enabled.
impl Wbg02OffscreenCanvasWindowHandle {
    /// Create a new `Wbg02OffscreenCanvasWindowHandle` from a [`wasm-bindgen`] object.
    ///
    /// This function is unstable. Its signature may be changed or even removed outright without a
    /// breaking version change.
    ///
    /// # Safety
    ///
    /// The [`JsValue`] must refer to an [`OffscreenCanvas`], and the lifetime must be longer than
    /// the `Wbg02OffscreenCanvasWindowHandle` lives for.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    #[cfg_attr(
        docsrs,
        doc(cfg(all(target_family = "wasm", feature = "unstable_web_handles_wbg_02")))
    )]
    pub unsafe fn from_wasm_bindgen_0_2(js_value: &wasm_bindgen::JsValue) -> Self {
        Self::new(NonNull::from(js_value).cast())
    }

    /// Convert to the underlying [`wasm-bindgen`] index.
    ///
    /// This function is unstable. Its signature may be changed or even removed outright without a
    /// breaking version change.
    ///
    /// # Safety
    ///
    /// The lifetime from the `from_wasm_bindgen_0_2` function must still be valid, and the
    /// underlying pointer must still be a [`wasm_bindgen`] object.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    #[cfg_attr(
        docsrs,
        doc(cfg(all(target_family = "wasm", feature = "unstable_web_handles_wbg_02")))
    )]
    pub unsafe fn as_wasm_bindgen_0_2(&self) -> &wasm_bindgen::JsValue {
        self.obj.cast().as_ref()
    }
}
