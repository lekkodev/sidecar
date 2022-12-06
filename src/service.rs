use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

use prost::{
    encoding::bool::{self},
    DecodeError, Message,
};
use prost_types::Value;
use tonic::{body::BoxBody, metadata::MetadataMap, Request, Response};

use crate::{
    evaluate::evaluator::evaluate,
    gen::lekko::backend::v1beta1::{
        configuration_service_client::ConfigurationServiceClient,
        configuration_service_server::ConfigurationService, GetBoolValueRequest,
        GetBoolValueResponse, GetJsonValueRequest, GetJsonValueResponse, GetProtoValueRequest,
        GetProtoValueResponse,
    },
    store::Store,
    types::{self, FeatureRequestParams, RepositoryKey, APIKEY},
};

// This is the main rpc entrypoint into the sidecar. All host pods will communicate with the
// sidecar via this Service, using the language-native SDK.
pub struct Service {
    pub config_client:
        ConfigurationServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
    pub store: Store,
    pub proxy_mode: bool, // If false, the sidecar will attempt local evaluation
}

// TODO: Send batched flag evaluation metrics back to the backend after local evaluation.

#[tonic::async_trait]
impl ConfigurationService for Service {
    async fn get_bool_value(
        &self,
        request: Request<GetBoolValueRequest>,
    ) -> Result<tonic::Response<GetBoolValueResponse>, tonic::Status> {
        if self.proxy_mode {
            println!("Got a request for GetBoolValue, proxying");
            let mut proxy_req = Request::new(request.get_ref().clone());
            self.proxy_headers(&mut proxy_req, request.metadata());
            let resp = self.config_client.clone().get_bool_value(proxy_req).await;
            if resp.is_err() {
                println!("error in proxying {:?}", resp);
                return Err(resp.err().unwrap());
            }
            return resp;
        };
        println!("Got a request for GetBoolValue, evaluating");
        let apikey = request.metadata().get(APIKEY).unwrap().to_owned();
        let inner = request.into_inner();
        let feature_data = self
            .store
            .get_feature(FeatureRequestParams {
                api_key: apikey,
                rk: RepositoryKey {
                    owner_name: inner.repo_key.clone().unwrap().owner_name,
                    repo_name: inner.repo_key.clone().unwrap().repo_name,
                },
                namespace: inner.namespace,
                feature: inner.key,
            })
            .await;
        if let Err(error) = feature_data {
            println!("error getting feature data {:?}", error);
            return Err(error);
        }
        let eval_result = evaluate(&feature_data.unwrap(), &inner.context)?;
        let b: Result<bool, DecodeError> = types::from_any(&eval_result.0);
        if let Err(err) = b {
            return Err(tonic::Status::invalid_argument(err.to_string()));
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
            if resp.is_err() {
                println!("error in proxying {:?}", resp);
                return Err(resp.err().unwrap());
            }
            return resp;
        }

        println!("Got a request for GetProtoValue, evaluating");
        let apikey = request.metadata().get(APIKEY).unwrap().to_owned();
        let inner = request.into_inner();
        let feature_data = self
            .store
            .get_feature(FeatureRequestParams {
                api_key: apikey,
                rk: RepositoryKey {
                    owner_name: inner.repo_key.clone().unwrap().owner_name,
                    repo_name: inner.repo_key.clone().unwrap().repo_name,
                },
                namespace: inner.namespace,
                feature: inner.key,
            })
            .await;
        if let Err(error) = feature_data {
            println!("error getting feature data {:?}", error);
            return Err(error);
        }
        let eval_result = evaluate(&feature_data.unwrap(), &inner.context)?;
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
            if resp.is_err() {
                println!("error in proxying {:?}", resp);
                return Err(resp.err().unwrap());
            }
            return resp;
        }
        println!("Got a request for GetJSONValue, evaluating");
        let apikey = request.metadata().get(APIKEY).unwrap().to_owned();
        let inner = request.into_inner();
        let feature_data = self
            .store
            .get_feature(FeatureRequestParams {
                api_key: apikey,
                rk: RepositoryKey {
                    owner_name: inner.repo_key.clone().unwrap().owner_name,
                    repo_name: inner.repo_key.clone().unwrap().repo_name,
                },
                namespace: inner.namespace,
                feature: inner.key,
            })
            .await;
        if let Err(error) = feature_data {
            println!("error getting feature data {:?}", error);
            return Err(error);
        }
        let eval_result = evaluate(&feature_data.unwrap(), &inner.context)?;
        let struct_value: Result<Value, DecodeError> = types::from_any(&eval_result.0);
        if let Err(e) = struct_value {
            return Err(tonic::Status::invalid_argument(e.to_string()));
        }
        let vec = struct_value.unwrap().encode_to_vec();
        Ok(Response::new(GetJsonValueResponse { value: vec }))
    }
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
