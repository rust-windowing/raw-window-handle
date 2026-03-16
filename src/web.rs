use core::marker::PhantomData;

use super::DisplayHandle;

/// Raw display handle for the Web.
///
/// ## Thread-Safety
///
/// WASM objects are usually bound to the main UI "thread" belonging to the
/// top-level webpage. Therefore this type is `!Send` and `!Sync`. It cannot be
/// sent to or used from other threads.
///
/// Note that this type does not contain any WASM objects. However,
/// it is kept `!Send` and `!Sync` for the event that WASM objects are
/// added to this type.
///
/// However, this status quo may change in the future, due to the adoption of
/// atomics in WASM code. Therefore this type may be made `Send` and `Sync` as
/// part of a non-breaking change.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WasmBindgenDisplay {
    _thread_unsafe: PhantomData<*mut ()>,
}

impl WasmBindgenDisplay {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::WasmBindgenDisplay;
    /// let handle = WasmBindgenDisplay::new();
    /// ```
    pub fn new() -> Self {
        Self {
            _thread_unsafe: PhantomData,
        }
    }
}

impl DisplayHandle<'static> {
    /// Create a `wasm-bindgen`-based display handle.
    ///
    /// As no data is borrowed by this handle, it is completely safe to create. This function
    /// may be useful to windowing framework implementations that want to avoid unsafe code.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::{DisplayHandle, HasDisplayHandle};
    /// # fn do_something(rwh: impl HasDisplayHandle) { let _ = rwh; }
    /// let handle = DisplayHandle::wasm_bindgen();
    /// do_something(handle);
    /// ```
    pub fn wasm_bindgen() -> Self {
        // SAFETY: No data is borrowed.
        unsafe { Self::borrow_raw(WasmBindgenDisplay::new().into()) }
    }
}

/// Raw window handle for a Web canvas registered via [`wasm-bindgen`].
///
/// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
///
/// ## Thread-Safety
///
/// WASM objects are usually bound to the main UI "thread" belonging to the
/// top-level webpage. Therefore this type is `!Send` and `!Sync`. It cannot be
/// sent to or used from other threads.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WasmBindgenCanvasWindowHandle {
    /// An inner index of the [`JsValue`] of an [`HtmlCanvasElement`].
    ///
    /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    pub obj: usize,

    /// Makes this type `!Send` and `!Sync`.
    _marker: PhantomData<*mut ()>,
}

impl WasmBindgenCanvasWindowHandle {
    /// Create a new handle from a pointer to [`HtmlCanvasElement`].
    ///
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    ///
    /// # Example
    ///
    #[cfg_attr(target_family = "wasm", doc = "```no_run")]
    #[cfg_attr(not(target_family = "wasm"), doc = "```no_compile")]
    /// # use raw_window_handle::WasmBindgenCanvasWindowHandle;
    /// use core::mem::ManuallyDrop;
    /// use wasm_bindgen::convert::{IntoWasmAbi, RefFromWasmAbi};
    /// use web_sys::HtmlCanvasElement;
    ///
    /// let value: HtmlCanvasElement;
    /// # value = todo!();
    ///
    /// // Convert to the raw index and convert to the handle.
    /// let index = (&value).into_abi();
    /// let mut handle = WasmBindgenCanvasWindowHandle::new(index as usize);
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
///
/// ## Thread-Safety
///
/// WASM objects are usually bound to the main UI "thread" belonging to the
/// top-level webpage. Therefore this type is `!Send` and `!Sync`. It cannot be
/// sent to or used from other threads.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WasmBindgenOffscreenCanvasWindowHandle {
    /// An inner index of the [`JsValue`] of an [`OffscreenCanvas`].
    ///
    /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    pub obj: usize,

    /// Makes this type `!Send` and `!Sync`.
    _marker: PhantomData<*mut ()>,
}

impl WasmBindgenOffscreenCanvasWindowHandle {
    /// Create a new handle from a pointer to an [`OffscreenCanvas`].
    ///
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    ///
    /// # Example
    ///
    #[cfg_attr(target_family = "wasm", doc = "```no_run")]
    #[cfg_attr(not(target_family = "wasm"), doc = "```no_compile")]
    /// # use raw_window_handle::WasmBindgenOffscreenCanvasWindowHandle;
    /// use core::mem::ManuallyDrop;
    /// use wasm_bindgen::convert::{IntoWasmAbi, RefFromWasmAbi};
    /// use web_sys::OffscreenCanvas;
    ///
    /// let value: OffscreenCanvas;
    /// # value = todo!();
    ///
    /// // Convert to the raw index and convert to the handle.
    /// let index = (&value).into_abi();
    /// let handle = WasmBindgenOffscreenCanvasWindowHandle::new(index as usize);
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
