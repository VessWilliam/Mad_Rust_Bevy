use log::info;
use bevy::prelude::*;
use rand::Rng;
use super::const_string::ENEMY_SIZE_SCALE;

use super::traits::EnemySpawner;
use super::components::{ EdgeEnemySpawner, Enemy };
use bevy_rapier2d::prelude::*;

use crate::game::tiled::resources::SpawnBounds;

impl EnemySpawner for EdgeEnemySpawner {
    fn spawn_enemy(
        &self,
        commands: &mut Commands,
        texture: Handle<bevy::image::Image>,
        spawn_bounds: &SpawnBounds,
        enemy_id: i32
    ) {
        let mut rng = rand::rng();

        let safe_margin = 60.0;
        let min_x = safe_margin;
        let min_y = safe_margin;
        let max_x = spawn_bounds.width - safe_margin;
        let max_y = spawn_bounds.height - safe_margin;

        let speed = 150.0;
        let edge = rng.random_range(0..4);

        let (x, y, velocity, edge_name) = match edge {
            0 => {
                let x = rng.random_range(min_x..max_x);
                let y = max_y - 10.0;
                let vel = Vec2::new(rng.random_range(-speed..speed), -speed);
                (x, y, vel, "TOP")
            }
            1 => {
                let x = rng.random_range(min_x..max_x);
                let y = min_y + 10.0;
                let vel = Vec2::new(rng.random_range(-speed..speed), speed);
                (x, y, vel, "BOTTOM")
            }
            2 => {
                let x = min_x + 10.0;
                let y = rng.random_range(min_y..max_y);
                let vel = Vec2::new(speed, rng.random_range(-speed..speed));
                (x, y, vel, "LEFT")
            }
            _ => {
                let x = max_x - 10.0;
                let y = rng.random_range(min_y..max_y);
                let vel = Vec2::new(-speed, rng.random_range(-speed..speed));
                (x, y, vel, "RIGHT")
            }
        };

        info!("Enemy {} spawning from {} at ({:.0}, {:.0})", enemy_id, edge_name, x, y);
        commands.spawn((
            Sprite::from_image(texture),
            Transform {
                translation: Vec3::new(x, y, 10.0),
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
