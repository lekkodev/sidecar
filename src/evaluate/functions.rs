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
// The feature value is salted using the repo name, namespace, and feature name.
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
        eval_context.owner_name.as_bytes(),
        eval_context.repo_name.as_bytes(),
        eval_context.namespace.as_bytes(),
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
                        owner_name: String::from("owner_1"),
                        repo_name: String::from("repo_1"),
                        namespace: String::from("ns_1"),
                    },
                    EvalContext {
                        owner_name: String::from("owner_2"),
                        repo_name: String::from("repo_2"),
                        namespace: String::from("ns_2"),
                    },
                ],
            }
        }
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
                (1, true),
                (2, false),
                (3, false),
                (4, true),
                (5, true),
                (101, false),
                (102, false),
                (103, true),
                (104, false),
                (105, true),
            ],
            vec![
                (1, false),
                (2, false),
                (3, true),
                (4, true),
                (5, true),
                (101, false),
                (102, false),
                (103, true),
                (104, true),
                (105, false),
            ],
        ];

        for (eval_i, eval_context) in setup.eval_contexts.iter().enumerate() {
            for test_case in test_cases[eval_i].iter() {
                let (key, expected) = test_case;

                // 50% bucketing
                let bucket_f = Bucket {
                    context_key: String::from("key"),
                    threshold: 50000,
                };
                let context = HashMap::from([(
                    String::from("key"),
                    LekkoValue {
                        kind: Some(LekkoKind::IntValue(*key)),
                    },
                )]);

                match bucket(&bucket_f, &context, eval_context) {
                    Ok(res) => assert_eq!(
                        expected, &res,
                        "key: {}, expected: {}, actual: {}",
                        key, expected, res
                    ),
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
                (2.7182, true),
                (1.6180, true),
                (6.6261, false),
                (6.0221, false),
                (2.9979, true),
                (6.6730, false),
                (1.3807, true),
                (1.4142, true),
                (2.0000, true),
            ],
            vec![
                (3.1415, false),
                (2.7182, false),
                (1.6180, false),
                (6.6261, true),
                (6.0221, false),
                (2.9979, true),
                (6.6730, false),
                (1.3807, true),
                (1.4142, false),
                (2.0000, false),
            ],
        ];

        for (eval_i, eval_context) in setup.eval_contexts.iter().enumerate() {
            for test_case in test_cases[eval_i].iter() {
                let (key, expected) = test_case;

                // 50% bucketing
                let bucket_f = Bucket {
                    context_key: String::from("key"),
                    threshold: 50000,
                };
                let context = HashMap::from([(
                    String::from("key"),
                    LekkoValue {
                        kind: Some(LekkoKind::DoubleValue(*key)),
                    },
                )]);

                match bucket(&bucket_f, &context, eval_context) {
                    Ok(res) => assert_eq!(
                        expected, &res,
                        "key: {}, expected: {}, actual: {}",
                        key, expected, res
                    ),
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
                (String::from("am"), false),
                (String::from("a"), false),
                (String::from("unit"), true),
                (String::from("test"), false),
                (String::from("case"), true),
                (String::from("for"), true),
                (String::from("bucket"), true),
            ],
            vec![
                (String::from("hello"), false),
                (String::from("world"), false),
                (String::from("i"), true),
                (String::from("am"), true),
                (String::from("a"), false),
                (String::from("unit"), true),
                (String::from("test"), true),
                (String::from("case"), false),
                (String::from("for"), true),
                (String::from("bucket"), false),
            ],
        ];

        for (eval_i, eval_context) in setup.eval_contexts.iter().enumerate() {
            for test_case in test_cases[eval_i].iter() {
                let (key, expected) = test_case;

                // 50% bucketing
                let bucket_f = Bucket {
                    context_key: String::from("key"),
                    threshold: 50000,
                };
                let context = HashMap::from([(
                    String::from("key"),
                    LekkoValue {
                        kind: Some(LekkoKind::StringValue(key.to_owned())),
                    },
                )]);

                match bucket(&bucket_f, &context, eval_context) {
                    Ok(res) => assert_eq!(
                        expected, &res,
                        "key: {}, expected: {}, actual: {}",
                        key, expected, res
                    ),
                    _ => println!("unexpected error"),
                }
            }
        }
    }
}
