use bevy::prelude::*;

pub mod components;
pub mod constants;
pub mod resources;
pub mod systems;

use self::{resources::CameraConfig, systems::setup_camera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraConfig>()
            .add_systems(Startup, setup_camera);
    }
}
