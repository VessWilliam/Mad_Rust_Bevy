use bevy::prelude::*;

use crate::game::player::resources::{GameTexture, PlayerConfig};

use self::systems::{player_keyboad_event, set_player_texture, spawn_player};

mod components;
mod constants;
mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTexture>()
            .init_resource::<PlayerConfig>()
            .add_systems(
                Startup,
                (set_player_texture, spawn_player.after(set_player_texture)),
            )
            .add_systems(Update, (player_keyboad_event,));
    }
}
