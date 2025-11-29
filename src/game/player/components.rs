use super::constants::{
    COYOTE_TIME_SEC, PHYSICS_FRICTION, PHYSICS_GRAVITY_SCALE, PHYSICS_LINEAR_DAMPING,
    PHYSICS_RESTITUTION,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CoyoteTime {
    pub timer: Timer,
}

impl Default for CoyoteTime {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(COYOTE_TIME_SEC, TimerMode::Once),
        }
    }
}

impl CoyoteTime {
    pub fn is_active(&self) -> bool {
        self.timer.finished()
    }

    pub fn reset(&mut self) {
        self.timer.reset();
    }
}

#[derive(Bundle)]
pub struct PlayerPhysicsBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
    pub velocity: Velocity,
    pub gravity_scale: GravityScale,
    pub friction: Friction,
    pub restitution: Restitution,
    pub damping: Damping,
}

impl PlayerPhysicsBundle {
    pub fn new(collider_half_w: f32, collider_half_h: f32) -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(collider_half_w, collider_half_h),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::zero(),
            gravity_scale: GravityScale(PHYSICS_GRAVITY_SCALE),
            friction: Friction {
                coefficient: PHYSICS_FRICTION,
                combine_rule: CoefficientCombineRule::Min,
            },
            restitution: Restitution::coefficient(PHYSICS_RESTITUTION),
            damping: Damping {
                linear_damping: PHYSICS_LINEAR_DAMPING,
                angular_damping: 0.0,
            },
        }
    }
}
