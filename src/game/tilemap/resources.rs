use crate::game::core::traits::SpawnBoundTrait;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameBackground {
    pub clear_colour: ClearColor,
}

impl Default for GameBackground {
    fn default() -> Self {
        Self {
            clear_colour: ClearColor(Color::srgb(0.0, 0.0, 0.0)),
        }
    }
}

#[derive(Resource)]
pub struct SpawnBounds {
    pub width: f32,
    pub height: f32,
}

impl Default for SpawnBounds {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
        }
    }
}

impl SpawnBoundTrait for SpawnBounds {
    fn height(&self) -> f32 {
        self.height
    }
    fn width(&self) -> f32 {
        self.width
    }
}
