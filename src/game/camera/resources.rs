use super::constants::{CAMERA_ORDER_Z, CAMERA_START_X, CAMERA_START_Y, CAMERA_VIEWPORT_HEIGHT};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

#[derive(Resource)]
pub struct CameraConfig {
    pub start_x: f32,
    pub start_y: f32,
    pub order_z: f32,
    pub viewport_h: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            start_x: CAMERA_START_X,
            start_y: CAMERA_START_Y,
            order_z: CAMERA_ORDER_Z,
            viewport_h: CAMERA_VIEWPORT_HEIGHT,
        }
    }
}

impl CameraConfig {
    pub fn projection(&self) -> Projection {
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: self.viewport_h,
            },
            ..OrthographicProjection::default_2d()
        })
    }
}
