use super::standard_units::{Temperature, Volume};
use bevy::{ecs::entity::MapEntities, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Component, Debug)]
pub struct Registered;

#[derive(Component, Debug, Reflect)]
pub enum ChemistryRule {
    Mix {
        consumes: Vec<(Entity, Volume)>,
        produces: Vec<(Entity, Volume)>,
    },
}

#[derive(Debug, Deserialize, FromReflect, Reflect, Serialize)]
pub enum Req {
    Present(Entity),
    StirClockwise,
    StirCounterClockwise,
}

#[derive(Component, Deserialize, Debug, Reflect, Serialize)]
pub struct ChemistryRuleReqs(pub Vec<Req>);

/// Component on ingredients to track related rules
#[derive(Component, Reflect)]
pub struct TriggerFor(pub Vec<Entity>);

impl MapEntities for TriggerFor {
    fn map_entities(
        &mut self,
        entity_map: &bevy::ecs::entity::EntityMap,
    ) -> Result<(), bevy::ecs::entity::MapEntitiesError> {
        for entity in self.0.iter_mut() {
            if let Ok(mapped_entity) = entity_map.get(*entity) {
                *entity = mapped_entity;
            }
        }
        return Ok(());
    }
}
