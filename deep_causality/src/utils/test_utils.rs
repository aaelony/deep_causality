// Copyright (c) "2023" . Marvin Hansen <marvin.hansen@gmail.com> All rights reserved.

use std::array;
use std::collections::HashMap;

use crate::errors::CausalityError;
use crate::prelude::*;
use crate::types::alias_types::{DescriptionValue, EvalFn, IdentificationValue, NumericalValue};
use crate::types::reasoning_types;

pub fn get_test_assumption_coll()
    -> Vec<Assumption>
{
    let a1 = get_test_assumption();
    let a2 = get_test_assumption();
    let a3 = get_test_assumption();
    Vec::from_iter([a1, a2, a3])
}

pub fn get_test_assumption_map()
    -> HashMap<i8, Assumption>
{
    let a1 = get_test_assumption();
    let a2 = get_test_assumption();
    let a3 = get_test_assumption();
    HashMap::from_iter([(1, a1), (2, a2), (3, a3)])
}

pub fn get_test_obs_coll()
    -> Vec<Observation>
{
    let o1 = Observation::new(0, 10.0, 1.0);
    let o2 = Observation::new(1, 10.0, 1.0);
    let o3 = Observation::new(2, 10.0, 1.0);
    let o4 = Observation::new(3, 12.0, 0.0);
    let o5 = Observation::new(4, 14.0, 0.0);
    Vec::from_iter([o1, o2, o3, o4, o5])
}

pub fn get_test_inf_coll()
    -> Vec<Inference>
{
    let i1 = get_test_inferable(0, true);
    let i2 = get_test_inferable(1, false);
    Vec::from_iter([i1, i2])
}

pub fn get_test_causality_array() -> [Causaloid; 10]
{
// Causaloid doesn't implement Copy hence the from_fn workaround for array initialization
    array::from_fn(|_| get_test_causaloid())
}

pub fn get_test_causality_coll()
    -> Vec<Causaloid>
{
    let q1 = get_test_causaloid();
    let q2 = get_test_causaloid();
    let q3 = get_test_causaloid();
    Vec::from_iter([q1, q2, q3])
}

pub fn get_test_causality_map()
// i8 as key b/c I assume all testing will be done with less than 265 items.
    -> HashMap<i8, Causaloid>
{
    let q1 = get_test_causaloid();
    let q2 = get_test_causaloid();
    let q3 = get_test_causaloid();
    HashMap::from_iter([(1, q1), (2, q2), (3, q3)])
}


pub fn get_test_causaloid()
    -> Causaloid
{
    let id: IdentificationValue = 1;
    let description: String = "tests whether data exceeds threshold of 0.55".to_string() as DescriptionValue;
    let data_set_id = "Test data".to_string() as DescriptionValue;
    let inferable_coll = get_inferable_coll(false);
    let inverse_inferable_coll = get_inferable_coll(true);

    fn causal_fn(obs: NumericalValue) -> Result<bool, CausalityError> {
        if obs.is_nan() {
            return Err(CausalityError("Observation is NULL/NAN".into()));
        }

        if obs.is_infinite() {
            return Err(CausalityError("Observation is infinite".into()));
        }

        if obs.is_sign_negative() {
            return Err(CausalityError("Observation is negative".into()));
        }

        let threshold: NumericalValue = 0.55;
        if !obs.ge(&threshold) {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    reasoning_types::causable::build_causaloid(
        id,
        causal_fn,
        description,
        data_set_id,
        &inferable_coll,
        &inverse_inferable_coll,
    )
        .unwrap()
}


pub fn get_inferable_coll(
    inverse: bool
)
    -> Vec<Inference>
{
    let i1 = get_test_inferable(0, inverse);
    let i2 = get_test_inferable(1, inverse);
    let i3 = get_test_inferable(1, inverse);
    Vec::from_iter([i1, i2, i3])
}

pub fn get_test_inferable(
    id: IdentificationValue,
    inverse: bool,
)
    -> Inference
{
    let question = "".to_string() as DescriptionValue;
    let all_obs = get_test_obs_coll();

    if inverse
    {
        let target_threshold = 11.0;
        let target_effect = 0.0;
        let observation = all_obs.percent_observation(target_threshold, target_effect);
        let threshold = 0.55;
        let effect = 0.0;// false
        let target = 0.0;// false

        Inference::new(id, question, observation, threshold, effect, target)
    } else {
        let target_threshold = 10.0;
        let target_effect = 1.0;
        let observation = all_obs.percent_observation(target_threshold, target_effect);
        let threshold = 0.55;
        let effect = 1.0; //true
        let target = 1.0; //true

        Inference::new(id, question, observation, threshold, effect, target)
    }
}

pub fn get_test_observation()
    -> Observation
{
    Observation::new(0, 14.0, 1.0)
}

pub fn get_test_assumption()
    -> Assumption
{
    let id: IdentificationValue = 1;
    let description: String = "Test assumption that data are there".to_string() as DescriptionValue;
    let assumption_fn: EvalFn = test_has_data;

    Assumption::new(id, description, assumption_fn)
}

fn test_has_data(
    data: &[NumericalValue]
)
    -> bool
{
    !data.is_empty()
}

pub fn get_test_num_array()
    -> [NumericalValue; 10]
{
    [8.4, 8.5, 9.1, 9.3, 9.4, 9.5, 9.7, 9.7, 9.9, 9.9]
}