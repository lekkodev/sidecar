use clap::Parser;
use hyper_rustls::HttpsConnectorBuilder;
use metrics::counter;
use prost::Message;
use sidecar::gen::cli::lekko::backend::v1beta1::distribution_service_client::DistributionServiceClient;
use sidecar::gen::cli::lekko::backend::v1beta1::GetRepositoryContentsRequest;
use sidecar::gen::cli::lekko::backend::v1beta1::GetRepositoryContentsResponse;
use sidecar::gen::cli::lekko::backend::v1beta1::Namespace;
use sidecar::gen::cli::lekko::backend::v1beta1::RepositoryKey;
use sidecar::logging;
use sidecar::metrics::RuntimeMetrics;
use sidecar::types::add_api_key;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use tonic::codegen::CompressionEncoding;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::transport::Uri;
use yaml_rust::Yaml;

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
    /// The url for the repo in "owner_name/repo_name" format, such as:
    /// lekkodev/example, representing github.com/lekkodev/example.
    repo_url: String,

    #[arg(short, long)]
    /// Absolute path to write to on desk. Application must have RW permission.
    output_path: String,
}

impl Debug for Args {
    // We manually implement Debug in order to avoid printing the api key.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{ lekko_addr: {}, api_key: {:?}, metrics_bind_addr: {}, poll_interval: {:?}, output_path: {} repo_url: {}}}", self.lekko_addr, "<lekko api key>", self.metrics_bind_addr, self.poll_interval, self.output_path, self.repo_url))
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
    let (owner, repo) = args.repo_url.split_once('/').unwrap_or_else(|| {
        panic!(
            "invalid repourl: {}, please use the format owner/repo i.e. lekkodev/example",
            args.repo_url
        )
    });

    let res = dist_client
        .clone()
        .get_repository_contents(add_api_key(
            GetRepositoryContentsRequest {
                repo_key: Some(RepositoryKey {
                    owner_name: owner.to_owned(),
                    repo_name: repo.to_owned(),
                }),
                feature_name: "".to_string(),
                namespace_name: "".to_string(),
                session_key: "".to_string(),
            },
            args.api_key,
        ))
        .await
        .unwrap_or_else(|e| panic!("error performing initial fetch: {:?}", e))
        .into_inner();
    let sha = res.commit_sha.clone();
    if let Err(err) = write_to_path(res, &args.output_path) {
        panic!("error writing results to disk: {:?}", err);
    }
    log::info!("sync completed of {sha} to {}", args.output_path);
    Ok(())
}

fn write_to_path(res: GetRepositoryContentsResponse, dest_path: &str) -> Result<(), tonic::Status> {
    let mut path = PathBuf::new();
    path.push(dest_path);
    write_git(path.as_os_str(), &res.commit_sha)?;
    path.push("lekko.root.yaml");
    write_root_yaml(path.as_os_str(), &res.namespaces)?;
    path.pop();
    for ns in res.namespaces {
        path.push(ns.name);
        path.push("gen");
        path.push("proto");
        create_dir_all(&path).map_err(|e| tonic::Status::internal(e.to_string()))?;
        for f in ns.features {
            path.push(format!("{}.proto.bin", f.name));
            let mut file =
                File::create(&path).map_err(|e| tonic::Status::internal(e.to_string()))?;
            let vec = f.feature.unwrap().encode_to_vec();
            file.write_all(&vec)
                .map_err(|e| tonic::Status::internal(e.to_string()))?;
            path.pop();
        }
        path.pop();
        path.pop();
        path.pop();
    }
    Ok(())
}

struct FmtToIoWriter<W> {
    writer: W,
}

impl<W> std::fmt::Write for FmtToIoWriter<W>
where
    W: std::io::Write,
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if self.writer.write(s.as_bytes()).is_err() {
            return Err(std::fmt::Error);
        }
        Ok(())
    }
}

fn write_git(root_path: &OsStr, head_sha: &str) -> Result<(), tonic::Status> {
    let mut p = PathBuf::new();
    p.push(root_path);
    p.push(".git");
    create_dir_all(&p).map_err(|e| tonic::Status::internal(e.to_string()))?;
    p.push("HEAD");
    let mut head = File::create(&p).map_err(|e| tonic::Status::internal(e.to_string()))?;
    writeln!(head, "{head_sha}").map_err(|e| tonic::Status::internal(e.to_string()))?;
    Ok(())
}

fn write_root_yaml(yaml_path: &OsStr, namespaces: &[Namespace]) -> Result<(), tonic::Status> {
    let mut file = FmtToIoWriter {
        writer: File::create(yaml_path).map_err(|e| tonic::Status::internal(e.to_string()))?,
    };
    let mut yaml = yaml_rust::YamlEmitter::new(&mut file);
    let val = Yaml::Hash(
        [(
            Yaml::String("namespaces".to_string()),
            Yaml::Array(
                namespaces
                    .iter()
                    .map(|s| Yaml::String(s.name.clone()))
                    .collect(),
            ),
        )]
        .into_iter()
        .collect(),
    );
    yaml.dump(&val)
        .map_err(|e| tonic::Status::internal(e.to_string()))
}
