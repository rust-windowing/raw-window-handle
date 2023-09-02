use core::fmt;

/// Raw display handle for the Web.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
/// # use raw_window_handle::{Wbg02CanvasWindowHandle, Wbg02Object};
/// # fn get_canvas() -> Wbg02Object { unimplemented!() }
/// let obj: Wbg02Object = get_canvas();
/// let mut window_handle = Wbg02CanvasWindowHandle::new(obj);
/// /* set fields */
/// ```
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct Wbg02CanvasWindowHandle {
    /// The object representing the [`HtmlCanvasElement`].
    ///
    /// It is implied that this object is registered in the [`wasm-bindgen`] table and is an instance
    /// of [`HtmlCanvasElement`].
    ///
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    pub obj: Wbg02Object,
}

impl Wbg02CanvasWindowHandle {
    /// Create a new handle to an [`HtmlCanvasElement`].
    ///
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    pub fn new(obj: Wbg02Object) -> Self {
        Self { obj }
    }
}

/// Raw window handle for a Web offscreen canvas registered via [`wasm-bindgen`].
///
/// ## Construction
/// ```no_run
/// # use raw_window_handle::{Wbg02OffscreenCanvasWindowHandle, Wbg02Object};
/// # fn get_offscreen_canvas() -> Wbg02Object { unimplemented!() }
/// let obj: Wbg02Object = get_offscreen_canvas();
/// let mut window_handle = Wbg02OffscreenCanvasWindowHandle::new(obj);
/// /* set fields */
/// ```
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct Wbg02OffscreenCanvasWindowHandle {
    /// The object representing the [`OffscreenCanvas`].
    ///
    /// It is implied that this object is registered in the [`wasm-bindgen`] table and is an instance
    /// of [`OffscreenCanvas`].
    ///
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    pub obj: Wbg02Object,
}

impl Wbg02OffscreenCanvasWindowHandle {
    /// Create a new handle to an [`OffscreenCanvas`].
    ///
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    pub fn new(obj: Wbg02Object) -> Self {
        Self { obj }
    }
}

/// An object currently stored in [`wasm-bindgen`].
///
/// This type represents an object stored in the [`wasm-bindgen`] object table. It represents some kind
/// of underlying web object, such as an `HtmlCanvasElement` or an [`OffscreenCanvas`].
///
/// For WASM targets, with the unstable `unstable_web_handles` feature enabled, this type contains
/// an index into the table corresponding to a JavaScript object. In other cases, this type is
/// uninhabited.
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
/// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
#[derive(Clone, PartialEq)]
pub struct Wbg02Object {
    /// For when `unstable_web_handles` is enabled, this is the index of the object in the
    /// `wasm-bindgen` table.
    #[cfg(all(target_family = "wasm", feature = "unstable_web_handles"))]
    idx: wasm_bindgen::JsValue,

    /// In other cases, this type is uninhabited.
    #[cfg(not(all(target_family = "wasm", feature = "unstable_web_handles")))]
    _uninhabited: core::convert::Infallible,
}

impl fmt::Debug for Wbg02Object {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(all(target_family = "wasm", feature = "unstable_web_handles"))]
        {
            _f.debug_tuple("Wbg02Object").field(&self.idx).finish()
        }

        #[cfg(not(all(target_family = "wasm", feature = "unstable_web_handles")))]
        match self._uninhabited {}
    }
}

#[cfg(all(target_family = "wasm", feature = "unstable_web_handles"))]
/// These implementations are only available when `unstable_web_handles` is enabled.
impl Wbg02Object {
    /// Create a new `Wbg02Object` from a [`wasm-bindgen`] object.
    ///
    /// This function is unstable. Its signature may be changed or even removed outright without a
    /// breaking version change.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    #[cfg_attr(
        docsrs,
        doc(cfg(all(target_family = "wasm", feature = "unstable_web_handles")))
    )]
    pub fn new(idx: wasm_bindgen::JsValue) -> Self {
        Self { idx }
    }

    /// Get the index of the object in the [`wasm-bindgen`] table.
    ///
    /// This function is unstable. Its signature may be changed or even removed outright without a
    /// breaking version change.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    #[cfg_attr(
        docsrs,
        doc(cfg(all(target_family = "wasm", feature = "unstable_web_handles")))
    )]
    pub fn idx(&self) -> &wasm_bindgen::JsValue {
        &self.idx
    }

    /// Convert to the underlying [`wasm-bindgen`] index.
    ///
    /// This function is unstable. Its signature may be changed or even removed outright without a
    /// breaking version change.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    #[cfg_attr(
        docsrs,
        doc(cfg(all(target_family = "wasm", feature = "unstable_web_handles")))
    )]
    pub fn into_idx(self) -> wasm_bindgen::JsValue {
        self.idx
    }
}
