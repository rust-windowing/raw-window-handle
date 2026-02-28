use core::ffi::{c_int, c_ulong, c_void};
use core::marker::PhantomData;
use core::num::NonZeroU32;
use core::ptr::NonNull;

/// Raw display handle for Xlib.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XlibDisplayHandle<'display> {
    display: Option<NonNull<c_void>>,
    screen: c_int,
    _marker: PhantomData<&'display ()>,
}

impl XlibDisplayHandle<'_> {
    /// Create a new handle to a display.
    ///
    /// # Safety
    ///
    /// `display` must be a valid pointer to an Xlib `Display` and must remain valid for the
    /// lifetime of this type.
    ///
    /// TODO: `screen` must be valid too?
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
    /// let handle = unsafe { XlibDisplayHandle::new(display, screen) };
    /// ```
    pub unsafe fn new(display: NonNull<c_void>, screen: c_int) -> Self {
        Self {
            display: Some(display),
            screen,
            _marker: PhantomData,
        }
    }

    /// Create a new handle to a screen with the default display.
    ///
    /// You are strongly encouraged to call [`XcbDisplayHandle::new`] when possible.
    ///
    /// # Safety
    ///
    /// TODO: Must be screen ID be valid? And what should the lifetime that this returns be?
    pub unsafe fn with_default_display(screen: c_int) -> Self {
        Self {
            display: None,
            screen,
            _marker: PhantomData,
        }
    }

    /// A pointer to an Xlib `Display`.
    ///
    /// It is strongly recommended to set this value, however it may be set to
    /// `None` to request the default display when using EGL.
    ///
    /// If the pointer is `Some`, it is guaranteed to be valid for at least as long as `self`.
    pub fn display(&self) -> Option<NonNull<c_void>> {
        self.display
    }

    /// An X11 screen to use with this display handle.
    ///
    /// Note, that X11 could have multiple screens, however
    /// graphics APIs could work only with one screen at the time,
    /// given that multiple screens usually reside on different GPUs.
    pub fn screen(&self) -> c_int {
        self.screen
    }
}

/// Raw window handle for Xlib.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XlibWindowHandle {
    // TODO(MSRV 1.79): Use `NonZero<c_ulong>`?
    window: c_ulong,
    // TODO(MSRV 1.79): Use `Option<NonZero<c_ulong>>`?
    visual_id: c_ulong,
}

impl XlibWindowHandle {
    /// Create a new handle to a window.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_ulong;
    /// # use raw_window_handle::XlibWindowHandle;
    /// #
    /// let window: c_ulong;
    /// # window = 0;
    /// let handle = XlibWindowHandle::new(window);
    /// ```
    pub fn new(window: c_ulong) -> Self {
        // Safe because window IDs are just that, and ID (so they're both Send+Sync, and have no
        // lifetime issues).
        //
        // The handle may be deleted by safe code in the same process. It is even possible for code
        // in a different process to delete the window. In fact, it is possible for code on a
        // different *machine* to delete the window.
        //
        // So users of the handle must always be ready to handle those error cases.

        // TODO: Assert window != 0?
        Self {
            window,
            visual_id: 0,
        }
    }

    /// Create a new handle to a window along with a visual ID.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_ulong;
    /// # use raw_window_handle::XlibWindowHandle;
    /// #
    /// let window: c_ulong;
    /// let visual_id: c_ulong;
    /// # window = 1;
    /// # visual_id = 1;
    /// let handle = XlibWindowHandle::with_visual_id(window, visual_id);
    /// ```
    pub fn with_visual_id(window: c_ulong, visual_id: c_ulong) -> Self {
        assert_ne!(visual_id, 0); // TODO: Should we have this check?
        Self { window, visual_id }
    }

    /// An Xlib `Window`.
    pub fn window(&self) -> c_ulong {
        self.window
    }

    /// An Xlib visual ID, or 0 if unknown.
    pub fn visual_id(&self) -> c_ulong {
        self.visual_id
    }
}

/// Raw display handle for Xcb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XcbDisplayHandle<'display> {
    connection: Option<NonNull<c_void>>,
    screen: c_int,
    _marker: PhantomData<&'display ()>,
}

impl XcbDisplayHandle<'_> {
    /// Create a new handle to a connection and screen.
    ///
    /// # Safety
    ///
    /// `display` must be a valid pointer to an Xlib `Display` and must remain valid for the
    /// lifetime of this type.
    ///
    /// TODO: `screen` must be valid too?
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
    /// let handle = unsafe { XcbDisplayHandle::new(connection, screen) };
    /// ```
    pub unsafe fn new(connection: NonNull<c_void>, screen: c_int) -> Self {
        Self {
            connection: Some(connection),
            screen,
            _marker: PhantomData,
        }
    }

    /// Create a new handle to a screen with the default connection.
    ///
    /// You are strongly encouraged to call [`XcbDisplayHandle::new`] when possible.
    ///
    /// # Safety
    ///
    /// TODO: Must be screen ID be valid? And what should the lifetime that this returns be?
    pub unsafe fn with_default_connection(screen: c_int) -> Self {
        Self {
            connection: None,
            screen,
            _marker: PhantomData,
        }
    }

    /// A pointer to an X server `xcb_connection_t`.
    ///
    /// It is strongly recommended that producers set this value, however it may be set to
    /// `None` to request the default display when using EGL.
    ///
    /// If the pointer is `Some`, it is guaranteed to be valid for at least as long as `self`.
    pub fn connection(&self) -> Option<NonNull<c_void>> {
        self.connection
    }

    /// An X11 screen to use with this display handle.
    ///
    /// Note, that X11 could have multiple screens, however
    /// graphics APIs could work only with one screen at the time,
    /// given that multiple screens usually reside on different GPUs.
    pub fn screen(&self) -> c_int {
        self.screen
    }
}

/// Raw window handle for Xcb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XcbWindowHandle {
    window: NonZeroU32, // Based on xproto.h
    visual_id: Option<NonZeroU32>,
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
    /// let handle = XcbWindowHandle::new(window);
    /// ```
    pub fn new(window: NonZeroU32) -> Self {
        // Safe because window IDs are just that, and ID (so they're both Send+Sync, and have no
        // lifetime issues).
        Self {
            window,
            visual_id: None,
        }
    }

    /// Create a new handle to a window along with a `xcb_visualid_t`.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::num::NonZeroU32;
    /// # use raw_window_handle::XcbWindowHandle;
    /// #
    /// let window: NonZeroU32;
    /// let visual_id: NonZeroU32;
    /// # window = NonZeroU32::new(1).unwrap();
    /// # visual_id = NonZeroU32::new(1).unwrap();
    /// let handle = XcbWindowHandle::with_visual_id(window, visual_id);
    /// ```
    pub fn with_visual_id(window: NonZeroU32, visual_id: NonZeroU32) -> Self {
        Self {
            window,
            visual_id: Some(visual_id),
        }
    }

    /// An X11 `xcb_window_t`.
    pub fn window(&self) -> NonZeroU32 {
        self.window
    }

    /// An X11 `xcb_visualid_t`.
    pub fn visual_id(&self) -> Option<NonZeroU32> {
        self.visual_id
    }
}
