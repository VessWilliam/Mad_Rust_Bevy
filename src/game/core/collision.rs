use bevy::prelude::*;

/// Simple Collision Configuration.
#[derive(Debug, Clone, Copy)]
pub struct CollisionConfig {
    pub bounce: f32,
    pub angle_variation: f32,
    pub min_speed: f32,
    pub min_dist_sq: f32,
}

impl CollisionConfig {
    pub fn new(bounce: f32, angle_variation: f32, min_speed: f32, min_dist_sq: f32) -> Self {
        Self {
            bounce,
            angle_variation,
            min_speed,
            min_dist_sq,
        }
    }

    pub fn resolve_collision(&self, vel1: &mut Vec2, vel2: &mut Vec2, pos1: Vec2, pos2: Vec2) {
        let normal = match self.collision_normal(pos1, pos2) {
            Some(n) => n,
            None => return,
        };

        let appr_speed = self.approach_speed(*vel1, *vel2, normal);

        if appr_speed > 0.0 {
            return;
        }

        let impulse = normal * self.impulse_amount(appr_speed);

        *vel1 += impulse;
        *vel2 -= impulse;

        self.apply_random_variation(vel1);
        self.apply_random_variation(vel2);
    }

    fn collision_normal(&self, pos1: Vec2, pos2: Vec2) -> Option<Vec2> {
        let offset = pos1 - pos2;
        if offset.length_squared() < self.min_dist_sq {
            return None;
        }
        Some(offset.normalize())
    }

    fn approach_speed(&self, v1: Vec2, v2: Vec2, normal: Vec2) -> f32 {
        (v1 - v2).dot(normal)
    }

    fn impulse_amount(&self, approach_speed: f32) -> f32 {
        -(1.0 + self.bounce) * approach_speed * 0.5
    }

    fn apply_random_variation(&self, vel: &mut Vec2) {
        let speed = vel.length();
        if speed < self.min_speed {
            return;
        }

        let base_angle = vel.y.atan2(vel.x);
        let rand_offset = (rand::random::<f32>() - 0.5) * self.angle_variation;

        let new_angle = base_angle + rand_offset;
        *vel = Vec2::from_angle(new_angle) * speed;
    }
}
