use core::{
    ffi::{c_ulong, c_void},
    num::NonZeroU32,
    ptr::NonNull,
};

/// A window handle for a particular windowing system.
///
/// Each variant contains fields specific to that windowing system (e.g. [`RawWindowHandle::Win32`]
/// contains a [HWND], [`RawWindowHandle::Wayland`] uses [wl_surface], etc.)
///
/// [HWND]: https://learn.microsoft.com/en-us/windows/win32/winmsg/about-windows#window-handle
/// [wl_surface]: https://wayland.freedesktop.org/docs/html/apa.html#protocol-spec-wl_surface
///
/// ## Thread safety
///
/// See individual variants for thread safety documentation. Since some window
/// handles are `!Send` and `!Sync`, this sum type is as well.
///
/// # Variant Availability
///
/// Note that all variants are present on all targets (none are disabled behind
/// `#[cfg]`s), but see the "Availability Hints" section on each variant for
/// some hints on where this variant might be expected.
///
/// Note that these "Availability Hints" are not normative. That is to say, a
/// [`HasWindowHandle`] implementor is completely allowed to return something
/// unexpected. (For example, it's legal for someone to return a
/// [`RawWindowHandle::Xlib`] on macOS, it would just be weird, and probably
/// requires something like XQuartz be used).
///
/// [`HasWindowHandle`]: crate::HasWindowHandle
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RawWindowHandle {
    /// A raw window handle for UIKit (Apple's non-macOS windowing library).
    ///
    /// ## Thread safety
    ///
    /// Handles to UIKit objects can only be safely used from the main thread.
    /// Therefore, all UIKit objects are `!Send` and `!Sync`.
    /// This means that this variant cannot be sent to or used from other threads.
    ///
    /// In addition, it is also expected that the consumer will take precautions to
    /// ensure that this object is only used on the main thread.
    /// It is recommended to use [`objc2::MainThreadMarker`] as a strategy for
    /// ensuring this.
    ///
    /// [`objc2::MainThreadMarker`]: https://docs.rs/objc2/latest/objc2/struct.MainThreadMarker.html
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
        /// A pointer to an `UIView` object.
        ///
        /// # Example
        ///
        /// Getting the view from a [`WindowHandle`][crate::WindowHandle].
        ///
        /// ```no_run
        /// # #[cfg(not(all(target_vendor = "apple", not(target_os = "macos"))))]
        /// # fn main() {}
        /// # fn main() {
        /// #![cfg(all(target_vendor = "apple", not(target_os = "macos")))]
        /// use objc2::MainThreadMarker;
        /// use objc2::rc::Retained;
        /// use objc2_ui_kit::UIView;
        /// use raw_window_handle::{WindowHandle, RawWindowHandle};
        ///
        /// let handle: WindowHandle<'_>; // Get the window handle from somewhere else
        /// # handle = unimplemented!();
        /// match handle.as_raw() {
        ///     RawWindowHandle::UiKit { ui_view, .. } => {
        ///         assert!(MainThreadMarker::new().is_some(), "can only access UIKit handles on the main thread");
        ///         // SAFETY: The pointer came from `WindowHandle`, which ensures
        ///         // that the `ui_view` contains a valid pointer to an `UIView`.
        ///         // Unwrap is fine, since the pointer came from `NonNull`.
        ///         let ui_view: Retained<UIView> = unsafe { Retained::retain(ui_view.as_ptr().cast()) }.unwrap();
        ///         // Do something with the UIView here.
        ///     }
        ///     handle => unreachable!("unknown handle {handle:?} for platform"),
        /// }
        /// # }
        /// ```
        ///
        /// Get a pointer to an `UIViewController` object by traversing the `UIView`'s responder
        /// chain:
        ///
        /// ```
        /// # #[cfg(not(all(target_vendor = "apple", not(target_os = "macos"))))]
        /// # fn main() {}
        /// # fn main() {
        /// #![cfg(all(target_vendor = "apple", not(target_os = "macos")))]
        /// use objc2::rc::Retained;
        /// use objc2_ui_kit::{UIResponder, UIView, UIViewController};
        ///
        /// // View gotten from somewhere (e.g. as in the example above).
        /// let view: Retained<UIView>;
        /// # view = unsafe { objc2_ui_kit::UIView::new(objc2::MainThreadMarker::new().unwrap()) };
        ///
        /// let mut current_responder: Retained<UIResponder> = view.into_super();
        /// let mut found_controller = None;
        /// while let Some(responder) = unsafe { current_responder.nextResponder() } {
        ///     match responder.downcast::<UIViewController>() {
        ///         Ok(controller) => {
        ///             found_controller = Some(controller);
        ///             break;
        ///         }
        ///         // Search next.
        ///         Err(responder) => current_responder = responder,
        ///     }
        /// }
        ///
        /// // Use found_controller here.
        /// # }
        /// ```
        ui_view: NonNull<c_void>,
    },

    /// A raw window handle for AppKit.
    ///
    /// ## Thread safety
    ///
    /// Handles to AppKit objects can only be safely used from the main thread.
    /// Therefore, all AppKit objects are `!Send` and `!Sync`.
    /// This means that this variant cannot be sent to or used from other threads.
    ///
    /// In addition, it is also expected that the consumer will take precautions to
    /// ensure that this object is only used on the main thread.
    /// It is recommended to use [`objc2::MainThreadMarker`] as a strategy for
    /// ensuring this.
    ///
    /// [`objc2::MainThreadMarker`]: https://docs.rs/objc2/latest/objc2/struct.MainThreadMarker.html
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on macOS, although Mac Catalyst can also use it
    /// despite being `target_os = "ios"`.
    #[non_exhaustive]
    AppKit {
        /// A pointer to an `NSView` object.
        ///
        /// # Example
        ///
        /// Getting the view from a [`WindowHandle`][crate::WindowHandle].
        ///
        /// ```no_run
        /// # #[cfg(not(target_os = "macos"))]
        /// # fn main() {}
        /// # fn main() {
        /// #![cfg(target_os = "macos")]
        /// use objc2::MainThreadMarker;
        /// use objc2::rc::Retained;
        /// use objc2_app_kit::NSView;
        /// use raw_window_handle::{WindowHandle, RawWindowHandle};
        ///
        /// let handle: WindowHandle<'_>; // Get the window handle from somewhere
        /// # handle = unimplemented!();
        /// match handle.as_raw() {
        ///     RawWindowHandle::AppKit { ns_view, ..} => {
        ///         assert!(MainThreadMarker::new().is_some(), "can only access AppKit handles on the main thread");
        ///         // SAFETY: The pointer came from `WindowHandle`, which ensures
        ///         // that the `ns_view` contains a valid pointer to an `NSView`.
        ///         // Unwrap is fine, since the pointer came from `NonNull`.
        ///         let ns_view: Retained<NSView> = unsafe { Retained::retain(ns_view.as_ptr().cast()) }.unwrap();
        ///         // Do something with the NSView here, like getting the `NSWindow`
        ///         let ns_window = ns_view.window().expect("view was not installed in a window");
        ///     }
        ///     handle => unreachable!("unknown handle {handle:?} for platform"),
        /// }
        /// # }
        /// ```
        ns_view: NonNull<c_void>,
    },

    /// A raw window handle for the Redox operating system.
    ///
    /// ## Thread safety
    ///
    /// The underlying window is a [file descriptor], and most calls on the window
    /// correspond directly to non-mutating file descriptor reads and writes.
    /// This means that this variant can be sent to and used from other threads.
    ///
    /// [file descriptor]: https://github.com/redox-os/orbclient/blob/77c28e88fcb180c750175f2dcf5c7342d357ab26/src/sys/orbital.rs#L64-L65
    ///
    /// ## Availability Hints
    ///
    /// This variant is used by the Orbital Windowing System in the Redox
    /// operating system.
    #[non_exhaustive]
    Orbital {
        /// A pointer to an orbclient window.
        // TODO(madsmtm): I think this is a file descriptor, so perhaps it should
        // actually use `std::os::fd::RawFd`, or some sort of integer instead?
        window: NonNull<c_void>,
    },

    /// A raw window handle for the OpenHarmony OS NDK.
    ///
    /// ## Background
    ///
    /// Applications on [OpenHarmony] use [ArkUI] for defining their UI. Applications can use an
    /// [XComponent] to render using native Code (e.g. Rust) via EGL.
    /// Native code will receive a callback `OnSurfaceCreatedCB(OH_NativeXComponent *component, void *window)`
    /// when the `XComponent` is created. The window argument has the type [`OHNativeWindow`] / `EGLNativeWindowType`.
    /// The window can then be used to create a surface with
    /// `eglCreateWindowSurface(eglDisplay_, eglConfig_, window, NULL)`
    ///
    /// [OpenHarmony]: https://gitee.com/openharmony/docs/blob/master/en/OpenHarmony-Overview.md
    /// [ArkUI]: https://gitee.com/openharmony/docs/blob/master/en/application-dev/ui/arkui-overview.md
    /// [XComponent]: https://gitee.com/openharmony/docs/blob/master/en/application-dev/ui/arkts-common-components-xcomponent.md
    /// [`OHNativeWindow`]: https://gitee.com/openharmony/docs/blob/master/en/application-dev/reference/apis-arkgraphics2d/_native_window.md
    ///
    /// ## Thread safety
    ///
    /// OpenHarmony [expects] that UI primitives will only be called from one
    /// thread. Therefore, all OHOS objects are `!Send` and `!Sync`. This means
    /// that this type cannot be sent to or used from other threads.
    ///
    /// [expects]: https://ai6s.net/6921b48882fbe0098cade00f.html
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on OpenHarmony OS (`target_env = "ohos"`).
    #[non_exhaustive]
    OhosNdk {
        /// An [`OHNativeWindow`].
        ///
        /// [`OHNativeWindow`]: https://gitee.com/openharmony/docs/blob/master/en/application-dev/reference/apis-arkgraphics2d/_native_window.md
        native_window: NonNull<c_void>,
    },

    /// A raw window handle for Xlib.
    ///
    /// ## Thread safety
    ///
    /// This type is nothing more than a numeric identifier, therefore it is `Send`
    /// and `Sync`. This means it can be safely sent to or used from other threads.
    ///
    /// ## Availability Hints
    ///
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that Xlib can be built for, which is to say, most (but not all)
    /// Unix systems.
    #[non_exhaustive]
    Xlib {
        /// An Xlib `Window`.
        window: c_ulong,
        /// An Xlib visual ID, or 0 if unknown.
        visual_id: c_ulong,
    },

    /// A raw window handle for Xcb.
    ///
    /// ## Thread safety
    ///
    /// This type is nothing more than a numeric identifier, therefore it is `Send`
    /// and `Sync`. This means it can be safely sent to or used from other threads.
    ///
    /// ## Availability Hints
    ///
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that XCB can be built for, which is to say, most (but not all)
    /// Unix systems.
    #[non_exhaustive]
    Xcb {
        /// An X11 `xcb_window_t`.
        window: NonZeroU32, // Based on xproto.h
        /// An X11 `xcb_visualid_t`.
        visual_id: Option<NonZeroU32>,
    },

    /// A raw window handle for Wayland.
    ///
    /// ## Thread safety
    ///
    /// `libwayland-client` is thread safe, so this variant is `Send` + `Sync` too.
    ///
    /// ## Availability Hints
    ///
    /// This variant should be expected anywhere Wayland works, which is
    /// currently some subset of unix systems.
    #[non_exhaustive]
    Wayland {
        /// A pointer to a `wl_surface`.
        surface: NonNull<c_void>,
    },

    /// A raw window handle for the Linux Kernel Mode Set/Direct Rendering Manager
    ///
    /// ## Thread safety
    ///
    /// DRM "windows" are just planes, which are just numbers, therefore it is `Send`
    /// and `Sync`. This means that it can be sent to or used from other threads.
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on Linux when neither X nor Wayland are available
    #[non_exhaustive]
    Drm {
        /// The primary drm plane handle.
        plane: u32,
    },

    /// A raw window handle for the Linux Generic Buffer Manager.
    ///
    /// ## Thread safety
    ///
    /// GBM surfaces are not bound to a single thread; however, they are not
    /// internally secured by mutexes and cannot be used by multiple threads at
    /// once. You can view that as this type being `Send` but not `Sync`. This
    /// means it can be sent to other threads but not used from other threads.
    ///
    /// ## Availability Hints
    ///
    /// This variant is present regardless of windowing backend and likely to be used with
    /// EGL_MESA_platform_gbm or EGL_KHR_platform_gbm.
    #[non_exhaustive]
    Gbm {
        /// The gbm surface.
        gbm_surface: NonNull<c_void>,
    },

    /// A raw window handle for Win32.
    ///
    /// ## Thread safety
    ///
    /// Window objects have [thread affinity]. Some functions read or modify window
    /// state non-atomically, making them unsafe to call from threads other than
    /// the one that created the window. When in doubt, only run the function on
    /// the thread the window object was created on.
    ///
    /// [thread affinity]: https://devblogs.microsoft.com/oldnewthing/20051010-09/?p=33843
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on Windows systems.
    #[non_exhaustive]
    Win32 {
        /// A Win32 `HWND` handle.
        hwnd: NonNull<c_void>,
        /// The `GWLP_HINSTANCE` associated with this type's `HWND`.
        hinstance: Option<NonNull<c_void>>,
    },

    /// A raw window handle for WinRT.
    ///
    /// ## Thread safety
    ///
    /// TODO.
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on Windows systems.
    #[non_exhaustive]
    WinRt {
        /// A WinRT `CoreWindow` handle.
        core_window: NonNull<c_void>,
    },

    /// A raw window handle for a Web canvas registered via [`wasm-bindgen`].
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    ///
    /// ## Thread safety
    ///
    /// WASM objects are usually bound to the main UI "thread" belonging to the
    /// top-level webpage. Logically this variant is `!Send` and `!Sync`. It
    /// cannot be sent to or used from other threads.
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
    #[non_exhaustive]
    WebCanvas {
        /// An inner index of the [`JsValue`] of an [`HtmlCanvasElement`].
        ///
        /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
        /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
        ///
        /// # Example
        ///
        /// ```no_run
        /// # #[cfg(not(target_family = "wasm"))]
        /// # fn main() {}
        /// # fn main() {
        /// #![cfg(target_family = "wasm")]
        /// use core::mem::ManuallyDrop;
        /// use raw_window_handle::{WindowHandle, RawWindowHandle};
        /// use wasm_bindgen::convert::RefFromWasmAbi;
        /// use web_sys::HtmlCanvasElement;
        ///
        /// let handle: WindowHandle<'_>; // Get the window handle from somewhere
        /// # handle = unimplemented!();
        /// match handle.as_raw() {
        ///     RawWindowHandle::WebCanvas { obj, ..} => {
        ///         // To get the canvas element back, convert the index back.
        ///         let element: ManuallyDrop<HtmlCanvasElement> = unsafe {
        ///             HtmlCanvasElement::ref_from_abi(obj as u32)
        ///         };
        ///     }
        ///     _ => todo!(),
        /// }
        /// # }
        /// ```
        obj: usize,
        // Logically, this variant is `!Send` and `!Sync`, even though it contains only `usize`.
    },

    /// A raw window handle for a Web offscreen canvas registered via [`wasm-bindgen`].
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    ///
    /// ## Thread safety
    ///
    /// WASM objects are usually bound to the main UI "thread" belonging to the
    /// top-level webpage. Logically this variant is `!Send` and `!Sync`. It
    /// cannot be sent to or used from other threads.
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
    #[non_exhaustive]
    WebOffscreenCanvas {
        /// An inner index of the [`JsValue`] of an [`OffscreenCanvas`].
        ///
        /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
        /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
        ///
        /// # Example
        ///
        /// ```no_run
        /// # #[cfg(not(target_family = "wasm"))]
        /// # fn main() {}
        /// # fn main() {
        /// #![cfg(target_family = "wasm")]
        /// use core::mem::ManuallyDrop;
        /// use raw_window_handle::{WindowHandle, RawWindowHandle};
        /// use wasm_bindgen::convert::RefFromWasmAbi;
        /// use web_sys::OffscreenCanvas;
        ///
        /// let handle: WindowHandle<'_>; // Get the window handle from somewhere
        /// # handle = unimplemented!();
        /// match handle.as_raw() {
        ///     RawWindowHandle::WebOffscreenCanvas { obj, ..} => {
        ///         // To get the canvas element back, convert the index back.
        ///         let element: ManuallyDrop<OffscreenCanvas> = unsafe {
        ///             OffscreenCanvas::ref_from_abi(obj as u32)
        ///         };
        ///     }
        ///     _ => todo!(),
        /// }
        /// # }
        /// ```
        obj: usize,
        // Logically, this variant is `!Send` and `!Sync`, even though it contains only `usize`.
    },

    /// A raw window handle for Android NDK.
    ///
    /// ## Thread safety
    ///
    /// Android native objects are thread-safe by default, therefore it is `Send`
    /// and `Sync`. This means that this variant can be sent to or used from
    /// any thread.
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on Android targets.
    #[non_exhaustive]
    AndroidNdk {
        /// A pointer to an `ANativeWindow`.
        a_native_window: NonNull<c_void>,
    },

    /// A raw window handle for Haiku.
    ///
    /// ## Thread safety
    ///
    /// Haiku objects are protected by a [global lock]. They are `Send` and `Sync`
    /// as long as producers/downstream consumers take this lock before the `BLooper`
    /// or `BWindow` are used outside of their origin threads.
    ///
    /// [global lock]: https://grok.nikisoft.one/opengrok/xref/haiku/src/kits/app/Looper.cpp?r=b47e8b0cadeb9a9d985d7f72d2e9a099cbcb8f90#591-627
    ///
    /// ## Availability Hints
    ///
    /// This variant is used on HaikuOS.
    #[non_exhaustive]
    Haiku {
        /// A pointer to a `BWindow` object.
        b_window: NonNull<c_void>,
        /// A pointer to a `BDirectWindow` object that might be null.
        b_direct_window: Option<NonNull<c_void>>,
    },
}

impl RawWindowHandle {
    /// Create a new handle to a UIView.
    ///
    /// # Example
    ///
    /// Create a handle from a `UIView`.
    ///
    /// ```
    /// # #[cfg(not(all(target_vendor = "apple", not(target_os = "macos"))))]
    /// # fn main() {}
    /// # fn main() {
    /// #![cfg(all(target_vendor = "apple", not(target_os = "macos")))]
    /// use std::ptr::NonNull;
    /// use objc2::rc::Retained;
    /// use objc2_ui_kit::UIView;
    /// use raw_window_handle::RawWindowHandle;
    ///
    /// // UIView gotten from somewhere.
    /// let ui_view: Retained<UIView>;
    /// # ui_view = unsafe { objc2_ui_kit::UIView::new(objc2::MainThreadMarker::new().unwrap()) };
    ///
    /// // Pass it to raw-window-handle.
    /// let ui_view: NonNull<UIView> = NonNull::from(&*ui_view);
    /// let handle = RawWindowHandle::new_uikit(ui_view.cast());
    /// # }
    /// ```
    pub fn new_uikit(ui_view: NonNull<c_void>) -> Self {
        Self::UiKit { ui_view }
    }

    /// Create a new handle to a NSView.
    ///
    /// # Example
    ///
    /// Create a handle from the content view of a `NSWindow`.
    ///
    /// ```
    /// # #[cfg(not(target_os = "macos"))]
    /// # fn main() {}
    /// # fn main() {
    /// #![cfg(target_os = "macos")]
    /// use std::ptr::NonNull;
    /// use objc2::rc::Retained;
    /// use objc2_app_kit::{NSWindow, NSView};
    /// use raw_window_handle::RawWindowHandle;
    ///
    /// // NSWindow gotten from somewhere.
    /// let ns_window: Retained<NSWindow>;
    /// # ns_window = unsafe { objc2_app_kit::NSWindow::new(objc2::MainThreadMarker::new().unwrap()) };
    ///
    /// // Use the window's content view.
    /// let ns_view: Retained<NSView> = ns_window.contentView().unwrap();
    /// let ns_view: NonNull<NSView> = NonNull::from(&*ns_view);
    /// let handle = RawWindowHandle::new_appkit(ns_view.cast());
    /// # }
    /// ```
    pub fn new_appkit(ns_view: NonNull<c_void>) -> Self {
        Self::AppKit { ns_view }
    }

    /// Create a new handle to an orbclient window.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::RawWindowHandle;
    /// # type Window = ();
    /// #
    /// let window: NonNull<Window>;
    /// # window = NonNull::from(&());
    /// let handle = RawWindowHandle::new_orbital(window.cast());
    /// ```
    pub fn new_orbital(window: NonNull<c_void>) -> Self {
        Self::Orbital { window }
    }

    /// Create a new handle to an `OHNativeWindow`.
    ///
    /// The handle will typically be created from an [`XComponent`], consult the
    /// [native `XComponent` Guidelines] for more details.
    ///
    /// [`XComponent`]: https://gitee.com/openharmony/docs/blob/master/en/application-dev/ui/arkts-common-components-xcomponent.md
    /// [native `XComponent` Guidelines]: https://gitee.com/openharmony/docs/blob/OpenHarmony-4.0-Release/en/application-dev/napi/xcomponent-guidelines.md
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use core::ffi::c_void;
    /// # use raw_window_handle::RawWindowHandle;
    /// # #[allow(non_camel_case_types)]
    /// # type OH_NativeXComponent = ();
    ///
    /// /// Called When the `XComponent` is created.
    /// ///
    /// /// See the [XComponent Guidelines](https://gitee.com/openharmony/docs/blob/OpenHarmony-4.0-Release/en/application-dev/napi/xcomponent-guidelines.md)
    /// /// for more details
    /// extern "C" fn on_surface_created_callback(component: *mut OH_NativeXComponent, window: *mut c_void) {
    ///     let handle = RawWindowHandle::new_ohosndk(NonNull::new(window).unwrap());
    /// }
    /// ```
    pub fn new_ohosndk(native_window: NonNull<c_void>) -> Self {
        Self::OhosNdk { native_window }
    }

    /// Create a new handle to an Xlib window.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_ulong;
    /// # use raw_window_handle::RawWindowHandle;
    /// #
    /// let window: c_ulong;
    /// # window = 0;
    /// let mut handle = RawWindowHandle::new_xlib(window);
    /// // Optionally set the visual ID.
    /// if let RawWindowHandle::Xlib { visual_id, .. } = &mut handle {
    ///     *visual_id = 0;
    /// } else {
    ///     unreachable!();
    /// }
    /// ```
    pub fn new_xlib(window: c_ulong) -> Self {
        Self::Xlib {
            window,
            visual_id: 0,
        }
    }

    /// Create a new handle to an XCB window.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::num::NonZeroU32;
    /// # use raw_window_handle::RawWindowHandle;
    /// #
    /// let window: NonZeroU32;
    /// # window = NonZeroU32::new(1).unwrap();
    /// let mut handle = RawWindowHandle::new_xcb(window);
    /// // Optionally set the visual ID.
    /// if let RawWindowHandle::Xcb { visual_id, .. } = &mut handle {
    ///     *visual_id = None;
    /// } else {
    ///     unreachable!();
    /// }
    /// ```
    pub fn new_xcb(window: NonZeroU32) -> Self {
        Self::Xcb {
            window,
            visual_id: None,
        }
    }

    /// Create a new handle to a `wl_surface`.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::RawWindowHandle;
    /// #
    /// let surface: NonNull<c_void>;
    /// # surface = NonNull::from(&()).cast();
    /// let handle = RawWindowHandle::new_wayland(surface);
    /// ```
    pub fn new_wayland(surface: NonNull<c_void>) -> Self {
        Self::Wayland { surface }
    }

    /// Create a new handle to a DRM plane.
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::RawWindowHandle;
    /// #
    /// let plane: u32;
    /// # plane = 0;
    /// let handle = RawWindowHandle::new_drm(plane);
    /// ```
    pub fn new_drm(plane: u32) -> Self {
        Self::Drm { plane }
    }

    /// Create a new handle to a GBM surface.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::RawWindowHandle;
    /// #
    /// let ptr: NonNull<c_void>;
    /// # ptr = NonNull::from(&()).cast();
    /// let handle = RawWindowHandle::new_gbm(ptr);
    /// ```
    pub fn new_gbm(gbm_surface: NonNull<c_void>) -> Self {
        Self::Gbm { gbm_surface }
    }

    /// Create a new handle to a Win32 window.
    ///
    /// # Safety
    ///
    /// Some APIs taking a `HWND` must observe its thread-affinity.
    /// Consumers are responsible to ensure these safety guarantees themselves.
    /// See [`GetWindowThreadProcessId()`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid).
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::c_void;
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::RawWindowHandle;
    /// # struct HWND(*mut c_void);
    /// #
    /// let window: HWND;
    /// # window = HWND(1 as *mut c_void);
    /// let mut handle = RawWindowHandle::new_win32(NonNull::new(window.0).unwrap());
    /// // Optionally set the GWLP_HINSTANCE.
    /// if let RawWindowHandle::Win32 { hinstance, .. } = &mut handle {
    ///     # #[cfg(only_for_showcase)]
    ///     let hinst = NonNull::new(unsafe { GetWindowLongPtrW(window, GWLP_HINSTANCE) }).unwrap();
    ///     # let hinst = None;
    ///     *hinstance = hinst;
    /// } else {
    ///     unreachable!();
    /// }
    ///
    /// // On the other end we need to check if we are on the
    /// // right thread when using API calls that require it:
    /// # #[cfg(only_for_showcase)]
    /// unsafe { assert_eq!(GetWindowThreadProcessId(HWND(handle.hwnd.as_ptr()), None), GetCurrentThreadId()) };
    /// ```
    pub fn new_win32(hwnd: NonNull<c_void>) -> Self {
        Self::Win32 {
            hwnd,
            hinstance: None,
        }
    }

    /// Create a new handle to a WinRT window.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::RawWindowHandle;
    /// # type CoreWindow = ();
    /// #
    /// let window: NonNull<CoreWindow>;
    /// # window = NonNull::from(&());
    /// let handle = RawWindowHandle::new_winrt(window.cast());
    /// ```
    pub fn new_winrt(core_window: NonNull<c_void>) -> Self {
        Self::WinRt { core_window }
    }

    /// Create a new handle from a pointer to [`HtmlCanvasElement`].
    ///
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # fn main() {}
    /// # fn main() {
    /// #![cfg(target_family = "wasm")]
    /// use raw_window_handle::RawWindowHandle;
    /// use wasm_bindgen::convert::IntoWasmAbi;
    /// use web_sys::HtmlCanvasElement;
    ///
    /// let value: HtmlCanvasElement;
    /// # value = todo!();
    ///
    /// // Convert to the raw index and convert to the handle.
    /// let index = (&value).into_abi();
    /// let mut handle = RawWindowHandle::new_webcanvas(index as usize);
    /// # }
    /// ```
    pub fn new_webcanvas(obj: usize) -> Self {
        Self::WebCanvas { obj }
    }

    /// Create a new handle from a pointer to an [`OffscreenCanvas`].
    ///
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    ///
    /// # Example
    ///
    /// ```no_run
    /// # #[cfg(not(target_family = "wasm"))]
    /// # fn main() {}
    /// # fn main() {
    /// #![cfg(target_family = "wasm")]
    /// use raw_window_handle::RawWindowHandle;
    /// use wasm_bindgen::convert::IntoWasmAbi;
    /// use web_sys::OffscreenCanvas;
    ///
    /// let value: OffscreenCanvas;
    /// # value = todo!();
    ///
    /// // Convert to the raw index and convert to the handle.
    /// let index = (&value).into_abi();
    /// let handle = RawWindowHandle::new_weboffscreencanvas(index as usize);
    /// # }
    /// ```
    pub fn new_weboffscreencanvas(obj: usize) -> Self {
        Self::WebOffscreenCanvas { obj }
    }

    /// Create a new handle to an `ANativeWindow`.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::RawWindowHandle;
    /// # type ANativeWindow = ();
    /// #
    /// let ptr: NonNull<ANativeWindow>;
    /// # ptr = NonNull::from(&());
    /// let handle = RawWindowHandle::new_androidndk(ptr.cast());
    /// ```
    pub fn new_androidndk(a_native_window: NonNull<c_void>) -> Self {
        Self::AndroidNdk { a_native_window }
    }

    /// Create a new handle to a Haiku window.
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::RawWindowHandle;
    /// # type BWindow = ();
    /// #
    /// let b_window: NonNull<BWindow>;
    /// # b_window = NonNull::from(&());
    /// let mut handle = RawWindowHandle::new_haiku(b_window.cast());
    /// // Optionally set `b_direct_window`.
    /// if let RawWindowHandle::Haiku { b_direct_window, .. } = &mut handle {
    ///     *b_direct_window = None;
    /// } else {
    ///     unreachable!()
    /// }
    /// ```
    pub fn new_haiku(b_window: NonNull<c_void>) -> Self {
        Self::Haiku {
            b_window,
            b_direct_window: None,
        }
    }
}
