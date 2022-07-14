# Changelog

## 0.5.0 (2022-07-14)

* **Breaking:** The `RawWindowHandle` variants were split into `RawDisplayHandle` and `RawWindowHandle`.
* The X11 screen is now present in new `XlibDisplayHandle` and `XcbDisplayHandle`.
- Add GBM support.

## 0.4.3 (2022-03-29)

* [Add visual IDs to X11 handles](https://github.com/rust-windowing/raw-window-handle/pull/83)
* [Add a link to the MDN page for data attributes in the documentation for WebHandle](https://github.com/rust-windowing/raw-window-handle/pull/86)
* [add haiku support](https://github.com/rust-windowing/raw-window-handle/pull/88)

## 0.4.2 (2021-11-24)

* Also implement `HasRawWindowHandle` for `Rc<T>`, and `Arc<T>` where `T: ?Sized`.

## 0.4.1 (2021-11-19)

* Added an impl of `HasRawWindowHandle` for `&T`, `Rc<T>`, and `Arc<T>`. The impls for `Rc<T>` and `Arc<T>` require the `alloc` feature.

## 0.4.0 (2021-11-15)

* **Breaking:** Remove `_do_not_use` tags to use `#[non_exhaustive]` macro
* **Breaking:** `RawWindowHandle` variants are no longer cfg-guarded by platform. 
* **Breaking:** Rename `IOS` to `UiKit`.
* **Breaking:** Rename `MacOS` to `AppKit`.
* **Breaking:** Rename `Windows` to `Win32`.
* **Breaking:** Rename `Redox` to `Orbital`.
* **Breaking:** Rename `Android` to `AndroidNdk`.
* **Breaking:** Inner window handle structs are now exported at crate root.
* Added Windows `WinRt` handle.

# 0.3.4 (2021-11-27)

* Add `HasRawWindowHandle` implementation for `HasRawWindowHandle` in the
  newer `v0.4`.
  This allows "provider" crates that implement `HasRawWindowHandle` (like
  `winit`, `sdl2`, `glfw`, `fltk`, ...) to upgrade to `v0.4` without a
  breaking change.
  Afterwards "consumer" crates (like `gfx`, `wgpu`, `rfd`, ...) can start
  upgrading with minimal breakage for their users.

## 0.3.3 (2019-12-1)

* Add missing `Hash` implementation for `AndroidHandle`.

## 0.3.2 (2019-11-29)

* Add `Hash` implementation for `RawWindowHandle`.

## 0.3.1 (2019-10-27)

* Remove `RawWindowHandle`'s `HasRawWindowHandle` implementation, as it was unsound (see [#35](https://github.com/rust-windowing/raw-window-handle/issues/35))
* Explicitly require that handles within `RawWindowHandle` be valid for the lifetime of the `HasRawWindowHandle` implementation that provided them.

## 0.3.0 (2019-10-5)

* **Breaking:** Rename `XLib.surface` to `XLib.window`, as that more accurately represents the underlying type.
* Implement `HasRawWindowHandle` for `RawWindowHandle`
* Add `HINSTANCE` field to `WindowsHandle`.

## 0.2.0 (2019-09-26)

* **Breaking:** Rename `X11` to `XLib`.
* Add XCB support.
* Add Web support.
* Add Android support.

## 0.1.2 (2019-08-13)

* Fix use of private `_non_exhaustive` field in platform handle structs preventing structs from getting initialized.

## 0.1.1 (2019-08-13)

* Flesh out Cargo.toml, adding crates.io info rendering tags.

## 0.1.0 (2019-08-13)

* Initial release.
