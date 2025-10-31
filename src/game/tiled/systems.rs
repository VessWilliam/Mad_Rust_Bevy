use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::game::tiled::resources::GameBackground;

use super::helper::on_map_created;

pub fn load_tile_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Loading Tiled map...");

    commands
        .spawn((
            TiledMap(asset_server.load("maps/map.tmx")),
            TiledMapLayerZOffset(0.0),
            Transform::from_xyz(0.0, 0.0, 1.0),
            TilemapAnchor::BottomLeft,
        ))
        .observe(on_map_created);
}

pub fn setup_background_color(bg_colour: Res<GameBackground>, mut commands: Commands) {
    commands.insert_resource(bg_colour.clear_colour.clone());
}
