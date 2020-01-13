use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw window handle for Android.
///
/// ## Construction
/// ```
/// # use raw_window_handle::android::AndroidHandle;
/// let handle = AndroidHandle {
///     /* fields */
///     ..AndroidHandle::empty()
/// };
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AndroidHandle {
    /// A pointer to an ANativeWindow.
    pub a_native_window: Option<NonNull<c_void>>,
}

impl AndroidHandle {
    pub fn empty() -> AndroidHandle {
        AndroidHandle {
            a_native_window: None,
        }
    }
}
