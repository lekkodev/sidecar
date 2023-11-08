use clap::Parser;
use hyper::client::HttpConnector;
use hyper::Body;
use hyper_rustls::HttpsConnector;
use hyper_rustls::HttpsConnectorBuilder;
use log::log;
use metrics::counter;
use moka::future::Cache;
use prost::Message;
use prost_types::{value::Kind, Any};
use sidecar::evaluate::evaluator::evaluate;
use sidecar::evaluate::evaluator::EvalContext;
use sidecar::gen::cli::lekko::backend::v1beta1::distribution_service_client::DistributionServiceClient;
use sidecar::gen::cli::lekko::backend::v1beta1::GetRepositoryContentsRequest;
use sidecar::gen::cli::lekko::backend::v1beta1::GetRepositoryContentsResponse;
use sidecar::gen::cli::lekko::backend::v1beta1::Namespace;
use sidecar::gen::cli::lekko::backend::v1beta1::RegisterClientRequest;
use sidecar::gen::cli::lekko::backend::v1beta1::RepositoryKey;
use sidecar::gen::cli::lekko::feature::v1beta1::FeatureType;
use sidecar::gen::sdk::lekko::client::v1beta1::configuration_service_server::ConfigurationServiceServer;
use sidecar::gen::sdk::lekko::client::v1beta1::{
    configuration_service_server::ConfigurationService, Any as LekkoAny, DeregisterRequest,
    DeregisterResponse, GetBoolValueRequest, GetBoolValueResponse, GetFloatValueRequest,
    GetFloatValueResponse, GetIntValueRequest, GetIntValueResponse, GetJsonValueRequest,
    GetJsonValueResponse, GetProtoValueRequest, GetProtoValueResponse, GetStringValueRequest,
    GetStringValueResponse, RegisterRequest, RegisterResponse, Value,
};
use sidecar::logging;
use sidecar::logging::InsertLogFields;
use sidecar::metrics::RuntimeMetrics;
use sidecar::store::Store;
use sidecar::types;
use sidecar::types::add_api_key;
use sidecar::types::convert_repo_key;
use sidecar::types::ConnectionCredentials;
use sidecar::types::FeatureRequestParams;
use sidecar::types::Mode;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tonic::body::BoxBody;
use tonic::codegen::CompressionEncoding;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::transport::{Server, Uri};
use tonic::{metadata::MetadataMap, Request, Response, Status};
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnFailure, TraceLayer},
    LatencyUnit,
};
use tracing::{info, Level, Span};

// Struct containing all the cmd-line args we accept
#[derive(Parser)]
#[clap(author="Lekko", version="0.0.1", about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t=String::from("https://prod.api.lekko.dev"))]
    /// Address to communicate with lekko backend.
    lekko_addr: String,

    #[arg(long, default_value_t=String::from("0.0.0.0:9000"))]
    /// Address to bind to on current host.
    metrics_bind_addr: String,

    #[arg(short, long, value_parser=parse_duration, default_value="15s")]
    /// How often to poll for a new version of a configuration repository.
    /// If unset, the binary will exit, functioning as an init container.
    poll_interval: Option<Duration>,
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

    let cache: Cache<StoreKey, Arc<Store>> = Cache::new(10_000);

    let proxy_config_service = ConfigurationServiceServer::new(ProxyConfigurationService {
        cache: cache,
        dist_client: dist_client,
    })
    .send_compressed(CompressionEncoding::Gzip)
    .accept_compressed(CompressionEncoding::Gzip);

    Server::builder()
        .add_service(proxy_config_service)
        .serve("[::1]:56651".parse().unwrap())
        .await?;

    Ok(())
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct StoreKey {
    api_key: MetadataValue<Ascii>,
    owner_name: Box<String>,
    repo_name: Box<String>,
}

pub struct ProxyConfigurationService {
    cache: Cache<StoreKey, Arc<Store>>,
    dist_client: DistributionServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
}

impl ProxyConfigurationService {
    async fn make_store(&self, key: StoreKey) -> Result<Arc<Store>, tonic::Status> {
        // TODO don't panic here, I think that will muck up things
        let repo_key = RepositoryKey {
            owner_name: key.owner_name.clone().to_string().to_owned(),
            repo_name: key.repo_name.clone().to_string().to_owned(),
        };

        let request = GetRepositoryContentsRequest {
            repo_key: Some(repo_key.clone()),
            feature_name: "".to_string(),
            namespace_name: "".to_string(),
            session_key: "".to_string(),
        };
        let bootstrap_data = self
            .dist_client
            .clone()
            .get_repository_contents(add_api_key(request, key.api_key.clone()))
            .await
            .unwrap_or_else(|e| panic!("error performing initial fetch: {:?}", e))
            .into_inner();

        let conn_creds = {
            let res = self
                .dist_client
                .clone()
                .register_client(add_api_key(
                    RegisterClientRequest {
                        repo_key: Some(repo_key.clone()),
                        initial_bootstrap_sha: bootstrap_data.commit_sha.clone(),
                        sidecar_version: "0.0".to_string(),
                        namespace_list: vec![],
                    },
                    key.api_key.clone(),
                ))
                .await
                .map(|resp| ConnectionCredentials {
                    session_key: resp.into_inner().session_key,
                    repo_key: repo_key.clone(),
                    api_key: key.api_key.clone(),
                });
            match res {
                Ok(conn) => Some(conn),
                Err(err) => {
                    panic!("error connecting to remote: {:?}", err);
                }
            }
        };

        let store = Arc::new(Store::new(
            self.dist_client.clone(),
            bootstrap_data,
            conn_creds.clone(),
            Duration::new(15, 0),
            Mode::Default,
            format!("{:?}/{:?}", key.owner_name, key.repo_name),
        ));
        Ok(store)
    }
}

#[tonic::async_trait]
impl ConfigurationService for ProxyConfigurationService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<tonic::Response<RegisterResponse>, tonic::Status> {
        Ok(Response::new(RegisterResponse::default()))
    }

    async fn deregister(
        &self,
        _request: Request<DeregisterRequest>,
    ) -> Result<tonic::Response<DeregisterResponse>, tonic::Status> {
        Ok(Response::new(DeregisterResponse::default()))
    }

    async fn get_bool_value(
        &self,
        request: Request<GetBoolValueRequest>,
    ) -> Result<tonic::Response<GetBoolValueResponse>, tonic::Status> {
        let requested_type = FeatureType::Bool;
        let api_key = request
            .metadata()
            .get("APIKEY")
            .ok_or_else(|| Status::invalid_argument("no api key provided"))?
            .clone();
        let inner = request.into_inner();
        let store_key = StoreKey {
            owner_name: Box::new(
                inner
                    .repo_key
                    .clone()
                    .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                    .owner_name,
            ),
            repo_name: Box::new(
                inner
                    .repo_key
                    .clone()
                    .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                    .repo_name,
            ),

            api_key: api_key,
        };
        let store = self
            .cache
            .try_get_with(store_key.clone(), self.make_store(store_key.clone()))
            .await
            .unwrap(); // TODO - not sure how error prop works in rust

        let context = &inner.context;
        let feature = FeatureRequestParams {
            rk: convert_repo_key(
                inner
                    .repo_key
                    .as_ref()
                    .ok_or_else(|| Status::invalid_argument("no repo key provided"))?,
            ),
            namespace: inner.namespace.clone(),
            feature: inner.key.clone(),
        };

        let feature_data = store
            .get_feature_local(feature.clone())
            .ok_or_else(|| Status::invalid_argument("feature not found"))?;
        if feature_data.feature.r#type() != FeatureType::Unspecified && // backwards compatibility
            feature_data.feature.r#type() != requested_type
        {
            return Err(tonic::Status::invalid_argument(format!(
                "type mismatch: requested feature is not of type {:?}",
                requested_type.as_str_name()
            )));
        }
        let eval_context = EvalContext {
            namespace: feature.namespace.to_owned(),
            feature_name: feature_data.feature.key.to_owned(),
        };
        let eval_result = evaluate(&feature_data.feature, context, &eval_context)?;
        let result = eval_result.0;

        Ok(Response::new(GetBoolValueResponse {
            value: types::from_any::<bool>(&result)
                .map_err(|e| tonic::Status::internal(e.to_string()))?,
        }))
    }

    async fn get_int_value(
        &self,
        request: Request<GetIntValueRequest>,
    ) -> Result<tonic::Response<GetIntValueResponse>, tonic::Status> {
        Ok(Response::new(GetIntValueResponse::default()))
    }

    async fn get_float_value(
        &self,
        request: Request<GetFloatValueRequest>,
    ) -> Result<tonic::Response<GetFloatValueResponse>, tonic::Status> {
        Ok(Response::new(GetFloatValueResponse::default()))
    }

    async fn get_string_value(
        &self,
        request: Request<GetStringValueRequest>,
    ) -> Result<tonic::Response<GetStringValueResponse>, tonic::Status> {
        Ok(Response::new(GetStringValueResponse::default()))
    }

    async fn get_proto_value(
        &self,
        request: Request<GetProtoValueRequest>,
    ) -> Result<tonic::Response<GetProtoValueResponse>, tonic::Status> {
        Ok(Response::new(GetProtoValueResponse::default()))
    }

    async fn get_json_value(
        &self,
        request: Request<GetJsonValueRequest>,
    ) -> Result<tonic::Response<GetJsonValueResponse>, tonic::Status> {
        Ok(Response::new(GetJsonValueResponse::default()))
    }
}
