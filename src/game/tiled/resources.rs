use bevy::prelude::*;

#[derive(Resource)]
pub struct GameBackground {
    pub clear_colour: ClearColor,
}

impl Default for GameBackground {
    fn default() -> Self {
        Self { clear_colour: ClearColor(Color::srgb(0.0, 0.0, 0.0)) }
    }
}
