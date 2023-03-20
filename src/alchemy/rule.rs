use std::marker::PhantomData;

use super::standard_units::Volume;
use super::AlchemyTool;
use bevy::{ecs::entity::MapEntities, prelude::*};
use serde::{Deserialize, Serialize};

// TODO Use this component to make sure rule ingredient components are updated. Might need to make it generic.
#[derive(Component, Debug)]
pub struct Registered;

#[derive(Component, Debug, Deserialize, Reflect, Serialize)]
pub struct AlchemyRuleReqs<T>
where
    T: AlchemyTool,
{
    vec: Vec<T::Req>,
}

#[derive(Component, Debug, Deserialize, Reflect, Serialize)]
pub struct AlchemyRule<T: AlchemyTool> {
    pub reqs: Vec<T::Req>,
    pub consumes: Vec<(Entity, Volume)>,
    pub produces: Vec<(Entity, Volume)>,
    // TODO other_effects: Vec<T::Effects> // Such as making the cauldron hotter
}

impl<T: AlchemyTool> Default for AlchemyRule<T> {
    fn default() -> Self {
        AlchemyRule {
            reqs: vec![],
            consumes: vec![],
            produces: vec![],
        }
    }
}

impl<T: AlchemyTool> AlchemyRule<T> {
    fn reqs_satisfied(&self, tool: &T) -> bool {
        if !self.reqs.iter().all(|req| tool.req_satisfied(req)) {
            return false;
        }
        self.consumes
            .iter()
            .all(|(consumed_ingredient, _)| tool.ingredients().contains_key(consumed_ingredient))
    }

    pub fn apply_rule(&self, tool: &mut T) {
        if self.reqs_satisfied(tool) {
            let ratio = self
                .consumes
                .iter()
                .fold(1.0f32, |max_ratio, (ingred_id, volume)| {
                    let available_ratio = tool
                        .ingredients()
                        .get(ingred_id)
                        .map(|available_volume| available_volume / volume)
                        .unwrap_or(0.0);
                    max_ratio.min(available_ratio)
                });
            if ratio > 0.0 {
                for (ingred_id, volume) in self.consumes.iter() {
                    let ingred_volume = tool.ingredients_mut().get_mut(ingred_id).unwrap();
                    *ingred_volume -= volume * ratio;
                    if *ingred_volume <= 0.0 {
                        tool.ingredients_mut().remove(ingred_id);
                    }
                }
                for (ingred_id, volume) in self.produces.iter() {
                    *tool.ingredients_mut().entry(*ingred_id).or_default() += volume * ratio;
                }
            }
        }
    }
}

/// Might consider rename
/// Component on ingredients to track related rules
/// T is the alchemy tool where this rule applies
#[derive(Component, Reflect)]
pub struct AlchemyTrigger<T>(pub Vec<Entity>, PhantomData<T>);

impl<T> AlchemyTrigger<T> {
    pub fn new(rules: Vec<Entity>) -> AlchemyTrigger<T> {
        AlchemyTrigger(rules, PhantomData)
    }
}

impl<T: AlchemyTool> MapEntities for AlchemyTrigger<T> {
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
