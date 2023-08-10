use std::sync::Arc;

use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tonic::{body::BoxBody, Request, Response};

use crate::{
    gen::cli::lekko::backend::v1beta1::{
        distribution_service_client::DistributionServiceClient,
        distribution_service_server::DistributionService, DeregisterClientRequest,
        DeregisterClientResponse, GetDeveloperAccessTokenRequest, GetDeveloperAccessTokenResponse,
        GetRepositoryContentsRequest, GetRepositoryContentsResponse, GetRepositoryVersionRequest,
        GetRepositoryVersionResponse, RegisterClientRequest, RegisterClientResponse, RepositoryKey,
        SendFlagEvaluationMetricsRequest, SendFlagEvaluationMetricsResponse,
    },
    store::Store,
    types::{override_api_key, ConnectionCredentials, APIKEY},
};

// This is the main rpc entrypoint into the sidecar. All host pods will communicate with the
// sidecar via this Service, using the language-native SDK.
pub struct Service {
    pub distro_client:
        DistributionServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
    pub conn_creds: Option<ConnectionCredentials>,
    pub store: Arc<Store>,
    pub repo_key: RepositoryKey,
}

#[tonic::async_trait]
impl DistributionService for Service {
    async fn get_repository_version(
        &self,
        _request: Request<GetRepositoryVersionRequest>,
    ) -> Result<tonic::Response<GetRepositoryVersionResponse>, tonic::Status> {
        return Ok(Response::new(GetRepositoryVersionResponse {
            commit_sha: self.store.get_version_local(),
        }));
    }
    async fn get_repository_contents(
        &self,
        request: Request<GetRepositoryContentsRequest>,
    ) -> Result<tonic::Response<GetRepositoryContentsResponse>, tonic::Status> {
        let request = request.into_inner();
        let (version, namespaces) = self
            .store
            .get_repo_contents_local(&request.namespace_name, &request.feature_name);
        Ok(Response::new(GetRepositoryContentsResponse {
            namespaces,
            commit_sha: version,
        }))
    }

    async fn send_flag_evaluation_metrics(
        &self,
        request: tonic::Request<SendFlagEvaluationMetricsRequest>,
    ) -> std::result::Result<tonic::Response<SendFlagEvaluationMetricsResponse>, tonic::Status>
    {
        if self.conn_creds.is_none() && request.metadata().get(APIKEY).is_none() {
            return Ok(tonic::Response::new(
                SendFlagEvaluationMetricsResponse::default(),
            ));
        }
        self.distro_client
            .clone()
            .to_owned()
            .send_flag_evaluation_metrics(override_api_key(request, &self.conn_creds))
            .await
    }

    async fn register_client(
        &self,
        request: tonic::Request<RegisterClientRequest>,
    ) -> std::result::Result<tonic::Response<RegisterClientResponse>, tonic::Status> {
        if self.conn_creds.is_none() && request.metadata().get(APIKEY).is_none() {
            return Ok(tonic::Response::new(RegisterClientResponse::default()));
        }
        self.distro_client
            .clone()
            .to_owned()
            .register_client(override_api_key(request, &self.conn_creds))
            .await
    }

    async fn deregister_client(
        &self,
        request: tonic::Request<DeregisterClientRequest>,
    ) -> std::result::Result<tonic::Response<DeregisterClientResponse>, tonic::Status> {
        if self.conn_creds.is_none() && request.metadata().get(APIKEY).is_none() {
            return Ok(tonic::Response::new(DeregisterClientResponse::default()));
        }
        self.distro_client
            .clone()
            .to_owned()
            .deregister_client(override_api_key(request, &self.conn_creds))
            .await
    }

    async fn get_developer_access_token(
        &self,
        _request: tonic::Request<GetDeveloperAccessTokenRequest>,
    ) -> std::result::Result<tonic::Response<GetDeveloperAccessTokenResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented(
            "cannot issue tokens from sidecar",
        ))
    }
}
