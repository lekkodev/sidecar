use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tonic::{body::BoxBody, Request};

use crate::{
    gen::lekko::{
        backend::v1beta1::{
            distribution_service_client::DistributionServiceClient, GetRepositoryContentsRequest,
            RepositoryKey,
        },
        feature::v1beta1::Feature,
    },
    types::{FeatureRequestParams, APIKEY},
};

// Store acts as the abstraction for the storage and retrieval of all features.
// For now, this object will defer to lekko backend's DistributionService to retrieve
// features. However, in the future, this object is expected to manage the storage of
// features in-memory.
pub struct Store {
    dist_client: DistributionServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
}

pub struct FeatureData {
    pub commit_sha: String,
    pub feature_sha: String,
    pub feature: Feature,
}

impl Store {
    pub fn new(
        dist_client: DistributionServiceClient<
            hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
        >,
    ) -> Self {
        Self { dist_client }
    }
    pub async fn get_feature(
        &self,
        request: FeatureRequestParams,
    ) -> Result<FeatureData, tonic::Status> {
        println!("Store: get feature {:?}", request);
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
        let resp = self
            .dist_client
            .clone()
            .get_repository_contents(dist_req)
            .await;
        if let Err(e) = resp {
            println!("error fetching feature from distribution service {:?}", e);
            return Err(e);
        }
        let success_resp = resp.unwrap().into_inner();
        println!(
            "received feature contents for commit sha {}",
            success_resp.commit_sha,
        );

        for namespace in success_resp.namespaces {
            for feature in namespace.features {
                println!(
                    "received feature {} with blob sha {}",
                    feature.name, feature.sha
                );
                if let Some(some_feature) = feature.feature {
                    return Ok(FeatureData {
                        commit_sha: success_resp.commit_sha,
                        feature_sha: feature.sha,
                        feature: some_feature,
                    });
                }
            }
        }
        Err(tonic::Status::not_found("feature not found"))
    }
}
