use super::components::{MapCollisionState, MapMetadata};
use super::events::MapFullyLoaded;
use super::resources::SpawnBounds;
use super::traits::CollisionBuilderTrait;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub fn on_map_created(
    trigger: On<TiledEvent<MapCreated>>,
    assets: Res<Assets<TiledMapAsset>>,
    mut commands: Commands,
    mut map_loaded_event: MessageWriter<MapFullyLoaded>,
) {
    let map_entity = trigger.origin.entity();

    // Get map directly from the event
    let Some(map_asset) = trigger.event().get_map(&assets) else {
        warn!("Map asset not found for entity {:?}", map_entity);
        return;
    };

    let map = map_asset;

    info!(
        "Map loaded: {}x{} with {} layers",
        map.width,
        map.height,
        map.layers().len()
    );

    // Build collision data
    let mut collision_state = MapCollisionState::default();
    collision_state.build_collision(map);

    // Create metadata
    let metadata = MapMetadata::from_map(map);

    // Set spawn bounds
    let world_width = (map.width as f32) * (map.tile_width as f32);
    let world_height = (map.height as f32) * (map.tile_height as f32);

    commands.insert_resource(SpawnBounds {
        width: world_width,
        height: world_height,
    });

    info!("Set spawn bounds: {}x{}", world_width, world_height);

    // Spawn colliders
    collision_state.spawn_collider(&mut commands, &metadata);

    // Attach components to the map entity
    commands
        .entity(map_entity)
        .insert((collision_state, metadata));

    map_loaded_event.write(MapFullyLoaded { map_entity });
}
