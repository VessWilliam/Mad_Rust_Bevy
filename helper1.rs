use bevy::prelude::*;
use bevy_rapier2d::{ prelude::{ Collider, Friction } };
use bevy_rapier2d::dynamics::RigidBody;
use super::resources::MapCollisionState;
use bevy_ecs_tiled::prelude::*;
use tiled::{ FiniteTileLayer, LayerType };

//Map Create Helper

pub fn on_map_created(
    trigger: Trigger<TiledEvent<MapCreated>>,
    assets: Res<Assets<TiledMapAsset>>,
    mut level_data: ResMut<MapCollisionState>,
    mut command: Commands
) {
    if let Some(map_asset) = trigger.event().get_map(&assets) {
        info!(
            "✅ Map loaded: {}x{} with {} layers",
            map_asset.width,
            map_asset.height,
            map_asset.layers().len()
        );

        level_data.map = Some(map_asset.clone());
        build_collision_map(map_asset, &mut level_data);

        spawn_collider_for_map(&mut command, map_asset, &level_data.collision_map);
    }
}

fn build_collision_map(map: &Map, level_data: &mut MapCollisionState) {
    let map_size = (map.width * map.height) as usize;
    level_data.collision_map = vec![0; map_size];

    info!("Map size: {}x{} = {} tiles", map.width, map.height, map_size);

    for layer in map.layers() {
        info!("Processing layer: {}", layer.name);

        if layer.name != "Collision" {
            continue;
        }

        info!("Found Collision layer!");

        match layer.layer_type() {
            LayerType::Tiles(tile_layer) =>
                match tile_layer {
                    TileLayer::Finite(finite_layer) => {
                        process_finite_layer(&finite_layer, map, level_data);
                    }
                    TileLayer::Infinite(_) => {
                        warn!("Skipping infinite tile layer: {}", layer.name);
                    }
                }
            _ => {
                warn!("Layer {} is not a tile layer", layer.name);
            }
        }
    }

    let solid_count = level_data.collision_map
        .iter()
        .filter(|&&x| x != 0)
        .count();
    info!("   Built collision map: {} solid tiles out of {} total", solid_count, map_size);
}

 pub fn process_finite_layer(finite_layer: &FiniteTileLayer, map: &Map, level_data: &mut MapCollisionState) {
    let mut tile_count = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            // Check the tile data directly - treat any tile present as collision
            if let Some(_tile) = finite_layer.get_tile(x as i32, y as i32) {
                let idx = (y * map.width + x) as usize;
                level_data.collision_map[idx] = 1;
                tile_count += 1;

                if tile_count <= 5 || y >= 18 {
                    info!("Collision tile at ({}, {}) -> idx {}", x, y, idx);
                }
            }
        }
    }

    info!("Processed {} collision tiles from layer", tile_count);
}

fn spawn_collider_for_map(commands: &mut Commands, map: &Map, collision_map: &[i32]) {
    let tile_w = map.tile_width as f32;
    let tile_h = map.tile_height as f32;
    let map_height = map.height as f32;
    let mut count = 0;

    for y in 0..map.height {
        let mut x = 0;
        while x < map.width {
            let idx = (y * map.width + x) as usize;

            if collision_map.get(idx) == Some(&1) {
                // Find consecutive solid tiles in this row
                let start_x = x;
                while x < map.width && collision_map.get((y * map.width + x) as usize) == Some(&1) {
                    x += 1;
                }
                let end_x = x;
                let width = end_x - start_x;

                // Create one collider for the entire horizontal span
                let flipped_y = map_height - 1.0 - (y as f32);
                let center_x = (start_x as f32 + (width as f32) / 2.0) * tile_w;
                let pos_y = flipped_y * tile_h + tile_h / 2.0;
                let half_width = (width as f32 * tile_w) / 2.0;

                commands.spawn((
                    Transform::from_xyz(center_x, pos_y, 1.0),
                    RigidBody::Fixed,
                    Collider::cuboid(half_width, tile_h / 2.0),
                    Friction::coefficient(1.0),
                ));

                count += 1;
            } else {
                x += 1;
            }
        }
    }

    info!("Spawned {} merged tile colliders", count);
}