use super::constants::{DEFAULT_MEDIUM_WINDOW_H, DEFAULT_MEDIUM_WINDOW_W};
use bevy::prelude::*;

#[derive(Resource)]
pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

impl Default for WinSize {
    fn default() -> Self {
        Self {
            width: DEFAULT_MEDIUM_WINDOW_W,
            height: DEFAULT_MEDIUM_WINDOW_H,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct WindowResolution {
    pub width: f32,
    pub height: f32,
}

impl WindowResolution {
    pub const DEFAULT_WINDOWED: Self = Self {
        width: DEFAULT_MEDIUM_WINDOW_W,
        height: DEFAULT_MEDIUM_WINDOW_H,
    };

    pub fn apply_to(&self, window: &mut Window) {
        window.resolution.set(self.width, self.height);
    }
}

impl Default for WindowResolution {
    fn default() -> Self {
        Self::DEFAULT_WINDOWED
    }
}

#[derive(Resource, Default)]
pub struct FullscreenState {
    pub is_fullscreen: bool,
    pub is_small: bool,
}
