#![allow(unused)] // temporal

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // position window
    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(3870, 4830));

    // spawn sprite
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1., 0.7, 0.7).into()),
            sprite: Sprite::new(Vec2::new(200.0, 100.0)),
            ..Default::default()
        });
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor{
            title: "Rust Invaders!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}
