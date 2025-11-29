use bevy_rapier2d::prelude::Velocity;

use super::constants::{COLLIDER_PADDING, COLLIDER_SCALE, GROUNDED_THRESHOLD};

pub fn calculate_collider_size(size: f32, scale: f32) -> (f32, f32) {
    let half_size = (size * scale) / 2.0 * COLLIDER_SCALE + COLLIDER_PADDING;
    (half_size, half_size)
}

pub fn is_grounded(velocity: &Velocity) -> bool {
    velocity.linvel.y.abs() < GROUNDED_THRESHOLD
}
