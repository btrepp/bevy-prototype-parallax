mod layer;
mod window_size;
use bevy::prelude::*;

pub use layer::{Layer, LayerComponents};
pub use window_size::WindowSize;
/// The plugin that enables parallax backgrounds
/// Note you will still need to make sure you add a background entity
pub struct ParallaxPlugin;
impl Plugin for ParallaxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(layer::layer_movement_system.system());
        app.add_system(layer::children_count_system.system());
        app.add_system(layer::children_layout_system.system());
        app.add_system(window_size::window_size.system());
        app.add_system(layer::debug_system.system());
        //app.add_system(layer::print_sprints.system());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
