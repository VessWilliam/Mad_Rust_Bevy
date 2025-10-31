use bevy::prelude::*;

use crate::game::camera::systems::setup_camera;

pub mod systems;
pub mod components;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_camera);
    }
}
