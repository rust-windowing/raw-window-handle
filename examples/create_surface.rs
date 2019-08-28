use raw_window_handle::HasRawWindowHandle;

fn main() {}

pub struct Instance {
    // ...
}

pub struct Surface;

pub enum Error {
    NotSupported,
}

impl Instance {
    pub fn create_surface<W: HasRawWindowHandle>(&mut self, window: &W) -> Result<Surface, Error> {
        #[cfg(target_os = "linux")]
        let surface = self.create_surface_for_unix(window)?;

        #[cfg(target_os = "macos")]
        let surface = self.create_surface_for_macos(window)?;

        // It will intentionally not compile in other platforms

        Ok(surface)
    }

    #[cfg(target_os = "linux")]
    fn create_surface_for_unix<W: HasRawWindowHandle>(
        &mut self,
        window: &W,
    ) -> Result<Surface, Error> {
        if let Some(x11_handle) = window.x11() {
            let _window_ptr = x11_handle.window();

            // ...

            Ok(Surface)
        } else if let Some(wayland_handle) = window.wayland() {
            let _surface_ptr = wayland_handle.surface();

            // ...

            Ok(Surface)
        } else {
            Err(Error::NotSupported)
        }
    }

    #[cfg(target_os = "macos")]
    fn create_surface_for_macos<W: HasRawWindowHandle>(
        &mut self,
        window: &W,
    ) -> Result<Surface, Error> {
        let _ns_window = window.ns_window();

        // ...

        Ok(surface)
    }
}
