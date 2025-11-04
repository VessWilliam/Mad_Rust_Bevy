use bevy::prelude::*;

use self::{
    resources::{ BounceState, GameTexture, MaxSpeed, SpeedUp, SpeedMultiplier },
    systems::{ set_enemy_texture, spawn_enemy },
};

mod components;
mod const_string;
pub(crate) mod resources;
mod systems;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTexture { enemy: default() })
            .insert_resource(BounceState { is_bounce: false })
            .insert_resource(MaxSpeed { max_speed: 200.0 })
            .insert_resource(SpeedUp { speed_up: 1.0 })
            .insert_resource(SpeedMultiplier { speed_multiplier: 2.0 })
            .add_systems(Startup, set_enemy_texture)
            .add_observer(spawn_enemy);
    }
}
