use bevy::{prelude::*, render::camera::Camera};
///! Helpers for getting the current window size.
///! We need to know this so we can determine how much to repeat an image

/// Simple struct storing the height and width of the window.
/// Hopefully this may be integrated into bevy in future.
#[derive(Default)]
pub struct WindowSize {
    pub height: f32,
    pub width: f32,
}

/// Syncs the window width to the camera
pub fn window_size(windows: Res<Windows>, mut camera_query: Query<(&Camera, &mut WindowSize)>) {
    for (cam, mut size) in camera_query.iter_mut() {
        if let Some(window) = windows.get(cam.window) {

            // Note: This conversion will be lossy, but u32 is used internally 
            // for texture calcs.
            let windowsize = WindowSize {
                height: window.height(), 
                width: window.width(),
            };
            *size = windowsize;
        }
    }
}
