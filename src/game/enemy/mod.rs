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
    systems::{
        set_enemy_texture,
        spawn_enemy,
        debug_enemy_collision,
        speed_limit,
        enemy_bounce_system,
        rotate_enemy_sprite,
    },
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MapFullyLoaded>()
            .init_resource::<MaxSpeed>()
            .insert_resource(GameTexture { enemy: default() })
            .add_systems(Startup, set_enemy_texture)
            .add_systems(Update, (
                spawn_enemy,
                speed_limit,
                debug_enemy_collision,
                enemy_bounce_system,
                rotate_enemy_sprite,
            ));
    }
}
