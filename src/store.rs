use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use crate::{
    gen::lekko::{
        backend::v1beta1::{
            distribution_service_client::DistributionServiceClient, GetRepositoryContentsRequest,
            GetRepositoryContentsResponse, GetRepositoryVersionRequest, Namespace,
            RegisterClientRequest, RepositoryKey,
        },
        feature::v1beta1::Feature,
    },
    types::{FeatureRequestParams, APIKEY},
};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use prost::Message;
use tonic::{
    body::BoxBody,
    metadata::{Ascii, MetadataValue},
    Request,
};

use tokio::sync::oneshot::{Receiver, Sender};

// Store acts as the abstraction for the storage and retrieval of all features.
// Internally there is a state machine that has two states: registered and unregistered.
// A store starts unregistered, and a single register call will make it registered.
// Once registered, a store will poll for new configuration and ignore subsequent register calls.
//
// We have an std RwLock around any concurrent state that we currently manage.
// Std locks are fine to use in async functions as long as we don't hold them across
// await's. This is enforced by the compiler, but just something to keep in mind.
pub struct Store {
    dist_client: DistributionServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
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
    session_key: String,
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
    // We begin the loop by waiting for a register call.
    // Once the register call comes through, we can start fetching on a poll.
    //
    // Keep conn_creds around to not have to read it from concurrent state all the time.
    let conn_creds = match rx.await {
        Ok(data) => {
            let version = data.repo_version.clone();
            let mut state_guard = state.write().unwrap();
            *state_guard = data;
            println!("got register message, starting polling from version: {version}",);
            state_guard.conn_creds.clone()
        }
        Err(err) => {
            // TODO: handle panics better.
            panic!("error encountered when initializing sidecar state: {err:?}",)
        }
    };

    // TODO: make configurable.
    let mut interval = tokio::time::interval(Duration::from_millis(1000));
    loop {
        interval.tick().await;
        // fetch version
        let new_version =
            match get_repo_version_remote(dist_client.clone(), conn_creds.clone()).await {
                Ok(v) => v,
                Err(err) => {
                    // TODO: exp backoff when we have errors
                    println!("got an error when fetching version {err:?}");
                    continue;
                }
            };

        {
            let state_guard = state.read().unwrap();
            if state_guard.repo_version == new_version {
                continue;
            }
            // release read lock to fetch data
        };

        println!("polled for new version: {new_version}, will fetch from remote",);

        match get_repo_contents_remote(
            dist_client.clone(),
            conn_creds_to_repo_contents_request(conn_creds.clone()),
        )
        .await
        {
            Ok(res) => {
                {
                    // obtain lock again to replace data
                    let mut state_guard = state.write().unwrap();
                    state_guard.cache = create_feature_store(res.namespaces);
                    state_guard.repo_version = res.commit_sha;
                    // drop state_guard
                }
                println!("updated to new version: {new_version}");
            }
            Err(err) => {
                // This is a problem, error loudly.
                println!("error encountered when fetching full repository state: {err:?}",);
            }
        }
    }
}

fn conn_creds_to_repo_contents_request(
    conn_creds: ConnectionCredentials,
) -> Request<GetRepositoryContentsRequest> {
    let mut req = Request::new(GetRepositoryContentsRequest {
        repo_key: Some(conn_creds.repo_key),
        session_key: conn_creds.session_key,
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

async fn get_repo_version_remote(
    mut dist_client: DistributionServiceClient<
        hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
    >,
    conn_creds: ConnectionCredentials,
) -> Result<String, tonic::Status> {
    match dist_client
        .get_repository_version(add_api_key(
            GetRepositoryVersionRequest {
                repo_key: Some(conn_creds.repo_key),
                session_key: conn_creds.session_key,
            },
            conn_creds.api_key,
        ))
        .await
        .map(|resp| resp.into_inner())
    {
        Ok(resp) => Ok(resp.commit_sha),
        Err(err) => {
            println!("error fetching repo version from distribution service {err:?}",);
            Err(err)
        }
    }
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
            println!("received contents for commit sha {}", resp.commit_sha);
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
        bootstrap_data: Option<GetRepositoryContentsResponse>,
    ) -> Self {
        let (tx, rx) = tokio::sync::oneshot::channel::<ConcurrentState>();
        let state = Arc::new(RwLock::new(match bootstrap_data {
            None => ConcurrentState {
                cache: HashMap::new(),
                repo_version: "".to_string(),
                conn_creds: ConnectionCredentials {
                    repo_key: RepositoryKey::default(),
                    api_key: MetadataValue::from_static(""),
                    session_key: "".to_string(),
                },
            },
            Some(contents) => ConcurrentState {
                cache: create_feature_store(contents.namespaces),
                repo_version: contents.commit_sha,
                conn_creds: ConnectionCredentials {
                    repo_key: RepositoryKey::default(),
                    api_key: MetadataValue::from_static(""),
                    session_key: "".to_string(),
                },
            },
        }));
        // TODO: worry about this join handle.
        tokio::spawn(poll_loop(rx, dist_client.clone(), state.clone()));
        Self {
            dist_client,
            state,
            initialize_tx: Mutex::new(Some(tx)),
        }
    }

    pub async fn register(
        &self,
        repo_key: RepositoryKey,
        // TODO: handle multiple namespaces by iterating over namespaces or changing GetRepoContents
        namespaces: &[String],
        api_key: MetadataValue<Ascii>,
    ) -> Result<(), tonic::Status> {
        // Proxy register call first
        //
        // Because we do this one line, we should drop the read lock right after.
        let bootstrap_sha = self.state.read().unwrap().repo_version.clone();

        let session_key = match self
            .dist_client
            .clone()
            .register_client(add_api_key(
                RegisterClientRequest {
                    repo_key: Some(repo_key.clone()),
                    initial_bootstrap_sha: bootstrap_sha,
                    // TODO sidecar version
                    sidecar_version: "".to_string(),
                    namespace_list: Vec::from(namespaces),
                },
                api_key.clone(),
            ))
            .await
        {
            Ok(resp) => resp.into_inner().session_key,
            Err(error) => {
                // If we have an error registering, we probably can't reach lekko. If we operate off of a bootstrap
                // we can continue to function off of that information. Unfortunately, we won't get updates. More
                // work will have to be done here to recover on a loop. For now, we return an error such that we expect
                // the SDK or client to retry the registration.
                return Err(tonic::Status::resource_exhausted(format!(
                    "error when registering with lekko {:?}",
                    error
                )));
            }
        };

        let success_resp = get_repo_contents_remote(
            self.dist_client.clone(),
            add_api_key(
                GetRepositoryContentsRequest {
                    repo_key: Some(repo_key.clone()),
                    ..Default::default()
                },
                api_key.clone(),
            ),
        )
        .await?;
        let mut guard = self.initialize_tx.lock().unwrap();
        match guard.deref_mut().take() {
            Some(sender) => sender
                .send(ConcurrentState {
                    cache: create_feature_store(success_resp.namespaces),
                    repo_version: success_resp.commit_sha,
                    conn_creds: ConnectionCredentials {
                        repo_key,
                        api_key,
                        session_key,
                    },
                })
                .map_err(|_| {
                    tonic::Status::internal(
                        "receiver dropped for register rpc. something is seriously wrong",
                    )
                }),
            // If we want multi-tenant sidecars in the future, this will have to change.
            None => Err(tonic::Status::already_exists(
                "register has already been called on this sidecar, ignoring register RPC",
            )),
        }
    }

    pub fn get_feature_local(&self, request: FeatureRequestParams) -> Option<FeatureData> {
        let ConcurrentState {
            cache,
            repo_version,
            conn_creds: _,
        } = &*self.state.read().unwrap();
        return cache
            .get(&FeatureKey {
                namespace: request.namespace.clone(),
                feature: request.feature,
            })
            .map(|feature| FeatureData {
                feature: feature.feature.clone(),
                commit_sha: repo_version.clone(),
                feature_sha: feature.version.clone(),
            });
    }
}

fn add_api_key<T: Message>(m: T, api_key: MetadataValue<Ascii>) -> tonic::Request<T> {
    let mut r = Request::new(m);
    r.metadata_mut().append(APIKEY, api_key);
    r
}
