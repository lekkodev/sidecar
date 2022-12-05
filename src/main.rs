use hyper_rustls::HttpsConnectorBuilder;
use sidecar::gen::lekko::backend::v1beta1::configuration_service_client::ConfigurationServiceClient;
use sidecar::gen::lekko::backend::v1beta1::configuration_service_server::ConfigurationServiceServer;
use sidecar::gen::lekko::backend::v1beta1::distribution_service_client::DistributionServiceClient;

use sidecar::metrics::Metrics;
use sidecar::service::Service;
use sidecar::store::Store;
use std::env;
use tonic::codegen::CompressionEncoding;
use tonic::transport::{Server, Uri};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = match env::var("LEKKO_BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:50051".to_owned())
        .parse()
    {
        Err(err) => panic!("parsing address failed: {:?}", err),
        Ok(addr) => addr,
    };
    println!("listening on port: {:?}", addr);
    let lekko_addr = env::var("LEKKO_PROXY_ADDR")
        .unwrap_or_else(|_| "https://grpc.lekko.dev".to_owned())
        .parse::<Uri>()?;

    // Setting proxy_mode to false will ensure that we perform rules evaluation locally in the sidecar.
    let proxy_mode = false;
    println!("lekko address: {}\nProxy mode: {}", lekko_addr, proxy_mode);

    let http_client = hyper::Client::builder().build(
        HttpsConnectorBuilder::new()
            // TODO: look into in the future, if we should just embed our own TLS
            // cert here instead of packaging with webpki.
            .with_webpki_roots()
            .https_or_http()
            .enable_http2()
            .build(),
    );

    // By default, send and accept GZip compression for both the client and the server.
    let config_client =
        ConfigurationServiceClient::with_origin(http_client.clone(), lekko_addr.clone())
            .send_compressed(CompressionEncoding::Gzip)
            .accept_compressed(CompressionEncoding::Gzip);
    let dist_client = DistributionServiceClient::with_origin(http_client, lekko_addr)
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);
    let store = Store::new(dist_client.clone());
    let metrics = Metrics::new(dist_client);
    let service = ConfigurationServiceServer::new(Service {
        config_client,
        store,
        proxy_mode,
        metrics,
    })
    .send_compressed(CompressionEncoding::Gzip)
    .accept_compressed(CompressionEncoding::Gzip);

    Server::builder().add_service(service).serve(addr).await?;

    Ok(())
}
