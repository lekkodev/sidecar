use clap::Parser;
use hyper_rustls::HttpsConnectorBuilder;
use sidecar::gen::mod_cli::lekko::backend::v1beta1::distribution_service_client::DistributionServiceClient;
use sidecar::gen::mod_sdk::lekko::client::v1beta1::configuration_service_client::ConfigurationServiceClient;
use sidecar::gen::mod_sdk::lekko::client::v1beta1::configuration_service_server::ConfigurationServiceServer;
use sidecar::repofs::RepoFS;

use hyper::{http::Request, Body};
use log::log;
use sidecar::logging;
use sidecar::metrics::Metrics;
use sidecar::service::Service;
use sidecar::state::{StateMachine, StateStore};
use sidecar::store::ConfigStore;
use sidecar::types::Mode;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::signal::unix::SignalKind;
use tonic::codegen::CompressionEncoding;
use tonic::transport::{Server, Uri};
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnFailure, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level, Span};

// Struct containing all the cmd-line args we accept
#[derive(Parser, Debug)]
#[clap(author="Lekko", version="0.1.0", about, long_about = None)]
/// Lekko sidecar that provides the host application with config
/// updates from Lekko and performs local evaluation.
struct Args {
    #[arg(short, long, default_value_t=String::from("https://prod.api.lekko.dev"))]
    /// Address to communicate with lekko backend.
    lekko_addr: String,

    #[arg(short, long, default_value_t=String::from("0.0.0.0:50051"))]
    /// Address to bind to on current host.
    bind_addr: String,

    #[arg(value_enum, long, default_value_t, verbatim_doc_comment)]
    /// Mode can be one of:
    ///   default - initialize from a bootstrap, poll local state from remote and evaluate locally.
    ///   static - operate off of a config repo found on disk at repo_path.{n}
    mode: Mode,

    #[arg(short, long, value_parser=parse_duration, default_value="15s")]
    /// How often to poll for a new version of a configuration repository.
    /// If this duration is too short, Lekko may apply rate limits.
    poll_internal: Duration,

    #[arg(short, long)]
    /// Absolute path to the directory on disk that contains the .git folder.
    /// This flag is required for static mode, and optional for default mode
    /// if bootstrap behavior is required.
    repo_path: Option<String>,
}

fn parse_duration(arg: &str) -> Result<std::time::Duration, humantime::DurationError> {
    arg.parse::<humantime::Duration>().map(Into::into)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init();
    let args = Args::parse();
    let addr = match args.bind_addr.parse::<SocketAddr>() {
        Err(err) => panic!("parsing bind_addr {} failed: {err:?}", args.bind_addr),
        Ok(a) => a,
    };
    let lekko_addr = match args.lekko_addr.parse::<Uri>() {
        Err(err) => panic!("parsing lekko_addr {} failed: {err:?}", args.lekko_addr),
        Ok(a) => a,
    };
    log!(
        log::max_level().to_level().unwrap_or(log::Level::Warn),
        "binding server to: {:} with args: {:?}",
        addr,
        args
    );

    let http_client = hyper::Client::builder().build(
        HttpsConnectorBuilder::new()
            // TODO: look into in the future, if we should just embed our own TLS
            // cert here instead of packaging with webpki.
            .with_webpki_roots()
            .https_or_http()
            .enable_http2()
            .build(),
    );

    let repo_fs_opt = match &args.repo_path {
        None => {
            if matches!(args.mode, Mode::Static) {
                panic!("no bootstrap provided for sidecar configured to be static")
            }
            None
        }
        Some(rp) => {
            // Panic for invalid bootstrap irregardless of the mode.
            Some(RepoFS::new(rp.to_owned()).unwrap())
        }
    };
    // By default, send and accept GZip compression for both the client and the server.
    let config_client =
        ConfigurationServiceClient::with_origin(http_client.clone(), lekko_addr.clone())
            .send_compressed(CompressionEncoding::Gzip)
            .accept_compressed(CompressionEncoding::Gzip);
    let dist_client = DistributionServiceClient::with_origin(http_client, lekko_addr)
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);

    let state_store = StateStore::new(
        repo_fs_opt
            .as_ref()
            .map(|data| (data.repo_key().unwrap(), data.git_commit_sha().unwrap())),
        args.mode,
    );
    let config_store = ConfigStore::new(
        dist_client.clone(),
        repo_fs_opt.map(|repo_fs| repo_fs.load().unwrap()),
        state_store.clone(),
        args.poll_internal,
        args.mode,
        args.repo_path,
    );

    let metrics = Metrics::new(dist_client.clone());
    let service = ConfigurationServiceServer::new(Service {
        config_client,
        dist_client,
        config_store,
        state_store: state_store.clone(),
        mode: args.mode,
        metrics,
    })
    .send_compressed(CompressionEncoding::Gzip)
    .accept_compressed(CompressionEncoding::Gzip);

    Server::builder()
        .layer(
            TraceLayer::new_for_grpc()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(|request: &Request<Body>, _span: &Span| {
                    let method = logging::http_uri_to_method(request.uri().to_string());
                    info!("request {} {}", method.service, method.method);
                })
                .on_response(
                    |response: &hyper::http::Response<_>,
                     latency: std::time::Duration,
                     _span: &Span| {
                        let extra_text = logging::get_trace_string(response.extensions());
                        info!(
                            "response {} ms {}",
                            latency.as_millis(),
                            extra_text.unwrap_or_default(),
                        );
                    },
                )
                .on_failure(
                    DefaultOnFailure::new()
                        .level(Level::ERROR)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .add_service(service)
        .serve_with_shutdown(addr, async move {
            tokio::signal::unix::signal(SignalKind::terminate())
                .unwrap()
                .recv()
                .await;

            log!(
                log::max_level().to_level().unwrap_or(log::Level::Warn),
                "got sigterm, waiting for deregister before gracefully shutting down"
            );
            state_store.shutdown();
            state_store
                .receiver()
                .wait_for(|state| matches!(state, StateMachine::Shutdown))
                .await
                .unwrap();
            log!(
                log::max_level().to_level().unwrap_or(log::Level::Warn),
                "gracefully shutting down"
            );
            // shutdown metrics
        })
        .await?;

    Ok(())
}
