use bevy::prelude::*;

struct SpriteScale(Vec3);
fn main() {
    let window = WindowDescriptor {
        title: "Forrest".to_string(),
        width: 1088,
        height: 640,
        vsync: true,
        resizable: false,
        ..Default::default()
    };

    let scale: SpriteScale = SpriteScale(Vec3::new(4.0, 4.0, 4.0));

    App::build()
        .add_resource(window)
        .add_resource(scale)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scale: Res<SpriteScale>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let forest_back_trees = asset_server.load("parallax-forest-back-trees.png");
    let forest_middle_trees= asset_server.load("parallax-forest-middle-trees.png");
    let forest_front_trees = asset_server.load("parallax-forest-front-trees.png");
    let forest_lights = asset_server.load("parallax-forest-lights.png");

    let mut sprite = | handle: &Handle<Texture> | -> SpriteComponents {
        SpriteComponents {
            material: materials.add(handle.clone().into()),
            transform: Transform {
                scale: scale.0,
                ..Default::default()
            },
            ..Default::default()
        }
    };

    commands
        .spawn(Camera2dComponents::default())
        .spawn(sprite(&forest_back_trees))
        .spawn(sprite(&forest_lights))
        .spawn(sprite(&forest_middle_trees))
        .spawn(sprite(&forest_front_trees))
        ;
}
