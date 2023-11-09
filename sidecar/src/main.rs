use clap::Parser;
use hyper_rustls::HttpsConnectorBuilder;
use metrics::counter;
use sidecar::gen::cli::lekko::backend::v1beta1::distribution_service_client::DistributionServiceClient;
use sidecar::gen::cli::lekko::backend::v1beta1::distribution_service_server::DistributionServiceServer;
use sidecar::gen::cli::lekko::backend::v1beta1::GetRepositoryContentsRequest;
use sidecar::gen::cli::lekko::backend::v1beta1::RegisterClientRequest;
use sidecar::gen::cli::lekko::backend::v1beta1::RepositoryKey;
use sidecar::gen::sdk::lekko::client::v1beta1::configuration_service_server::ConfigurationServiceServer;
use sidecar::repofs::RepoFS;

use hyper::{http::Request, Body};
use log::log;
use sidecar::config_service;
use sidecar::distro_service;
use sidecar::logging;
use sidecar::metrics::Metrics;
use sidecar::metrics::RuntimeMetrics;
use sidecar::store::Store;
use sidecar::types::{add_api_key, ConnectionCredentials, Mode};
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal::unix::SignalKind;
use tokio::time::sleep;
use tonic::codegen::CompressionEncoding;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::transport::{Server, Uri};
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnFailure, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level, Span};

// Struct containing all the cmd-line args we accept
#[derive(Parser)]
#[clap(author="Lekko", version="0.0.12", about, long_about = None)]
/// Lekko sidecar that provides the host application with config
/// updates from Lekko and performs local evaluation.
struct Args {
    #[arg(short, long, default_value_t=String::from("https://prod.api.lekko.dev"))]
    /// Address to communicate with lekko backend.
    lekko_addr: String,

    #[arg(long, default_value_t=String::from("0.0.0.0:50051"))]
    /// Address to bind to on current host.
    bind_addr: String,

    #[arg(short, long)]
    /// API Key to connect to Lekko backend. Required for default mode.
    /// If provided in static mode, metrics will be sent to Lekko.
    api_key: Option<MetadataValue<Ascii>>,

    #[arg(long, default_value_t=String::from("0.0.0.0:9000"))]
    /// Address to bind to on current host.
    metrics_bind_addr: String,

    #[arg(value_enum, long, default_value_t, verbatim_doc_comment)]
    /// Mode can be one of:
    ///   default - initialize from a bootstrap, poll local state from remote and evaluate locally.
    ///   consistent - always evaluate using the latest value of a flag from remote.
    ///   static - operate off of a config repo found on disk at repo_path.{n}
    mode: Mode,

    #[arg(short, long, value_parser=parse_duration, default_value="15s")]
    /// How often to poll for a new version of a configuration repository.
    /// If this duration is too short, Lekko may apply rate limits.
    poll_interval: Duration,

    #[arg(short, long, default_value = "")]
    /// Absolute path to the directory on disk that contains the .git folder.
    /// This is required for static mode.
    repo_path: String,

    #[arg(short = 'u', long, default_value = "")]
    /// The url for the repo in "owner_name/repo_name" format, such as:
    /// lekkodev/example, representing github.com/lekkodev/example. This is required for default backend.
    repo_url: String,
}

impl Debug for Args {
    // We manually implement Debug in order to avoid printing the api key.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{ lekko_addr: {}, bind_addr: {} api_key: {:?}, metrics_bind_addr: {}, mode: {:?}, poll_interval: {:?}, repo_path: {}, repo_url: {} }}", self.lekko_addr, self.bind_addr, self.api_key.as_ref().map(|_| "Some(<lekko api key>)"), self.metrics_bind_addr, self.mode, self.poll_interval, self.repo_path, self.repo_url))
    }
}

fn parse_duration(arg: &str) -> Result<std::time::Duration, humantime::DurationError> {
    arg.parse::<humantime::Duration>().map(Into::into)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init();

    const VERSION: &str = env!("SIDECAR_VERSION");
    const GIT_COMMIT: &str = env!("SIDECAR_GIT_COMMIT");
    let sidecar_version: String = format!("{VERSION}_{GIT_COMMIT}");

    log!(
        log::max_level().to_level().unwrap_or(log::Level::Info),
        "sidecar version '{}', commit '{}'",
        VERSION,
        GIT_COMMIT
    );

    let args = Args::parse();
    let addr = match args.bind_addr.parse::<SocketAddr>() {
        Err(err) => panic!("parsing bind_addr {} failed: {err:?}", args.bind_addr),
        Ok(a) => a,
    };
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

    log!(
        log::max_level().to_level().unwrap_or(log::Level::Warn),
        "binding server to: {:} with args: {:?}",
        addr,
        args
    );

    // For some reason, this sends http1 requests, which causes it to fail when chaining sidecars
    let connection_builder = HttpsConnectorBuilder::new()
        // TODO: look into in the future, if we should just embed our own TLS
        // cert here instead of packaging with webpki.
        .with_webpki_roots()
        .https_or_http()
        .enable_http2()
        .build();

    let http_client = hyper::Client::builder().build(connection_builder);

    // By default, send and accept GZip compression for both the client and the server.
    let dist_client = DistributionServiceClient::with_origin(http_client, lekko_addr)
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);

    let (bootstrap_data, rk) = match &args.mode {
        Mode::Static => {
            if args.repo_path.is_empty() {
                panic!("repo-path needs to be set in static mode")
            }
            let bootstrap_data = RepoFS::new(args.repo_path.clone()).expect("invalid repository");
            let repo_key = bootstrap_data
                .repo_key()
                .expect("invalid remote information in repo path");
            let bootstrap = bootstrap_data.load().expect("error loading info");
            (bootstrap, repo_key)
        }
        Mode::Default => {
            if args.repo_url.is_empty() {
                panic!("repo-url needs to be set in default mode")
            }
            let api_key = args
                .api_key
                .as_ref()
                .expect("no api key provided in default mode");
            let (owner, repo) = args.repo_url.split_once('/').unwrap_or_else(|| {
                panic!(
                    "invalid repo-url: {}, please use the format owner/repo i.e. lekkodev/example",
                    args.repo_url
                )
            });

            let repo_key = RepositoryKey {
                owner_name: owner.to_owned(),
                repo_name: repo.to_owned(),
            };

            let bootstrap = dist_client
                .clone()
                .get_repository_contents(add_api_key(
                    GetRepositoryContentsRequest {
                        repo_key: Some(repo_key.clone()),
                        feature_name: "".to_string(),
                        namespace_name: "".to_string(),
                        session_key: "".to_string(),
                    },
                    api_key.clone(),
                ))
                .await
                .unwrap_or_else(|e| panic!("error performing initial fetch: {:?}", e))
                .into_inner();
            (bootstrap, repo_key)
        }
    };

    let api_key = args.api_key.as_ref();

    let conn_creds = match api_key {
        Some(key) => {
            let res = dist_client
                .clone()
                .register_client(add_api_key(
                    RegisterClientRequest {
                        repo_key: Some(rk.clone()),
                        initial_bootstrap_sha: bootstrap_data.commit_sha.clone(),
                        sidecar_version: sidecar_version.clone(),
                        namespace_list: vec![],
                    },
                    key.clone(),
                ))
                .await
                .map(|resp| ConnectionCredentials {
                    session_key: resp.into_inner().session_key,
                    repo_key: rk.clone(),
                    api_key: key.clone(),
                });
            match (res, &args.mode) {
                (Ok(conn), _) => Some(conn),
                // don't panic in static mode, sidecar can work without remote
                (Err(_), Mode::Static) => Some(ConnectionCredentials {
                    session_key: "".to_string(),
                    repo_key: rk.clone(),
                    api_key: key.clone(),
                }),
                (Err(err), _) => {
                    panic!("error connecting to remote: {:?}", err);
                }
            }
        }
        None => None,
    };

    let store = Arc::new(Store::new(
        dist_client.clone(),
        bootstrap_data,
        conn_creds.clone(),
        args.poll_interval,
        args.mode.to_owned(),
        args.repo_path,
        None,
    ));

    let session_key_opt = conn_creds.as_ref().map(|c| c.session_key.clone());
    let config_service: ConfigurationServiceServer<config_service::Service> =
        ConfigurationServiceServer::new(config_service::Service {
            store: store.clone(),
            mode: args.mode,
            metrics: api_key.map(|k| Metrics::new(dist_client.clone(), k.clone(), session_key_opt)),
            repo_key: rk.clone(),
        })
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);

    let distro_service: DistributionServiceServer<distro_service::Service> =
        DistributionServiceServer::new(distro_service::Service {
            distro_client: dist_client,
            store,
            repo_key: rk,
            conn_creds,
            sidecar_version,
        })
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<ConfigurationServiceServer<config_service::Service>>()
        .await;
    health_reporter
        .set_serving::<DistributionServiceServer<distro_service::Service>>()
        .await;

    Server::builder()
        .layer(
            TraceLayer::new_for_grpc()
                .make_span_with(DefaultMakeSpan::new())
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
                            "response {} μs {}",
                            latency.as_micros(),
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
        .add_service(distro_service)
        .add_service(config_service)
        .add_service(health_service)
        .serve_with_shutdown(addr, async move {
            tokio::signal::unix::signal(SignalKind::terminate())
                .unwrap()
                .recv()
                .await;
            // wait on signal from deregister
            log!(
                log::max_level().to_level().unwrap_or(log::Level::Warn),
                "got sigterm, waiting for shutdown duration before gracefully shutting down"
            );
            // TODO make configurable.
            sleep(Duration::from_secs(5)).await;
            log!(
                log::max_level().to_level().unwrap_or(log::Level::Warn),
                "got deregister, gracefully shutting down"
            );
            // shutdown metrics
        })
        .await?;

    Ok(())
}
