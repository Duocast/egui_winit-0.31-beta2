use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle};
use winit::error::RequestError;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes};

use crate::private::Sealed;

/// [`ActiveEventLoop`] is used to interact with the event loop.
pub trait GlutinEventLoop: Sealed {
    /// Create the window.
    ///
    /// See [`ActiveEventLoop::create_window`] for details.
    fn create_window(
        &self,
        window_attributes: WindowAttributes,
    ) -> Result<Box<dyn Window>, RequestError>;

    /// Get a handle to the display controller of the windowing system.
    fn glutin_display_handle(&self) -> Result<DisplayHandle<'_>, HandleError>;
}

impl<T: ActiveEventLoop + HasDisplayHandle + ?Sized> Sealed for T {}

impl<T: ActiveEventLoop + HasDisplayHandle + ?Sized> GlutinEventLoop for T {
    fn create_window(
        &self,
        window_attributes: WindowAttributes,
    ) -> Result<Box<dyn Window>, RequestError> {
        self.create_window(window_attributes)
    }

    fn glutin_display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.display_handle()
    }
}
