use std::{collections::HashMap, sync::Arc};

use prost_types::{value::Kind, Any};
use tonic::{Request, Response, Status};

use crate::{
    evaluate::evaluator::{evaluate, EvalContext},
    gen::cli::lekko::{backend::v1beta1::RepositoryKey, feature::v1beta1::FeatureType},
    gen::sdk::lekko::client::v1beta1::{
        configuration_service_server::ConfigurationService, Any as LekkoAny, DeregisterRequest,
        DeregisterResponse, GetBoolValueRequest, GetBoolValueResponse, GetFloatValueRequest,
        GetFloatValueResponse, GetIntValueRequest, GetIntValueResponse, GetJsonValueRequest,
        GetJsonValueResponse, GetProtoValueRequest, GetProtoValueResponse, GetStringValueRequest,
        GetStringValueResponse, RegisterRequest, RegisterResponse, Value,
    },
    logging::InsertLogFields,
    metrics::Metrics,
    store::Store,
    types::{self, convert_repo_key, FeatureRequestParams, Mode},
};

// This is the main rpc entrypoint into the sidecar. All host pods will communicate with the
// sidecar via this Service, using the language-native SDK.
pub struct Service {
    pub store: Arc<Store>,
    pub mode: Mode,
    pub metrics: Option<Metrics>,
    pub repo_key: RepositoryKey,
}

impl Service {
    fn get_value_local(
        &self,
        feature: FeatureRequestParams,
        context: &HashMap<String, Value>,
        requested_type: FeatureType,
    ) -> Result<Any, tonic::Status> {
        let feature_data = self
            .store
            .get_feature_local(feature.clone())
            .ok_or_else(|| Status::invalid_argument("feature not found"))?;
        if feature_data.feature.r#type() != FeatureType::Unspecified && // backwards compatibility
            feature_data.feature.r#type() != requested_type
        {
            return Err(tonic::Status::invalid_argument(format!(
                "type mismatch: requested feature is not of type {:?}",
                requested_type.as_str_name()
            )));
        }
        let eval_context = EvalContext {
            namespace: feature.namespace.to_owned(),
            feature_name: feature_data.feature.key.to_owned(),
        };
        let eval_result = evaluate(&feature_data.feature, context, &eval_context)?;
        if let Some(m) = self.metrics.as_ref() {
            m.track_flag_evaluation(&feature, &feature_data, context, &eval_result.1);
        }
        Ok(eval_result.0)
    }
}

#[tonic::async_trait]
impl ConfigurationService for Service {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<tonic::Response<RegisterResponse>, tonic::Status> {
        let requested_rk = request
            .into_inner()
            .repo_key
            .ok_or_else(|| Status::invalid_argument("no repo key provided"))?;
        if self.repo_key.owner_name != requested_rk.owner_name
            || self.repo_key.repo_name != requested_rk.repo_name
        {
            return Err(Status::invalid_argument(format!(
                "registration mismatch: requested_repo: {:?}, vs. repo: {:?}",
                requested_rk, self.repo_key
            )));
        }
        Ok(Response::new(RegisterResponse::default()))
    }

    async fn deregister(
        &self,
        _request: Request<DeregisterRequest>,
    ) -> Result<tonic::Response<DeregisterResponse>, tonic::Status> {
        Ok(Response::new(DeregisterResponse::default()))
    }

    async fn get_bool_value(
        &self,
        request: Request<GetBoolValueRequest>,
    ) -> Result<tonic::Response<GetBoolValueResponse>, tonic::Status> {
        let inner = request.into_inner();
        let params = FeatureRequestParams {
            rk: convert_repo_key(
                inner
                    .repo_key
                    .as_ref()
                    .ok_or_else(|| Status::invalid_argument("no repo key provided"))?,
            ),
            namespace: inner.namespace.clone(),
            feature: inner.key.clone(),
        };
        let result = &self.get_value_local(params, &inner.context, FeatureType::Bool)?;

        Ok(inner.insert_log_fields(Response::new(GetBoolValueResponse {
            value: types::from_any::<bool>(result)
                .map_err(|e| tonic::Status::internal(e.to_string()))?,
        })))
    }

    async fn get_int_value(
        &self,
        request: Request<GetIntValueRequest>,
    ) -> Result<tonic::Response<GetIntValueResponse>, tonic::Status> {
        let inner = request.into_inner();
        let params = FeatureRequestParams {
            rk: convert_repo_key(
                inner
                    .repo_key
                    .as_ref()
                    .ok_or_else(|| Status::invalid_argument("no repo key provided"))?,
            ),
            namespace: inner.namespace.clone(),
            feature: inner.key.clone(),
        };
        let i = types::from_any::<i64>(&self.get_value_local(
            params,
            &inner.context,
            FeatureType::Int,
        )?)
        .map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(inner.insert_log_fields(Response::new(GetIntValueResponse { value: i })))
    }

    async fn get_float_value(
        &self,
        request: Request<GetFloatValueRequest>,
    ) -> Result<tonic::Response<GetFloatValueResponse>, tonic::Status> {
        let inner = request.into_inner();
        let params = FeatureRequestParams {
            rk: convert_repo_key(
                inner
                    .repo_key
                    .as_ref()
                    .ok_or_else(|| Status::invalid_argument("no repo key provided"))?,
            ),
            namespace: inner.namespace.clone(),
            feature: inner.key.clone(),
        };

        let f = types::from_any::<f64>(&self.get_value_local(
            params,
            &inner.context,
            FeatureType::Float,
        )?)
        .map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(inner.insert_log_fields(Response::new(GetFloatValueResponse { value: f })))
    }

    async fn get_string_value(
        &self,
        request: Request<GetStringValueRequest>,
    ) -> Result<tonic::Response<GetStringValueResponse>, tonic::Status> {
        let inner = request.into_inner();
        let params = FeatureRequestParams {
            rk: convert_repo_key(
                inner
                    .repo_key
                    .as_ref()
                    .ok_or_else(|| Status::invalid_argument("no repo key provided"))?,
            ),
            namespace: inner.namespace.clone(),
            feature: inner.key.clone(),
        };

        let s = types::from_any::<String>(&self.get_value_local(
            params,
            &inner.context,
            FeatureType::String,
        )?)
        .map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(inner.insert_log_fields(Response::new(GetStringValueResponse { value: s })))
    }

    async fn get_proto_value(
        &self,
        request: Request<GetProtoValueRequest>,
    ) -> Result<tonic::Response<GetProtoValueResponse>, tonic::Status> {
        let inner = request.into_inner();
        let params = FeatureRequestParams {
            rk: convert_repo_key(
                inner
                    .repo_key
                    .as_ref()
                    .ok_or_else(|| Status::invalid_argument("no repo key provided"))?,
            ),
            namespace: inner.namespace.clone(),
            feature: inner.key.clone(),
        };

        let any = self.get_value_local(params, &inner.context, FeatureType::Proto)?;
        Ok(
            inner.insert_log_fields(Response::new(GetProtoValueResponse {
                value: Some(any.clone()),
                value_v2: Some(LekkoAny {
                    type_url: any.clone().type_url,
                    value: any.value,
                }),
            })),
        )
    }

    async fn get_json_value(
        &self,
        request: Request<GetJsonValueRequest>,
    ) -> Result<tonic::Response<GetJsonValueResponse>, tonic::Status> {
        let inner = request.into_inner();
        let v = types::from_any::<prost_types::Value>(
            &self.get_value_local(
                FeatureRequestParams {
                    rk: convert_repo_key(
                        inner
                            .repo_key
                            .as_ref()
                            .ok_or_else(|| Status::invalid_argument("no repo key provided"))?,
                    ),
                    namespace: inner.namespace.clone(),
                    feature: inner.key.clone(),
                },
                &inner.context,
                FeatureType::Json,
            )?,
        )
        .map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(inner.insert_log_fields(Response::new(GetJsonValueResponse {
            value: serde_json::to_vec(&ValueWrapper(&v)).map_err(|e| {
                Status::internal("failure serializing json ".to_owned() + &e.to_string())
            })?,
        })))
    }
}

fn serialize_value<S>(value: &prost_types::Value, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ::serde::Serializer,
{
    match &value.kind {
        None | Some(Kind::NullValue(_)) => serializer.serialize_none(),
        Some(Kind::NumberValue(f)) => serializer.serialize_f64(*f),
        Some(Kind::StringValue(s)) => serializer.serialize_str(s),
        Some(Kind::BoolValue(b)) => serializer.serialize_bool(*b),
        Some(Kind::StructValue(st)) => serialize_struct(st, serializer),
        Some(Kind::ListValue(l)) => serialize_list(l, serializer),
    }
}

pub struct ValueWrapper<'a>(&'a prost_types::Value);

impl<'a> serde::Serialize for ValueWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serialize_value(self.0, serializer)
    }
}

fn serialize_struct<S>(st: &prost_types::Struct, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ::serde::Serializer,
{
    serializer.collect_map(st.fields.iter().map(|(k, v)| (k, ValueWrapper(v))))
}

fn serialize_list<S>(st: &prost_types::ListValue, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ::serde::Serializer,
{
    serializer.collect_seq(st.values.iter().map(ValueWrapper))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    struct SerTestCase {
        val: prost_types::Value,
        res: &'static str,
    }

    fn string_value(literal: &'static str) -> prost_types::Value {
        prost_types::Value {
            kind: Some(prost_types::value::Kind::StringValue(literal.to_owned())),
        }
    }

    fn number_value(num: f64) -> prost_types::Value {
        prost_types::Value {
            kind: Some(prost_types::value::Kind::NumberValue(num)),
        }
    }

    fn bool_value(b: bool) -> prost_types::Value {
        prost_types::Value {
            kind: Some(prost_types::value::Kind::BoolValue(b)),
        }
    }

    #[test]
    fn test_serialization() {
        let tcs = [
            SerTestCase {
                val: prost_types::Value {
                    kind: Some(prost_types::value::Kind::ListValue(
                        prost_types::ListValue { values: vec![] },
                    )),
                },
                res: r#"[]"#,
            },
            SerTestCase {
                val: prost_types::Value {
                    kind: Some(prost_types::value::Kind::ListValue(
                        prost_types::ListValue {
                            values: vec![string_value("a1"), string_value("a2")],
                        },
                    )),
                },
                res: r#"["a1","a2"]"#,
            },
            SerTestCase {
                val: prost_types::Value {
                    kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                        // We can deterministically get the string of a btreemap since it
                        // has a well defined order vs. hashmap.
                        fields: BTreeMap::<String, prost_types::Value>::from_iter(vec![
                            ("a".to_owned(), string_value("val")),
                            ("b".to_owned(), number_value(-1.0)),
                            ("c".to_owned(), bool_value(false)),
                        ]),
                    })),
                },
                res: r#"{"a":"val","b":-1.0,"c":false}"#,
            },
        ];
        tcs.iter().for_each(|tc| {
            assert_eq!(
                serde_json::to_string(&super::ValueWrapper(&tc.val)).unwrap(),
                tc.res
            )
        })
    }
}
