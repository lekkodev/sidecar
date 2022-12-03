use tonic::metadata::{Ascii, MetadataValue};

pub const APIKEY: &str = "apikey";

#[derive(Debug)]
pub struct RepositoryKey {
    pub owner_name: String,
    pub repo_name: String,
}

// Contains all parameters needed to fetch a feature.
#[derive(Debug)]
pub struct FeatureRequestParams {
    // The API key that allows Lekko to identify the team that the feature is under.
    pub api_key: MetadataValue<Ascii>,
    pub rk: RepositoryKey,
    pub namespace: String,
    pub feature: String,
}
