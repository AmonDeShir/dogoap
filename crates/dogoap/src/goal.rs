use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use bevy_reflect::*;
use crate::action::Action;
use crate::compare::{compare_values, Compare};
use crate::localstate::LocalState;

/// Goal is a map of what we want our final [`LocalState`](crate::localstate::LocalState) to be, using String as
/// keys and [`Compare`] to assert what we want the [`Datum`](crate::datum::Datum) to be
#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct Goal {
    /// All the requirements needed to be met in order to consider us to be at our final state
    pub requirements: BTreeMap<String, Compare>,

    /// The priority of the goal, determining which goal the planner will focus on.
    /// The goal with the highest priority will be executed first.
    /// If two goals have the same priority, the one added first will be chosen.
    pub priority: usize,
}

impl Hash for Goal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.requirements.len().hash(state);
        for (key, value) in &self.requirements {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl Goal {
    pub fn new() -> Self {
        Self {
            requirements: BTreeMap::new(),
            priority: 0,
        }
    }

    /// The priority of the goal, determining which goal the planner will focus on.
    /// The goal with the highest priority will be executed first.
    /// If two goals have the same priority, the one added first will be chosen.
    pub fn with_priority(mut self, priority: usize) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_req(mut self, key: &str, compare: Compare) -> Self {
        self.requirements.insert(key.to_string(), compare);
        self
    }

    pub fn from_reqs(preconditions: &[(String, Compare)]) -> Goal {
        let mut goal = Goal::new();
        for (k, v) in preconditions {
            goal = goal.with_req(k, v.clone());
        }
        goal
    }
}

/// Checks all the requirements from the `Goal` against the provided `LocalState`.
/// Returns `true` if all the requirements pass (or if there are none), otherwise `false`.
pub fn check_goal(state: &LocalState, goal: &Goal) -> bool {
    goal.requirements.iter().all(|(key, value)| {
        let state_value = state
            .data
            .get(key)
            .unwrap_or_else(|| panic!("Couldn't find key {:#?} in LocalState", key));
        compare_values(value, state_value)
    })
}