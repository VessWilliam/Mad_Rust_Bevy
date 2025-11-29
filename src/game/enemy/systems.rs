use super::components::*;
use super::constants::{
    COLLISION_ANGLE_VARIATION, COLLISION_BOUNCE, COLLISION_MIN_DISTANCE_SQ, COLLISION_MIN_SPEED,
    ENEMY_SPRITE, SPRITE_ROTATION_MIN_VELOCITY,
};
use super::resources::*;
use crate::game::core::collision::CollisionConfig;
use crate::game::enemy::traits::EnemySpawner;
use crate::game::tilemap::events::MapFullyLoaded;
use crate::game::tilemap::resources::SpawnBounds;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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

        // Spawn 2 enemies directly
        for id in 0..2 {
            EdgeSpawner::spawn_enemy_default_config(
                &mut commands,
                gametexture.enemy.clone(),
                &*spawn_bounds,
                id,
            );
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
        if velocity.linvel.length() < SPRITE_ROTATION_MIN_VELOCITY {
            continue;
        }
        let angle = velocity.linvel.y.atan2(velocity.linvel.x);
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

pub fn enemy_bounce_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut enemies: Query<(&mut Velocity, &Transform), With<Enemy>>,
) {
    let collision_config = CollisionConfig::new(
        COLLISION_BOUNCE,
        COLLISION_ANGLE_VARIATION,
        COLLISION_MIN_SPEED,
        COLLISION_MIN_DISTANCE_SQ,
    );

    for event in collision_events.read() {
        let CollisionEvent::Started(e1, e2, _) = event else {
            continue;
        };

        let Ok([(mut vel1, t1), (mut vel2, t2)]) = enemies.get_many_mut([*e1, *e2]) else {
            continue;
        };

        let pos1 = t1.translation.truncate();
        let pos2 = t2.translation.truncate();

        collision_config.resolve_collision(&mut vel1.linvel, &mut vel2.linvel, pos1, pos2);
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
