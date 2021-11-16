use std::{f32::consts::PI};

use bevy::{core::FixedTimestep, ecs::query, prelude::*};
use rand::{Rng, thread_rng};

use crate::{ActiveEnemies, Enemy, FromEnemy, Laser, MAX_ENEMIES, Materials, SCALE, Speed, TIME_STEP, WinSize};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(enemy_laser_movement.system())
            .add_system(enemy_movement.system())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(enemy_spawn.system())
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.9))
                    .with_system(enemy_fire.system())
            );
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut active_enemies: ResMut<ActiveEnemies>,
    win_size: Res<WinSize>,
    materials: Res<Materials>
) {
    if active_enemies.0 < MAX_ENEMIES {
        // compute random position
        let mut rng = thread_rng();
        let w_span = win_size.w / 2.0 - 100.0;
        let h_span = win_size.h / 2.0 - 100.0;
        let x = rng.gen_range(-w_span..w_span) as f32;
        let y = rng.gen_range(-h_span..h_span) as f32;

        // enemy spawn
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.0),
                    scale: Vec3::new(SCALE, SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(Speed::default());
        
        active_enemies.0 += 1;
    }
}

fn enemy_fire(
    mut commands: Commands,
    materials: Res<Materials>,
    enemy_query: Query<&Transform, With<Enemy>>
) {
    for &transform in enemy_query.iter() {
        let x = transform.translation.x;
        let y = transform.translation.y;

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.enemy_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y -15.0, 0.0),
                    scale: Vec3::new(SCALE, -SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Laser)
            .insert(FromEnemy)
            .insert(Speed::default());
    }
}

fn enemy_laser_movement(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Speed, &mut Transform), (With<Laser>, With<FromEnemy>)>
) {
    for (laser_entity, speed, mut laser_transform) in query.iter_mut() {
        let translation = &mut laser_transform.translation;
        translation.y -= speed.0 * TIME_STEP;

        if translation.y < -win_size.h / 2.0 - 50.0 {
            commands.entity(laser_entity).despawn();
        }
    }
}

fn enemy_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed), With<Enemy>>
) {
    let now = time.seconds_since_startup() as f32;
    for (mut transform, speed) in query.iter_mut() {
        let max_distance = TIME_STEP * speed.0;
        let x_org = transform.translation.x;
        let y_org = transform.translation.y;

        // Get the ellipse
        let (x_offset, y_offset) = (0., 100.);
        let (x_radius, y_radius) = (150., 100.);

        // Compute the next angle
        let angle = speed.0 * TIME_STEP * now % 360. / PI;

        // Compute the destination
        let x_dst = x_radius * angle.cos() + x_offset;
        let y_dst = y_radius * angle.sin() + y_offset;

        // Calculate the distance
        let dx = x_org - x_dst;
        let dy = y_org - y_dst;
        let distance = (dx * dx + dy * dy).sqrt();
        let distance_ratio = if distance == 0. {
            0.
        } else {
            max_distance / distance
        };

        // Calculate the final x/y (make sure to not overshoot)
        let x = x_org - dx * distance_ratio;
        let x = if dx > 0. {x.max(x_dst)} else {x.min(x_dst)};
        let y = y_org - dy * distance_ratio;
        let y = if dy > 0. {y.max(y_dst)} else {y.min(y_dst)};

        // Apply transformation
        transform.translation.x = x;
        transform.translation.y = y;
    }
}