use crate::game::enemy::traits::EnemySpawner;
use crate::game::tilemap::events::MapFullyLoaded;
use crate::game::tilemap::resources::SpawnBounds;

use bevy::prelude::*;

use bevy_rapier2d::prelude::CollisionEvent;
use bevy_rapier2d::prelude::Velocity;

use super::components::*;
use super::resources::*;

use super::constants::{ENEMY_SAFE_MARGIN, ENEMY_SPEED, ENEMY_SPRITE};

pub fn set_enemy_texture(asset_server: Res<AssetServer>, mut gametexture: ResMut<GameTexture>) {
    gametexture.enemy = asset_server.load(ENEMY_SPRITE);
}

pub fn spawn_enemy(
    mut map_loaded_event: EventReader<MapFullyLoaded>,
    mut commands: Commands,
    gametexture: Res<GameTexture>,
    spawn_bounds: Res<SpawnBounds>,
) {
    for map_event in map_loaded_event.read() {
        info!(
            "✅ Map {:?} fully loaded. Spawning enemies.",
            map_event.map_entity
        );

        let spawner = EdgeEnemySpawner::new(ENEMY_SPEED, ENEMY_SAFE_MARGIN);

        // Spawn 2 enemies directly
        for id in 0..2 {
            spawner.spawn_enemy(&mut commands, gametexture.enemy.clone(), &*spawn_bounds, id);
        }
    }
}

pub fn speed_limit(max: ResMut<MaxSpeed>, mut query: Query<&mut Velocity, With<Enemy>>) {
    for mut velocity in query.iter_mut() {
        let speed = velocity.linvel.length();
        if speed == 0.0 {
            continue;
        }

        match speed {
            s if s > max.max_speed => {
                velocity.linvel = velocity.linvel.normalize() * max.max_speed;
            }

            s if s < max.min_speed => {
                velocity.linvel = velocity.linvel.normalize() * max.min_speed;
            }
            _ => {}
        }
    }
}

pub fn rotate_enemy_sprite(mut query: Query<(&Velocity, &mut Transform), With<Enemy>>) {
    for (velocity, mut transform) in query.iter_mut() {
        if velocity.linvel.length() < 0.1 {
            continue;
        }
        let angle = velocity.linvel.y.atan2(velocity.linvel.x);
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

pub fn enemy_bounce_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut enemies: Query<&mut Velocity, With<Enemy>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if !enemies.contains(*e1) || !enemies.contains(*e2) {
                continue;
            }

            for entity in [e1, e2] {
                if let Ok(mut vel) = enemies.get_mut(*entity) {
                    vel.linvel = -vel.linvel;

                    let random_angle = (rand::random::<f32>() - 0.5) * 0.4;
                    let speed = vel.linvel.length();
                    let current_angle = vel.linvel.y.atan2(vel.linvel.x);
                    let new_angle = current_angle + random_angle;
                    vel.linvel = Vec2::new(new_angle.cos(), new_angle.sin()) * speed;
                }
            }
        }
    }
}

pub fn keep_enemies_in_bound(
    mut enemies: Query<&mut Transform, With<Enemy>>,
    spawn_bound: Res<SpawnBounds>,
) {
    for mut transform in enemies.iter_mut() {
        transform.translation.x = transform.translation.x.clamp(0.0, spawn_bound.width);
        transform.translation.y = transform.translation.y.clamp(0.0, spawn_bound.height);
    }
}

pub fn debug_enemy_collision(
    mut collision_events: EventReader<CollisionEvent>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _flag) = collision_event {
            if enemy_query.contains(*e1) || enemy_query.contains(*e2) {
                info!("Enemy Bounce");
            }
        }
    }
}
