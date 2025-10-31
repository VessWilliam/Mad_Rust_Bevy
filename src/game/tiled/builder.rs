use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_rapier2d::{ dynamics::RigidBody, prelude::{ Collider, Friction } };
use super::traits::CollisionBuilderTrait;
use super::components::{ MapCollisionState, MapMetadata };
use tiled::{ LayerType, TileLayer, FiniteTileLayer };

impl CollisionBuilderTrait for MapCollisionState {
    fn build_collision(&mut self, map: &Map) {
        let map_size = (map.width * map.height) as usize;
        self.collision_map = vec![0u8; map_size];

        for layer in map.layers() {
            if layer.name != "Collision" {
                continue;
            }

            info!("Found Collision layer!");

            match layer.layer_type() {
                LayerType::Tiles(tile_layer) =>
                    match tile_layer {
                        TileLayer::Finite(finite_layer) => {
                            self.process_collision_tiles(&finite_layer, map);
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
    }

    fn spawn_collider(&self, commands: &mut Commands, meta: &MapMetadata) {
        let mut collider_count = 0;

        for y in 0..meta.map_height {
            let mut x = 0;
            while x < meta.map_width {
                let idx = (y * meta.map_width + x) as usize;

                if self.collision_map[idx] != 1 {
                    x += 1;
                    continue;
                }

                let start_x = x;
                while
                    x < meta.map_width &&
                    self.collision_map[(y * meta.map_width + x) as usize] == 1
                {
                    x += 1;
                }

                let width = (x - start_x) as f32;

                let center = self.tile_to_world_center(
                    (start_x as f32) + width / 2.0,
                    y as f32,
                    meta
                );

                commands.spawn((
                    Transform::from_translation(center),
                    RigidBody::Fixed,
                    Collider::cuboid((width * meta.tile_width) / 2.0, meta.tile_height / 2.0),
                    Friction::coefficient(1.0),
                ));

                collider_count += 1;
                info!("Spawned {} merged tile colliders", collider_count);
            }
        }
    }
}

impl MapCollisionState {
    fn tile_to_world_center(&self, x: f32, y: f32, meta: &MapMetadata) -> Vec3 {
        let flipped_y = meta.map_height as f32 - 1.0 - y;

        Vec3::new(x * meta.tile_width, (flipped_y + 0.5) * meta.tile_height, 1.0)
    }

    fn process_collision_tiles(&mut self, finite_layer: &FiniteTileLayer, map: &Map) {
        let mut tile_count = 0;

        for y in 0..map.height {
            for x in 0..map.width {
                match finite_layer.get_tile(x as i32, y as i32) {
                    Some(_tile) => {
                        let idx = (y * map.width + x) as usize;
                        self.collision_map[idx] = 1;
                        tile_count += 1;

                        if tile_count <= 5 || y >= 18 {
                            info!("Collision tile at ({}, {}) -> idx {}", x, y, idx);
                        }
                    }
                    None => {}
                }
            }
        }

        info!("Processed {} collision tiles from layer", tile_count);
    }
}
