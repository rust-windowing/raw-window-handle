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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AndroidHandle {
    /// A pointer to an ANativeWindow.
    pub a_native_window: Option<NonNull<c_void>>,
    #[doc(hidden)]
    #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
    pub _non_exhaustive_do_not_use: crate::seal::Seal,
}

impl AndroidHandle {
    pub fn empty() -> AndroidHandle {
        #[allow(deprecated)]
        AndroidHandle {
            a_native_window: None,
            _non_exhaustive_do_not_use: crate::seal::Seal,
        }
    }
}
