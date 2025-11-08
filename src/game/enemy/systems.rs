use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;
use bevy_rapier2d::prelude::Velocity;
use crate::game::enemy::traits::EnemySpawner;
use crate::game::tiled::resources::SpawnBounds;
use crate::game::tiled::events::MapFullyLoaded;

use super::components::*;
use super::resources::*;

use super::const_string::ENEMY_SPRITE;

pub fn set_enemy_texture(asset_server: Res<AssetServer>, mut gametexture: ResMut<GameTexture>) {
    gametexture.enemy = asset_server.load(ENEMY_SPRITE);
}

pub fn spawn_enemy(
    mut map_loaded_event: EventReader<MapFullyLoaded>,
    mut commands: Commands,
    gametexture: Res<GameTexture>,
    spawn_bounds: Res<SpawnBounds>
) {
    for map_event in map_loaded_event.read() {
        info!("✅ Map {:?} fully loaded. Spawning enemies.", map_event.map_entity);

        let spawner = EdgeEnemySpawner;

        // Spawn 2 enemies directly
        for id in 0..2 {
            spawner.spawn_enemy(&mut commands, gametexture.enemy.clone(), &spawn_bounds, id);
        }
    }
}

pub fn speed_limit(max: Res<MaxSpeed>, mut query: Query<&mut Velocity, With<Enemy>>) {
    for mut velocity in query.iter_mut() {
        let speed = velocity.linvel.length();

        if speed > max.max_speed {
            let scale = max.max_speed / speed;
            velocity.linvel *= scale;
        } else if speed < max.max_speed * 0.95 {
            let scale = max.max_speed / speed;
            velocity.linvel *= scale;
        }
    }
}

pub fn debug_enemy_collision(
    mut collision_events: EventReader<CollisionEvent>,
    enemy_query: Query<&Transform, With<Enemy>>
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _flag) = collision_event {
            if enemy_query.contains(*e1) || enemy_query.contains(*e2) {
                info!("Enemy Bounce");
            }
        }
    }
}
pub fn debug_camera(
    camera_query: Query<&Transform, With<Camera>>,
    enemy_query: Query<&Transform, With<Enemy>>
) {
    for cam_transform in &camera_query {
        info!(
            "📷 Camera at: ({:.0}, {:.0}, {:.0})",
            cam_transform.translation.x,
            cam_transform.translation.y,
            cam_transform.translation.z
        );
    }

    for enemy_transform in &enemy_query {
        info!(
            "👾 Enemy at: ({:.0}, {:.0}, {:.0})",
            enemy_transform.translation.x,
            enemy_transform.translation.y,
            enemy_transform.translation.z
        );
    }
}
