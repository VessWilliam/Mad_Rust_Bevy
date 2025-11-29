use bevy::prelude::*;

use crate::game::player::resources::{GameTexture, PlayerConfig};

use self::systems::{
    adjust_player_damping, player_keyboard_event, set_player_texture, spawn_player,
    update_coyote_time,
};

mod components;
mod constants;
mod helper;
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
            .add_systems(
                Update,
                (
                    update_coyote_time,
                    adjust_player_damping,
                    player_keyboard_event.after(update_coyote_time),
                ),
            );
    }
}
