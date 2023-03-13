use super::standard_units::Temperature;
use super::AlchemyTool;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Component, Debug, Deserialize, Serialize, Reflect)]
pub struct Cauldron {
    pub ingredients: HashMap<Entity, f32>,
    pub temperature: Temperature,
    total_volume: f32,
    capacity: f32,
}

impl Default for Cauldron {
    fn default() -> Self {
        Cauldron {
            ingredients: Default::default(),
            temperature: Temperature::default(),
            total_volume: 0.0,
            capacity: 62.0,
        }
    }
}

#[derive(Component, Debug, Deserialize, Serialize, Reflect)]
pub enum CauldronEvent {
    AdjustTemperature(Entity, Temperature),
    StirClockwise(Entity),
    StirCounterClockwise(Entity),
    Add {
        cauldron: Entity,
        ingredient: Entity,
        liters: f32,
    },
}

impl CauldronEvent {
    pub fn get_cauldron_entity(&self) -> Entity {
        *match self {
            CauldronEvent::AdjustTemperature(e, _) => e,
            CauldronEvent::StirClockwise(e) => e,
            CauldronEvent::StirCounterClockwise(e) => e,
            CauldronEvent::Add { cauldron, .. } => cauldron,
        }
    }
}

/*
impl AlchemyTool for Cauldron {

}
*/