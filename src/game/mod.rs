use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use tiled::TiledMapPlugin;
use camera::CameraPlugin;
use player::PlayerPlugin;
use window_camera::CustomWindowPlugins;
use enemy::EnemyPlugin;

pub mod tiled;
pub mod camera;
pub mod player;
pub mod enemy;
pub mod window_camera;



pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .add_plugins(TiledMapPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(CustomWindowPlugins)
            .add_plugins(EnemyPlugin);
    }
}
