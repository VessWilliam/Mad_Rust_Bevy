use crate::game::tiled::resources::SpawnBounds;
use bevy::prelude::*;

pub trait EnemySpawner {
    fn spawn_enemy(
        &self,
        commands: &mut Commands,
        texture: Handle<Image>,
        spawn_bounds: &SpawnBounds,
        enemy_id: i32,
    );
}
