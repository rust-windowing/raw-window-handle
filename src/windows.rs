use libc::c_void;

/// Window that wraps around a Windows window handle.
pub trait HasWindowsHandle {
    fn hwnd(&self) -> *mut c_void;
}
