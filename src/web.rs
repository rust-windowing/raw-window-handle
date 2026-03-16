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
