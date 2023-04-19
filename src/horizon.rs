use core::ffi::c_uint;

/// Raw display handle for the 3DS.
///
/// ## Construction
/// ```
/// # use raw_window_handle::HorizonDisplayHandle;
/// let mut display_handle = HorizonDisplayHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HorizonDisplayHandle;

impl HorizonDisplayHandle {
    /// Create an empty display handle.
    pub fn empty() -> Self {
        Self {}
    }
}

/// Raw window handle for the 3DS.
///
/// ## Construction
/// ```
/// # use raw_window_handle::HorizonWindowHandle;
/// let mut window_handle = HorizonWindowHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HorizonWindowHandle {
    /// The screen that this window exists on.
    ///
    /// Since the top screen is usually represented by zero, the empty version of this type is
    /// represented by `c_uint::MAX`.
    pub screen: c_uint,
}

impl HorizonWindowHandle {
    /// Create an empty window handle.
    pub fn empty() -> Self {
        Self {
            screen: c_uint::MAX,
        }
    }
}
