use core::{
    ffi::{c_int, c_void},
    ptr::NonNull,
};

/// A display server handle for a particular windowing system.
///
/// The display usually represents a connection to some display server, but it is not necessarily
/// tied to a particular window. Some APIs can use the display handle without ever creating a window
/// handle (e.g. offscreen rendering, headless event handling).
///
/// Each variant fields specific to that windowing system (e.g. [`RawDisplayHandle::Xlib`] contains
/// a [Display] connection to an X Server, [`RawDisplayHandle::Wayland`] uses [wl_display] to
/// connect to a compositor). Not all windowing systems have a separate display handle (or they
/// haven't been implemented yet) and their variants are empty.
///
/// [Display]: https://www.x.org/releases/current/doc/libX11/libX11/libX11.html#Display_Functions
/// [wl_display]: https://wayland.freedesktop.org/docs/html/apb.html#Client-classwl__display
///
/// ## Thread safety
///
/// See individual variants for thread safety documentation. Since some
/// window handle types are `!Send` and `!Sync`, this sum type is as well.
///
/// # Variant Availability
///
/// Note that all variants are present on all targets (none are disabled behind
/// `#[cfg]`s), but see the "Availability Hints" section on each variant for
/// some hints on where this variant might be expected.
///
/// Note that these "Availability Hints" are not normative. That is to say, a
/// [`HasDisplayHandle`] implementor is completely allowed to return something
/// unexpected. (For example, it's legal for someone to return a
/// [`RawDisplayHandle::Xlib`] on macOS, it would just be weird, and probably
/// requires something like XQuartz be used).
///
/// [`HasDisplayHandle`]: crate::HasDisplayHandle
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawDisplayHandle {
    /// A raw display handle for UIKit (Apple's non-macOS windowing library).
    ///
    /// ## Thread safety
    ///
    /// This type has the same thread safety guarantees as [`RawWindowHandle::UiKit`].
    ///
    /// Note that this type does not contain any UIKit objects. However,
    /// it is kept `!Send` and `!Sync` for the event that UIKit objects are
    /// added to this type.
    ///
    /// [`RawWindowHandle::UiKit`]: crate::RawWindowHandle::UiKit
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on iOS, tvOS, watchOS, visionOS, and Mac
    /// Catalyst, as these are the targets that (currently) support UIKit.
    ///
    /// Note that Mac Catalyst (`$arch-apple-ios-macabi` targets), can use
    /// UIKit *or* AppKit.
    #[non_exhaustive]
    UiKit {
        // Empty for now, logically `!Send` + `!Sync`.
    },

    /// A raw display handle for AppKit.
    ///
    /// ## Thread safety
    ///
    /// This type has the safe thread safety guarantees as [`RawWindowHandle::AppKit`].
    ///
    /// Note that this type does not contain any Appkit objects. However,
    /// it is kept `!Send` and `!Sync` for the event that Appkit objects are
    /// added to this type.
    ///
    /// [`RawWindowHandle::AppKit`]: crate::RawWindowHandle::AppKit
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on macOS, although Mac Catalyst can also use it
    /// despite being `target_os = "ios"`.
    #[non_exhaustive]
    AppKit {
        // Empty for now, logically `!Send` + `!Sync`.
        // Could probably be either `NSApplication` or `CATransaction`.
    },

    /// A raw display handle for the Redox operating system.
    ///
    /// ## Thread safety
    ///
    /// The underlying window is a [file descriptor], and most calls on the window
    /// correspond directly to non-mutating file descriptor reads and writes.
    /// Therefore this type is `Send` and `Sync`. This means that this type can be
    /// sent to and used from other threads.
    ///
    /// Note that this type does not currently contain any Orbital file descriptors.
    /// This type is kept as `Send` and `Sync` in preparation for file descriptors
    /// to be added to this type.
    ///
    /// [file descriptor]: https://github.com/redox-os/orbclient/blob/77c28e88fcb180c750175f2dcf5c7342d357ab26/src/sys/orbital.rs#L64-L65
    ///
    /// ## Availability Hints
    ///
    /// This variant is used by the Orbital Windowing System in the Redox
    /// operating system.
    #[non_exhaustive]
    Orbital {
        // Empty for now.
    },

    /// A raw display handle for OpenHarmony OS NDK
    ///
    /// ## Thread safety
    ///
    /// OpenHarmony [expects] that UI primitives will only be called from one
    /// thread. Therefore, all OHOS objects are `!Send` and `!Sync`. This means
    /// that this type cannot be sent to or used from other threads.
    ///
    /// Note that this type does not contain any OHOS objects. However, it is kept
    /// `!Send` and `!Sync` for the event that OHOS objects are added to this
    /// type.
    ///
    /// [expects]: https://ai6s.net/6921b48882fbe0098cade00f.html
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on OpenHarmony OS (`target_env = "ohos"`).
    #[non_exhaustive]
    Ohos {
        // Empty for now, logically `!Send` + `!Sync`.
    },

    /// A raw display handle for Xlib.
    ///
    /// ## Thread safety
    ///
    /// Reads and writes to and from the X server are internally secure by a [mutex].
    /// Therefore this type is `Send` and `Sync`. This means it can be sent to or
    /// used from other threads.
    ///
    /// [mutex]: https://gitlab.freedesktop.org/xorg/lib/libx11/-/blob/master/src/locking.c?ref_type=heads#L596
    ///
    /// ## Availability Hints
    ///
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that Xlib can be built for, which is to say, most (but not all)
    /// Unix systems.
    #[non_exhaustive]
    Xlib {
        /// A pointer to an Xlib `Display`.
        ///
        /// It is strongly recommended to set this value, however it may be set to
        /// `None` to request the default display when using EGL.
        display: Option<NonNull<c_void>>,

        /// An X11 screen to use with this display handle.
        ///
        /// Note, that X11 could have multiple screens, however
        /// graphics APIs could work only with one screen at the time,
        /// given that multiple screens usually reside on different GPUs.
        screen: c_int,
    },

    /// A raw display handle for Xcb.
    ///
    /// ## Thread safety
    ///
    /// Reads and writes to and from the X server are internally secure by a [mutex].
    /// Therefore this type is `Send` and `Sync`. This means it can be sent to or
    /// used from other threads.
    ///
    /// [mutex]: https://gitlab.freedesktop.org/xorg/lib/libxcb/-/blob/master/src/xcb_conn.c?ref_type=heads#L165
    ///
    /// ## Availability Hints
    ///
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that XCB can be built for, which is to say, most (but not all)
    /// Unix systems.
    #[non_exhaustive]
    Xcb {
        /// A pointer to an X server `xcb_connection_t`.
        ///
        /// It is strongly recommended to set this value, however it may be set to
        /// `None` to request the default display when using EGL.
        connection: Option<NonNull<c_void>>,

        /// An X11 screen to use with this display handle.
        ///
        /// Note, that X11 could have multiple screens, however
        /// graphics APIs could work only with one screen at the time,
        /// given that multiple screens usually reside on different GPUs.
        screen: c_int,
    },

    /// A raw display handle for Wayland.
    ///
    /// ## Thread safety
    ///
    /// `libwayland-client` is thread safe, therefore this type is `Send` and `Sync`.
    /// This means that this type can be sent to and from other threads.
    ///
    /// ## Availability Hints
    ///
    /// This variant should be expected anywhere Wayland works, which is
    /// currently some subset of unix systems.
    #[non_exhaustive]
    Wayland {
        /// A pointer to a `wl_display`.
        display: NonNull<c_void>,
    },

    /// A raw display handle for the Linux Kernel Mode Set/Direct Rendering Manager
    ///
    /// ## Thread safety
    ///
    /// The DRM display handle is a file descriptor, and file descriptors in Unix
    /// are thread-safe by default. Therefore this type is `Send` and `Sync`. This
    /// means that it can be sent to or used from other threads.
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on Linux when neither X nor Wayland are available
    #[non_exhaustive]
    Drm {
        /// The drm file descriptor.
        // TODO: Use `std::os::fd::RawFd`?
        fd: i32,
    },

    /// A raw display handle for the Linux Generic Buffer Manager.
    ///
    /// ## Thread safety
    ///
    /// GBM devices are not bound to a single thread; however, they are not
    /// internally secured by mutexes and cannot be used by multiple threads at
    /// once. Therefore this type is `Send` but not `Sync`. This means it can be
    /// sent to other threads but not used from other threads.
    ///
    /// ## Availability Hints
    ///
    /// This variant is present regardless of windowing backend and likely to be used with
    /// EGL_MESA_platform_gbm or EGL_KHR_platform_gbm.
    #[non_exhaustive]
    Gbm {
        /// The gbm device.
        gbm_device: NonNull<c_void>,
    },

    /// A raw display handle for Win32.
    ///
    /// This can be used regardless of Windows window backend.
    ///
    /// ## Thread safety
    ///
    /// TODO.
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on Windows systems.
    #[non_exhaustive]
    Windows {
        // Empty for now.
    },

    /// A raw display handle for the Web.
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
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
    #[non_exhaustive]
    Web {
        // Empty for now, logically `!Send` + `!Sync`.
    },

    /// A raw display handle for Android NDK.
    ///
    /// ## Thread safety
    ///
    /// Android native objects are thread-safe by default; therefore this type is
    /// `Send` and `Sync`. This means that this variant can be sent to or used from
    /// any thread.
    ///
    /// Note that this variant does not contain any Android native objects. However,
    /// it is kept `Send` and `Sync` for the event that Android native objects are
    /// added to this type.
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on Android targets.
    #[non_exhaustive]
    Android {
        // Empty for now.
    },

    /// A raw display handle for Haiku.
    ///
    /// ## Thread Safety
    ///
    /// Haiku objects are protected by a [global lock]. They are `Send` and `Sync`
    /// as long as producers/downstream consumers take this lock before the `BLooper`
    /// or `BWindow` are used outside of their origin threads.
    ///
    /// Note that this type does not currently contain any Haiku objects. However,
    /// it is kept `Send` and `Sync` for the event that Haiku objects are added to
    /// this type.
    ///
    /// [global lock]: https://grok.nikisoft.one/opengrok/xref/haiku/src/kits/app/Looper.cpp?r=b47e8b0cadeb9a9d985d7f72d2e9a099cbcb8f90#591-627
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on HaikuOS.
    #[non_exhaustive]
    Haiku {
        // Empty for now.
    },
}

impl RawDisplayHandle {
    /// Create a new empty display handle.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::RawDisplayHandle;
    /// let handle = RawDisplayHandle::new_uikit();
    /// ```
    pub fn new_uikit() -> Self {
        Self::UiKit {}
    }

    /// Create a new empty display handle.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::RawDisplayHandle;
    /// let handle = RawDisplayHandle::new_appkit();
    /// ```
    pub fn new_appkit() -> Self {
        Self::AppKit {}
    }

    /// Create a new empty display handle.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::RawDisplayHandle;
    /// let handle = RawDisplayHandle::new_orbital();
    /// ```
    pub fn new_orbital() -> Self {
        Self::Orbital {}
    }

    /// Create a new empty display handle.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::RawDisplayHandle;
    /// let handle = RawDisplayHandle::new_ohos();
    /// ```
    pub fn new_ohos() -> Self {
        Self::Ohos {}
    }

    /// Create a new handle to a display.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::RawDisplayHandle;
    /// #
    /// let display: NonNull<c_void>;
    /// let screen;
    /// # display = NonNull::from(&()).cast();
    /// # screen = 0;
    /// let handle = RawDisplayHandle::new_xlib(Some(display), screen);
    /// ```
    pub fn new_xlib(display: Option<NonNull<c_void>>, screen: c_int) -> Self {
        Self::Xlib { display, screen }
    }

    /// Create a new handle to a connection and screen.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::RawDisplayHandle;
    /// #
    /// let connection: NonNull<c_void>;
    /// let screen;
    /// # connection = NonNull::from(&()).cast();
    /// # screen = 0;
    /// let handle = RawDisplayHandle::new_xcb(Some(connection), screen);
    /// ```
    pub fn new_xcb(connection: Option<NonNull<c_void>>, screen: c_int) -> Self {
        Self::Xcb { connection, screen }
    }

    /// Create a new display handle.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::RawDisplayHandle;
    /// #
    /// let display: NonNull<c_void>;
    /// # display = NonNull::from(&()).cast();
    /// let handle = RawDisplayHandle::new_wayland(display);
    /// ```
    pub fn new_wayland(display: NonNull<c_void>) -> Self {
        Self::Wayland { display }
    }

    /// Create a new handle to a file descriptor.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::RawDisplayHandle;
    /// #
    /// let fd: i32;
    /// # fd = 0;
    /// let handle = RawDisplayHandle::new_drm(fd);
    /// ```
    pub fn new_drm(fd: i32) -> Self {
        Self::Drm { fd }
    }

    /// Create a new handle to a device.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::RawDisplayHandle;
    /// #
    /// let ptr: NonNull<c_void>;
    /// # ptr = NonNull::from(&()).cast();
    /// let handle = RawDisplayHandle::new_gbm(ptr);
    /// ```
    pub fn new_gbm(gbm_device: NonNull<c_void>) -> Self {
        Self::Gbm { gbm_device }
    }

    /// Create a new empty display handle.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::RawDisplayHandle;
    /// let handle = RawDisplayHandle::new_windows();
    /// ```
    pub fn new_windows() -> Self {
        Self::Windows {}
    }

    /// Create a new empty display handle.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::RawDisplayHandle;
    /// let handle = RawDisplayHandle::new_web();
    /// ```
    pub fn new_web() -> Self {
        Self::Web {}
    }

    /// Create a new empty display handle.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::RawDisplayHandle;
    /// let handle = RawDisplayHandle::new_android();
    /// ```
    pub fn new_android() -> Self {
        Self::Android {}
    }

    /// Create a new empty display handle.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::RawDisplayHandle;
    /// let handle = RawDisplayHandle::new_haiku();
    /// ```
    pub fn new_haiku() -> Self {
        Self::Haiku {}
    }
}
