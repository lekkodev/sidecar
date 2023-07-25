use std::{
    collections::HashMap,
    net::SocketAddr,
    time::{Duration, SystemTime},
};

use futures::{stream::FuturesUnordered, StreamExt};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use itertools::Itertools;
use log::{debug, error, warn};
use metrics_exporter_prometheus::PrometheusBuilder;
use prost_types::Timestamp;
use tokio::{
    select, spawn,
    sync::mpsc::{channel, Receiver, Sender},
};
use tonic::{
    body::BoxBody,
    metadata::{Ascii, MetadataValue},
    Request,
};

use crate::{
    gen::cli::lekko::backend::v1beta1::{
        distribution_service_client::DistributionServiceClient, ContextKey, FlagEvaluationEvent,
        SendFlagEvaluationMetricsRequest,
    },
    gen::sdk::lekko::client::v1beta1::{
        value::Kind::{BoolValue, DoubleValue, IntValue, StringValue},
        Value,
    },
    store::FeatureData,
    types::{FeatureRequestParams, APIKEY},
};

// Component responsible for receiving evaluation metrics as they come in
// and delivering them to lekko backend.
pub struct Metrics {
    tx: Sender<TrackFlagEvaluationEvent>,
}

#[derive(Debug)]
pub struct TrackFlagEvaluationEvent {
    event: FlagEvaluationEvent,
}

impl Metrics {
    pub fn new(
        dist_client: DistributionServiceClient<
            hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
        >,
        api_key: MetadataValue<Ascii>,
        session_key: Option<String>,
    ) -> Self {
        // create a sync channel to send and receive metrics.
        // approximate size of each event is 1KB. this buffer is sized to not exceed 1MB total memory.
        // This can be modified as necessary.
        let (tx, rx) = channel(1024);
        // spawn a new thread that receives metrics and sends them over rpc
        spawn(async {
            Metrics::worker(rx, dist_client, api_key, session_key).await;
        });
        Self { tx }
    }

    // Sends a flag evaluation event to an async thread for delivery to lekko backend.
    // This method is non-blocking.
    pub fn track_flag_evaluation(
        &self,
        feature_params: &FeatureRequestParams,
        feature_data: &FeatureData,
        context: &HashMap<String, Value>,
        result_path: &[usize],
    ) {
        let event = FlagEvaluationEvent {
            client_event_time: Some(Timestamp::from(SystemTime::now())),
            repo_key: Some(feature_params.rk.clone()),
            commit_sha: feature_data.commit_sha.clone(),
            feature_sha: feature_data.feature_sha.clone(),
            namespace_name: feature_params.namespace.to_owned(),
            feature_name: feature_data.feature.key.clone(),
            context_keys: context
                .iter()
                .map(|(k, v)| ContextKey {
                    key: k.clone(),
                    r#type: Metrics::value_to_type(v),
                })
                .collect_vec(),
            result_path: result_path.iter().map(|e| *e as i32).collect_vec(),
        };
        // Try to send the event over the channel. This can fail if (a) the buffer is full, or (b) the
        // receiver has dropped or been closed. In either case, we drop the metric and print the error.
        // try_send is non-blocking.
        let result = self.tx.try_send(TrackFlagEvaluationEvent { event });
        if let Err(e) = result {
            warn!("failed to send metrics to internal metrics handler {e:?}");
        }
    }

    async fn worker(
        mut rx: Receiver<TrackFlagEvaluationEvent>,
        dist_client: DistributionServiceClient<
            hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
        >,
        api_key: MetadataValue<Ascii>,
        session_key: Option<String>,
    ) {
        // Pool of futures allows this thread to not block on I/O, sending out multiple
        // metrics at once while also receiving from the channel.
        let mut futures = FuturesUnordered::new();

        // preallocate memory to avoid reallocation when receiving messages.
        let mut buffer = Vec::with_capacity(1024);

        let mut interval = tokio::time::interval(Duration::from_secs(10));

        loop {
            select! {
                _ = interval.tick() => {
                    if !buffer.is_empty() {
                        futures.push(Metrics::send_flag_evaluations(dist_client.clone(), buffer.drain(..).collect(), api_key.clone(), session_key.clone()));
                    }
                },
                // recv returns None if the channel is closed or the sender goes out of scope. We
                // don't expect this to happen.
                Some(event) = rx.recv() => {
                    buffer.push(event);
                    if buffer.len() >= 1024 {
                        futures.push(Metrics::send_flag_evaluations(dist_client.clone(), buffer.drain(..).collect(), api_key.clone(), session_key.clone()));
                    }
                },
                Some(result) = futures.next() => {
                    if let Err(e) = result {
                        error!("error handling send flag evaluation future {e:?}");
                    }
                },
                else => break,
            }
        }
    }

    async fn send_flag_evaluations(
        mut dist_client: DistributionServiceClient<
            hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
        >,
        events: Vec<TrackFlagEvaluationEvent>,
        api_key: MetadataValue<Ascii>,
        session_key: Option<String>,
    ) -> Result<(), tonic::Status> {
        debug!("sending {} flag evaluation metrics to lekko", events.len());
        let mut req = Request::new(SendFlagEvaluationMetricsRequest {
            events: events.into_iter().map(|event| event.event).collect(),
            session_key: session_key.unwrap_or_default(),
        });
        req.metadata_mut().append(APIKEY, api_key);
        dist_client.send_flag_evaluation_metrics(req).await?;
        Ok(())
    }

    fn value_to_type(v: &Value) -> String {
        if let Some(kind) = &v.kind {
            match kind {
                BoolValue(_) => return String::from("bool"),
                IntValue(_) => return String::from("int"),
                DoubleValue(_) => return String::from("float"),
                StringValue(_) => return String::from("string"),
            }
        }
        String::from("unknown")
    }
}

// RuntimeMetrics initializes a prometheus scrape endpoint on the metrics_bind_addr and provides a set of runtime metrics for the sidecar app
pub struct RuntimeMetrics {
    pub startup_counter: String,
}

impl RuntimeMetrics {
    pub fn new(metrics_bind_addr: SocketAddr) -> Self {
        let builder = PrometheusBuilder::new();
        let builder = builder.with_http_listener(metrics_bind_addr);
        builder.install().unwrap();
        Self {
            startup_counter: "lekko_sidecar_startup_counter".to_string(),
        }
    }
}
