use bevy::prelude::*;

use super::constants::{PLAYER_SIZE, PLAYER_SIZE_SCALE};

#[derive(Resource)]
pub struct GameTexture {
    pub player: Handle<Image>,
}

impl Default for GameTexture {
    fn default() -> Self {
        Self {
            player: Handle::default(),
        }
    }
}

#[derive(Resource)]
pub struct PlayerConfig {
    pub spawn_x: f32,
    pub spawn_y: f32,
    pub size: f32,
    pub scale: f32,
    pub speed: f32,
    pub jump: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            spawn_x: 40.0,
            spawn_y: 50.0,
            size: PLAYER_SIZE,
            scale: PLAYER_SIZE_SCALE,
            speed: 150.0,
            jump: 500.0,
        }
    }
}
