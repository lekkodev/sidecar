use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tonic::{body::BoxBody, metadata::MetadataMap, Request, Response};

use crate::{
    gen::lekko::backend::v1beta1::{
        configuration_service_client::ConfigurationServiceClient,
        configuration_service_server::ConfigurationService, GetBoolValueRequest,
        GetBoolValueResponse, GetJsonValueRequest, GetJsonValueResponse, GetProtoValueRequest,
        GetProtoValueResponse,
    },
    store::Store,
    types::{FeatureRequestParams, RepositoryKey, APIKEY},
};

#[derive(Clone)]
pub struct Service {
    pub config_client:
        ConfigurationServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
    pub store: Store,
    pub proxy_mode: bool, // If false, the sidecar will attempt local evaluation
}

#[tonic::async_trait]
impl ConfigurationService for Service {
    async fn get_bool_value(
        &self,
        request: Request<GetBoolValueRequest>,
    ) -> Result<tonic::Response<GetBoolValueResponse>, tonic::Status> {
        if self.proxy_mode {
            println!("Got a request for GetBoolValue, proxying {:?}", request);
            let mut proxy_req = Request::new(request.get_ref().clone());
            self.proxy_headers(&mut proxy_req, request.metadata());
            let resp = self.config_client.clone().get_bool_value(proxy_req).await;
            if resp.is_err() {
                println!("error in proxying {:?}", resp);
                return Result::Err(resp.err().unwrap());
            }
            return resp;
        };
        println!("Got a request for GetBoolValue, evaluating {:?}", request);
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
        if feature_data.is_err() {
            println!(
                "error getting feature data {:?}",
                feature_data.clone().err().unwrap()
            );
            return Result::Err(feature_data.err().unwrap());
        }
        Result::Ok(Response::new(GetBoolValueResponse { value: false }))
    }
    async fn get_proto_value(
        &self,
        request: Request<GetProtoValueRequest>,
    ) -> Result<tonic::Response<GetProtoValueResponse>, tonic::Status> {
        if self.proxy_mode {
            println!("Got a request for GetProtoValue, proxying {:?}", request);
            let mut proxy_req = Request::new(request.get_ref().clone());
            self.proxy_headers(&mut proxy_req, request.metadata());
            let resp = self.config_client.clone().get_proto_value(proxy_req).await;
            if resp.is_err() {
                println!("error in proxying {:?}", resp);
                return Result::Err(resp.err().unwrap());
            }
            return resp;
        }

        println!("Got a request for GetProtoValue, evaluating {:?}", request);
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
        if feature_data.is_err() {
            println!(
                "error getting feature data {:?}",
                feature_data.clone().err().unwrap()
            );
            return Result::Err(feature_data.err().unwrap());
        }
        Result::Ok(Response::new(GetProtoValueResponse {
            value: Option::None,
        }))
    }
    async fn get_json_value(
        &self,
        request: Request<GetJsonValueRequest>,
    ) -> Result<tonic::Response<GetJsonValueResponse>, tonic::Status> {
        if self.proxy_mode {
            println!("Got a request for GetJSONValue, proxying {:?}", request);
            let mut proxy_req = Request::new(request.get_ref().clone());
            self.proxy_headers(&mut proxy_req, request.metadata());
            let resp = self.config_client.clone().get_json_value(proxy_req).await;
            if resp.is_err() {
                println!("error in proxying {:?}", resp);
                return Result::Err(resp.err().unwrap());
            }
            return resp;
        }
        println!("Got a request for GetJSONValue, evaluating {:?}", request);
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
        if feature_data.is_err() {
            println!(
                "error getting feature data {:?}",
                feature_data.clone().err().unwrap()
            );
            return Result::Err(feature_data.err().unwrap());
        }
        Result::Ok(Response::new(GetJsonValueResponse { value: Vec::new() }))
    }
}

impl Service {
    // Sets the headers that we wish to forward to lekko. The apikey header is copied over as
    // the server needs it to authenticate the caller.
    fn proxy_headers<T>(&self, proxy_request: &mut Request<T>, incoming_headers: &MetadataMap) {
        if let Some(apikey) = incoming_headers.get("apikey") {
            proxy_request
                .metadata_mut()
                .append("apikey", apikey.to_owned());
        }
    }
}
