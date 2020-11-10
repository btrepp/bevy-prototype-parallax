mod window_size;
use bevy::prelude::*;

pub use window_size::WindowSize;
/// The plugin that enables parallax backgrounds
/// Note you will still need to make sure you add a background entity
pub struct ParallaxPlugin;
impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(window_size::WindowSize::default());
        app.add_startup_system(window_size::initial_window.system());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
