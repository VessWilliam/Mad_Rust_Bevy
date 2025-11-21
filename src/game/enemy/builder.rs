use super::components::{EdgeEnemySpawner, Enemy};
use super::constants::ENEMY_SIZE_SCALE;
use super::traits::EnemySpawner;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use log::info;
use rand::Rng;

use crate::game::tiled::resources::SpawnBounds;

impl EnemySpawner for EdgeEnemySpawner {
    fn spawn_enemy(
        &self,
        commands: &mut Commands,
        texture: Handle<bevy::image::Image>,
        spawn_bounds: &SpawnBounds,
        enemy_id: i32,
    ) {
        let mut rng = rand::rng();

        let min_x = self.safe_margin;
        let min_y = self.safe_margin;
        let max_x = spawn_bounds.width - self.safe_margin;
        let max_y = spawn_bounds.height - self.safe_margin;
        let edge = rng.random_range(0..4);

        if max_x <= min_x || max_y <= min_y {
            warn!(
                "invalid spawn bound ! width={}, height={}",
                spawn_bounds.width, spawn_bounds.height
            );
            return;
        }

        let (x, y, velocity, edge_name) = match edge {
            0 => {
                let x = rng.random_range(min_x..max_x);
                let y = max_y - self.safe_margin;
                let vel = Vec2::new(rng.random_range(-self.speed..self.speed), -self.speed);
                (x, y, vel, "TOP")
            }
            1 => {
                let x = rng.random_range(min_x..max_x);
                let y = min_y + self.safe_margin;
                let vel = Vec2::new(rng.random_range(-self.speed..self.speed), self.speed);
                (x, y, vel, "BOTTOM")
            }
            2 => {
                let x = min_x + self.safe_margin;
                let y = rng.random_range(min_y..max_y);
                let vel = Vec2::new(self.speed, rng.random_range(-self.speed..self.speed));
                (x, y, vel, "LEFT")
            }
            _ => {
                let x = max_x - self.safe_margin;
                let y = rng.random_range(min_y..max_y);
                let vel = Vec2::new(-self.speed, rng.random_range(-self.speed..self.speed));
                (x, y, vel, "RIGHT")
            }
        };

        info!(
            "Enemy {} spawning from {} at ({:.0}, {:.0})",
            enemy_id, edge_name, x, y
        );

        let clamped_x = x.clamp(min_x, max_x);
        let clamped_y = y.clamp(min_y, max_y);

        commands.spawn((
            Sprite::from_image(texture),
            Transform {
                translation: Vec3::new(clamped_x, clamped_y, 10.0),
                scale: Vec3::splat(ENEMY_SIZE_SCALE),
                ..Default::default()
            },
            RigidBody::Dynamic,
            Collider::ball(16.0),
            Velocity::linear(velocity),
            Restitution {
                coefficient: 1.0,
                combine_rule: CoefficientCombineRule::Max,
            },
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            GravityScale(0.0),
            Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            },
            Ccd::enabled(),
            Enemy,
        ));
    }
}
