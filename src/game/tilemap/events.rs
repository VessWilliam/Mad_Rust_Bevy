use bevy::prelude::*;

#[derive(Message)]
pub struct MapFullyLoaded {
    pub map_entity: Entity,
}
