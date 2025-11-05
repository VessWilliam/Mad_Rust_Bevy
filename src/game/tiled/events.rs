use bevy::prelude::*;

#[derive(Event)]
pub struct MapFullyLoaded{
    pub map_entity: Entity
}