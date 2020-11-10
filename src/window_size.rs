use bevy::prelude::*;
///! Helpers for getting the current window size.
///! We need to know this so we can determine how much to repeat an image

/// Simple struct storing the height and width of the window.
/// Hopefully this may be integrated into bevy in future.
#[derive(Default)]
pub struct WindowSize {
    pub height: u32,
    pub width: u32,
}

/// Startup system that will set the window size struct based on the window descriptor
pub fn initial_window(window_desc: Res<WindowDescriptor>, mut window_size: ResMut<WindowSize>) {
    window_size.height = window_desc.height;
    window_size.width = window_desc.width;
}
