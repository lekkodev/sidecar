use clap::Parser;
use hyper_rustls::HttpsConnectorBuilder;
use sidecar::bootstrap::Bootstrap;
use sidecar::gen::mod_cli::lekko::backend::v1beta1::distribution_service_client::DistributionServiceClient;
use sidecar::gen::mod_sdk::lekko::client::v1beta1::configuration_service_client::ConfigurationServiceClient;
use sidecar::gen::mod_sdk::lekko::client::v1beta1::configuration_service_server::ConfigurationServiceServer;

use hyper::{http::Request, Body};
use log::{error, log};
use sidecar::logging;
use sidecar::metrics::Metrics;
use sidecar::service::{Mode, Service};
use sidecar::store::Store;
use std::net::SocketAddr;
use std::sync::Mutex;
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
    ///   consistent - always evaluate using the latest value of a flag from remote.
    ///   static - operate only off of a bootstrap.{n}
    mode: Mode,

    #[arg(short, long, value_parser=parse_duration, default_value="15s")]
    /// How often to poll for a new version of a configuration repository.
    /// If this duration is too short, Lekko may apply rate limits.
    poll_internal: Duration,

    #[arg(short, long)]
    /// Absolute path to the directory on disk that contains the .git folder.
    /// Provide this flag to turn on bootstrap behavior.
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

    let bootstrap_data = match &args.repo_path {
        None => {
            if matches!(args.mode, Mode::Static) {
                panic!("no bootstrap provided for sidecar configured to be static")
            }
            None
        }
        Some(rp) => {
            let mut bootstrap = Bootstrap::new(rp.to_owned());
            Some(
                bootstrap
                    .load()
                    .unwrap_or_else(|e| panic!("failed bootstrap load: {e:?}")),
            )
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

    // Have a oneshot for deregister to release shutdown in case of a SIGTERM. This behavior is intended
    // to keep the sidecar up longer than its client process.
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

    let store = Store::new(dist_client.clone(), bootstrap_data, args.poll_internal, args.mode.to_owned(), args.repo_path);
    let metrics = Metrics::new(dist_client);
    let service = ConfigurationServiceServer::new(Service {
        shutdown_tx: Mutex::new(Some(shutdown_tx)),
        config_client,
        store,
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
                            extra_text.unwrap_or("".to_string()),
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
            // wait on signal from deregister
            log!(
                log::max_level().to_level().unwrap_or(log::Level::Warn),
                "got sigterm, waiting for deregister before gracefully shutting down"
            );
            match shutdown_rx.await {
                Ok(()) => {}
                Err(error) => {
                    error!("error when shutting down: {error:?}")
                }
            };
            log!(
                log::max_level().to_level().unwrap_or(log::Level::Warn),
                "got deregister, gracefully shutting down"
            );
            // shutdown metrics
        })
        .await?;

    Ok(())
}
