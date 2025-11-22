use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use camera::CameraPlugin;
use core::CorePlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use tilemap::TiledMapPlugin;
use window_camera::CustomWindowPlugins;

pub mod camera;
pub mod core;
pub mod enemy;
pub mod player;
pub mod tilemap;
pub mod window_camera;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .add_plugins(TiledMapPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(CustomWindowPlugins)
            .add_plugins(CorePlugin)
            .add_plugins(EnemyPlugin);
    }
}
