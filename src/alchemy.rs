mod cauldron;
pub mod rule;
mod standard_units;

pub use cauldron::{Cauldron, CauldronEvent, Req as CauldronReq};
use rule::{AlchemyRule, AlchemyTrigger};
use standard_units::Volume;

use bevy::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub trait AlchemyTool: Component + FromReflect {
    type Req: DeserializeOwned + Serialize + Send + Sync;

    fn ingredients(&self) -> &HashMap<Entity, Volume>;
    fn ingredients_mut(&mut self) -> &mut HashMap<Entity, Volume>;
    fn capacity(&self) -> Option<Volume>;
    fn req_satisfied(&self, req: &Self::Req) -> bool;
    fn post_update(&mut self);
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
            AlchemyRule::<Cauldron> {
                reqs: vec![CauldronReq::StirCounterClockwise],
                consumes: vec![(eye_of_newt, 1.0), (pepper, 1.0)],
                produces: vec![(newt_pepper, 2.0)],
            },
        ))
        .id();
    let rule_2 = commands
        .spawn((
            Name::new("Stir CW + Eye of Newt + Pepper = More Pepper"),
            AlchemyRule::<Cauldron> {
                reqs: vec![CauldronReq::StirClockwise],
                consumes: vec![(eye_of_newt, 1.0), (pepper, 1.0)],
                produces: vec![(pepper, 1.5)],
            },
        ))
        .id();
    let rule_3 = commands
        .spawn((
            Name::new("Newt Pepper with A Rock evaporates"),
            // TODO Swap this with a ChemistryRule::Evaporate?
            AlchemyRule::<Cauldron> {
                reqs: vec![CauldronReq::Present(a_rock)],
                consumes: vec![(newt_pepper, 0.03)],
                ..default()
            },
        ))
        .id();
    let rule_4 = commands
        .spawn((
            Name::new("Dried Dryness + Water = Nothing"),
            AlchemyRule::<Cauldron> {
                consumes: vec![(dried_dryness, 0.05), (water, 1.0)],
                ..default()
            },
        ))
        .id();

    commands
        .entity(pepper)
        .insert(AlchemyTrigger::<Cauldron>::new(vec![rule_1, rule_2]));
    commands
        .entity(eye_of_newt)
        .insert(AlchemyTrigger::<Cauldron>::new(vec![rule_1, rule_2]));
    commands
        .entity(newt_pepper)
        .insert(AlchemyTrigger::<Cauldron>::new(vec![rule_3]));
    commands
        .entity(a_rock)
        .insert(AlchemyTrigger::<Cauldron>::new(vec![rule_3]));
    commands
        .entity(water)
        .insert(AlchemyTrigger::<Cauldron>::new(vec![rule_4]));
    commands
        .entity(dried_dryness)
        .insert(AlchemyTrigger::<Cauldron>::new(vec![rule_4]));
}

pub fn adjust_cauldron_state(
    mut cauldrons: Query<(Entity, &mut Cauldron)>,
    mut events: EventReader<CauldronEvent>,
) {
    for (entity, mut cauldron) in cauldrons.iter_mut() {
        cauldron.clockwise_stir = (cauldron.clockwise_stir - 0.25).max(0.0);
        cauldron.counter_clockwise_stir = (cauldron.counter_clockwise_stir - 0.25).max(0.0);
        for event in events.iter() {
            if event.get_cauldron_entity() == entity {
                match event {
                    CauldronEvent::AdjustTemperature(_, temp) => cauldron.temperature = *temp,
                    CauldronEvent::StirClockwise(_) => cauldron.clockwise_stir += 0.05,
                    CauldronEvent::StirCounterClockwise(_) => {
                        cauldron.counter_clockwise_stir += 0.05
                    }
                    CauldronEvent::Add {
                        ingredient, liters, ..
                    } => *cauldron.ingredients.entry(*ingredient).or_default() += liters,
                }
            }
        }
    }
}

pub fn evaluate_chemistry_rules<T: AlchemyTool>(
    mut alchemy_tools: Query<&mut T>,
    ingredients: Query<&AlchemyTrigger<T>, With<Ingredient>>,
    rules: Query<&AlchemyRule<T>>,
) {
    for mut alchemy_tool in alchemy_tools.iter_mut() {
        let relevant_rules: HashSet<Entity> = ingredients
            .iter_many(alchemy_tool.ingredients().keys())
            .flat_map(|tf| &tf.0)
            .copied()
            .collect();
        for rule in rules.iter_many(relevant_rules) {
            rule.apply_rule(&mut alchemy_tool);
        }
        alchemy_tool.post_update();
    }
}
