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
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebCanvasWindowHandle {
    /// The pointer to the [`JsValue`] of an [`HtmlCanvasElement`].
    ///
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
    pub obj: NonNull<c_void>,
}

impl WebCanvasWindowHandle {
    /// Create a new handle to a pointer to [`HtmlCanvasElement`].
    ///
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::WebCanvasWindowHandle;
    /// # type HtmlCanvasElement = ();
    /// # type JsValue = ();
    /// let canvas: &HtmlCanvasElement;
    /// # canvas = &();
    /// let value: &JsValue = &canvas; // Deref to `JsValue`
    /// let obj: NonNull<c_void> = NonNull::from(value).cast();
    /// let mut handle = WebCanvasWindowHandle::new(obj);
    /// ```
    pub fn new(obj: NonNull<c_void>) -> Self {
        Self { obj }
    }
}

#[cfg(all(target_family = "wasm", feature = "wasm-bindgen-0-2"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(target_family = "wasm", feature = "wasm-bindgen-0-2")))
)]
/// These implementations are only available when `wasm-bindgen-0-2` is enabled.
impl WebCanvasWindowHandle {
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
    pub fn from_wasm_bindgen_0_2(js_value: &wasm_bindgen::JsValue) -> Self {
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
    pub unsafe fn as_wasm_bindgen_0_2(&self) -> &wasm_bindgen::JsValue {
        self.obj.cast().as_ref()
    }
}

/// Raw window handle for a Web offscreen canvas registered via [`wasm-bindgen`].
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebOffscreenCanvasWindowHandle {
    /// The pointer to the [`JsValue`] of an [`OffscreenElement`].
    ///
    /// [`OffscreenElement`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenElement.html
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
    pub obj: NonNull<c_void>,
}

impl WebOffscreenCanvasWindowHandle {
    /// Create a new handle to a pointer to an [`OffscreenCanvas`].
    ///
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::WebOffscreenCanvasWindowHandle;
    /// # type OffscreenCanvas = ();
    /// # type JsValue = ();
    /// let canvas: &OffscreenCanvas;
    /// # canvas = &();
    /// let value: &JsValue = &canvas; // Deref to `JsValue`
    /// let obj: NonNull<c_void> = NonNull::from(value).cast();
    /// let mut handle = WebOffscreenCanvasWindowHandle::new(obj);
    /// ```
    pub fn new(obj: NonNull<c_void>) -> Self {
        Self { obj }
    }
}

#[cfg(all(target_family = "wasm", feature = "wasm-bindgen-0-2"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(target_family = "wasm", feature = "wasm-bindgen-0-2")))
)]
/// These implementations are only available when `wasm-bindgen-0-2` is enabled.
impl WebOffscreenCanvasWindowHandle {
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
    pub fn from_wasm_bindgen_0_2(js_value: &wasm_bindgen::JsValue) -> Self {
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
    pub unsafe fn as_wasm_bindgen_0_2(&self) -> &wasm_bindgen::JsValue {
        self.obj.cast().as_ref()
    }
}
