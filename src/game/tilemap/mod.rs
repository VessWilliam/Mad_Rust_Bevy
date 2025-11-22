use bevy::prelude::*;
use bevy_ecs_tiled::tiled::TiledPlugin;

pub mod builder;
pub mod components;
pub mod events;
pub mod helper;
pub mod resources;
pub mod systems;
pub mod traits;

use self::{
    events::MapFullyLoaded,
    resources::{GameBackground, SpawnBounds},
    systems::{load_tile_map, setup_background_color},
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
