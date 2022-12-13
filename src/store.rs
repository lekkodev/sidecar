use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use crate::{
    gen::lekko::{
        backend::v1beta1::{
            configuration_service_client::ConfigurationServiceClient,
            distribution_service_client::DistributionServiceClient, GetRepositoryContentsRequest,
            GetRepositoryContentsResponse, Namespace, RepositoryKey,
        },
        feature::v1beta1::Feature,
    },
    types::{FeatureRequestParams, APIKEY},
};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tonic::{
    body::BoxBody,
    metadata::{Ascii, MetadataValue},
    Request,
};

use tokio::sync::oneshot::{Receiver, Sender};

// Store acts as the abstraction for the storage and retrieval of all features.
// We have an std RwLock around any concurrent state that we currently manage.
// Std locks are fine to use in async functions as long as we don't hold them across
// await's. This is enforced by the compiler, but just something to keep in mind.
pub struct Store {
    dist_client: DistributionServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
    // TODO: proxy register request
    _config_client:
        ConfigurationServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
    state: Arc<RwLock<ConcurrentState>>,
    initialize_tx: Mutex<Option<Sender<ConcurrentState>>>,
}

#[derive(PartialEq, Eq, Hash)]
struct FeatureKey {
    namespace: String,
    feature: String,
}

struct FeatureInfo {
    feature: Feature,
    version: String,
}

#[derive(Clone)]
struct ConnectionCredentials {
    repo_key: RepositoryKey,
    api_key: MetadataValue<Ascii>,
}

type FeatureStore = HashMap<FeatureKey, FeatureInfo>;

struct ConcurrentState {
    cache: FeatureStore,
    repo_version: String,
    conn_creds: ConnectionCredentials,
}

pub struct FeatureData {
    pub commit_sha: String,
    pub feature_sha: String,
    pub feature: Feature,
}

async fn poll_loop(
    rx: Receiver<ConcurrentState>,
    dist_client: DistributionServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
    state: Arc<RwLock<ConcurrentState>>,
) {
    match rx.await {
        Ok(data) => {
            let version = data.repo_version.clone();
            let mut state_guard = state.write().unwrap();
            *state_guard = data;
            // release state_guard
            drop(state_guard);
            println!(
                "got register message, starting polling from version: {}",
                version
            );
        }
        Err(err) => {
            // TODO: handle panics better.
            panic!(
                "error encountered when initializing sidecar state: {:?}",
                err
            )
        }
    }

    // TODO: make configurable.
    let mut interval = tokio::time::interval(Duration::from_millis(1000));
    loop {
        interval.tick().await;
        // fetch version
        let new_version = "".to_string();

        let conn_creds = {
            let state_guard = state.read().unwrap();

            if state_guard.repo_version == new_version {
                continue;
            }
            state_guard.conn_creds.clone()
            // release read lock to fetch data
        };

        match get_repo_contents_remote(
            dist_client.clone(),
            conn_creds_to_repo_contents_request(conn_creds),
        )
        .await
        {
            Ok(res) => {
                // obtain lock again to replace data
                let mut state_guard = state.write().unwrap();
                (*state_guard).cache = create_feature_store(res.namespaces);
                (*state_guard).repo_version = res.commit_sha;
                // drop state_guard
            }
            Err(err) => {
                // This is a problem, error loudly.
                println!(
                    "error encountered when fetching full repository state: {:?}",
                    err
                );
            }
        }
    }
}

fn conn_creds_to_repo_contents_request(
    conn_creds: ConnectionCredentials,
) -> Request<GetRepositoryContentsRequest> {
    let mut req = Request::new(GetRepositoryContentsRequest {
        repo_key: Some(conn_creds.repo_key),
        // TODO: do namespaces correctly.
        namespace_name: "".to_string(),
        feature_name: "".to_string(),
    });
    req.metadata_mut().append(APIKEY, conn_creds.api_key);
    req
}

fn create_feature_store(namespaces: Vec<Namespace>) -> FeatureStore {
    // TODO add a flatmap here to only init once since we know the size beforehand.
    let mut feature_store = HashMap::new();
    namespaces.into_iter().for_each(|namespace| {
        namespace.features.into_iter().for_each(|feature| {
            feature_store.insert(
                FeatureKey {
                    namespace: namespace.name.clone(),
                    feature: feature.name,
                },
                FeatureInfo {
                    feature: feature.feature.unwrap(),
                    version: feature.sha,
                },
            );
        })
    });
    feature_store
}

async fn get_repo_contents_remote(
    mut dist_client: DistributionServiceClient<
        hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
    >,
    request: Request<GetRepositoryContentsRequest>,
) -> Result<GetRepositoryContentsResponse, tonic::Status> {
    match dist_client
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

impl Store {
    pub fn new(
        dist_client: DistributionServiceClient<
            hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
        >,
        config_client: ConfigurationServiceClient<
            hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
        >,
    ) -> Self {
        // TODO: worry about this join handle.
        let (tx, rx) = tokio::sync::oneshot::channel::<ConcurrentState>();
        let state = Arc::new(RwLock::new(ConcurrentState {
            cache: HashMap::new(),
            repo_version: "".to_string(),
            conn_creds: ConnectionCredentials {
                repo_key: RepositoryKey::default(),
                api_key: MetadataValue::from_static(""),
            },
        }));
        tokio::spawn(poll_loop(rx, dist_client.clone(), state.clone()));
        Self {
            dist_client,
            _config_client: config_client,
            state,
            initialize_tx: Mutex::new(Some(tx)),
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
            repo_key: Some(repo_key.clone()),
            ..Default::default()
        });
        // Add the apikey header
        dist_req.metadata_mut().append(APIKEY, api_key.clone());
        let success_resp = get_repo_contents_remote(self.dist_client.clone(), dist_req).await?;
        let mut guard = self.initialize_tx.lock().unwrap();
        match guard.deref_mut().take() {
            Some(sender) => sender
                .send(ConcurrentState {
                    cache: create_feature_store(success_resp.namespaces),
                    repo_version: success_resp.commit_sha,
                    conn_creds: ConnectionCredentials { repo_key, api_key },
                })
                .map_err(|_| {
                    tonic::Status::internal(
                        "receiver dropped for register rpc. something is seriously wrong",
                    )
                }),
            None => Err(tonic::Status::already_exists(
                "register has already been called on this sidecar, ignoring register RPC",
            )),
        }
    }

    pub async fn get_feature(
        &self,
        request: FeatureRequestParams,
    ) -> Result<FeatureData, tonic::Status> {
        {
            let ConcurrentState {
                cache,
                repo_version,
                conn_creds: _,
            } = &*self.state.read().unwrap();
            if let Some(feature) = cache.get(&FeatureKey {
                namespace: request.namespace.clone(),
                feature: request.feature.clone(),
            }) {
                // TODO: revisit if we should borrow in this signature.
                return Ok(FeatureData {
                    feature: feature.feature.clone(),
                    commit_sha: repo_version.clone(),
                    feature_sha: feature.version.clone(),
                });
            }
            // drop read_lock
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
        let success_resp = get_repo_contents_remote(self.dist_client.clone(), dist_req).await?;
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
