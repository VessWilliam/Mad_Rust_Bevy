use bevy::prelude::*;
use bevy_ecs_tiled::prelude::tiled::Map;

use super::components::MapMetadata;

pub trait CollisionBuilderTrait {
    fn build_collision(&mut self, map: &Map);
    fn spawn_collider(&self, commands: &mut Commands, map: &MapMetadata);
}
