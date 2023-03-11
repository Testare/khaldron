pub mod rule;
mod standard_units;

use rule::{ChemistryRule, ChemistryRuleReqs, Req, TriggerFor};
use standard_units::{Temperature, Volume};

use bevy::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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
    fn get_cauldron_entity(&self) -> Entity {
        *match self {
            CauldronEvent::AdjustTemperature(e, _) => e,
            CauldronEvent::StirClockwise(e) => e,
            CauldronEvent::StirCounterClockwise(e) => e,
            CauldronEvent::Add { cauldron, .. } => cauldron,
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
        Ingredient {
            concentration_per_liter,
            color,
            mixes,
        }
    }
}

pub fn add_default_ingredients(commands: &mut Commands) {
    let pepper = commands
        .spawn((Ingredient::new(0.5, Color::RED, true), Name::new("Pepper")))
        .id();
    let eye_of_newt = commands
        .spawn((
            Ingredient::new(1.5, Color::GREEN, true),
            Name::new("Eye of Newt"),
        ))
        .id();
    let newt_pepper = commands
        .spawn((
            Ingredient::new(3.0, Color::ORANGE, true),
            Name::new("Newt Pepper"),
        ))
        .id();
    let water = commands
        .spawn((
            Ingredient::new(
                1.5,
                Color::Rgba {
                    red: 0.9,
                    green: 0.9,
                    blue: 0.9,
                    alpha: 0.95,
                },
                true,
            ),
            Name::new("Water"),
        ))
        .id();
    let a_rock = commands
        .spawn((
            Ingredient::new(1.5, Color::BLACK, false),
            Name::new("A Rock"),
        ))
        .id();
    let dried_dryness = commands
        .spawn((
            Ingredient::new(1.5, Color::BLACK, false),
            Name::new("Dried Dryness"),
        ))
        .id();

    let rule_1 = commands
        .spawn((
            Name::new("Stir CCW + Eye of Newt + Pepper = Newt Pepper"),
            ChemistryRule::Mix {
                consumes: vec![(eye_of_newt, 1.0), (pepper, 1.0)],
                produces: vec![(newt_pepper, 2.0)],
            },
            ChemistryRuleReqs(vec![Req::StirCounterClockwise]),
        ))
        .id();
    let rule_2 = commands
        .spawn((
            Name::new("Stir CW + Eye of Newt + Pepper = More Pepper"),
            ChemistryRule::Mix {
                consumes: vec![(eye_of_newt, 1.0), (pepper, 1.0)],
                produces: vec![(pepper, 1.5)],
            },
            ChemistryRuleReqs(vec![Req::StirClockwise]),
        ))
        .id();
    let rule_3 = commands
        .spawn((
            Name::new("Newt Pepper with A Rock evaporates"),
            // TODO Swap this with a ChemistryRule::Evaporate?
            ChemistryRule::Mix {
                consumes: vec![(newt_pepper, 0.03)],
                produces: vec![],
            },
            ChemistryRuleReqs(vec![Req::Present(a_rock)]),
        ))
        .id();
    let rule_4 = commands
        .spawn((
            Name::new("Dried Dryness + Water = Nothing"),
            ChemistryRule::Mix {
                consumes: vec![(dried_dryness, 0.05), (water, 1.0)],
                produces: vec![],
            },
        ))
        .id();

    commands
        .entity(pepper)
        .insert(TriggerFor(vec![rule_1, rule_2]));
    commands
        .entity(eye_of_newt)
        .insert(TriggerFor(vec![rule_1, rule_2]));
    commands
        .entity(newt_pepper)
        .insert(TriggerFor(vec![rule_3]));
    commands.entity(a_rock).insert(TriggerFor(vec![rule_3]));
    commands.entity(water).insert(TriggerFor(vec![rule_4]));
    commands
        .entity(dried_dryness)
        .insert(TriggerFor(vec![rule_4]));
}

pub fn evaluate_chemistry_rules(
    mut cauldrons: Query<(Entity, &mut Cauldron)>,
    mut events: EventReader<CauldronEvent>,
    ingredients: Query<&TriggerFor, With<Ingredient>>,
    rules: Query<(&ChemistryRule, Option<&ChemistryRuleReqs>)>,
) {
    for (cauldron_entity, mut cauldron) in cauldrons.iter_mut() {
        let mut stir_clockwise = false;
        let mut stir_counterclockwise = false;
        for event in events.iter() {
            if event.get_cauldron_entity() == cauldron_entity {
                match event {
                    CauldronEvent::AdjustTemperature(_, temp) => {
                        cauldron.temperature = *temp;
                    }
                    CauldronEvent::StirClockwise(_) => {
                        stir_clockwise = true;
                    }
                    CauldronEvent::StirCounterClockwise(_) => {
                        stir_counterclockwise = true;
                    }
                    CauldronEvent::Add {
                        ingredient, liters, ..
                    } => *cauldron.ingredients.entry(*ingredient).or_default() += liters,
                }
            }
        }
        let relevant_rules: HashSet<Entity> = ingredients
            .iter_many(cauldron.ingredients.keys())
            .flat_map(|tf| &tf.0)
            .copied()
            .collect();
        for (rule, reqs_opt) in rules.iter_many(relevant_rules) {
            if let Some(ChemistryRuleReqs(reqs)) = reqs_opt {
                let reqs_met = reqs.iter().all(|req| match req {
                    Req::Present(ingredient) => cauldron
                        .ingredients
                        .get(ingredient)
                        .map(|v| *v != 0.0)
                        .unwrap_or(false),
                    Req::StirClockwise => stir_clockwise,
                    Req::StirCounterClockwise => stir_counterclockwise,
                });
                if !reqs_met {
                    continue;
                }
            }
            match rule {
                ChemistryRule::Mix { consumes, produces } => {
                    let ratio = consumes
                        .iter()
                        .fold(1.0f32, |max_ratio, (ingred_id, volume)| {
                            let available_ratio = cauldron
                                .ingredients
                                .get(ingred_id)
                                .map(|available_volume| available_volume / volume)
                                .unwrap_or(0.0);
                            max_ratio.min(available_ratio)
                        });
                    if ratio > 0.0 {
                        for (ingred_id, volume) in consumes {
                            let ingred_volume = cauldron.ingredients.get_mut(ingred_id).unwrap();
                            *ingred_volume -= volume * ratio;
                            if *ingred_volume <= 0.0 {
                                cauldron.ingredients.remove(ingred_id);
                            }
                        }
                        for (ingred_id, volume) in produces {
                            *cauldron.ingredients.entry(*ingred_id).or_default() += volume * ratio;
                        }
                    }
                }
            }
        }
    }
}
