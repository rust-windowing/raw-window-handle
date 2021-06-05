use core::ffi::c_void;
use core::ptr;

/// Raw window handle for Redox OS.
///
/// ## Construction
/// ```
/// # use raw_window_handle::redox::RedoxHandle;
/// let mut handle = RedoxHandle::empty();
///  /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedoxHandle {
    /// A pointer to an orbclient window.
    pub window: *mut c_void,
}

impl RedoxHandle {
    pub fn empty() -> RedoxHandle {
        RedoxHandle {
            window: ptr::null_mut(),
        }
    }
}
