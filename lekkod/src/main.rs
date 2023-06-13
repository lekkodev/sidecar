use clap::Parser;
use hyper_rustls::HttpsConnectorBuilder;
use metrics::counter;
use sidecar::gen::mod_cli::lekko::backend::v1beta1::GetRepositoryContentsRequest;
use sidecar::gen::mod_cli::lekko::backend::v1beta1::RepositoryKey;
use sidecar::gen::mod_cli::lekko::backend::v1beta1::distribution_service_client::DistributionServiceClient;

use log::{error, log};
use sidecar::logging;
use sidecar::metrics::Metrics;
use sidecar::metrics::RuntimeMetrics;
use std::fmt::Debug;
use std::net::SocketAddr;
use std::time::Duration;
use tonic::codegen::CompressionEncoding;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::transport::Uri;

// Struct containing all the cmd-line args we accept
#[derive(Parser)]
#[clap(author="Lekko", version="0.0.12", about, long_about = None)]
/// Lekko sidecar that provides the host application with config
/// updates from Lekko and performs local evaluation.
struct Args {
    #[arg(short, long, default_value_t=String::from("https://prod.api.lekko.dev"))]
    /// Address to communicate with lekko backend.
    lekko_addr: String,

    #[arg(short, long)]
    /// API Key to connect to Lekko backend.
    api_key: MetadataValue<Ascii>,

    #[arg(long, default_value_t=String::from("0.0.0.0:9000"))]
    /// Address to bind to on current host.
    metrics_bind_addr: String,

    #[arg(short, long, value_parser=parse_duration, default_value="15s")]
    /// How often to poll for a new version of a configuration repository.
    /// If unset, the binary will exit, functioning as an init container.
    poll_interval: Option<Duration>,

    #[arg(short, long)]
    /// Absolute path to write to on desk. Application must have read permission.
    repo_path: String,
}

impl Debug for Args {
    // We manually implement Debug in order to avoid printing the api key.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{ lekko_addr: {}, api_key: {:?}, metrics_bind_addr: {}, poll_interval: {:?}, repo_path: {} }}", self.lekko_addr, "<lekko api key>", self.metrics_bind_addr, self.poll_interval, self.repo_path))
    }
}

fn parse_duration(arg: &str) -> Result<std::time::Duration, humantime::DurationError> {
    arg.parse::<humantime::Duration>().map(Into::into)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init();

    let args = Args::parse();

    let lekko_addr = match args.lekko_addr.parse::<Uri>() {
        Err(err) => panic!("parsing lekko_addr {} failed: {err:?}", args.lekko_addr),
        Ok(a) => a,
    };

    let metrics_bind_addr = match args.metrics_bind_addr.parse::<std::net::SocketAddr>() {
        Err(err) => panic!(
            "parsing metrics_bind_addr {} failed: {err:?}",
            args.metrics_bind_addr
        ),
        Ok(a) => a,
    };

    let runtime_metrics = RuntimeMetrics::new(metrics_bind_addr);
    counter!(runtime_metrics.startup_counter, 1);

    let http_client = hyper::Client::builder().build(
        HttpsConnectorBuilder::new()
            // TODO: look into in the future, if we should just embed our own TLS
            // cert here instead of packaging with webpki.
            .with_webpki_roots()
            .https_or_http()
            .enable_http2()
            .build(),
    );

    let dist_client = DistributionServiceClient::with_origin(http_client, lekko_addr)
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);
    let res = dist_client
        .clone()
        .get_repository_contents(GetRepositoryContentsRequest{
            repo_key: Some(RepositoryKey{
                owner_name: "".to_string(),
                repo_name: "".to_string(),
            }),
            feature_name: "".to_string(),
            namespace_name: "".to_string(),
            session_key: "".to_string(),
        })
        .await;
    // TODO write to disk.
    Ok(())
}
