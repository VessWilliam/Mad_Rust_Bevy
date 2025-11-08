use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTexture {
    pub enemy: Handle<Image>,
}

#[derive(Resource)]
pub struct MaxSpeed {
    pub max_speed: f32,
}
