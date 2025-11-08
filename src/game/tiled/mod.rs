use bevy_ecs_tiled::tiled::TiledPlugin;
use bevy::prelude::*;

pub mod resources;
pub mod traits;
pub mod systems;
pub mod helper;
pub mod builder;
pub mod components;
pub mod events;

use self::{
    systems::{ load_tile_map, setup_background_color },
    resources::{ GameBackground, SpawnBounds },
    events::MapFullyLoaded,
};

pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TiledPlugin::default())
            .init_resource::<GameBackground>()
            .init_resource::<SpawnBounds>()
            .add_event::<MapFullyLoaded>()
            .add_systems(Startup, (load_tile_map, setup_background_color));
    }
}
