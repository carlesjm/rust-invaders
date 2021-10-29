#![allow(unused)] // temporal

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const TIME_STEP: f32 = 1. / 60.;

// Resources
pub struct Materials {
    player_materials: Handle<ColorMaterial>,
    laser_materials: Handle<ColorMaterial>,
}

struct WinSize {
    w: f32,
    h: f32,
}

// Components
struct Player;
struct Laser;
struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self(500.)
    }
}

// Systems
fn player_spawn(
    mut commands: Commands,
    mut materials: Res<Materials>,
    win_size: Res<WinSize>,
) {
    // spawn sprite
    let bottom = - win_size.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_materials.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + 75. / 4. + 5., 10.),
                scale: Vec3::new(0.5, 0.5, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Speed::default());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform), With<Player>>
) {
    if let Ok((speed, mut transform)) = query.single_mut() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            -1.
        } else if keyboard_input.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };
        transform.translation.x += dir * speed.0 * TIME_STEP;
    }
}

fn player_fire(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut materials: Res<Materials>,
    query: Query<(&Transform), With<Player>>
) {
    if let Ok((player_transform)) = query.single() {
        if keyboard_input.pressed(KeyCode::Space) {
            let x = player_transform.translation.x;
            let y = player_transform.translation.y;

            commands.spawn_bundle(SpriteBundle {
                material: materials.laser_materials.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(Speed);
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>
) {
    let window = windows.get_primary_mut().unwrap();

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // create main resources
    commands.insert_resource(Materials {
        player_materials: materials.add(asset_server.load(PLAYER_SPRITE).into()),
        laser_materials: materials.add(asset_server.load(LASER_SPRITE).into()),
    });

    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    // position window
    window.set_position(IVec2::new(3870, 4830));
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
        .add_startup_stage("game_setup_actors", SystemStage::single(player_spawn.system()))
        .add_system(player_movement.system())
        .add_system(player_fire.system())
        .run();
}
