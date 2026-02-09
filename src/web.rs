use core::marker::PhantomData;

use super::DisplayHandle;

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

impl DisplayHandle<'static> {
    /// Create a Web-based display handle.
    ///
    /// As no data is borrowed by this handle, it is completely safe to create. This function
    /// may be useful to windowing framework implementations that want to avoid unsafe code.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::web();
    /// do_something(handle);
    /// ```
    pub fn web() -> Self {
        // SAFETY: No data is borrowed.
        unsafe { Self::borrow_raw(WebDisplayHandle::new().into()) }
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
    /// An inner index of the [`JsValue`] of an [`HtmlCanvasElement`].
    ///
    /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    pub obj: usize,

    /// Makes this type `!Send` and `!Sync`.
    _marker: PhantomData<*mut ()>,
}

impl WebCanvasWindowHandle {
    /// Create a new handle from a pointer to [`HtmlCanvasElement`].
    ///
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    ///
    /// # Example
    ///
    #[cfg_attr(target_family = "wasm", doc = "```no_run")]
    #[cfg_attr(not(target_family = "wasm"), doc = "```no_compile")]
    /// # use raw_window_handle::WebCanvasWindowHandle;
    /// use core::mem::ManuallyDrop;
    /// use wasm_bindgen::convert::{IntoWasmAbi, RefFromWasmAbi};
    /// use web_sys::HtmlCanvasElement;
    ///
    /// let value: HtmlCanvasElement;
    /// # value = todo!();
    ///
    /// // Convert to the raw index and convert to the handle.
    /// let index = (&value).into_abi();
    /// let mut handle = WebCanvasWindowHandle::new(index as usize);
    ///
    /// // To get the canvas element back, convert the index back.
    /// let other_end: ManuallyDrop<HtmlCanvasElement> = unsafe {
    ///     HtmlCanvasElement::ref_from_abi(handle.obj as u32)
    /// };
    /// ```
    pub fn new(obj: usize) -> Self {
        Self {
            obj,
            _marker: PhantomData,
        }
    }
}

/// Raw window handle for a Web offscreen canvas registered via
/// [`wasm-bindgen`].
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebOffscreenCanvasWindowHandle {
    /// An inner index of the [`JsValue`] of an [`OffscreenCanvas`].
    ///
    /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    pub obj: usize,

    /// Makes this type `!Send` and `!Sync`.
    _marker: PhantomData<*mut ()>,
}

impl WebOffscreenCanvasWindowHandle {
    /// Create a new handle from a pointer to an [`OffscreenCanvas`].
    ///
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    ///
    /// # Example
    ///
    #[cfg_attr(target_family = "wasm", doc = "```no_run")]
    #[cfg_attr(not(target_family = "wasm"), doc = "```no_compile")]
    /// # use raw_window_handle::WebOffscreenCanvasWindowHandle;
    /// use core::mem::ManuallyDrop;
    /// use wasm_bindgen::convert::{IntoWasmAbi, RefFromWasmAbi};
    /// use web_sys::OffscreenCanvas;
    ///
    /// let value: OffscreenCanvas;
    /// # value = todo!();
    ///
    /// // Convert to the raw index and convert to the handle.
    /// let index = (&value).into_abi();
    /// let handle = WebOffscreenCanvasWindowHandle::new(index as usize);
    ///
    /// // To get the canvas element back, convert the index back.
    /// let other_end: ManuallyDrop<OffscreenCanvas> = unsafe {
    ///     OffscreenCanvas::ref_from_abi(handle.obj as u32)
    /// };
    /// ```
    pub fn new(obj: usize) -> Self {
        Self {
            obj,
            _marker: PhantomData,
        }
    }
}
