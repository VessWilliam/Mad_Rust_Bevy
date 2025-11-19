use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTexture {
    pub enemy: Handle<Image>,
}

#[derive(Resource)]
pub struct MaxSpeed {
    pub max_speed: f32,
    pub  min_speed: f32,
}

impl Default for MaxSpeed {
    fn default() -> Self {
        Self { max_speed: 200.0, min_speed: 150.0 }
    }
}

