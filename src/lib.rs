#![no_std]

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub mod unix;
#[cfg(target_os = "windows")]
pub mod windows;
// pub mod android;
#[cfg(target_os = "ios")]
pub mod ios;
// pub mod wasm;

mod platform {
    #[cfg(target_os = "macos")]
    pub use crate::macos::*;
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub use crate::unix::*;
    #[cfg(target_os = "windows")]
    pub use crate::windows::*;
    // #[cfg(target_os = "android")]
    // #[path = "android/mod.rs"]
    // mod platform;
    #[cfg(target_os = "ios")]
    pub use crate::ios::*;
}

pub trait HasRawWindowHandle {
    fn raw_window_handle(&self) -> RawWindowHandle;
}

pub struct RawWindowHandle {
    handle: platform::RawWindowHandle,
}
