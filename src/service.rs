use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

use prost::{
    encoding::bool::{self},
    DecodeError,
};
use prost_types::{value::Kind, Value};
use tonic::{body::BoxBody, metadata::MetadataMap, Request, Response, Status};

use crate::{
    evaluate::evaluator::evaluate,
    gen::lekko::backend::v1beta1::{
        configuration_service_client::ConfigurationServiceClient,
        configuration_service_server::ConfigurationService, GetBoolValueRequest,
        GetBoolValueResponse, GetJsonValueRequest, GetJsonValueResponse, GetProtoValueRequest,
        GetProtoValueResponse, RegisterRequest, RegisterResponse,
    },
    metrics::Metrics,
    store::Store,
    types::{self, FeatureRequestParams, APIKEY},
};

// This is the main rpc entrypoint into the sidecar. All host pods will communicate with the
// sidecar via this Service, using the language-native SDK.
pub struct Service {
    pub config_client:
        ConfigurationServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
    pub store: Store,
    pub proxy_mode: bool, // If false, the sidecar will attempt local evaluation
    pub metrics: Metrics,
}

// TODO: Send batched flag evaluation metrics back to the backend after local evaluation.

#[tonic::async_trait]
impl ConfigurationService for Service {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<tonic::Response<RegisterResponse>, tonic::Status> {
        let apikey = request.metadata().get(APIKEY).unwrap().clone();
        let request = request.into_inner();
        self.store
            .register(request.repo_key.unwrap(), &request.namespace_list, apikey)
            .await?;
        Ok(Response::new(RegisterResponse::default()))
    }
    async fn get_bool_value(
        &self,
        request: Request<GetBoolValueRequest>,
    ) -> Result<tonic::Response<GetBoolValueResponse>, tonic::Status> {
        if self.proxy_mode {
            println!("Got a request for GetBoolValue, proxying");
            let mut proxy_req = Request::new(request.get_ref().clone());
            self.proxy_headers(&mut proxy_req, request.metadata());
            let resp = self.config_client.clone().get_bool_value(proxy_req).await;
            if let Err(e) = resp {
                println!("error in proxying {:?}", e);
                return Err(e);
            }
            return resp;
        };
        println!("Got a request for GetBoolValue, evaluating");
        let apikey = request
            .metadata()
            .get(APIKEY)
            .ok_or_else(|| Status::invalid_argument("no apikey header provided"))?
            .to_owned();
        let inner = request.into_inner();
        let rk = inner
            .repo_key
            .ok_or_else(|| Status::invalid_argument("no repo key provided"))?;
        let feature_data = self
            .store
            .get_feature(FeatureRequestParams {
                api_key: apikey.clone(),
                rk: rk.clone(),
                namespace: inner.namespace.clone(),
                feature: inner.key.clone(),
            })
            .await;
        if let Err(error) = feature_data {
            println!("error getting feature data {:?}", error);
            return Err(error);
        }
        let some_feature_data = feature_data.unwrap();
        let eval_result = evaluate(&some_feature_data.feature, &inner.context)?;
        self.metrics.track_flag_evaluation(
            &rk,
            &inner.namespace,
            &some_feature_data,
            &inner.context,
            &eval_result.1,
            &apikey,
        );
        let b: Result<bool, DecodeError> = types::from_any(&eval_result.0);
        if let Err(e) = b {
            return Err(tonic::Status::internal(e.to_string()));
        }
        Ok(Response::new(GetBoolValueResponse { value: b.unwrap() }))
    }
    async fn get_proto_value(
        &self,
        request: Request<GetProtoValueRequest>,
    ) -> Result<tonic::Response<GetProtoValueResponse>, tonic::Status> {
        if self.proxy_mode {
            println!("Got a request for GetProtoValue, proxying");
            let mut proxy_req = Request::new(request.get_ref().clone());
            self.proxy_headers(&mut proxy_req, request.metadata());
            let resp = self.config_client.clone().get_proto_value(proxy_req).await;
            if let Err(e) = resp {
                println!("error in proxying {:?}", e);
                return Err(e);
            }
            return resp;
        }

        println!("Got a request for GetProtoValue, evaluating");
        let apikey = request
            .metadata()
            .get(APIKEY)
            .ok_or_else(|| Status::invalid_argument("no apikey header provided"))?
            .to_owned();
        let inner = request.into_inner();
        let rk = inner
            .repo_key
            .ok_or_else(|| Status::invalid_argument("no repo key provided"))?;
        let feature_data = self
            .store
            .get_feature(FeatureRequestParams {
                api_key: apikey.clone(),
                rk: rk.clone(),
                namespace: inner.namespace.clone(),
                feature: inner.key.clone(),
            })
            .await;
        if let Err(error) = feature_data {
            println!("error getting feature data {:?}", error);
            return Err(error);
        }
        let some_feature_data = feature_data.unwrap();
        let eval_result = evaluate(&some_feature_data.feature, &inner.context)?;
        self.metrics.track_flag_evaluation(
            &rk,
            &inner.namespace,
            &some_feature_data,
            &inner.context,
            &eval_result.1,
            &apikey,
        );
        Ok(Response::new(GetProtoValueResponse {
            value: Some(eval_result.0),
        }))
    }
    async fn get_json_value(
        &self,
        request: Request<GetJsonValueRequest>,
    ) -> Result<tonic::Response<GetJsonValueResponse>, tonic::Status> {
        if self.proxy_mode {
            println!("Got a request for GetJSONValue, proxying");
            let mut proxy_req = Request::new(request.get_ref().clone());
            self.proxy_headers(&mut proxy_req, request.metadata());
            let resp = self.config_client.clone().get_json_value(proxy_req).await;
            if let Err(e) = resp {
                println!("error in proxying {:?}", e);
                return Err(e);
            }
            return resp;
        }
        println!("Got a request for GetJSONValue, evaluating");
        let apikey = request
            .metadata()
            .get(APIKEY)
            .ok_or_else(|| Status::invalid_argument("no apikey header provided"))?
            .to_owned();
        let inner = request.into_inner();
        let rk = inner
            .repo_key
            .ok_or_else(|| Status::invalid_argument("no repo key provided"))?;
        let feature_data = self
            .store
            .get_feature(FeatureRequestParams {
                api_key: apikey.clone(),
                rk: rk.clone(),
                namespace: inner.namespace.clone(),
                feature: inner.key.clone(),
            })
            .await;
        if let Err(error) = feature_data {
            println!("error getting feature data {:?}", error);
            return Err(error);
        }
        let some_feature_data = feature_data.unwrap();
        let eval_result = evaluate(&some_feature_data.feature, &inner.context)?;
        self.metrics.track_flag_evaluation(
            &rk,
            &inner.namespace,
            &some_feature_data,
            &inner.context,
            &eval_result.1,
            &apikey,
        );
        let struct_value: Value = match types::from_any(&eval_result.0) {
            Err(err) => {
                println!("error from decoding from any {:?}", err);
                return Err(Status::internal("invalid internal protobuf type"));
            }
            Ok(v) => v,
        };

        Ok(Response::new(GetJsonValueResponse {
            value: serde_json::to_vec(&ValueWrapper(&struct_value)).map_err(|e| {
                Status::internal("failure serializing json ".to_owned() + &e.to_string())
            })?,
        }))
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
