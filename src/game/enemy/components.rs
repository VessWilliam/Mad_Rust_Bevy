use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EdgeSpawner {
    pub speed: f32,
    pub safe_margin: f32,
}

impl EdgeSpawner {
    pub fn new(speed: f32, safe_margin: f32) -> Self {
        Self { speed, safe_margin }
    }
}
