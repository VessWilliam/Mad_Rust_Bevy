use super::components::*;
use super::resources::CameraConfig;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands, config: Res<CameraConfig>) {
    commands.spawn((
        Camera2d,
        Camera {
            ..Default::default()
        },
        Transform::from_xyz(config.start_x, config.start_y, config.order_z),
        config.projection(),
        MainCamera,
    ));
}
