use bevy::prelude::*;

mod builder;
mod components;
mod constants;
mod resources;
mod systems;
mod traits;

use crate::game::tilemap::events::MapFullyLoaded;

use self::{
    resources::{GameTexture, MaxSpeed},
    systems::{
        enemy_bounce_system, keep_enemies_in_bound, rotate_enemy_sprite, set_enemy_texture,
        spawn_enemy, speed_limit,
    },
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MapFullyLoaded>()
            .init_resource::<MaxSpeed>()
            .insert_resource(GameTexture { enemy: default() })
            .add_systems(Startup, set_enemy_texture)
            .add_systems(
                Update,
                (
                    spawn_enemy,
                    speed_limit,
                    enemy_bounce_system,
                    rotate_enemy_sprite,
                    keep_enemies_in_bound,
                ),
            );
    }
}
