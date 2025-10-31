use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use crate::game::tiled::traits::CollisionBuilderTrait;

use super::components::{ MapCollisionState, MapMetadata };

pub fn on_map_created(
    trigger: Trigger<TiledEvent<MapCreated>>,
    asset: Res<Assets<TiledMapAsset>>,
    mut commands: Commands
) {
    let map_entity = trigger.origin.entity();

    match trigger.event().get_map(&asset) {
        Some(map_asset) => {
            info!(
                "✅ Map loaded: {}x{} with {} layers",
                map_asset.width,
                map_asset.height,
                map_asset.layers().len()
            );

            // Build collision data
            let mut collision_state = MapCollisionState::default();
            collision_state.build_collision(map_asset);

            // Create metadata
            let metadata = MapMetadata::from_map(map_asset);

            // Spawn colliders immediately
            collision_state.spawn_collider(&mut commands, &metadata);

            // Attach components to the map entity
            commands.entity(map_entity).insert((collision_state, metadata));
        }
        None => {
            warn!("Map asset not found for entity {:?}", map_entity);
        }
    }
}
