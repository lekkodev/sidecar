use clap::Parser;
use hyper::client::HttpConnector;
use log::log;
use sidecar::gen::cli::lekko::backend::v1beta1::distribution_service_server::DistributionServiceServer;
use std::net::SocketAddr;
use tokio::signal::unix::SignalKind;
use tokio::time::sleep;
use tower_http::cors::AllowOrigin;
use tower_http::cors::CorsLayer;

use hyper_rustls::HttpsConnector;
use hyper_rustls::HttpsConnectorBuilder;

use metrics::counter;
use moka::future::Cache;

use prost_types::value::Kind;
use sidecar::evaluate::evaluator::evaluate;
use sidecar::evaluate::evaluator::EvalContext;
use sidecar::gen::cli::lekko::backend::v1beta1::distribution_service_client::DistributionServiceClient;

use sidecar::gen::cli::lekko::backend::v1beta1::{
    distribution_service_server::DistributionService, DeregisterClientRequest,
    DeregisterClientResponse, GetDeveloperAccessTokenRequest, GetDeveloperAccessTokenResponse,
    GetRepositoryContentsRequest, GetRepositoryContentsResponse, GetRepositoryVersionRequest,
    GetRepositoryVersionResponse, RegisterClientRequest, RegisterClientResponse, RepositoryKey,
    SendFlagEvaluationMetricsRequest, SendFlagEvaluationMetricsResponse,
};
use sidecar::gen::cli::lekko::feature::v1beta1::FeatureType;
use sidecar::gen::sdk::lekko::client::v1beta1::configuration_service_server::ConfigurationServiceServer;
use sidecar::gen::sdk::lekko::client::v1beta1::{
    configuration_service_server::ConfigurationService, Any as LekkoAny, DeregisterRequest,
    DeregisterResponse, GetBoolValueRequest, GetBoolValueResponse, GetFloatValueRequest,
    GetFloatValueResponse, GetIntValueRequest, GetIntValueResponse, GetJsonValueRequest,
    GetJsonValueResponse, GetProtoValueRequest, GetProtoValueResponse, GetStringValueRequest,
    GetStringValueResponse, RegisterRequest, RegisterResponse,
};
use sidecar::logging;
use sidecar::logging::InsertLogFields;
use sidecar::metrics::Metrics;
use sidecar::metrics::RuntimeMetrics;
use sidecar::store::Store;
use sidecar::types;
use sidecar::types::add_api_key;
use sidecar::types::convert_repo_key;
use sidecar::types::ConnectionCredentials;
use sidecar::types::FeatureRequestParams;
use sidecar::types::Mode;

use std::sync::Arc;
use std::time::Duration;
use tonic::body::BoxBody;
use tonic::codegen::CompressionEncoding;

use tonic::metadata::{Ascii, MetadataValue};
use tonic::transport::{Server, Uri};
use tonic::{Request, Response, Status};

use hyper::Body;
use tower_http::trace::DefaultMakeSpan;
use tower_http::trace::DefaultOnFailure;
use tower_http::trace::TraceLayer;
use tower_http::LatencyUnit;
use tracing::info;
use tracing::Level;
use tracing::Span;
// Struct containing all the cmd-line args we accept
#[derive(Parser)]
#[clap(author="Lekko", version="0.0.1", about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t=String::from("https://prod.api.lekko.dev"))]
    /// Address to communicate with lekko backend.
    lekko_addr: String,

    #[arg(long, default_value_t=String::from("0.0.0.0:50051"))]
    /// Address to bind to on current host.
    bind_addr: String,

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

/*macro_rules! get_result {
    ($request:ident, $requested_type: ident) => { {
        let api_key = $request
            .metadata()
            .get("APIKEY")
            .ok_or_else(|| Status::invalid_argument("no api key provided"))?
            .clone();
        let inner = $request.into_inner();
        let store_key = StoreKey {
            owner_name:
                inner
                    .repo_key
                    .clone()
                    .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                    .owner_name,
            repo_name: inner
                    .repo_key
                    .clone()
                    .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                    .repo_name,
            api_key,
        };
        let (store, metrics) = &*self
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
            feature_data.feature.r#type() != $requested_type
        {
            return Err(tonic::Status::invalid_argument(format!(
                "type mismatch: requested feature is not of type {:?}",
                $requested_type.as_str_name()
            )));
        }
        let eval_context = EvalContext {
            namespace: feature.namespace.to_owned(),
            feature_name: feature_data.feature.key.to_owned(),
        };
        let eval_result = evaluate(&feature_data.feature, context, &eval_context)?;
        let result = eval_result.0;
        metrics.track_flag_evaluation(&feature, &feature_data, context, &eval_result.1);
    Ok(result)
    }

    };
}*/

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

    let cache: Cache<StoreKey, Arc<(Store, Metrics)>> = Cache::new(10_000);

    let proxy_config_service = ConfigurationServiceServer::new(ProxyConfigurationService {
        cache: cache.clone(),
        dist_client: dist_client.clone(),
    })
    .send_compressed(CompressionEncoding::Gzip)
    .accept_compressed(CompressionEncoding::Gzip);
    let proxy_dist_service = DistributionServiceServer::new(ProxyDistributionService {
        cache: cache.clone(),
        dist_client: dist_client.clone(),
    })
    .send_compressed(CompressionEncoding::Gzip)
    .accept_compressed(CompressionEncoding::Gzip);

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<ConfigurationServiceServer<ProxyConfigurationService>>()
        .await;
    health_reporter
        .set_serving::<DistributionServiceServer<ProxyDistributionService>>()
        .await;

    Server::builder()
        .accept_http1(true)
        .layer(
            TraceLayer::new_for_grpc()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(|request: &hyper::http::Request<Body>, _span: &Span| {
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
        .layer(
            CorsLayer::new()
                .allow_headers(tower_http::cors::Any)
                .allow_origin(AllowOrigin::mirror_request()),
        )
        .add_service(tonic_web::enable(proxy_config_service))
        .add_service(tonic_web::enable(proxy_dist_service))
        .add_service(tonic_web::enable(health_service))
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
pub struct StoreKey {
    api_key: MetadataValue<Ascii>,
    owner_name: String,
    repo_name: String,
}

pub struct ProxyConfigurationService {
    cache: Cache<StoreKey, Arc<(Store, Metrics)>>,
    dist_client: DistributionServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
}

impl ProxyConfigurationService {
    async fn make_store(&self, key: StoreKey) -> Result<Arc<(Store, Metrics)>, tonic::Status> {
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
            .await?
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
                    panic!("error connecting to remote: {:?}", err); // TODO
                }
            }
        };

        let store = Arc::new((
            Store::new(
                self.dist_client.clone(),
                bootstrap_data,
                conn_creds.clone(),
                Duration::new(15, 0),
                Mode::Default,
                format!("{:?}/{:?}", key.owner_name, key.repo_name),
            ),
            Metrics::new(self.dist_client.clone(), key.api_key.clone(), None),
        ));
        Ok(store)
    }
}

#[tonic::async_trait]
impl ConfigurationService for ProxyConfigurationService {
    async fn register(
        &self,
        _request: Request<RegisterRequest>,
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
        //let result = get_result!(request, requested_type)?;
        let api_key = request
            .metadata()
            .get("APIKEY")
            .ok_or_else(|| Status::invalid_argument("no api key provided"))?
            .clone();
        let inner = request.into_inner();
        let store_key = StoreKey {
            owner_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .owner_name,
            repo_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .repo_name,
            api_key,
        };
        let (store, metrics) = &*self
            .cache
            .try_get_with(store_key.clone(), self.make_store(store_key.clone()))
            .await
            .map_err(|e| (*e).clone())?;

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
        metrics.track_flag_evaluation(&feature, &feature_data, context, &eval_result.1);
        Ok(Response::new(GetBoolValueResponse {
            value: types::from_any::<bool>(&result)
                .map_err(|e| tonic::Status::internal(e.to_string()))?,
        }))
    }

    async fn get_int_value(
        &self,
        request: Request<GetIntValueRequest>,
    ) -> Result<tonic::Response<GetIntValueResponse>, tonic::Status> {
        let requested_type = FeatureType::Int;
        let api_key = request
            .metadata()
            .get("APIKEY")
            .ok_or_else(|| Status::invalid_argument("no api key provided"))?
            .clone();
        let inner = request.into_inner();
        let store_key = StoreKey {
            owner_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .owner_name,
            repo_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .repo_name,
            api_key,
        };

        let (store, metrics) = &*self
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
        metrics.track_flag_evaluation(&feature, &feature_data, context, &eval_result.1);
        let value =
            types::from_any::<i64>(&result).map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(inner.insert_log_fields(Response::new(GetIntValueResponse { value })))
    }

    async fn get_float_value(
        &self,
        request: Request<GetFloatValueRequest>,
    ) -> Result<tonic::Response<GetFloatValueResponse>, tonic::Status> {
        let requested_type = FeatureType::Float;

        let api_key = request
            .metadata()
            .get("APIKEY")
            .ok_or_else(|| Status::invalid_argument("no api key provided"))?
            .clone();
        let inner = request.into_inner();
        let store_key = StoreKey {
            owner_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .owner_name,
            repo_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .repo_name,
            api_key,
        };

        let (store, metrics) = &*self
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
        metrics.track_flag_evaluation(&feature, &feature_data, context, &eval_result.1);

        let value =
            types::from_any::<f64>(&result).map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(inner.insert_log_fields(Response::new(GetFloatValueResponse { value })))
    }

    async fn get_string_value(
        &self,
        request: Request<GetStringValueRequest>,
    ) -> Result<tonic::Response<GetStringValueResponse>, tonic::Status> {
        let requested_type = FeatureType::String;
        let api_key = request
            .metadata()
            .get("APIKEY")
            .ok_or_else(|| Status::invalid_argument("no api key provided"))?
            .clone();
        let inner = request.into_inner();
        let store_key = StoreKey {
            owner_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .owner_name,
            repo_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .repo_name,
            api_key,
        };

        let (store, metrics) = &*self
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
        metrics.track_flag_evaluation(&feature, &feature_data, context, &eval_result.1);
        let value = types::from_any::<String>(&result)
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(inner.insert_log_fields(Response::new(GetStringValueResponse { value })))
    }

    async fn get_proto_value(
        &self,
        request: Request<GetProtoValueRequest>,
    ) -> Result<tonic::Response<GetProtoValueResponse>, tonic::Status> {
        let requested_type = FeatureType::Proto;
        let api_key = request
            .metadata()
            .get("APIKEY")
            .ok_or_else(|| Status::invalid_argument("no api key provided"))?
            .clone();
        let inner = request.into_inner();
        let store_key = StoreKey {
            owner_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .owner_name,
            repo_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .repo_name,
            api_key,
        };

        let (store, metrics) = &*self
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
        metrics.track_flag_evaluation(&feature, &feature_data, context, &eval_result.1);
        Ok(
            inner.insert_log_fields(Response::new(GetProtoValueResponse {
                value: Some(result.clone()),
                value_v2: Some(LekkoAny {
                    type_url: result.clone().type_url,
                    value: result.value,
                }),
            })),
        )
    }

    async fn get_json_value(
        &self,
        request: Request<GetJsonValueRequest>,
    ) -> Result<tonic::Response<GetJsonValueResponse>, tonic::Status> {
        let requested_type = FeatureType::Json;
        let api_key = request
            .metadata()
            .get("APIKEY")
            .ok_or_else(|| Status::invalid_argument("no api key provided"))?
            .clone();
        let inner = request.into_inner();
        let store_key = StoreKey {
            owner_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .owner_name,
            repo_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .repo_name,
            api_key,
        };

        let (store, metrics) = &*self
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
        metrics.track_flag_evaluation(&feature, &feature_data, context, &eval_result.1);
        let value = types::from_any::<prost_types::Value>(&result)
            .map_err(|e| tonic::Status::internal(e.to_string()))?;
        Ok(inner.insert_log_fields(Response::new(GetJsonValueResponse {
            value: serde_json::to_vec(&ValueWrapper(&value)).map_err(|e| {
                Status::internal("failure serializing json ".to_owned() + &e.to_string())
            })?,
        })))
    }
}

fn serialize_value<S>(value: &prost_types::Value, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ::serde::Serializer,
{
    match &value.kind {
        None | Some(Kind::NullValue(_)) => serializer.serialize_none(),
        Some(Kind::NumberValue(f)) => serializer.serialize_f64(*f),
        Some(Kind::StringValue(s)) => serializer.serialize_str(s),
        Some(Kind::BoolValue(b)) => serializer.serialize_bool(*b),
        Some(Kind::StructValue(st)) => serialize_struct(st, serializer),
        Some(Kind::ListValue(l)) => serialize_list(l, serializer),
    }
}

pub struct ValueWrapper<'a>(&'a prost_types::Value);

impl<'a> serde::Serialize for ValueWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serialize_value(self.0, serializer)
    }
}

fn serialize_struct<S>(st: &prost_types::Struct, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ::serde::Serializer,
{
    serializer.collect_map(st.fields.iter().map(|(k, v)| (k, ValueWrapper(v))))
}

fn serialize_list<S>(st: &prost_types::ListValue, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ::serde::Serializer,
{
    serializer.collect_seq(st.values.iter().map(ValueWrapper))
}

pub struct ProxyDistributionService {
    cache: Cache<StoreKey, Arc<(Store, Metrics)>>,
    dist_client: DistributionServiceClient<hyper::Client<HttpsConnector<HttpConnector>, BoxBody>>,
}
impl ProxyDistributionService {
    async fn make_store(&self, key: StoreKey) -> Result<Arc<(Store, Metrics)>, tonic::Status> {
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
            .unwrap_or_else(|e| panic!("error performing initial fetch: {:?}", e)) // TODO
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
                    panic!("error connecting to remote: {:?}", err); // TODO
                }
            }
        };

        let store = Arc::new((
            Store::new(
                self.dist_client.clone(),
                bootstrap_data,
                conn_creds.clone(),
                Duration::new(15, 0),
                Mode::Default,
                format!("{:?}/{:?}", key.owner_name, key.repo_name),
            ),
            Metrics::new(self.dist_client.clone(), key.api_key.clone(), None),
        ));
        Ok(store)
    }
}

#[tonic::async_trait]
impl DistributionService for ProxyDistributionService {
    async fn get_repository_version(
        &self,
        request: Request<GetRepositoryVersionRequest>,
    ) -> Result<tonic::Response<GetRepositoryVersionResponse>, tonic::Status> {
        let api_key = request
            .metadata()
            .get("APIKEY")
            .ok_or_else(|| Status::invalid_argument("no api key provided"))?
            .clone();
        let inner = request.into_inner();
        let store_key = StoreKey {
            owner_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .owner_name,
            repo_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .repo_name,
            api_key,
        };
        let (store, _) = &*self
            .cache
            .try_get_with(store_key.clone(), self.make_store(store_key.clone()))
            .await
            .unwrap(); // TODO - not sure how error prop works in rust

        return Ok(Response::new(GetRepositoryVersionResponse {
            commit_sha: store.get_version_local(),
        }));
    }

    async fn get_repository_contents(
        &self,
        request: Request<GetRepositoryContentsRequest>,
    ) -> Result<tonic::Response<GetRepositoryContentsResponse>, tonic::Status> {
        let api_key = request
            .metadata()
            .get("APIKEY")
            .ok_or_else(|| Status::invalid_argument("no api key provided"))?
            .clone();
        let inner = request.into_inner();
        let store_key = StoreKey {
            owner_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .owner_name,
            repo_name: inner
                .repo_key
                .clone()
                .ok_or_else(|| Status::invalid_argument("no repo key provided"))?
                .repo_name,
            api_key,
        };
        let (store, _) = &*self
            .cache
            .try_get_with(store_key.clone(), self.make_store(store_key.clone()))
            .await
            .unwrap(); // TODO - not sure how error prop works in rust

        let (version, namespaces) =
            store.get_repo_contents_local(&inner.namespace_name, &inner.feature_name);
        Ok(Response::new(GetRepositoryContentsResponse {
            namespaces,
            commit_sha: version,
        }))
    }

    async fn send_flag_evaluation_metrics(
        &self,
        _request: tonic::Request<SendFlagEvaluationMetricsRequest>,
    ) -> std::result::Result<tonic::Response<SendFlagEvaluationMetricsResponse>, tonic::Status>
    {
        // TODO not sure what is going on here
        Ok(tonic::Response::new(
            SendFlagEvaluationMetricsResponse::default(),
        ))
    }

    async fn register_client(
        &self,
        _request: tonic::Request<RegisterClientRequest>,
    ) -> std::result::Result<tonic::Response<RegisterClientResponse>, tonic::Status> {
        // TODO
        Ok(tonic::Response::new(RegisterClientResponse::default()))
    }

    async fn deregister_client(
        &self,
        _request: tonic::Request<DeregisterClientRequest>,
    ) -> std::result::Result<tonic::Response<DeregisterClientResponse>, tonic::Status> {
        Ok(tonic::Response::new(DeregisterClientResponse::default()))
    }
    async fn get_developer_access_token(
        &self,
        _request: tonic::Request<GetDeveloperAccessTokenRequest>,
    ) -> std::result::Result<tonic::Response<GetDeveloperAccessTokenResponse>, tonic::Status> {
        Err(tonic::Status::unimplemented(
            "cannot issue tokens from plekko",
        ))
    }
}
