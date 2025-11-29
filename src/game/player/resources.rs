use bevy::prelude::*;

use super::constants::{
    DEFAULT_PLAYER_JUMP, DEFAULT_PLAYER_SPAWN_X, DEFAULT_PLAYER_SPAWN_Y, DEFAULT_PLAYER_SPEED,
    PLAYER_SIZE, PLAYER_SIZE_SCALE,
};

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
            spawn_x: DEFAULT_PLAYER_SPAWN_X,
            spawn_y: DEFAULT_PLAYER_SPAWN_Y,
            size: PLAYER_SIZE,
            scale: PLAYER_SIZE_SCALE,
            speed: DEFAULT_PLAYER_SPEED,
            jump: DEFAULT_PLAYER_JUMP,
        }
    }
}
