use std::collections::HashMap;

use prost_types::Any;
use tonic::Status;

use crate::{
    gen::lekko::client::v1beta1::Value,
    gen::lekko::feature::v1beta1::{Constraint, Feature},
};

use super::rules_v3::check_rule as check_rule_v3;

// Meta-level context for evaluation, separate from the context given
// by users as part of the evaluation request.
// Information required to evaluate rules but not part of the feature's
// context.
pub struct EvalContext {
    pub namespace: String,
    pub feature_name: String,
}

// Performs evaluation of the feature tree using the given context.
pub fn evaluate(
    feature: &Feature,
    context: &HashMap<String, Value>,
    eval_context: &EvalContext,
) -> Result<(Any, Vec<usize>), Status> {
    let tree = feature
        .tree
        .as_ref()
        .ok_or_else(|| Status::internal("empty tree"))?;
    for (i, constraint) in tree.constraints.iter().enumerate() {
        if let Some((child_val, child_path)) = traverse(constraint, context, eval_context)? {
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
            .ok_or_else(|| Status::internal("empty default value"))?
            .clone(),
        Vec::new(),
    ))
}

// PassedEvaluation contains information when a traversal of a portion of the feature tree passes.
// It contains an optional default value, and a path of which subtrees were evaluated.
// If the default value is None, the caller's default value is meant to be returned.
type PassedEvaluation = (Option<Any>, Vec<usize>);

// traverse is a recursive function that performs tree-traversal on the feature tree,
// evaluating the rules along the way.
// The option of PassedEvaluation denotes whether the traversal of this constraint resulted
// in a pass.
fn traverse(
    constraint: &Constraint,
    context: &HashMap<String, Value>,
    eval_context: &EvalContext,
) -> Result<Option<PassedEvaluation>, Status> {
    let passes = match &constraint.rule_ast_new {
        Some(ast) => check_rule_v3(ast, context, eval_context)?,
        None => return Err(Status::internal("empty rule v3")),
    };
    if !passes {
        // if the rule fails, we avoid further traversal
        return Ok(None);
    }
    // rule passed
    for (i, child) in constraint.constraints.iter().enumerate() {
        if let Some((child_val, child_path)) = traverse(child, context, eval_context)? {
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
