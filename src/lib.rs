use serde::{Deserialize, Serialize};
use bevy::prelude::*;
use std::collections::HashMap;


#[derive(Debug, Deserialize, Serialize, Reflect)]
pub struct Temperature(u16);

impl Temperature {
    const ABSOLUTE_ZERO: Temperature = Temperature(0);
    const FREEZING: Temperature = Temperature(273);
    const ROOM_TEMPERATURE: Temperature = Temperature(293);
    const BOILING: Temperature = Temperature(373);
}

impl Default for Temperature {
    fn default() -> Self {
        Self::ROOM_TEMPERATURE
    }
}

#[derive(Component, Debug, Deserialize, Serialize, Reflect)]
pub struct Cauldron {
    ingredients: HashMap<Entity, f32>,
    temperature: Temperature,
    total_volume: f32,
    capacity: f32,
}

impl Default for Cauldron {
    fn default() -> Self {
        Cauldron {
            ingredients: default(),
            temperature: default(),
            total_volume: 0.0,
            capacity: 62.0,
        }
    }
}


#[derive(Component, Debug, Deserialize, Serialize, Reflect)]
pub enum CauldronEvent {
    AdjustTemperature(Entity, u16),
    StirClockwise(Entity),
    StirCounterClockwise(Entity),
    Add {
        cauldron: Entity,
        ingredient: Entity,
        liters: f32,
    }
}

impl CauldronEvent {
    fn get_cauldron_entity(&self) -> Entity {
        *match self {
            CauldronEvent::AdjustTemperature(e, _) => e,
            CauldronEvent::StirClockwise(e) => e,
            CauldronEvent::StirCounterClockwise(e) => e,
            CauldronEvent::Add { cauldron, ..} => cauldron,
        }
    }
}

#[derive(Component, Debug, Deserialize, Serialize, Reflect)]
pub struct Ingredient {
    pub concentration_per_liter: f32,
    pub color: Color,
    pub mixes: bool,
}

#[derive(Debug, Deserialize, Serialize, Reflect)]
struct IngredientId(usize);

impl Ingredient {
    fn new(concentration_per_liter: f32, color: Color, mixes: bool) -> Ingredient {
        Ingredient { concentration_per_liter, color, mixes }
    }
}

pub fn add_default_ingredients(commands: &mut Commands) {
    commands.spawn((
        Ingredient::new(0.5, Color::RED, true),
        Name::new("Pepper"),
        StirClockwiseCauldronAction
    ));
    commands.spawn((
        Ingredient::new(1.5, Color::GREEN, true),
        Name::new("Eye of Newt")
    ));
    commands.spawn((
        Ingredient::new(1.5, Color::GREEN, true),
        Name::new("Eye of Newt")
    ));
    commands.spawn((
        Ingredient::new(1.5, Color::Rgba { red: 0.9, green: 0.9, blue: 0.9, alpha: 0.95}, true),
        Name::new("Water")
    ));
    commands.spawn((
        Ingredient::new(1.5, Color::BLACK, false),
        Name::new("A Rock")
    ));
}

type Volume = f32;

enum CauldronEffect {
    RemoveIngredient(Entity, Volume),
    ProduceIngredient(Entity, Volume),
    RaiseHeatBy(Temperature),
    ReduceHeatBy(Temperature),
}

trait CauldronAction {
    fn applicable_event(event: &CauldronEvent) -> bool;
    fn apply(&self, cauldron: &Cauldron) -> Vec<CauldronEffect>;
}

#[derive(Component, Debug, Serialize, Deserialize)]
struct StirClockwiseCauldronAction;

impl CauldronAction for StirClockwiseCauldronAction {
    fn applicable_event(event: &CauldronEvent) -> bool{
        matches!(event, CauldronEvent::StirClockwise(_))
    }

    fn apply(&self, cauldron: &Cauldron) -> Vec<CauldronEffect> {
        vec![CauldronEffect::RaiseHeatBy(Temperature(10))]
    }
}

fn cauldron_actions_to_effects_system<T: CauldronAction + Component>(mut events: EventReader<CauldronEvent>, cauldrons: Query<&Cauldron>, actions: Query<&T, With<Ingredient>>) {
    for event in events.iter() {
        if T::applicable_event(event) {
            // TODO what to do if cauldron doesn't exist?
            if let Ok(cauldron) = cauldrons.get(event.get_cauldron_entity()) {
                let effects = actions
                    .iter_many(cauldron.ingredients.keys())
                    .flat_map(|action|action.apply(cauldron));

            }
        }
    }
}

/*
StirClockwise
-- Rule Entity
--> [1, 2] -> Mix
 */