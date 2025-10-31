use bevy::prelude::*;
use super::components::*;
use bevy::render::camera::ScalingMode;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: false,
            ..Default::default()
        },
        Transform::from_xyz(240.0, 160.0, 999.9),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical { viewport_height: 280.0 },
            ..OrthographicProjection::default_2d()
        }),
        MainCamera,
    ));
}
