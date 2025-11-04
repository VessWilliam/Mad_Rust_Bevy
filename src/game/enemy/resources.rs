use bevy::{ math::bool, prelude::* };

#[derive(Resource)]
pub struct GameTexture {
    pub enemy: Handle<Image>,
}

#[derive(Resource)]
pub struct BounceState {
    pub is_bounce: bool,
}

#[derive(Resource)]
pub struct MaxSpeed {
    pub max_speed: f32,
}

#[derive(Resource)]
pub struct SpeedUp {
    pub speed_up: f32,
}

#[derive(Resource)]
pub struct SpeedMultiplier {
    pub speed_multiplier: f32,
}
