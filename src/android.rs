use core::ffi::c_void;
use core::ptr;

/// Raw window handle for Android NDK.
///
/// ## Construction
/// ```
/// # use raw_window_handle::AndroidNDKHandle;
/// let mut handle = AndroidNDKHandle::empty();
/// /* set fields */
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AndroidNDKHandle {
    /// A pointer to an `ANativeWindow`.
    pub a_native_window: *mut c_void,
}

impl AndroidNDKHandle {
    pub fn empty() -> Self {
        Self {
            a_native_window: ptr::null_mut(),
        }
    }
}
