#![allow(unused)] // temporal

mod player;

use bevy::prelude::*;
use player::PlayerPlugin;

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const TIME_STEP: f32 = 1. / 60.;
const SCALE: f32 = 0.5;

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
struct PlayerReadyFire(bool);
struct Laser;
struct Speed(f32);
impl Default for Speed {
    fn default() -> Self {
        Self(500.)
    }
}

// Systems
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
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup.system())
        .run();
}
