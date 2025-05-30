use std::collections::HashMap;

use prost_types::{
    value::Kind,
    value::Kind::{BoolValue, NumberValue, StringValue},
};
use tonic::Status;

use crate::gen::sdk::lekko::client::{
    self, v1beta1::value::Kind as LekkoKind, v1beta1::Value as LekkoValue,
};

use crate::gen::cli::lekko::rules::v1beta3::{
    call_expression::Function,
    rule::Rule::{Atom, BoolConst, CallExpression, LogicalExpression, Not},
    ComparisonOperator as CmpOp,
    LogicalOperator::{self, And, Or},
    Rule,
};

use super::evaluator::EvalContext;
use super::functions::bucket;

// TODO: make all error messages contain dynamic variable info.
// check_rule evaluates the rule using the given context to determine whether or not the rule passed.
// it is a recursive method.
pub fn check_rule(
    rule: &Rule,
    context: &HashMap<String, LekkoValue>,
    eval_context: &EvalContext,
) -> Result<bool, Status> {
    let r = rule
        .rule
        .as_ref()
        .ok_or_else(|| Status::internal("empty rule"))?;
    match r {
        // Base case
        BoolConst(b) => Ok(*b),
        // Recursive case
        Not(not_rule) => Ok(!check_rule(not_rule.as_ref(), context, eval_context)?),
        // Recursive case
        LogicalExpression(le) => Ok(check_rules(
            le.rules.as_ref(),
            &le.logical_operator(),
            context,
            eval_context,
        )?),
        // Base case
        Atom(a) => {
            let ctx_key = &a.context_key;
            let present = context.contains_key(ctx_key);
            if a.comparison_operator().eq(&CmpOp::Present) {
                return Ok(present);
            }
            if a.comparison_value.is_none() {
                return Err(Status::internal("empty comparison value"));
            }
            if !present {
                // All other comparison operators expect the context key to be present. If
                // it is not present, return false.
                return Ok(false);
            }
            let rule_kind = a
                .comparison_value
                .as_ref()
                .unwrap()
                .kind
                .as_ref()
                .ok_or_else(|| Status::internal("empty rule value kind"))?;
            let ctx_kind = context
                .get(ctx_key)
                .ok_or_else(|| Status::internal("empty ctx value"))?
                .kind
                .as_ref()
                .ok_or_else(|| Status::internal("empty ctx value kind"))?;
            match a.comparison_operator() {
                CmpOp::Equals => check_equals_cmp(rule_kind, ctx_kind),
                CmpOp::NotEquals => match check_equals_cmp(rule_kind, ctx_kind) {
                    Ok(b) => Ok(!b),
                    Err(e) => Err(e),
                },
                CmpOp::LessThan => check_num_cmp(&a.comparison_operator(), rule_kind, ctx_kind),
                CmpOp::LessThanOrEquals => {
                    check_num_cmp(&a.comparison_operator(), rule_kind, ctx_kind)
                }
                CmpOp::GreaterThan => check_num_cmp(&a.comparison_operator(), rule_kind, ctx_kind),
                CmpOp::GreaterThanOrEquals => {
                    check_num_cmp(&a.comparison_operator(), rule_kind, ctx_kind)
                }
                CmpOp::ContainedWithin => check_list_cmp(rule_kind, ctx_kind),
                CmpOp::StartsWith => check_str_cmp(&a.comparison_operator(), rule_kind, ctx_kind),
                CmpOp::EndsWith => check_str_cmp(&a.comparison_operator(), rule_kind, ctx_kind),
                CmpOp::Contains => check_str_cmp(&a.comparison_operator(), rule_kind, ctx_kind),
                CmpOp::Present => Err(Status::internal("present should be handled above")),
                CmpOp::Unspecified => Err(Status::internal("unknown comparison operator")),
            }
        }
        CallExpression(ce) => {
            let function = ce
                .function
                .as_ref()
                .ok_or_else(|| Status::internal("empty function"))?;
            match function {
                Function::Bucket(bucket_f) => bucket(bucket_f, context, eval_context),
                Function::EvaluateTo(_) => Err(Status::internal(
                    "evaluate_to is not currently supported in the sidecar",
                )),
            }
        }
    }
}

pub fn check_rules(
    rules: &[Rule],
    operator: &LogicalOperator,
    context: &HashMap<String, LekkoValue>,
    eval_context: &EvalContext,
) -> Result<bool, Status> {
    if rules.is_empty() {
        return Err(Status::internal("no rules found in logical expression"));
    }
    let result: Result<Vec<bool>, Status> = rules
        .iter()
        .map(|rule| check_rule(rule, context, eval_context))
        .collect();
    return match (result, operator) {
        (_, LogicalOperator::Unspecified) => Err(Status::internal("unknown logical operator")),
        (Err(e), _) => Err(e),
        (Ok(bools), And) => Ok(bools.iter().all(|b| b.to_owned())),
        (Ok(bools), Or) => Ok(bools.iter().any(|b| b.to_owned())),
    };
}

fn check_equals_cmp(rule_kind: &Kind, ctx_kind: &LekkoKind) -> Result<bool, Status> {
    match rule_kind {
        BoolValue(rule_bool) => match ctx_kind {
            client::v1beta1::value::Kind::BoolValue(ctx_bool) => Ok(rule_bool == ctx_bool),
            _ => Err(Status::invalid_argument("type mismatch")),
        },
        NumberValue(rule_num) => match ctx_kind {
            client::v1beta1::value::Kind::IntValue(ctx_num) => Ok(*rule_num == *ctx_num as f64),
            client::v1beta1::value::Kind::DoubleValue(ctx_num) => Ok(rule_num == ctx_num),
            _ => Err(Status::invalid_argument("type mismatch")),
        },
        StringValue(rule_str) => match ctx_kind {
            client::v1beta1::value::Kind::StringValue(ctx_str) => Ok(rule_str == ctx_str),
            _ => Err(Status::invalid_argument("type mismatch")),
        },
        _ => Err(Status::internal("unsupported rule value kind")),
    }
}

fn check_num_cmp(co: &CmpOp, rule_kind: &Kind, ctx_kind: &LekkoKind) -> Result<bool, Status> {
    let rule_num = get_number(rule_kind)?;
    let ctx_num = get_lekko_number(ctx_kind)?;
    match co {
        CmpOp::LessThan => Ok(ctx_num < rule_num),
        CmpOp::LessThanOrEquals => Ok(ctx_num <= rule_num),
        CmpOp::GreaterThan => Ok(ctx_num > rule_num),
        CmpOp::GreaterThanOrEquals => Ok(ctx_num >= rule_num),
        _ => Err(Status::internal("invalid comparison operator")),
    }
}

fn check_list_cmp(rule_kind: &Kind, ctx_kind: &LekkoKind) -> Result<bool, Status> {
    match rule_kind {
        Kind::ListValue(rule_list) => {
            for rule_elem in &rule_list.values {
                let rule_elem_kind = rule_elem
                    .kind
                    .as_ref()
                    .ok_or_else(|| Status::internal("empty rule value kind"))?;
                let elem_equal = check_equals_cmp(rule_elem_kind, ctx_kind);
                if elem_equal.is_ok() && elem_equal.unwrap() {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        _ => Err(Status::invalid_argument("type mismatch")),
    }
}

fn check_str_cmp(co: &CmpOp, rule_kind: &Kind, ctx_kind: &LekkoKind) -> Result<bool, Status> {
    let rule_str = get_string(rule_kind)?;
    let ctx_str = get_lekko_string(ctx_kind)?;
    match co {
        CmpOp::StartsWith => Ok(ctx_str.starts_with(&rule_str)),
        CmpOp::EndsWith => Ok(ctx_str.ends_with(&rule_str)),
        CmpOp::Contains => Ok(ctx_str.contains(&rule_str)),
        _ => Err(Status::internal("invalid comparison operator")),
    }
}

fn get_number(kind: &Kind) -> Result<f64, Status> {
    match kind {
        NumberValue(num_value) => Ok(*num_value),
        _ => Err(Status::invalid_argument("type mismatch")),
    }
}

fn get_lekko_number(kind: &LekkoKind) -> Result<f64, Status> {
    match kind {
        LekkoKind::IntValue(int_value) => Ok(*int_value as f64),
        LekkoKind::DoubleValue(double_value) => Ok(*double_value),
        _ => Err(Status::invalid_argument("type mismatch")),
    }
}

fn get_string(kind: &Kind) -> Result<String, Status> {
    match kind {
        StringValue(str) => Ok(str.clone()),
        _ => Err(Status::invalid_argument("type mismatch")),
    }
}

fn get_lekko_string(kind: &LekkoKind) -> Result<String, Status> {
    match kind {
        LekkoKind::StringValue(str_value) => Ok(str_value.clone()),
        _ => Err(Status::invalid_argument("type mismatch")),
    }
}
