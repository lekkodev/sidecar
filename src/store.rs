use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, RwLock},
    time::Duration,
};

use crate::{
    gen::mod_cli::lekko::{
        backend::v1beta1::{
            distribution_service_client::DistributionServiceClient, GetRepositoryContentsRequest,
            GetRepositoryContentsResponse, GetRepositoryVersionRequest, Namespace,
        },
        feature::v1beta1::Feature,
    },
    repofs::RepoFS,
    types::{add_api_key, ConnectionCredentials, FeatureRequestParams, Mode, APIKEY},
};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use itertools::any;
use lazy_static::lazy_static;
use log::{error, info, warn};
use notify::{
    Error, Event,
    EventKind::{Create, Modify, Remove},
    PollWatcher, RecursiveMode, Watcher,
};
use regex::Regex;
use tonic::{body::BoxBody, Request};

use tokio::task::JoinHandle;

// Store acts as the abstraction for the storage and retrieval of all features.
// Internally there is a state machine that has two states: registered and unregistered.
// A store starts unregistered, and a single register call will make it registered.
// Once registered, a store will poll for new configuration and ignore subsequent register calls.
//
// We have an std RwLock around any concurrent state that we currently manage.
// Std locks are fine to use in async functions as long as we don't hold them across
// await's. This is enforced by the compiler, but just something to keep in mind.
pub struct Store {
    state: Arc<RwLock<ConcurrentState>>,
    // keeping the join handle around keeps the poll watcher in scope,
    // which is necessary to receive watch events from the filesystem.
    _join_handle: Option<JoinHandle<PollWatcher>>,
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

type FeatureStore = HashMap<FeatureKey, FeatureInfo>;

struct ConcurrentState {
    cache: FeatureStore,
    repo_version: String,
}

pub struct FeatureData {
    pub commit_sha: String,
    pub feature_sha: String,
    pub feature: Feature,
}

lazy_static! {
    // Lazily initialize static regex so we don't need to compile it repeatedly
    static ref PROTO_BIN_FILE: Regex = match Regex::new(r"^.*/gen/proto/[\w.-]+\.proto\.bin$") {
        Ok(r) => r,
        Err(e) => panic!("failed to initialize regex {e:}"),
    };
}

async fn fs_watch(path: String, state: Arc<RwLock<ConcurrentState>>) -> PollWatcher {
    let watch_path = path.clone();
    let mut watcher = match notify::PollWatcher::new(
        move |res: Result<Event, Error>| match res {
            Ok(event) => {
                match event.kind {
                    Create(_) | Modify(_) | Remove(_) => {
                        if any(&event.paths, |s| {
                            match s.to_owned().into_os_string().into_string() {
                                Ok(st) => PROTO_BIN_FILE.to_owned().is_match(&st),
                                Err(e) => {
                                    warn!("failed to convert path {e:?} to string");
                                    false // don't reload contents for paths that aren't unicode
                                }
                            }
                        }) {
                            let path = path.clone();
                            match RepoFS::new(path).and_then(|r| r.load()) {
                                Ok(res) => {
                                    {
                                        // obtain lock again to replace data
                                        let mut state_guard = state.write().unwrap();
                                        state_guard.cache = create_feature_store(res.namespaces);
                                        state_guard.repo_version = res.commit_sha.to_owned();
                                        // drop state_guard
                                    }
                                    info!(
                                        "loaded repo contents for commit sha {:}",
                                        res.commit_sha
                                    );
                                }
                                Err(e) => {
                                    warn!("failed to load repo contents from filesystem: {e:}")
                                }
                            };
                        }
                    }
                    _ => (),
                }
            }
            Err(e) => error!("fs watch error: {:?}", e),
        },
        notify::Config::default()
            .with_compare_contents(false)
            .with_poll_interval(Duration::from_secs(1)),
    ) {
        Ok(w) => w,
        Err(e) => {
            panic!("error initializing poll watcher {e:}")
        }
    };
    if let Err(e) = watcher.watch(Path::new(&watch_path), RecursiveMode::Recursive) {
        panic!("error watching path {watch_path}: {e:}")
    }
    watcher
}

async fn poll_loop(
    dist_client: DistributionServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
    state: Arc<RwLock<ConcurrentState>>,
    conn_creds: ConnectionCredentials,
    poll_duration: Duration,
) {
    let mut interval = tokio::time::interval(poll_duration);
    loop {
        interval.tick().await;
        // fetch version
        let new_version =
            match get_repo_version_remote(dist_client.clone(), conn_creds.clone()).await {
                Ok(v) => v,
                Err(err) => {
                    // TODO: exp backoff when we have errors
                    error!("got an error when fetching version {err:?}");
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

        info!("found new version: {new_version}, fetching");

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
                    state_guard.repo_version = res.commit_sha.to_owned();
                    // drop state_guard
                }
                info!("loaded repo contents for commit sha {:}", res.commit_sha);
            }
            Err(err) => {
                // This is a problem, error loudly.
                error!("error encountered when fetching full repository state: {err:?}",);
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
    dist_client
        .get_repository_version(add_api_key(
            GetRepositoryVersionRequest {
                repo_key: Some(conn_creds.repo_key),
                session_key: conn_creds.session_key,
            },
            conn_creds.api_key,
        ))
        .await
        .map(|resp| resp.into_inner().commit_sha)
}

async fn get_repo_contents_remote(
    mut dist_client: DistributionServiceClient<
        hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
    >,
    request: Request<GetRepositoryContentsRequest>,
) -> Result<GetRepositoryContentsResponse, tonic::Status> {
    dist_client
        .get_repository_contents(request)
        .await
        .map(|resp| resp.into_inner())
}

impl Store {
    pub fn new(
        dist_client: DistributionServiceClient<
            hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
        >,
        contents: GetRepositoryContentsResponse,
        conn_creds: Option<ConnectionCredentials>,
        poll_interval: Duration,
        mode: Mode,
        repo_path: String,
    ) -> Self {
        let state = Arc::new(RwLock::new(ConcurrentState {
            cache: create_feature_store(contents.namespaces),
            repo_version: contents.commit_sha,
        }));
        // Depending on the mode, we will either subscribe to dynamic updates
        // from the filesystem (static mode), or from Lekko backend (default mode).
        let jh = match mode {
            Mode::Static => Some(tokio::spawn(fs_watch(repo_path, state.clone()))),
            _ => {
                // TODO: worry about this join handle.
                tokio::spawn(poll_loop(
                    dist_client,
                    state.clone(),
                    conn_creds.unwrap(),
                    poll_interval,
                ));
                None
            }
        };
        Self {
            state,
            _join_handle: jh,
        }
    }

    pub fn get_feature_local(&self, request: FeatureRequestParams) -> Option<FeatureData> {
        let ConcurrentState {
            cache,
            repo_version,
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
