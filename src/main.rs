use clap::Parser;
use hyper_rustls::HttpsConnectorBuilder;
use sidecar::bootstrap::Bootstrap;
use sidecar::gen::lekko::backend::v1beta1::configuration_service_client::ConfigurationServiceClient;
use sidecar::gen::lekko::backend::v1beta1::configuration_service_server::ConfigurationServiceServer;
use sidecar::gen::lekko::backend::v1beta1::distribution_service_client::DistributionServiceClient;

use sidecar::metrics::Metrics;
use sidecar::service::{Mode, Service};
use sidecar::store::Store;
use std::net::SocketAddr;
use tonic::codegen::CompressionEncoding;
use tonic::transport::{Server, Uri};

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

    #[arg(value_enum, long, default_value_t)]
    /// Mode can be one of:
    /// default - initialize from a bootstrap, poll local state from remote and evaluate locally.
    /// consistent - always evaluate using the latest value of a flag from remote.
    /// static - operate only off of a bootstrap.
    mode: Mode,

    #[arg(short, long)]
    /// Absolute path to the directory on disk that contains the .git folder.
    /// Provide this flag to turn on bootstrap behavior.
    repo_path: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("args: {:?}", args);
    let addr = match args.bind_addr.parse::<SocketAddr>() {
        Err(err) => panic!("parsing bind_addr failed: {err:?}"),
        Ok(a) => a,
    };
    let lekko_addr = match args.lekko_addr.parse::<Uri>() {
        Err(err) => panic!("parsing lekko_addr failed: {err:?}"),
        Ok(a) => a,
    };
    println!("listening on port: {:?}", addr.to_owned());

    let http_client = hyper::Client::builder().build(
        HttpsConnectorBuilder::new()
            // TODO: look into in the future, if we should just embed our own TLS
            // cert here instead of packaging with webpki.
            .with_webpki_roots()
            .https_or_http()
            .enable_http2()
            .build(),
    );
    let bootstrap_data = match args.repo_path {
        None => {
            if matches!(args.mode, Mode::Static) {
                panic!("no bootstrap provided for sidecar configured to be static")
            }
            None
        }
        Some(rp) => {
            let mut bootstrap = Bootstrap::new(rp);
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
    let store = Store::new(dist_client.clone(), bootstrap_data);
    let metrics = Metrics::new(dist_client);
    let service = ConfigurationServiceServer::new(Service {
        config_client,
        store,
        mode: args.mode,
        metrics,
    })
    .send_compressed(CompressionEncoding::Gzip)
    .accept_compressed(CompressionEncoding::Gzip);

    Server::builder().add_service(service).serve(addr).await?;

    Ok(())
}
