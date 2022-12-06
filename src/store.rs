use crate::{
    gen::lekko::{
        backend::v1beta1::{
            configuration_service_client::ConfigurationServiceClient,
            distribution_service_client::DistributionServiceClient, GetRepositoryContentsRequest,
            GetRepositoryContentsResponse, RepositoryKey,
        },
        feature::v1beta1::Feature,
    },
    types::{FeatureRequestParams, APIKEY},
};
use dashmap::DashMap;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tonic::{
    body::BoxBody,
    metadata::{Ascii, MetadataValue},
    Request,
};

// Store acts as the abstraction for the storage and retrieval of all features.
// For now, this object will defer to lekko backend's DistributionService to retrieve
// features. However, in the future, this object is expected to manage the storage of
// features in-memory.
pub struct Store {
    dist_client: DistributionServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
    // TODO: proxy register request
    _config_client:
        ConfigurationServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
    cache: DashMap<(String, String), Feature>,
}

impl Store {
    pub fn new(
        dist_client: DistributionServiceClient<
            hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
        >,
        config_client: ConfigurationServiceClient<
            hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
        >,
    ) -> Self {
        Self {
            dist_client,
            _config_client: config_client,
            cache: DashMap::new(),
        }
    }

    pub async fn register(
        &self,
        repo_key: RepositoryKey,
        // TODO: handle multiple namespaces by iterating over namespaces or changing GetRepoContents
        _namespaces: &[String],
        api_key: MetadataValue<Ascii>,
    ) -> Result<(), tonic::Status> {
        let mut dist_req = Request::new(GetRepositoryContentsRequest {
            repo_key: Some(repo_key),
            ..Default::default()
        });
        // Add the apikey header
        dist_req.metadata_mut().append(APIKEY, api_key);
        let success_resp = self.get_repo_contents_remote(dist_req).await?;
        for namespace in success_resp.namespaces {
            for feature in namespace.features {
                self.cache.insert(
                    (namespace.name.clone(), feature.name),
                    feature.feature.unwrap(),
                );
            }
        }
        Ok(())
    }

    async fn get_repo_contents_remote(
        &self,
        request: Request<GetRepositoryContentsRequest>,
    ) -> Result<GetRepositoryContentsResponse, tonic::Status> {
        match self
            .dist_client
            .clone()
            .get_repository_contents(request)
            .await
            .map(|resp| resp.into_inner())
        {
            Ok(resp) => {
                println!(
                    "received feature contents for commit sha {}",
                    resp.commit_sha,
                );
                Ok(resp)
            }
            Err(error) => {
                println!(
                    "error fetching feature from distribution service {:?}",
                    error
                );
                Err(error)
            }
        }
    }
    pub async fn get_feature(
        &self,
        request: FeatureRequestParams,
    ) -> Result<Feature, tonic::Status> {
        if let Some(feature) = self
            .cache
            .get(&(request.namespace.clone(), request.feature.clone()))
        {
            // TODO: revisit if we should borrow in this signature.
            return Ok(feature.clone());
        }

        println!(
            "Store: get feature {:?} without a register, falling back to remote",
            request
        );
        let mut dist_req = Request::new(GetRepositoryContentsRequest {
            repo_key: Some(RepositoryKey {
                owner_name: request.rk.owner_name,
                repo_name: request.rk.repo_name,
            }),
            namespace_name: request.namespace,
            feature_name: request.feature,
        });
        // Add the apikey header
        dist_req.metadata_mut().append(APIKEY, request.api_key);
        let success_resp = self.get_repo_contents_remote(dist_req).await?;
        for namespace in success_resp.namespaces {
            for feature in namespace.features {
                println!(
                    "received feature {} with blob sha {}",
                    feature.name, feature.sha
                );
                if feature.feature.is_some() {
                    return Ok(feature.feature.unwrap());
                }
            }
        }
        Err(tonic::Status::not_found("feature not found"))
    }
}
