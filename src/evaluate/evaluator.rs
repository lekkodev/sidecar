use std::collections::HashMap;

use prost_types::Any;
use tonic::Status;

use crate::gen::lekko::{
    backend::v1beta1::Value,
    feature::v1beta1::{Constraint, Feature},
};

use super::rules::check_rule;

// Performs evaluation of the feature tree using the given context.
pub fn evaluate(
    feature: Feature,
    context: HashMap<String, Value>,
) -> Result<(Any, Vec<usize>), Status> {
    let tree = feature.tree.ok_or(Status::internal("empty tree"))?;
    let default_value = tree
        .default
        .ok_or(Status::internal("empty default value"))?;
    for (i, constraint) in tree.constraints.iter().enumerate() {
        let (child_val, child_passes, child_path) = traverse(constraint, context.clone())?;
        if child_passes {
            if let Some(some_child_val) = child_val {
                return Ok((
                    some_child_val,
                    itertools::concat(vec![vec![i; 1], child_path]),
                ));
            }
            break; // a child node passed, but no value was present. return the default value instead.
        }
        // Child evaluation did not pass, continue iterating
    }
    Ok((default_value, Vec::new()))
}

// traverse is a recursive function that performs tree-traversal on the feature tree,
// evaluating the rules along the way.
fn traverse(
    constraint: &Constraint,
    context: HashMap<String, Value>,
) -> Result<(Option<Any>, bool, Vec<usize>), Status> {
    let passes = check_rule(
        constraint
            .rule_ast
            .clone()
            .ok_or(Status::internal("empty rule ast"))?,
        context.clone(),
    )?;
    if !passes {
        // if the rule fails, we avoid further traversal
        return Ok((None, false, Vec::new()));
    }
    // rule passed
    for (i, child) in constraint.constraints.iter().enumerate() {
        let (child_val, child_passes, child_path) = traverse(child, context.clone())?;
        if child_passes {
            if let Some(some_child_val) = child_val {
                return Ok((
                    Some(some_child_val),
                    true,
                    itertools::concat(vec![vec![i; 1], child_path]),
                ));
            }
            break; // a child node passed, but no value was present. return the current node's value instead.
        }
        // Child evaluation did not pass, continue iterating
    }
    Ok((constraint.value.clone(), true, Vec::new()))
}
