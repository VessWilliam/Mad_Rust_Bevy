use bevy::prelude::*;

pub mod resources;
pub mod traits;
pub mod systems;
pub mod helper;
pub mod builder;
pub mod components;

use bevy_ecs_tiled::tiled::TiledPlugin;
use systems::{ load_tile_map, setup_background_color };
use resources::{ GameBackground };

pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TiledPlugin::default())
            .init_resource::<GameBackground>()
            .add_systems(Startup, (load_tile_map, setup_background_color));
    }
}
