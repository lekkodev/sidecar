use std::{collections::HashMap, ops::DerefMut, sync::Mutex};

use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use log::{error};
use prost_types::{value::Kind, Any};
use tonic::{
    body::BoxBody,
    metadata::{Ascii, MetadataMap, MetadataValue},
    Request, Response, Status,
};

use crate::{
    evaluate::evaluator::evaluate,
    gen::mod_cli::lekko::feature::v1beta1::FeatureType,
    gen::mod_sdk::lekko::client::v1beta1::{
        configuration_service_client::ConfigurationServiceClient,
        configuration_service_server::ConfigurationService, DeregisterRequest, DeregisterResponse,
        GetBoolValueRequest, GetBoolValueResponse, GetFloatValueRequest, GetFloatValueResponse,
        GetIntValueRequest, GetIntValueResponse, GetJsonValueRequest, GetJsonValueResponse,
        GetProtoValueRequest, GetProtoValueResponse, GetStringValueRequest, GetStringValueResponse,
        RegisterRequest, RegisterResponse, Value,
    },
    logging::InsertLogFields,
    metrics::Metrics,
    store::Store,
    types::{self, convert_repo_key, FeatureRequestParams, APIKEY},
};

// Mode represents the running mode of the sidecar.
//
// Default implies waiting for a Register call, fetching from a bootstrap,
// and evaluating locally while polling for updates.
//
// Consistent always fetches from Lekko ignoring local caches, resulting in strong
// consistency.
//
// Static fetches from the bootstrap and always evaluates against those values. No
// connection is made to Lekko services.
#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum Mode {
    #[default]
    Default,
    Consistent,
    Static,
}

// This is the main rpc entrypoint into the sidecar. All host pods will communicate with the
// sidecar via this Service, using the language-native SDK.
pub struct Service {
    pub config_client:
        ConfigurationServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
    pub store: Store,
    pub mode: Mode,
    pub metrics: Metrics,
    pub shutdown_tx: Mutex<Option<tokio::sync::oneshot::Sender<()>>>,
}

trait SharedRequest {
    fn metadata(&self) -> &MetadataMap;
}

impl Service {
    // Sets the headers that we wish to forward to lekko. The apikey header is copied over as
    // the server needs it to authenticate the caller.
    fn proxy_headers<T>(&self, proxy_request: &mut Request<T>, incoming_headers: &MetadataMap) {
        if let Some(apikey) = incoming_headers.get(APIKEY) {
            proxy_request
                .metadata_mut()
                .append(APIKEY, apikey.to_owned());
        }
    }

    fn get_value_local(
        &self,
        feature: FeatureRequestParams,
        context: &HashMap<String, Value>,
        api_key: MetadataValue<Ascii>,
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
        let eval_result = evaluate(&feature_data.feature, context)?;
        self.metrics.track_flag_evaluation(
            &feature,
            &feature_data,
            context,
            &eval_result.1,
            &api_key,
        );
        Ok(eval_result.0)
    }
}

#[tonic::async_trait]
impl ConfigurationService for Service {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<tonic::Response<RegisterResponse>, tonic::Status> {
        if matches!(self.mode, Mode::Consistent) || matches!(self.mode, Mode::Static) {
            // Here we can effectively ignore the register call.
            // We should reconsider some design here to make sure the SDK matches the sidecar configuration.
            return Ok(Response::new(RegisterResponse::default()));
        }
        let apikey = request
            .metadata()
            .get(APIKEY)
            .ok_or_else(|| Status::invalid_argument("no apikey header provided"))?
            .to_owned();
        let request = request.into_inner();
        self.store
            .register(
                convert_repo_key(
                    &request
                        .repo_key
                        .ok_or_else(|| Status::invalid_argument("no repo key provided"))?,
                ),
                &request.namespace_list,
                apikey,
            )
            .await?;
        Ok(Response::new(RegisterResponse::default()))
    }

    async fn deregister(
        &self,
        _request: Request<DeregisterRequest>,
    ) -> Result<tonic::Response<DeregisterResponse>, tonic::Status> {
        // Only proxy register in the case of default. We still continue here
        // to consume the oneshot even in static or consistent mode.
        if matches!(self.mode, Mode::Default) {
            match self.store.deregister().await {
                Ok(_) => {}
                Err(err) => {
                    error!("error encountered when proxying deregistration. continuing with shutdown {err:?}")
                }
            }
        }
        // There is a potential race condition here of if we got SIGTERM,
        // we never return this error message because the oneshot has released our
        // graceful shutdown handler and we exit too fast. This is unlikely, and worst
        // case results in an error message for the caller.

        let mut guard = self.shutdown_tx.lock().unwrap();
        match guard.deref_mut().take() {
            Some(sender) => sender
                .send(())
                .map_err(|_| tonic::Status::internal("shutdown already initiated"))?,
            None => {
                return Err(tonic::Status::already_exists(
                    "deregister has already been called on this sidecar, ignoring deregister RPC",
                ))
            }
        }
        Ok(Response::new(DeregisterResponse::default()))
    }

    async fn get_bool_value(
        &self,
        request: Request<GetBoolValueRequest>,
    ) -> Result<tonic::Response<GetBoolValueResponse>, tonic::Status> {
        let apikey = request
            .metadata()
            .get(APIKEY)
            .ok_or_else(|| Status::invalid_argument("no apikey header provided"))?
            .to_owned();

        if matches!(self.mode, Mode::Consistent) {
            let mut proxy_req = Request::new(request.get_ref().clone());
            self.proxy_headers(&mut proxy_req, request.metadata());
            return self.config_client.clone().get_bool_value(proxy_req).await;
        }

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
        let result = &self.get_value_local(params, &inner.context, apikey, FeatureType::Bool)?;
        Ok(inner.insert_log_fields(Response::new(GetBoolValueResponse {
            value: types::from_any::<bool>(result)
                .map_err(|e| tonic::Status::internal(e.to_string()))?,
        })))
    }

    async fn get_int_value(
        &self,
        request: Request<GetIntValueRequest>,
    ) -> Result<tonic::Response<GetIntValueResponse>, tonic::Status> {
        let apikey = request
            .metadata()
            .get(APIKEY)
            .ok_or_else(|| Status::invalid_argument("no apikey header provided"))?
            .to_owned();

        if matches!(self.mode, Mode::Consistent) {
            let mut proxy_req = Request::new(request.get_ref().clone());
            self.proxy_headers(&mut proxy_req, request.metadata());
            return self.config_client.clone().get_int_value(proxy_req).await;
        }

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
            apikey,
            FeatureType::Int,
        )?)
        .map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(inner.insert_log_fields(Response::new(GetIntValueResponse { value: i })))
    }

    async fn get_float_value(
        &self,
        request: Request<GetFloatValueRequest>,
    ) -> Result<tonic::Response<GetFloatValueResponse>, tonic::Status> {
        let apikey = request
            .metadata()
            .get(APIKEY)
            .ok_or_else(|| Status::invalid_argument("no apikey header provided"))?
            .to_owned();

        if matches!(self.mode, Mode::Consistent) {
            let mut proxy_req = Request::new(request.get_ref().clone());
            self.proxy_headers(&mut proxy_req, request.metadata());
            return self.config_client.clone().get_float_value(proxy_req).await;
        }

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
            apikey,
            FeatureType::Float,
        )?)
        .map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(inner.insert_log_fields(Response::new(GetFloatValueResponse { value: f })))
    }

    async fn get_string_value(
        &self,
        request: Request<GetStringValueRequest>,
    ) -> Result<tonic::Response<GetStringValueResponse>, tonic::Status> {
        let apikey = request
            .metadata()
            .get(APIKEY)
            .ok_or_else(|| Status::invalid_argument("no apikey header provided"))?
            .to_owned();

        if matches!(self.mode, Mode::Consistent) {
            let mut proxy_req = Request::new(request.get_ref().clone());
            self.proxy_headers(&mut proxy_req, request.metadata());
            return self.config_client.clone().get_string_value(proxy_req).await;
        }

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
            apikey,
            FeatureType::String,
        )?)
        .map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(inner.insert_log_fields(Response::new(GetStringValueResponse { value: s })))
    }

    async fn get_proto_value(
        &self,
        request: Request<GetProtoValueRequest>,
    ) -> Result<tonic::Response<GetProtoValueResponse>, tonic::Status> {
        let apikey = request
            .metadata()
            .get(APIKEY)
            .ok_or_else(|| Status::invalid_argument("no apikey header provided"))?
            .to_owned();

        if matches!(self.mode, Mode::Consistent) {
            let mut proxy_req = Request::new(request.get_ref().clone());
            self.proxy_headers(&mut proxy_req, request.metadata());
            return self.config_client.clone().get_proto_value(proxy_req).await;
        }
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

        let any = self.get_value_local(params, &inner.context, apikey, FeatureType::Proto)?;
        Ok(inner.insert_log_fields(Response::new(GetProtoValueResponse { value: Some(any) })))
    }

    async fn get_json_value(
        &self,
        request: Request<GetJsonValueRequest>,
    ) -> Result<tonic::Response<GetJsonValueResponse>, tonic::Status> {
        let apikey = request
            .metadata()
            .get(APIKEY)
            .ok_or_else(|| Status::invalid_argument("no apikey header provided"))?
            .to_owned();

        if matches!(self.mode, Mode::Consistent) {
            let mut proxy_req = Request::new(request.get_ref().clone());
            self.proxy_headers(&mut proxy_req, request.metadata());
            return self.config_client.clone().get_json_value(proxy_req).await;
        }

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
                apikey,
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
        let tcs = vec![
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
                        fields: BTreeMap::<String, prost_types::Value>::from_iter(
                            vec![
                                ("a".to_owned(), string_value("val")),
                                ("b".to_owned(), number_value(-1.0)),
                                ("c".to_owned(), bool_value(false)),
                            ]
                            .into_iter(),
                        ),
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
