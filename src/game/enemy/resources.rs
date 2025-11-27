use super::constants::{MAX_SPEED_LIMIT, MIN_SPEED_LIMIT};
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTexture {
    pub enemy: Handle<Image>,
}

#[derive(Resource)]
pub struct MaxSpeed {
    pub max_speed: f32,
    pub min_speed: f32,
}

impl Default for MaxSpeed {
    fn default() -> Self {
        Self {
            max_speed: MAX_SPEED_LIMIT,
            min_speed: MIN_SPEED_LIMIT,
        }
    }
}
