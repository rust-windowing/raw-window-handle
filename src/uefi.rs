use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw display handle for UEFI.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UefiDisplayHandle {
    /// The UEFI Graphics Output Protocol handle.
    ///
    /// From the UEFI spec on <https://uefi.org>, can be found in versions since 2.0 as
    /// `EFI_GRAPHICS_OUTPUT_PROTOCOL`.
    pub handle: NonNull<c_void>,
}

impl UefiDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::UefiDisplayHandle;
    /// # type GraphicsOutput = ();
    /// let ptr: NonNull<GraphicsOutput>;
    /// # ptr = NonNull::from(&());
    /// let handle = UefiDisplayHandle::new(ptr.cast());
    /// ```
    pub fn new(handle: NonNull<c_void>) -> Self {
        Self { handle }
    }
}

/// Raw window handle for UEFI.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UefiWindowHandle {}

impl UefiWindowHandle {
    pub fn new() -> Self {
        Self {}
    }
}
