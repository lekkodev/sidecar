use std::{
    collections::HashMap,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    thread::spawn,
};

use futures::executor::block_on;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use itertools::Itertools;
use tonic::{
    body::BoxBody,
    metadata::{Ascii, MetadataValue},
    Request,
};

use crate::{
    gen::lekko::backend::v1beta1::{
        distribution_service_client::DistributionServiceClient,
        value::Kind::{BoolValue, DoubleValue, IntValue, StringValue},
        ContextKey, FlagEvaluationEvent, RepositoryKey, SendFlagEvaluationMetricsRequest, Value,
    },
    store::FeatureData,
    types::APIKEY,
};

// Component responsible for receiving evaluation metrics as they come in
// and delivering them to lekko backend.
pub struct Metrics {
    tx: SyncSender<TrackFlagEvaluationEvent>,
}

#[derive(Debug)]
pub struct TrackFlagEvaluationEvent {
    apikey: MetadataValue<Ascii>, // TODO: receive apikey during registration and store it.
    event: FlagEvaluationEvent,
}

impl Metrics {
    pub fn new(
        dist_client: DistributionServiceClient<
            hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
        >,
    ) -> Self {
        // create a sync channel to send and receive metrics.
        let (tx, rx) = sync_channel(3);
        // spawn a new thread that receives metrics and sends them over rpc
        spawn(move || {
            block_on(Metrics::worker(rx, dist_client));
        });
        Self { tx }
    }

    // Sends a flag evaluation event to an async thread for delivery to lekko backend.
    // This method is non-blocking.
    pub fn track_flag_evaluation(
        &self,
        rk: &RepositoryKey,
        namespace_name: &str,
        feature_data: &FeatureData,
        context: &HashMap<String, Value>,
        result_path: &[usize],
        apikey: &MetadataValue<Ascii>,
    ) {
        let event = FlagEvaluationEvent {
            repo_key: Some(rk.clone()),
            commit_sha: feature_data.commit_sha.clone(),
            feature_sha: feature_data.feature_sha.clone(),
            namespace_name: namespace_name.to_owned(),
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
        let _r = self.tx.clone().send(TrackFlagEvaluationEvent {
            apikey: apikey.clone(),
            event,
        });
    }

    async fn worker(
        rx: Receiver<TrackFlagEvaluationEvent>,
        dist_client: DistributionServiceClient<
            hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
        >,
    ) {
        for received in rx {
            let resp = Metrics::send_flag_evaluation(dist_client.clone(), received).await;
            match resp {
                Ok(_) => println!("successfully sent flag evaluation event"),
                Err(e) => println!("failed sending flag evaluation event {:?}", e.message()),
            }
        }
    }

    async fn send_flag_evaluation(
        dist_client: DistributionServiceClient<
            hyper::Client<HttpsConnector<HttpConnector>, BoxBody>,
        >,
        event: TrackFlagEvaluationEvent,
    ) -> Result<(), tonic::Status> {
        let mut req = Request::new(SendFlagEvaluationMetricsRequest {
            events: vec![event.event],
        });
        req.metadata_mut().append(APIKEY, event.apikey);
        let _resp = dist_client
            .clone()
            .send_flag_evaluation_metrics(req)
            .await?;
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
