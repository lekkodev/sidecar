use std::collections::HashMap;

use tonic::Status;
use xxhash_rust::xxh32::xxh32;

use super::evaluator::EvalContext;
use crate::gen::lekko::{
    client::v1beta1::value::Kind as LekkoKind, client::v1beta1::Value as LekkoValue,
    rules::v1beta3::call_expression::Bucket,
};

// If the hashed feature value % 100 <= threshold, it fits in the "bucket".
// In reality, we internally store the threshold as an integer in [0,100000]
// to account for up to 3 decimal places.
// The feature value is salted using the namespace, feature name, and context key.
pub fn bucket(
    bucket_f: &Bucket,
    context: &HashMap<String, LekkoValue>,
    eval_context: &EvalContext,
) -> Result<bool, Status> {
    let ctx_key = &bucket_f.context_key;
    let present = context.contains_key(ctx_key);
    // If key is missing in context map, evaluate to false - move to next rule
    if !present {
        return Ok(false);
    }
    let ctx_kind = context
        .get(ctx_key)
        .ok_or_else(|| Status::internal("empty ctx value"))?
        .kind
        .as_ref()
        .ok_or_else(|| Status::internal("empty ctx value kind"))?;

    let value_bytes = match ctx_kind {
        LekkoKind::StringValue(value) => value.as_bytes().to_owned(),
        LekkoKind::IntValue(value) => value.to_be_bytes().to_vec(),
        LekkoKind::DoubleValue(value) => value.to_be_bytes().to_vec(),
        _ => {
            return Err(Status::internal("unsupported type for bucket"));
        }
    };

    let salted_bytes = [
        eval_context.namespace.as_bytes(),
        eval_context.feature_name.as_bytes(),
        ctx_key.as_bytes(),
        value_bytes.as_slice(),
    ]
    .concat();
    let hash = xxh32(salted_bytes.as_slice(), 0);

    Ok(hash % 100000 <= bucket_f.threshold)
}

#[allow(clippy::approx_constant)]
#[cfg(test)]
mod tests {
    use super::*;

    struct Setup {
        eval_contexts: Vec<EvalContext>,
    }

    impl Setup {
        fn new() -> Self {
            Self {
                eval_contexts: vec![
                    EvalContext {
                        namespace: String::from("ns_1"),
                        feature_name: String::from("feature_1"),
                    },
                    EvalContext {
                        namespace: String::from("ns_2"),
                        feature_name: String::from("feature_2"),
                    },
                ],
            }
        }
    }

    // NOTE: to test consistency of the hashing/bucketing algorithms cross-platform
    // test cases (data and expected evaluation results) should be identical
    fn assert_bucket<T: std::fmt::Display>(
        expected: &bool,
        actual: &bool,
        context_value: &T,
        eval_context: &EvalContext,
    ) {
        assert_eq!(
            expected,
            actual,
            "key: {}/{}/{}:{}, expected: {}, actual: {}",
            eval_context.namespace,
            eval_context.feature_name,
            "key",
            context_value,
            expected,
            actual
        );
    }

    #[test]
    fn test_bucket_unsupported_type() {
        let setup = Setup::new();

        let bucket_f = Bucket {
            context_key: String::from("key"),
            threshold: 50000,
        };
        // Using context value with unsupported type
        let context = HashMap::from([(
            String::from("key"),
            LekkoValue {
                kind: Some(LekkoKind::BoolValue(false)),
            },
        )]);

        assert!(bucket(&bucket_f, &context, &setup.eval_contexts[0]).is_err());
    }

    #[test]
    fn test_bucket_ints() {
        let setup = Setup::new();

        // Different expected results with same value across different eval contexts
        let test_cases = vec![
            vec![
                (1, false),
                (2, false),
                (3, true),
                (4, false),
                (5, true),
                (101, true),
                (102, true),
                (103, false),
                (104, false),
                (105, true),
            ],
            vec![
                (1, false),
                (2, true),
                (3, false),
                (4, false),
                (5, true),
                (101, true),
                (102, true),
                (103, false),
                (104, true),
                (105, true),
            ],
        ];

        for (eval_i, eval_context) in setup.eval_contexts.iter().enumerate() {
            for test_case in test_cases[eval_i].iter() {
                let (context_value, expected) = test_case;

                // 50% bucketing
                let bucket_f = Bucket {
                    context_key: String::from("key"),
                    threshold: 50000,
                };
                let context = HashMap::from([(
                    String::from("key"),
                    LekkoValue {
                        kind: Some(LekkoKind::IntValue(*context_value)),
                    },
                )]);

                match bucket(&bucket_f, &context, eval_context) {
                    Ok(res) => assert_bucket(expected, &res, context_value, eval_context),
                    _ => println!("unexpected error"),
                }
            }
        }
    }

    #[test]
    fn test_bucket_doubles() {
        let setup = Setup::new();

        // Different expected results with same value across different eval contexts
        let test_cases = vec![
            vec![
                (3.1415, false),
                (2.7182, false),
                (1.6180, true),
                (6.6261, true),
                (6.0221, false),
                (2.9979, true),
                (6.6730, false),
                (1.3807, true),
                (1.4142, true),
                (2.0000, false),
            ],
            vec![
                (3.1415, true),
                (2.7182, false),
                (1.6180, true),
                (6.6261, false),
                (6.0221, false),
                (2.9979, false),
                (6.6730, false),
                (1.3807, false),
                (1.4142, true),
                (2.0000, false),
            ],
        ];

        for (eval_i, eval_context) in setup.eval_contexts.iter().enumerate() {
            for test_case in test_cases[eval_i].iter() {
                let (context_value, expected) = test_case;

                // 50% bucketing
                let bucket_f = Bucket {
                    context_key: String::from("key"),
                    threshold: 50000,
                };
                let context = HashMap::from([(
                    String::from("key"),
                    LekkoValue {
                        kind: Some(LekkoKind::DoubleValue(*context_value)),
                    },
                )]);

                match bucket(&bucket_f, &context, eval_context) {
                    Ok(res) => assert_bucket(expected, &res, context_value, eval_context),
                    _ => println!("unexpected error"),
                }
            }
        }
    }

    #[test]
    fn test_bucket_strings() {
        let setup = Setup::new();

        // Different expected results with same value across different eval contexts
        let test_cases = vec![
            vec![
                (String::from("hello"), false),
                (String::from("world"), false),
                (String::from("i"), true),
                (String::from("am"), true),
                (String::from("a"), true),
                (String::from("unit"), false),
                (String::from("test"), true),
                (String::from("case"), true),
                (String::from("for"), false),
                (String::from("bucket"), false),
            ],
            vec![
                (String::from("hello"), true),
                (String::from("world"), false),
                (String::from("i"), true),
                (String::from("am"), true),
                (String::from("a"), true),
                (String::from("unit"), false),
                (String::from("test"), true),
                (String::from("case"), false),
                (String::from("for"), false),
                (String::from("bucket"), false),
            ],
        ];

        for (eval_i, eval_context) in setup.eval_contexts.iter().enumerate() {
            for test_case in test_cases[eval_i].iter() {
                let (context_value, expected) = test_case;

                // 50% bucketing
                let bucket_f = Bucket {
                    context_key: String::from("key"),
                    threshold: 50000,
                };
                let context = HashMap::from([(
                    String::from("key"),
                    LekkoValue {
                        kind: Some(LekkoKind::StringValue(context_value.to_owned())),
                    },
                )]);

                match bucket(&bucket_f, &context, eval_context) {
                    Ok(res) => assert_bucket(expected, &res, context_value, eval_context),
                    _ => println!("unexpected error"),
                }
            }
        }
    }
}
