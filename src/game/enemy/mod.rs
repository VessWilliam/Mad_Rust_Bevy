use bevy::prelude::*;

mod components;
mod const_string;
mod resources;
mod systems;
mod builder;
mod traits;

use crate::game::tiled::events::MapFullyLoaded;

use self::{
    resources::{ GameTexture, MaxSpeed },
    systems::{ set_enemy_texture, spawn_enemy, debug_enemy_collision, speed_limit, debug_camera },
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MapFullyLoaded>()
            .insert_resource(GameTexture { enemy: default() })
            .insert_resource(MaxSpeed { max_speed: 200.0 })
            .add_systems(Startup, (set_enemy_texture, debug_camera))
            .add_systems(Update, (spawn_enemy, speed_limit, debug_enemy_collision));
    }
}
