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
    feature: &Feature,
    context: &HashMap<String, Value>,
) -> Result<(Any, Vec<usize>), Status> {
    let tree = feature
        .tree
        .as_ref()
        .ok_or(Status::internal("empty tree"))?;
    for (i, constraint) in tree.constraints.iter().enumerate() {
        if let Some((child_val, child_path)) = traverse(constraint, context)? {
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
    Ok((
        tree.default
            .as_ref()
            .ok_or(Status::internal("empty default value"))?
            .clone(),
        Vec::new(),
    ))
}

// traverse is a recursive function that performs tree-traversal on the feature tree,
// evaluating the rules along the way.
fn traverse(
    constraint: &Constraint,
    context: &HashMap<String, Value>,
) -> Result<Option<(Option<Any>, Vec<usize>)>, Status> {
    let passes = check_rule(
        constraint
            .rule_ast
            .as_ref()
            .ok_or(Status::internal("empty rule ast"))?,
        &context,
    )?;
    if !passes {
        // if the rule fails, we avoid further traversal
        return Ok(None);
    }
    // rule passed
    for (i, child) in constraint.constraints.iter().enumerate() {
        if let Some((child_val, child_path)) = traverse(child, context)? {
            if let Some(some_child_val) = child_val {
                return Ok(Some((
                    Some(some_child_val),
                    itertools::concat(vec![vec![i; 1], child_path]),
                )));
            }
            break; // a child node passed, but no value was present. return the current node's value instead.
        }
        // Child evaluation did not pass, continue iterating
    }
    Ok(Some((constraint.value.clone(), Vec::new())))
}
