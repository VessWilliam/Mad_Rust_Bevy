use self::{
    resources::{FullscreenState, WinSize, WindowResolution},
    systems::{exit_window, set_window_init, toggle_fullscreen, update_window_size},
};
use bevy::prelude::*;

pub mod constants;
pub mod resources;
pub mod systems;

pub struct CustomWindowPlugins;

impl Plugin for CustomWindowPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<WinSize>()
            .init_resource::<FullscreenState>()
            .init_resource::<WindowResolution>()
            .add_systems(Startup, set_window_init)
            .add_systems(Update, (update_window_size, exit_window, toggle_fullscreen));
    }
}
