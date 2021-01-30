use crate::window_size::WindowSize;
use bevy::{prelude::*, render::camera::Camera};

#[derive(Default, Debug)]
pub struct Layer {
    pub speed: f32,
}
#[derive(Bundle, Default)]
pub struct LayerComponents {
    pub layer: Layer,
    pub transform: Transform,
    pub global: GlobalTransform,
    pub children: Children,
    pub material: Handle<ColorMaterial>,
    pub sprite: Sprite,
}

/// Gets the 'screen' width of the sprite.
/// This takes into account the scaling
fn sprite_scaled_width(sprite: &Sprite, transform: &Transform) -> f32 {
    sprite.size[0] * transform.scale.x
}

/// Calculate the amount of sprites we need for the effect
fn desired_children_count(window: &WindowSize, sprite: &Sprite, transform: &Transform) -> u32 {
    let tex_width = sprite_scaled_width(sprite, transform) as u32;
    if tex_width > 0 {
        window.width.div_euclid(tex_width) + 2
    } else {
        0
    }
}

/// Caculates an offset to put the layer at the left edge of the 'container'
/// This is because the camera seems to center on 0.0
fn camera_left_edge_offset(window: &WindowSize) -> f32 {
    let left_side = 0.0 - window.width as f32 / 2.0;
    left_side
}

/// How far to offset the layer due to the camera position
/// Will be clamped by the sprite offset
fn camera_sprite_offset(
    camera: &Vec3,
    layer: &Layer,
    sprite: &Sprite,
    transform: &Transform,
) -> f32 {
    let sprite_width = sprite_scaled_width(sprite, transform);
    -(camera.x * layer.speed).rem_euclid(sprite_width)
}

/// Mutates the layer based on the camera position
/// this allows us to have the parallax effect by having the layers move at different rates
/// once we move past the width of the sprite, it resets to 0
fn move_layer_position(
    window: &WindowSize,
    camera: &Vec3,
    sprite: &Sprite,
    layer: &Layer,
    transform: &mut Transform,
) -> () {
    let offset = camera_left_edge_offset(&window);
    let camera_x = camera_sprite_offset(camera, layer, sprite, transform);

    let translation = Vec3::new(
        offset + camera_x,
        transform.translation.y,
        transform.translation.z
    );

    *transform = Transform::from_translation(translation);
}

/// Manages the amount of child sprites we need to repeat
/// Based on the windows size
pub fn children_count_system(
    commands: &mut Commands,
    cameras_query: Query<(&Camera, &WindowSize, &Children)>,
    mut layer_query: Query<(
        Entity,
        &Parent,
        &Children,
        &Sprite,
        &Handle<ColorMaterial>,
        &Transform,
    ), With<(Layer, Entity)>>,
) -> () {
    for (entity, parent, children, sprite, material, transform) in layer_query.iter_mut() {
        if let Ok(window) = cameras_query.get_component(parent.0) {
            let desired_children = desired_children_count(&window, &sprite, &transform);
            let current_children = children.len();
            let to_add = desired_children as usize - current_children;

            for _ in 0..to_add {
                let child = SpriteBundle {
                    material: material.clone(),
                    sprite: Sprite::default(),
                    ..Default::default()
                };

                commands.spawn(child).with(Parent(entity));
            }

            // TODO: remove sprites if they aren't needed
        }
    }
}

/// Responsible for setting the positioning of the sprites
pub fn children_layout_system(
    layers: Query<(&Sprite, &Children), With<Layer>>,
    mut sprites: Query<&mut Transform>,
) {
    for (sprite, children) in layers.iter() {
        for (index, child) in children.iter().enumerate() {
            if let Ok(mut transform) = sprites.get_component_mut::<Transform>(*child) {
                let translation = Vec3::new(
                    index as f32 * sprite_scaled_width(sprite, &transform),
                    transform.translation.y,
                    -999.0
                );

                *transform = Transform::from_translation(translation);
            }
        }
    }
}

/// Matches the layer to the camera.
/// Note the layer is offset to the left by half the window to make
pub fn layer_movement_system(
    cameras: Query<(&Transform, &WindowSize, &Children), With<Camera>>,
    mut layers: Query<(&Layer, &Sprite, &mut Transform)>,
) -> () {
    for (transform, window, children) in cameras.iter() {
        let camera = transform.translation;
        for child in children.iter() {
            if let Ok((layer, sprite, mut trans)) = layers.get_mut(*child) {
                move_layer_position(window, &camera, sprite, layer, &mut trans);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        width,
        expected,
        case(1024, -512.0),
        case(1000, -500.0)
    )]
    fn test_left_edge(width: u32, expected: f32) {
        let window = WindowSize {
            height: 576,
            width: width,
        };
        let result = camera_left_edge_offset(&window);
        assert_eq!(expected, result);
    }

    #[rstest(
        camera,
        speed,
        sprite,
        expected,
        case(0.0, 1.0,100.0, 0.0),
        case(1.0, 1.0,100.0, -1.0),
        case(101.0, 1.0, 100.0, -1.0),
        case(200.0, 1.0, 100.0, 0.0),
        case(220.0, 1.0, 100.0, -20.0)
        ::trace
    )]
    fn test_layer_offset(camera: f32, speed: f32, sprite: f32, expected: f32) {
        let camera = Vec3::new(camera, 0.0, 0.0);
        let sprite = Sprite::new(Vec2::splat(sprite));
        let transform = Transform::default();
        let layer = Layer { speed };
        let result = camera_sprite_offset(&camera, &layer, &sprite, &transform);
        assert_eq!(expected, result);
    }

    #[rstest(
        sprite,
        scale,
        expected,
        case(100.0, 1.0, 100.0),
        case(100.0, 2.0, 200.0),
        case(100.0, 0.0, 0.0),
        case(512.0, 1.0, 512.0)
        ::trace
    )]
    fn test_scaled_width(sprite: f32, scale: f32, expected: f32) {
        let transform = Transform {
            scale: Vec3::splat(scale),
            ..Default::default()
        };
        let sprite = Sprite::new(Vec2::splat(sprite));
        let result = sprite_scaled_width(&sprite, &transform);
        assert_eq!(expected, result);
    }

    #[rstest(
        screen,
        texture,
        scale,
        expected,
        case(1024, 100, 1.0,12),
        case(1024, 1025,1.0, 2),
        case(1024, 800, 1.0,3),
        case(1024, 0, 1.0,0)
        ::trace
    )]
    fn test_desired_children_count(screen: u32, texture: u32, scale: f32, expected: u32) {
        let window = WindowSize {
            height: 576,
            width: screen,
        };

        let transform = Transform {
            scale: Vec3::splat(scale),
            ..Default::default()
        };

        let texture = Sprite::new(Vec2::new(texture as f32, window.height as f32));
        let result = desired_children_count(&window, &texture, &transform);
        assert_eq!(expected, result);
    }

    #[rstest(
        screen,camera,sprite,speed,expected,
        case(1024,0.0, 512.0,0.0,-512.0),
        case(1024,1.0, 512.0,0.0,-512.0),
        case(1024,512.0, 512.0,1.0,-512.0),
        case(1024,513.0, 512.0,1.0,-513.0),
        case(1024,1.0, 512.0,1.0,-513.0),
        case(1024,2.0, 512.0,0.5,-513.0),
        case(1024,1024.0, 512.0,1.0,-512.0)
        ::trace
    )]
    fn test_layer_translation(screen: u32, camera: f32, sprite: f32, speed: f32, expected: f32) {
        let window_size = WindowSize {
            height: 576,
            width: screen,
        };

        let camera = Vec3::new(camera, 0.0, 0.0);
        let speed = Layer { speed };
        let sprite = Sprite::new(Vec2::new(sprite, window_size.height as f32));
        let mut transform = Transform::default();
        move_layer_position(&window_size, &camera, &sprite, &speed, &mut transform);
        assert_eq!(expected, transform.translation.x());
    }
}
