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
/// ```
/// # use raw_window_handle::Wbg02CanvasWindowHandle;
/// let mut window_handle = Wbg02CanvasWindowHandle::empty();
/// /* set fields */
/// ```
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wbg02CanvasWindowHandle {
    /// The index of the canvas element in the [`wasm-bindgen`] table.
    ///
    /// If this index if non-zero, it is implied that it represents an [`HtmlCanvasElement`]
    /// that is registered in the [`wasm-bindgen`] table. It can be converted to and from the
    /// [`HtmlCanvasElement`] using [`wasm-bindgen`]'s [`FromWasmAbi`] and [`IntoWasmAbi`] traits.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    /// [`FromWasmAbi`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/convert/trait.FromWasmAbi.html
    /// [`IntoWasmAbi`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/convert/trait.IntoWasmAbi.html
    pub idx: u32,
}

impl Wbg02CanvasWindowHandle {
    pub fn empty() -> Self {
        Self { idx: 0 }
    }
}

/// Raw window handle for a Web offscreen canvas registered via [`wasm-bindgen`].
///
/// ## Construction
/// ```
/// # use raw_window_handle::Wbg02OffscreenCanvasWindowHandle;
/// let mut window_handle = Wbg02OffscreenCanvasWindowHandle::empty();
/// /* set fields */
/// ```
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wbg02OffscreenCanvasWindowHandle {
    /// The index of the canvas element in the [`wasm-bindgen`] table.
    ///
    /// If this index if non-zero, it is implied that it represents an [`OffscreenCanvas`]
    /// that is registered in the [`wasm-bindgen`] table. It can be converted to and from the
    /// [`OffscreenCanvas`] using [`wasm-bindgen`]'s [`FromWasmAbi`] and [`IntoWasmAbi`] traits.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    /// [`FromWasmAbi`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/convert/trait.FromWasmAbi.html
    /// [`IntoWasmAbi`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/convert/trait.IntoWasmAbi.html
    pub idx: u32,
}

impl Wbg02OffscreenCanvasWindowHandle {
    pub fn empty() -> Self {
        Self { idx: 0 }
    }
}
