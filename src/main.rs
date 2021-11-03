#![allow(unused)] // temporal

mod player;
mod enemy;

use bevy::{prelude::*, sprite::collide_aabb::collide};
use player::PlayerPlugin;
use enemy::EnemyPlugin;

const PLAYER_SPRITE: &str = "player_a_01.png";
const LASER_SPRITE: &str = "laser_a_01.png";
const ENEMY_SPRITE: &str = "enemy_a_01.png";
const TIME_STEP: f32 = 1. / 60.;
const SCALE: f32 = 0.5;

// Resources
pub struct Materials {
    player_materials: Handle<ColorMaterial>,
    laser_materials: Handle<ColorMaterial>,
    enemy_materials: Handle<ColorMaterial>,
}

struct WinSize {
    w: f32,
    h: f32,
}

struct ActiveEnemies(u32);

// Components
struct Player;
struct PlayerReadyFire(bool);
struct Laser;
struct Enemy;
struct Speed(f32);

impl Default for Speed {
    fn default() -> Self {
        Self(500.)
    }
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
        .insert_resource(ActiveEnemies(0))
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system(setup.system())
        .add_system(laser_hit_enemy.system())
        .run();
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
        enemy_materials: materials.add(asset_server.load(ENEMY_SPRITE).into()),
    });

    commands.insert_resource(WinSize {
        w: window.width(),
        h: window.height(),
    });

    // position window
    window.set_position(IVec2::new(3870, 4830));
}

fn laser_hit_enemy(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform, &Sprite), With<Laser>>,
    enemy_query: Query<(Entity, &Transform, &Sprite), With<Enemy>>,
    mut active_enemies: ResMut<ActiveEnemies>
) {
    for (laser_entity, laser_transform, laser_sprite) in laser_query.iter() {
        for (enemy_entity, enemy_transform, enemy_sprite) in enemy_query.iter() {
            let laser_scale = Vec2::from(laser_transform.scale);
            let enemy_scale = Vec2::from(enemy_transform.scale);
            let collision = collide(
                laser_transform.translation,
                laser_sprite.size * laser_scale,
                enemy_transform.translation,
                enemy_sprite.size * enemy_scale,
            );

            if let Some(_) = collision {
                commands.entity(enemy_entity).despawn();
                active_enemies.0 -= 1;

                commands.entity(laser_entity).despawn();
            }
        }
    }
}