use super::constants::{
    PHYSICS_ANGULAR_DAMPING, PHYSICS_COLLIDER_RADIUS, PHYSICS_FRICTION, PHYSICS_GRAVITY,
    PHYSICS_LINEAR_DAMPING, PHYSICS_RESTITUTION,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EdgeSpawner;

#[derive(Bundle)]
pub struct EnemyPhysicsBundle {
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub restitution: Restitution,
    pub friction: Friction,
    pub gravity: GravityScale,
    pub damping: Damping,
    pub ccd: Ccd,
}

impl EnemyPhysicsBundle {
    pub fn new(velocity: Vec2) -> Self {
        Self {
            rigidbody: RigidBody::Dynamic,
            collider: Collider::ball(PHYSICS_COLLIDER_RADIUS),
            velocity: Velocity::linear(velocity),
            restitution: Restitution {
                coefficient: PHYSICS_RESTITUTION,
                combine_rule: CoefficientCombineRule::Max,
            },
            friction: Friction {
                coefficient: PHYSICS_FRICTION,
                combine_rule: CoefficientCombineRule::Min,
            },
            gravity: GravityScale(PHYSICS_GRAVITY),
            damping: Damping {
                linear_damping: PHYSICS_LINEAR_DAMPING,
                angular_damping: PHYSICS_ANGULAR_DAMPING,
            },
            ccd: Ccd::enabled(),
        }
    }
}
