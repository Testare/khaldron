use super::standard_units::{Temperature, Volume};
use super::AlchemyTool;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, FromReflect, Reflect, Serialize)]
pub enum Req {
    Present(Entity),
    StirClockwise,
    StirCounterClockwise,
}

#[derive(Component, Debug, Deserialize, FromReflect, Serialize, Reflect)]
pub struct Cauldron {
    pub ingredients: HashMap<Entity, f32>,
    pub temperature: Temperature,
    pub counter_clockwise_stir: f32, // units are Radians/PI
    pub clockwise_stir: f32,         // units are Radians/PI
    capacity: f32,
    total_volume: f32,
}

impl Default for Cauldron {
    fn default() -> Self {
        Cauldron {
            ingredients: Default::default(),
            temperature: Temperature::default(),
            counter_clockwise_stir: Default::default(),
            clockwise_stir: Default::default(),
            capacity: 0.0,
            total_volume: 62.0,
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

impl AlchemyTool for Cauldron {
    type Req = Req;

    fn ingredients(&self) -> &HashMap<Entity, Volume> {
        &self.ingredients
    }
    fn ingredients_mut(&mut self) -> &mut HashMap<Entity, Volume> {
        &mut self.ingredients
    }
    fn capacity(&self) -> Option<Volume> {
        Some(self.capacity)
    }
    fn req_satisfied(&self, req: &Self::Req) -> bool {
        match req {
            Req::Present(ingredient) => self
                .ingredients
                .get(ingredient)
                .map(|v| *v != 0.0)
                .unwrap_or(false),
            Req::StirClockwise => self.clockwise_stir > 0.0,
            Req::StirCounterClockwise => self.counter_clockwise_stir > 0.0,
        }
    }

    fn post_update(&mut self) {
        self.clockwise_stir = (self.clockwise_stir - 0.05).max(0.0);
        self.counter_clockwise_stir = (self.counter_clockwise_stir - 0.05).max(0.0);
    }
}
