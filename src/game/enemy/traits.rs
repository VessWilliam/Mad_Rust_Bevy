use crate::game::core::traits::SpawnBoundTrait;
use bevy::prelude::*;

pub trait EnemySpawner {
    fn spawn_enemy_default_config<B: SpawnBoundTrait>(
        commands: &mut Commands,
        texture: Handle<Image>,
        spawn_bounds: &B,
        enemy_id: i32,
    );
}
