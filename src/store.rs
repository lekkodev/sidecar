use std::{
    collections::HashMap,
    ops::DerefMut,
    path::Path,
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use crate::{
    bootstrap::Bootstrap,
    gen::mod_cli::lekko::{
        backend::v1beta1::{
            distribution_service_client::DistributionServiceClient, DeregisterClientRequest,
            GetRepositoryContentsRequest, GetRepositoryContentsResponse,
            GetRepositoryVersionRequest, Namespace, RegisterClientRequest, RepositoryKey,
        },
        feature::v1beta1::Feature,
    },
    service::Mode,
    types::{FeatureRequestParams, APIKEY},
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
use prost::Message;
use regex::Regex;
use tonic::{
    body::BoxBody,
    metadata::{Ascii, MetadataValue},
    Request,
};

use tokio::{
    sync::oneshot::{Receiver, Sender},
    task::JoinHandle,
};

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

lazy_static! {
    static ref RG: Regex = match Regex::new(r"^.*/gen/proto/[\w.-]+\.proto\.bin$") {
        Ok(r) => r,
        Err(e) => panic!("failed to initialize regex {e:}"),
    };
}

async fn fs_watch(path: String, state: Arc<RwLock<ConcurrentState>>) -> PollWatcher {
    info!("STARTING WATCHER");
    let watch_path = path.clone();
    let mut watcher = match notify::PollWatcher::new(
        move |res: Result<Event, Error>| match res {
            Ok(event) => {
                match event.kind {
                    Create(_) | Modify(_) | Remove(_) => {
                        if any(&event.paths, |s| {
                            match s.to_owned().into_os_string().into_string() {
                                Ok(st) => RG.to_owned().is_match(&st),
                                Err(e) => {
                                    warn!("failed to convert path {e:?} to string");
                                    false
                                }
                            }
                        }) {
                            info!("WOW: event: {:?}", event);
                            let path = path.clone();
                            match Bootstrap::new(path).load() {
                                Ok(res) => {
                                    {
                                        // obtain lock again to replace data
                                        let mut state_guard = state.write().unwrap();
                                        state_guard.cache = create_feature_store(res.namespaces);
                                        state_guard.repo_version = res.commit_sha;
                                        // drop state_guard
                                    }
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
            .with_compare_contents(true)
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
    info!("STARTED WATCHER");
    watcher
}

async fn poll_loop(
    rx: Receiver<ConcurrentState>,
    dist_client: DistributionServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
    state: Arc<RwLock<ConcurrentState>>,
    poll_duration: Duration,
) {
    // We begin the loop by waiting for a register call.
    // Once the register call comes through, we can start fetching on a poll.
    //
    // Keep conn_creds around to not have to read it from concurrent state all the time.
    let conn_creds = match rx.await {
        Ok(data) => {
            let version = data.repo_version.clone();
            info!(
                "got register message, starting polling for {}/{} from version: {version}",
                &data.conn_creds.repo_key.owner_name, &data.conn_creds.repo_key.repo_name
            );
            let mut state_guard = state.write().unwrap();
            *state_guard = data;
            state_guard.conn_creds.clone()
        }
        Err(err) => {
            // TODO: handle panics better.
            panic!("error encountered when initializing sidecar state: {err:?}",)
        }
    };

    let mut interval = tokio::time::interval(poll_duration);
    loop {
        interval.tick().await;
        // fetch version
        let new_version =
            match get_repo_version_remote(dist_client.clone(), conn_creds.clone()).await {
                Ok(v) => v,
                Err(err) => {
                    // TODO: exp backoff when we have errors
                    warn!("got an error when fetching version {err:?}");
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
                    state_guard.repo_version = res.commit_sha;
                    // drop state_guard
                }
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
        bootstrap_data: Option<GetRepositoryContentsResponse>,
        poll_interval: Duration,
        mode: Mode,
        repo_path: Option<String>,
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
        let jh = match mode {
            Mode::Static => {
                let path = match repo_path {
                    None => panic!("no repo path provided for sidecar configured to be static"),
                    Some(p) => p,
                };
                Some(tokio::spawn(fs_watch(path, state.clone())))
            }
            _ => {
                tokio::spawn(poll_loop(
                    rx,
                    dist_client.clone(),
                    state.clone(),
                    poll_interval,
                ));
                None
            }
        };
        Self {
            dist_client,
            state,
            initialize_tx: Mutex::new(Some(tx)),
            _join_handle: jh,
        }
    }

    pub async fn deregister(&self) -> Result<(), tonic::Status> {
        // TODO: block connections and/or figure out multi-tenant sidecar with a semaphore for our state machine.
        let ConnectionCredentials {
            session_key,
            api_key,
            ..
        } = self.state.read().unwrap().conn_creds.clone();
        self.dist_client
            .clone()
            .deregister_client(add_api_key(
                DeregisterClientRequest { session_key },
                api_key,
            ))
            .await
            .map(|_| ())
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
            Err(err) => {
                // If we have an error registering, we probably can't reach lekko. If we operate off of a bootstrap
                // we can continue to function off of that information. Unfortunately, we won't get updates. More
                // work will have to be done here to recover on a loop. For now, we return an error such that we expect
                // the SDK or client to retry the registration.
                return Err(tonic::Status::resource_exhausted(format!(
                    "error when registering with lekko {err:?}",
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
