use std::sync::Arc;

use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tonic::{body::BoxBody, Request, Response, Status};

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
    pub sidecar_version: String,
}

const TIERED_PREFIX: &str = "tiered";

#[tonic::async_trait]
impl DistributionService for Service {
    async fn get_repository_version(
        &self,
        request: Request<GetRepositoryVersionRequest>,
    ) -> Result<tonic::Response<GetRepositoryVersionResponse>, tonic::Status> {
        let requested_rk = request
            .get_ref()
            .repo_key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("no repo key provided"))?;
        if self.repo_key.owner_name != requested_rk.owner_name
            || self.repo_key.repo_name != requested_rk.repo_name
        {
            return Err(Status::invalid_argument(format!(
                "registration mismatch: requested_repo: {:?}, vs. repo: {:?}",
                requested_rk, self.repo_key
            )));
        }
        return Ok(Response::new(GetRepositoryVersionResponse {
            commit_sha: self.store.get_version_local(),
        }));
    }
    async fn get_repository_contents(
        &self,
        request: Request<GetRepositoryContentsRequest>,
    ) -> Result<tonic::Response<GetRepositoryContentsResponse>, tonic::Status> {
        let requested_rk = request
            .get_ref()
            .repo_key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("no repo key provided"))?;
        if self.repo_key.owner_name != requested_rk.owner_name
            || self.repo_key.repo_name != requested_rk.repo_name
        {
            return Err(Status::invalid_argument(format!(
                "registration mismatch: requested_repo: {:?}, vs. repo: {:?}",
                requested_rk, self.repo_key
            )));
        }
        let request = request.into_inner();
        let (version, namespaces, file_descriptor_set) = self
            .store
            .get_repo_contents_local(&request.namespace_name, &request.feature_name);
        Ok(Response::new(GetRepositoryContentsResponse {
            namespaces,
            commit_sha: version,
            file_descriptor_set,
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
        let mut new_req: Request<SendFlagEvaluationMetricsRequest> = tonic::Request::new(request.get_ref().clone());
        new_req.metadata_mut().clone_from(request.metadata());
        new_req.metadata_mut().remove("content-length");
        self.distro_client
            .clone()
            .to_owned()
            .send_flag_evaluation_metrics(override_api_key(new_req, &self.conn_creds))
            .await
    }

    async fn register_client(
        &self,
        request: tonic::Request<RegisterClientRequest>,
    ) -> std::result::Result<tonic::Response<RegisterClientResponse>, tonic::Status> {
        let requested_rk = request
            .get_ref()
            .repo_key
            .as_ref()
            .ok_or_else(|| Status::invalid_argument("no repo key provided"))?;
        if self.repo_key.owner_name != requested_rk.owner_name
            || self.repo_key.repo_name != requested_rk.repo_name
        {
            return Err(Status::invalid_argument(format!(
                "registration mismatch: requested_repo: {:?}, vs. repo: {:?}",
                requested_rk, self.repo_key
            )));
        }
        if self.conn_creds.is_none() && request.metadata().get(APIKEY).is_none() {
            return Ok(tonic::Response::new(RegisterClientResponse::default()));
        }
        let mut register_request = request.get_ref().clone();
        match register_request.sidecar_version.len() {
            0 => {
                register_request.sidecar_version =
                    format!("{}_{}", TIERED_PREFIX, self.sidecar_version.clone())
            }
            _ => register_request
                .sidecar_version
                .push_str(format!("_{}_{}", TIERED_PREFIX, self.sidecar_version.clone()).as_str()),
        }
        let mut new_req: Request<RegisterClientRequest> = tonic::Request::new(register_request);
        new_req.metadata_mut().clone_from(request.metadata());
        new_req.metadata_mut().remove("content-length");
        self.distro_client
            .clone()
            .to_owned()
            .register_client(override_api_key(new_req, &self.conn_creds))
            .await
    }

    async fn deregister_client(
        &self,
        request: tonic::Request<DeregisterClientRequest>,
    ) -> std::result::Result<tonic::Response<DeregisterClientResponse>, tonic::Status> {
        if self.conn_creds.is_none() && request.metadata().get(APIKEY).is_none() {
            return Ok(tonic::Response::new(DeregisterClientResponse::default()));
        }
        let mut req = tonic::Request::new(request.get_ref().clone());
        req.metadata_mut().clone_from(request.metadata());
        req.metadata_mut().remove("content-length");
        self.distro_client
            .clone()
            .to_owned()
            .deregister_client(override_api_key(req, &self.conn_creds))
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
