use bevy::prelude::*;

use crate::game::player::{
    resources::{ GameTexture, PlayerConfig },
    systems::sync_velocity_to_rapier,
};

use self::{ systems::{ player_keyboad_event, set_player_texture, spawn_player } };

mod components;
mod const_string;
mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTexture>()
            .init_resource::<PlayerConfig>()
            .add_systems(Startup, (set_player_texture, spawn_player.after(set_player_texture)))
            .add_systems(Update, (
                player_keyboad_event,
                sync_velocity_to_rapier.after(player_keyboad_event),
            ));
    }
}
