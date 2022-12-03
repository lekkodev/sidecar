use std::collections::HashMap;

use prost_types::{
    value::Kind,
    value::Kind::{BoolValue, NumberValue, StringValue},
};
use tonic::Status;

use crate::gen::lekko::{
    backend::{self, v1beta1::value::Kind as LekkoKind, v1beta1::Value as LekkoValue},
    rules::v1beta2::{
        rule::Rule::{Atom, BoolConst, LogicalExpression, Not},
        ComparisonOperator as CmpOp,
        LogicalOperator::{And, Or},
        Rule,
    },
};

// TODO: make all error messages contain dynamic variable info.
// check_rule evaluates the rule using the given context to determine whether or not the rule passed.
// it is a recursive method.
pub fn check_rule(rule: Rule, context: HashMap<String, LekkoValue>) -> Result<bool, Status> {
    let r = rule.rule.ok_or(Status::internal("empty rule"))?;
    match r {
        // Base case
        BoolConst(b) => return Ok(b),
        // Recursive case
        Not(not_rule) => {
            let inner = check_rule(*not_rule, context);
            if inner.is_err() {
                return Err(inner.unwrap_err());
            }
            return Ok(!inner.unwrap()); // not
        }
        // Recursive case
        LogicalExpression(le_box) => {
            let le = *le_box;
            let first = check_rule(
                *(le.clone()
                    .first_rule
                    .ok_or(Status::internal("empty first rule"))?),
                context.clone(),
            )?;
            let second = check_rule(
                *(le.clone()
                    .second_rule
                    .ok_or(Status::internal("empty second rule"))?),
                context.clone(),
            )?;
            match le.logical_operator() {
                And => return Ok(first && second),
                Or => return Ok(first || second),
                _ => return Err(Status::internal("unknown logical operator")),
            }
        }
        // Base case
        Atom(a) => {
            let ctx_key = a.clone().context_key;
            let present = context.contains_key(&ctx_key);
            if a.clone().comparison_operator().eq(&CmpOp::Present) {
                return Ok(present);
            }
            if a.clone().comparison_value.is_none() {
                return Err(Status::internal("empty comparison value"));
            }
            if !present {
                // All other comparison operators expect the context key to be present. If
                // it is not present, return false.
                return Ok(false);
            }
            let rule_kind = &a
                .clone()
                .comparison_value
                .unwrap()
                .kind
                .ok_or(Status::internal("empty rule value kind"))?;
            let ctx_kind = &context
                .clone()
                .get(&ctx_key)
                .ok_or(Status::internal("empty ctx value"))?
                .kind
                .clone()
                .ok_or(Status::internal("empty ctx value kind"))?;
            match a.comparison_operator() {
                CmpOp::Equals => return check_equals_cmp(rule_kind, ctx_kind),
                CmpOp::LessThan => {
                    return check_num_cmp(a.comparison_operator(), rule_kind, ctx_kind)
                }
                CmpOp::LessThanOrEquals => {
                    return check_num_cmp(a.comparison_operator(), rule_kind, ctx_kind)
                }
                CmpOp::GreaterThan => {
                    return check_num_cmp(a.comparison_operator(), rule_kind, ctx_kind)
                }
                CmpOp::GreaterThanOrEquals => {
                    return check_num_cmp(a.comparison_operator(), rule_kind, ctx_kind)
                }
                CmpOp::ContainedWithin => return check_list_cmp(rule_kind, ctx_kind),
                CmpOp::StartsWith => {
                    return check_str_cmp(a.comparison_operator(), rule_kind, ctx_kind)
                }
                CmpOp::EndsWith => {
                    return check_str_cmp(a.comparison_operator(), rule_kind, ctx_kind)
                }
                CmpOp::Contains => {
                    return check_str_cmp(a.comparison_operator(), rule_kind, ctx_kind)
                }
                CmpOp::Present => return Err(Status::internal("present should be handled above")),
                _ => return Err(Status::internal("unknown comparison operator")),
            }
        }
    }
}

fn check_equals_cmp(rule_kind: &Kind, ctx_kind: &LekkoKind) -> Result<bool, Status> {
    match rule_kind {
        BoolValue(rule_bool) => match ctx_kind {
            backend::v1beta1::value::Kind::BoolValue(ctx_bool) => return Ok(rule_bool == ctx_bool),
            _ => return Err(Status::invalid_argument("type mismatch")),
        },
        NumberValue(rule_num) => match ctx_kind {
            backend::v1beta1::value::Kind::IntValue(ctx_num) => {
                return Ok(*rule_num == *ctx_num as f64)
            }
            backend::v1beta1::value::Kind::DoubleValue(ctx_num) => return Ok(rule_num == ctx_num),
            _ => return Err(Status::invalid_argument("type mismatch")),
        },
        StringValue(rule_str) => match ctx_kind {
            backend::v1beta1::value::Kind::StringValue(ctx_str) => return Ok(rule_str == ctx_str),
            _ => return Err(Status::invalid_argument("type mismatch")),
        },
        _ => return Err(Status::internal("unsupported rule value kind")),
    }
}

fn check_num_cmp(co: CmpOp, rule_kind: &Kind, ctx_kind: &LekkoKind) -> Result<bool, Status> {
    let rule_num = get_number(rule_kind)?;
    let ctx_num = get_lekko_number(ctx_kind)?;
    match co {
        CmpOp::LessThan => return Ok(ctx_num < rule_num),
        CmpOp::LessThanOrEquals => return Ok(ctx_num <= rule_num),
        CmpOp::GreaterThan => return Ok(ctx_num > rule_num),
        CmpOp::GreaterThanOrEquals => return Ok(ctx_num >= rule_num),
        _ => return Err(Status::internal("invalid comparison operator")),
    }
}

fn check_list_cmp(rule_kind: &Kind, ctx_kind: &LekkoKind) -> Result<bool, Status> {
    match rule_kind {
        Kind::ListValue(rule_list) => {
            for rule_elem in &rule_list.values {
                let rule_elem_kind = &rule_elem
                    .clone()
                    .kind
                    .ok_or(Status::internal("empty rule value kind"))?;
                let elem_equal = check_equals_cmp(rule_elem_kind, ctx_kind);
                if elem_equal.is_ok() && elem_equal.unwrap() {
                    return Ok(true);
                }
            }
            return Ok(false);
        }
        _ => return Err(Status::invalid_argument("type mismatch")),
    }
}

fn check_str_cmp(co: CmpOp, rule_kind: &Kind, ctx_kind: &LekkoKind) -> Result<bool, Status> {
    let rule_str = get_string(rule_kind)?;
    let ctx_str = get_lekko_string(ctx_kind)?;
    match co {
        CmpOp::StartsWith => return Ok(ctx_str.starts_with(&rule_str)),
        CmpOp::EndsWith => return Ok(ctx_str.ends_with(&rule_str)),
        CmpOp::Contains => return Ok(ctx_str.contains(&rule_str)),
        _ => return Err(Status::internal("invalid comparison operator")),
    }
}

fn get_number(kind: &Kind) -> Result<f64, Status> {
    match kind {
        NumberValue(num_value) => return Ok(*num_value),
        _ => return Err(Status::invalid_argument("type mismatch")),
    }
}

fn get_lekko_number(kind: &LekkoKind) -> Result<f64, Status> {
    match kind {
        LekkoKind::IntValue(int_value) => return Ok(*int_value as f64),
        LekkoKind::DoubleValue(double_value) => return Ok(*double_value),
        _ => return Err(Status::invalid_argument("type mismatch")),
    }
}

fn get_string(kind: &Kind) -> Result<String, Status> {
    match kind {
        StringValue(str) => return Ok(str.clone()),
        _ => return Err(Status::invalid_argument("type mismatch")),
    }
}

fn get_lekko_string(kind: &LekkoKind) -> Result<String, Status> {
    match kind {
        LekkoKind::StringValue(str_value) => return Ok(str_value.clone()),
        _ => return Err(Status::invalid_argument("type mismatch")),
    }
}
