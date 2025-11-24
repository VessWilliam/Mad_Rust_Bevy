use bevy::prelude::*;

pub mod collision;
pub mod spawn;
pub mod traits;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, _app: &mut App) {}
}
