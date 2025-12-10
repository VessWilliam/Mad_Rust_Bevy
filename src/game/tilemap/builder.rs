use super::components::{MapCollisionState, MapMetadata};
use super::traits::CollisionBuilderTrait;
use ::tiled::{FiniteTileLayer, LayerType, TileLayer};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::tiled::Map;
use bevy_rapier2d::{
    dynamics::RigidBody,
    prelude::{Collider, Friction},
};

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
                LayerType::Tiles(tile_layer) => match tile_layer {
                    TileLayer::Finite(finite_layer) => {
                        self.process_collision_tiles(&finite_layer, map);
                    }
                    TileLayer::Infinite(_) => {
                        warn!("Skipping infinite tile layer: {}", layer.name);
                    }
                },
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
                let start_x = x;
                if !self.is_solid_tile(x, y, meta.map_width) {
                    x += 1;
                    continue;
                }

                while x < meta.map_width && self.is_solid_tile(x, y, meta.map_width) {
                    x += 1;
                }

                self.spawn_merge_collider(commands, start_x, x, y, meta);
                collider_count += 1;
            }
        }
        info!("Spawned {} merged tile colliders", collider_count);
    }
}

impl MapCollisionState {
    #[inline]
    fn is_solid_tile(&self, x: u32, y: u32, map_width: u32) -> bool {
        let idx = (y * map_width + x) as usize;
        self.collision_map.get(idx) == Some(&1u8)
    }

    fn spawn_merge_collider(
        &self,
        commands: &mut Commands,
        start_x: u32,
        end_x: u32,
        y: u32,
        meta: &MapMetadata,
    ) {
        let width = (end_x - start_x) as f32;
        let center_x = start_x as f32 + width / 2.0;
        let center = self.tile_to_world_center(center_x, y as f32, meta);

        let half_x = (width * meta.tile_width) / 2.0;
        let half_y = meta.tile_height / 2.0;

        commands.spawn((
            Transform::from_translation(center),
            RigidBody::Fixed,
            Collider::cuboid(half_x, half_y),
            Friction::coefficient(1.0),
        ));
    }

    fn tile_to_world_center(&self, x: f32, y: f32, meta: &MapMetadata) -> Vec3 {
        let flipped_y = meta.map_height as f32 - 1.0 - y;

        Vec3::new(
            x * meta.tile_width,
            (flipped_y + 0.5) * meta.tile_height,
            1.0,
        )
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
