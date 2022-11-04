use hyper::client::HttpConnector;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use sidecar::gen::backend_beta::configuration_service_client::ConfigurationServiceClient;
use sidecar::gen::backend_beta::configuration_service_server::{
    ConfigurationService, ConfigurationServiceServer,
};
use sidecar::gen::backend_beta::{
    GetBoolValueRequest, GetBoolValueResponse, GetJsonValueRequest, GetJsonValueResponse,
    GetProtoValueRequest, GetProtoValueResponse,
};
use std::env;
use tonic::codegen::CompressionEncoding;
use tonic::metadata::MetadataMap;
use tonic::Request;
use tonic::{
    body::BoxBody,
    transport::{Server, Uri},
};

#[derive(Clone)]
pub struct Passthrough {
    client: ConfigurationServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
}

#[tonic::async_trait]
impl ConfigurationService for Passthrough {
    async fn get_bool_value(
        &self,
        request: Request<GetBoolValueRequest>,
    ) -> Result<tonic::Response<GetBoolValueResponse>, tonic::Status> {
        println!("Got a request for GetBoolValue, proxying {:?}", request);
        let mut proxy_req = Request::new(request.get_ref().clone());
        proxy_headers(&mut proxy_req, request.metadata());
        let resp = self.client.clone().get_bool_value(proxy_req).await;
        if resp.is_err() {
            println!("error in proxying {:?}", resp)
        }
        resp
    }
    async fn get_proto_value(
        &self,
        request: Request<GetProtoValueRequest>,
    ) -> Result<tonic::Response<GetProtoValueResponse>, tonic::Status> {
        println!("Got a request for GetProtoValue, proxying {:?}", request);
        let mut proxy_req = Request::new(request.get_ref().clone());
        proxy_headers(&mut proxy_req, request.metadata());
        let resp = self.client.clone().get_proto_value(proxy_req).await;
        if resp.is_err() {
            println!("error in proxying {:?}", resp)
        }
        resp
    }
    async fn get_json_value(
        &self,
        request: Request<GetJsonValueRequest>,
    ) -> Result<tonic::Response<GetJsonValueResponse>, tonic::Status> {
        println!("Got a request for GetJSONValue, proxying {:?}", request);
        let mut proxy_req = Request::new(request.get_ref().clone());
        proxy_headers(&mut proxy_req, request.metadata());
        let resp = self.client.clone().get_json_value(proxy_req).await;
        if resp.is_err() {
            println!("error in proxying {:?}", resp)
        }
        resp
    }
}

// Sets the headers that we wish to forward to lekko. The apikey header is copied over as
// the server needs it to authenticate the caller.
fn proxy_headers<T>(proxy_request: &mut Request<T>, incoming_headers: &MetadataMap) {
    if let Some(apikey) = incoming_headers.get("apikey") {
        proxy_request
            .metadata_mut()
            .append("apikey", apikey.to_owned());
    }
}

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
    let proxy_addr = env::var("LEKKO_PROXY_ADDR")
        .unwrap_or_else(|_| "https://grpc.lekko.dev".to_owned())
        .parse::<Uri>()?;

    println!("proxying to: {}", proxy_addr);

    let client = hyper::Client::builder().build(
        HttpsConnectorBuilder::new()
            // TODO: look into in the future, if we should just embed our own TLS
            // cert here instead of packaging with webpki.
            .with_webpki_roots()
            .https_or_http()
            .enable_http2()
            .build(),
    );

    // By default, send and accept GZip compression for both the client and the server.
    let client = ConfigurationServiceClient::with_origin(client, proxy_addr)
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);
    let passthrough = ConfigurationServiceServer::new(Passthrough { client })
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);

    Server::builder()
        .add_service(passthrough)
        .serve(addr)
        .await?;

    Ok(())
}
