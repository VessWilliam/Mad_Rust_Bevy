use bevy::prelude::*;
use tiled::Map;

#[derive(Component, Default)]
pub struct MapCollisionState {
    pub collision_map: Vec<u8>,
}

#[derive(Component)]
pub struct MapMetadata {
    pub tile_width: f32,
    pub tile_height: f32,
    pub map_width: u32,
    pub map_height: u32,
}

impl MapMetadata {
    pub fn from_map(map: &Map) -> Self {
        Self {
            tile_width: map.tile_width as f32,
            tile_height: map.tile_height as f32,
            map_width: map.width,
            map_height: map.height,
        }
    }
}
