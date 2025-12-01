use self::{
    resources::{FullscreenState, WinSize},
    systems::{exit_window, set_window_init, toggle_fullscreen, update_window_size},
};
use bevy::prelude::*;

pub mod constant;
pub mod resources;
pub mod systems;

pub struct CustomWindowPlugins;

impl Plugin for CustomWindowPlugins {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinSize { w: 0.0, h: 0.0 })
            .insert_resource(FullscreenState {
                is_fullscreen: false,
                is_small: false,
            })
            .add_systems(Startup, set_window_init)
            .add_systems(Update, update_window_size)
            .add_systems(Update, exit_window)
            .add_systems(Update, toggle_fullscreen);
    }
}
