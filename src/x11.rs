use core::ffi::{c_int, c_ulong, c_void};
use core::num::NonZeroU32;
use core::ptr::NonNull;

/// Raw display handle for Xlib.
///
/// ## Thread Safety
///
/// Reads and writes to and from the X server are internally secure by a [mutex].
/// Therefore this type is `Send` and `Sync`. This means it can be sent to or
/// used from other threads.
///
/// [mutex]: https://gitlab.freedesktop.org/xorg/lib/libx11/-/blob/master/src/locking.c?ref_type=heads#L596
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XlibDisplayHandle {
    /// A pointer to an Xlib `Display`.
    ///
    /// It is strongly recommended to set this value, however it may be set to
    /// `None` to request the default display when using EGL.
    pub display: Option<NonNull<c_void>>,

    /// An X11 screen to use with this display handle.
    ///
    /// Note, that X11 could have multiple screens, however
    /// graphics APIs could work only with one screen at the time,
    /// given that multiple screens usually reside on different GPUs.
    pub screen: c_int,
}

unsafe impl Send for XlibDisplayHandle {}
unsafe impl Sync for XlibDisplayHandle {}

impl XlibDisplayHandle {
    /// Create a new handle to a display.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::XlibDisplayHandle;
    /// #
    /// let display: NonNull<c_void>;
    /// let screen;
    /// # display = NonNull::from(&()).cast();
    /// # screen = 0;
    /// let handle = XlibDisplayHandle::new(Some(display), screen);
    /// ```
    pub fn new(display: Option<NonNull<c_void>>, screen: c_int) -> Self {
        Self { display, screen }
    }
}

/// Raw window handle for Xlib.
///
/// ## Thread Safety
///
/// This type is nothing more than a numeric identifier, therefore it is `Send`
/// and `Sync`. This means it can be safely sent to or used from other threads.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XlibWindowHandle {
    /// An Xlib `Window`.
    pub window: c_ulong,
    /// An Xlib visual ID, or 0 if unknown.
    pub visual_id: c_ulong,
}

impl XlibWindowHandle {
    /// Create a new handle to a window.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_ulong;
    /// # use raw_window_handle::XlibWindowHandle;
    /// #
    /// let window: c_ulong;
    /// # window = 0;
    /// let mut handle = XlibWindowHandle::new(window);
    /// // Optionally set the visual ID.
    /// handle.visual_id = 0;
    /// ```
    pub fn new(window: c_ulong) -> Self {
        Self {
            window,
            visual_id: 0,
        }
    }
}

/// Raw display handle for Xcb.
///
/// ## Thread Safety
///
/// Reads and writes to and from the X server are internally secure by a [mutex].
/// Therefore this type is `Send` and `Sync`. This means it can be sent to or
/// used from other threads.
///
/// [mutex]: https://gitlab.freedesktop.org/xorg/lib/libxcb/-/blob/master/src/xcb_conn.c?ref_type=heads#L165
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XcbDisplayHandle {
    /// A pointer to an X server `xcb_connection_t`.
    ///
    /// It is strongly recommended to set this value, however it may be set to
    /// `None` to request the default display when using EGL.
    pub connection: Option<NonNull<c_void>>,

    /// An X11 screen to use with this display handle.
    ///
    /// Note, that X11 could have multiple screens, however
    /// graphics APIs could work only with one screen at the time,
    /// given that multiple screens usually reside on different GPUs.
    pub screen: c_int,
}

unsafe impl Send for XcbDisplayHandle {}
unsafe impl Sync for XcbDisplayHandle {}

impl XcbDisplayHandle {
    /// Create a new handle to a connection and screen.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::XcbDisplayHandle;
    /// #
    /// let connection: NonNull<c_void>;
    /// let screen;
    /// # connection = NonNull::from(&()).cast();
    /// # screen = 0;
    /// let handle = XcbDisplayHandle::new(Some(connection), screen);
    /// ```
    pub fn new(connection: Option<NonNull<c_void>>, screen: c_int) -> Self {
        Self { connection, screen }
    }
}

/// Raw window handle for Xcb.
///
/// ## Thread Safety
///
/// This type is nothing more than a numeric identifier, therefore it is `Send`
/// and `Sync`. This means it can be safely sent to or used from other threads.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XcbWindowHandle {
    /// An X11 `xcb_window_t`.
    pub window: NonZeroU32, // Based on xproto.h
    /// An X11 `xcb_visualid_t`.
    pub visual_id: Option<NonZeroU32>,
}

impl XcbWindowHandle {
    /// Create a new handle to a window.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::num::NonZeroU32;
    /// # use raw_window_handle::XcbWindowHandle;
    /// #
    /// let window: NonZeroU32;
    /// # window = NonZeroU32::new(1).unwrap();
    /// let mut handle = XcbWindowHandle::new(window);
    /// // Optionally set the visual ID.
    /// handle.visual_id = None;
    /// ```
    pub fn new(window: NonZeroU32) -> Self {
        Self {
            window,
            visual_id: None,
        }
    }
}
