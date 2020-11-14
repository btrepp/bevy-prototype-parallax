use bevy::{prelude::*, render::camera::Camera};
use bevy_prototype_parallax::{Layer, LayerComponents, ParallaxPlugin, WindowSize};

struct SpriteScale(Vec3);
struct Player {
    pub run: Handle<TextureAtlas>,
    pub idle: Handle<TextureAtlas>,
}

fn main() {
    let window = WindowDescriptor {
        title: "Forrest".to_string(),
        width: 1088,
        height: 640,
        vsync: true,
        resizable: false,
        ..Default::default()
    };

    let scale: SpriteScale = SpriteScale(Vec3::splat(1.0));

    App::build()
        .add_resource(window)
        .add_resource(scale)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_parallax.system())
        .add_startup_system(setup_character.system())
        .add_system(move_character_system.system())
        .add_system(follow_player_camera.system())
        .add_system(animate_sprite_system.system())
        .add_plugin(ParallaxPlugin)
        .run();
}

/// Set up our background layers
fn setup_parallax(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scale: Res<SpriteScale>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Helper that loads an asset as a parallax layer
    // layers should have different speeds to achieve the effect
    let mut layer = |path: &str, speed: f32| -> LayerComponents {
        let handle = {
            let handle = asset_server.load(path);
            let color = materials.add(handle.into());
            color
        };
        LayerComponents {
            layer: Layer {
                speed: speed,
                ..Default::default()
            },
            material: handle,
            transform: Transform {
                scale: scale.0,
                translation: Vec3::new(0.0, 0.0, -1.0),
                ..Default::default()
            },
            ..Default::default()
        }
    };

    // Note the backgrounds are associated with a camera.
    commands
        .spawn(Camera2dComponents::default())
        .with(WindowSize::default())
        .with_children(|cb| {
            // Spawn the layers.
            // We can have as many as we like
            cb.spawn(layer("parallax-forest-back-trees.png", 0.0));
            cb.spawn(layer("parallax-forest-lights.png", 0.0));
            cb.spawn(layer("parallax-forest-middle-trees.png", 0.1));
            cb.spawn(layer("parallax-forest-front-trees.png", 0.2));
        });
}

/// Spawns our character and loads it's resources
fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player = {
        let texture_handle_run = asset_server.load("Run.png");
        let texture_atlas_run =
            TextureAtlas::from_grid(texture_handle_run, Vec2::new(24.0, 24.0), 8, 1);
        let texture_handle_idle = asset_server.load("Idle.png");
        let texture_atlas_idle =
            TextureAtlas::from_grid(texture_handle_idle, Vec2::new(24.0, 24.0), 8, 1);
        let run = texture_atlases.add(texture_atlas_run);
        let idle = texture_atlases.add(texture_atlas_idle);
        Player { run, idle }
    };

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: player.idle.clone(),
            transform: Transform {
                scale: Vec3::splat(10.0),
                translation: Vec3::new(0.0, -40.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(player);
}

/// From bevy examples, will animate the sprites in an atlas
fn animate_sprite_system(
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        if timer.finished {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

/// Moves the character and sets the appropriate atlas for animation
fn move_character_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform, &mut Handle<TextureAtlas>)>,
) {
    for (player, mut transform, mut atlas) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            *transform.translation.x_mut() += -1.0 * 5.0;
            *atlas = player.run.clone();
        } else if keyboard_input.pressed(KeyCode::D) {
            *transform.translation.x_mut() += 1.0 * 5.0;
            *atlas = player.run.clone();
        } else {
            *atlas = player.idle.clone();
        }
    }
}

/// A simple system that will cause the camera to follow the character
fn follow_player_camera(
    player: Query<With<Player, &Transform>>,
    mut camera: Query<With<Camera, &mut Transform>>,
) {
    if let Some(first_player) = player.iter().next() {
        for mut transform in camera.iter_mut() {
            *transform.translation.x_mut() = first_player.translation.x();
        }
    }
}
