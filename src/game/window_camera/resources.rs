use bevy::prelude::*;

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct FullscreenState {
    pub is_fullscreen: bool,
    pub is_small: bool,
}


