use super::components::{EdgeSpawner, Enemy};
use super::constants::{ENEMY_SAFE_MARGIN, ENEMY_SIZE_SCALE, ENEMY_SPEED};
use super::traits::EnemySpawner;
use crate::game::core::spawn::{SpawnArea, SpawnEdge};
use crate::game::core::traits::SpawnBoundTrait;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use log::info;

impl EnemySpawner for EdgeSpawner {
    fn spawn_enemy_default_config<B: SpawnBoundTrait>(
        commands: &mut Commands,
        texture: Handle<Image>,
        spawn_bounds: &B,
        enemy_id: i32,
    ) {
        Self::spawn_enemy_internal(
            commands,
            texture,
            spawn_bounds,
            ENEMY_SPEED,
            ENEMY_SAFE_MARGIN,
            enemy_id,
        );
    }
}

impl EdgeSpawner {
    fn spawn_enemy_internal<B: SpawnBoundTrait>(
        commands: &mut Commands,
        texture: Handle<Image>,
        spawn_bounds: &B,
        speed: f32,
        margin: f32,
        enemy_id: i32,
    ) {
        let mut rng = rand::rng();

        let area = SpawnArea::new(
            margin,
            margin,
            spawn_bounds.width() - margin,
            spawn_bounds.height() - margin,
        );

        if !area.is_valid() {
            warn!(
                "Invalid spawn bounds! width={}, height={}",
                spawn_bounds.width(),
                spawn_bounds.height()
            );
            return;
        }

        let edge = SpawnEdge::random(&mut rng);
        let (x, y, velocity) = edge.calculate_spawn(&mut rng, area, margin, speed);

        info!(
            "Enemy {} spawning at {}: ({:.0}, {:.0})",
            enemy_id,
            edge.as_string(),
            x,
            y
        );

        let clamped_x = x.clamp(area.min_x, area.max_x);
        let clamped_y = y.clamp(area.min_y, area.max_y);

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
