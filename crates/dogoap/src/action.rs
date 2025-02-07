use std::fmt::Debug;
// crate "dogoap" file action.rs
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use bevy_reflect::Reflect;

use crate::compare::Compare;
use crate::effect::Effect;
use crate::localstate::LocalState;
use crate::mutator::Mutator;

/// An `Action` represents something your Entity can do, granted the LocalState
/// is as defined in the `preconditions`. It has a list of `Effect`s that apply
/// if the NPC successfully executed the task.
#[derive(Reflect, Clone, Default)]
pub struct Action {
    /// String like `eat_action`
    pub key: String,
    // TODO arguments coupled with Effects, maybe
    // pub argument: Option<Datum>,
    /// What preconditions need to be true before we can execute this action
    pub preconditions: Vec<(String, Compare)>,
    /// What preconditions need to be true before we can execute this action
    pub dynamic_preconditions: Vec<(String, Arc<dyn Fn(&LocalState) -> Compare + Send + Sync>)>,
    /// What is the outcome from doing this action
    // TODO temporarily plural effects, as maybe we want to implement arguments with many effects...
    pub effects: Vec<Effect>,
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        f.debug_struct("Action")
            .field("key", &self.key)
            .field("preconditions", &self.preconditions)
            .field("dynamic_preconditions", &self.get_dynamic_precondition())
            .field("effects", &self.effects)
            .finish()
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key &&
        self.preconditions == other.preconditions &&
        self.effects == other.effects &&
        self.get_dynamic_precondition() == other.get_dynamic_precondition()
    }
}

impl Hash for Action {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.preconditions.hash(state);
        self.effects.hash(state);
    }
}

impl Action {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            preconditions: vec![],
            dynamic_preconditions: vec![],
            effects: vec![],
        }
    }

    fn get_dynamic_precondition(&self) -> Vec<String> {
        self.dynamic_preconditions.iter().map(|(k, _)| k.to_string()).collect()
    }

    pub fn with_precondition(mut self, key: &str, compare: Compare) -> Self {
        self.preconditions.push((key.to_string(), compare));
        self
    }

    pub fn with_effect(mut self, effect: Effect) -> Self {
        self.effects.push(effect);
        self
    }

    pub fn add_precondition(mut self, precondition: (String, Compare)) -> Self {
        self.preconditions.push(precondition);
        self
    }

    pub fn add_dynamic_precondition(mut self, precondition: (String, Arc<dyn Fn(&LocalState) -> Compare + Send + Sync>)) -> Self {
        self.dynamic_preconditions.push(precondition);
        self
    }

    pub fn get_preconditions(&self, state: &LocalState) -> Vec<(String, Compare)> {
        let mut preconditions = self.preconditions.clone();

        for (key, getter) in &self.dynamic_preconditions {
            preconditions.push((key.clone(), getter(state)));
        }

        preconditions
    }

    // TODO currently only handles one effect
    pub fn add_mutator(mut self, mutator: Mutator) -> Self {
        if self.effects.len() == 0 {
            self.effects = vec![Effect::new(&self.key.clone()).with_mutator(mutator)];
        } else {
            let mut effect = self.effects[0].clone();
            effect.mutators.push(mutator);
            self.effects[0] = effect;
        }
        self
    }

    pub fn set_cost(mut self, new_cost: usize) -> Self {
        let mut effect = self.effects[0].clone();
        effect.cost = new_cost;
        self.effects[0] = effect;
        self
    }
}
