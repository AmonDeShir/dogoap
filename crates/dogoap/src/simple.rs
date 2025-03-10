use crate::prelude::*;

pub fn simple_action<T>(name: &str, key_to_mutate: &str, from_value: T) -> Action
where
    Datum: From<T>,
{
    simple_multi_mutate_action(name, vec![(key_to_mutate, from_value)])
}

pub fn simple_multi_mutate_action<T>(name: &str, muts: Vec<(&str, T)>) -> Action
where
    Datum: From<T>,
{
    let mut mutators = vec![];

    for m in muts {
        mutators.push(Mutator::Set(m.0.to_string(), m.1.into()));
    }

    Action {
        key: name.to_string(),
        preconditions: vec![],
        dynamic_preconditions: vec![],
        effects: vec![Effect {
            action: name.to_string(),
            mutators,
            state: LocalState::new(),
            cost: 1,
        }],
    }
}

pub fn simple_increment_action<T>(name: &str, key_to_mutate: &str, from_value: T) -> Action
where
    Datum: From<T>,
{
    let mut action = simple_multi_mutate_action(name, vec![]);
    action.effects = vec![Effect {
        action: name.to_string(),
        mutators: vec![Mutator::Increment(
            key_to_mutate.to_string(),
            from_value.into(),
        )],
        state: LocalState::new(),
        cost: 1,
    }];
    action
}

pub fn simple_decrement_action<T>(name: &str, key_to_mutate: &str, from_value: T) -> Action
where
    Datum: From<T>,
{
    let mut action = simple_multi_mutate_action(name, vec![]);
    action.effects = vec![Effect {
        action: name.to_string(),
        mutators: vec![Mutator::Decrement(
            key_to_mutate.to_string(),
            from_value.into(),
        )],
        state: LocalState::new(),
        cost: 1,
    }];
    action
}
