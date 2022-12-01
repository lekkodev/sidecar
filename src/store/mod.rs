use std::str::FromStr;

use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tonic::{
    body::BoxBody,
    metadata::{AsciiMetadataValue, MetadataValue},
    Request,
};

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

#[derive(Clone)]
pub struct Store {
    dist_client: DistributionServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
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
    ) -> Result<Feature, tonic::Status> {
        println!("Store: get feature {:?}", request);
        let mut dist_req = Request::new(GetRepositoryContentsRequest {
            repo_key: Option::Some(RepositoryKey {
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
        if resp.is_err() {
            println!(
                "error fetching feature from distribution service {:?}",
                resp
            );
            return Result::Err(resp.unwrap_err());
        }
        let success_resp = resp.unwrap().into_inner();
        println!(
            "received feature contents for commit sha {}",
            success_resp.commit_sha
        );

        for namespace in success_resp.namespaces {
            for feature in namespace.features {
                println!(
                    "received feature {} with blob sha {}",
                    feature.name, feature.sha
                );
                if feature.feature.is_some() {
                    return Result::Ok(feature.feature.unwrap());
                }
            }
        }
        return Result::Err(tonic::Status::not_found("feature not found"));
    }
}