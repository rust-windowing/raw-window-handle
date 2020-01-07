use core::ptr;
use orbclient::Window;

/// Raw window handle for Redox OS.
///
/// ## Construction
/// ```
/// # use raw_window_handle::redox::RedoxHandle;
/// let handle = RedoxHandle {
///     /* fields */
///     ..RedoxHandle::empty()
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedoxHandle {
    pub window: *mut Window,

    #[doc(hidden)]
    #[deprecated = "This field is used to ensure that this struct is non-exhaustive, so that it may be extended in the future. Do not refer to this field."]
    pub _non_exhaustive_do_not_use: crate::seal::Seal,
}

impl RedoxHandle {
    pub fn empty() -> RedoxHandle {
        #[allow(deprecated)]
        RedoxHandle {
            window: ptr::null_mut(),
            _non_exhaustive_do_not_use: crate::seal::Seal,
        }
    }
}
