use std::fs::{read, read_dir};

use prost::Message;
use sha1::Digest;
use tonic::Status;

use crate::gen::lekko::{
    backend::v1beta1::{Feature, GetRepositoryContentsResponse, Namespace, RepositoryKey},
    feature,
};

pub struct Fallback {
    // If none, fallback behavior is not enabled.
    repo_path: Option<String>,
}

impl Fallback {
    pub fn new(repo_path: Option<String>) -> Self {
        Self { repo_path }
    }
    pub fn enabled(&self) -> bool {
        self.repo_path.is_some()
    }
    pub fn load(
        &self,
        repo_key: RepositoryKey,
        namespaces: &[String],
    ) -> Result<GetRepositoryContentsResponse, Status> {
        println!(
            "Fallback: load {:?} {:?} {:?}",
            self.repo_path, repo_key, namespaces
        );
        let mut ns_results = vec![];
        for namespace in namespaces {
            ns_results.push(self.load_namespace(namespace)?);
        }
        let resp = GetRepositoryContentsResponse {
            commit_sha: "".to_string(), // TODO: get commit sha from git repo
            namespaces: ns_results,
        };
        println!("git-sync commit sha {:?}", resp.commit_sha);
        Ok(resp)
    }

    fn load_namespace(&self, namespace: &str) -> Result<Namespace, Status> {
        let ns_path = format!(
            "{}/{}/gen/proto",
            self.repo_path.to_owned().unwrap(),
            namespace
        );
        let paths = read_dir(ns_path).unwrap();

        let mut features = vec![];

        for path in paths {
            match path {
                Err(e) => {
                    return Err(Status::invalid_argument(format!(
                        "failed to read dir content: {:?}",
                        e
                    )))
                }
                Ok(p) => match p.file_type() {
                    Err(e) => {
                        return Err(Status::invalid_argument(format!(
                            "failed to get file type {:?}",
                            e
                        )))
                    }
                    Ok(ft) => {
                        if ft.is_file() {
                            match read(p.path()) {
                                Err(e) => {
                                    return Err(Status::internal(format!(
                                        "failed to read path: {:?}",
                                        e
                                    )))
                                }
                                Ok(bytes) => {
                                    let filename = p.file_name();
                                    let filename = match filename.to_str() {
                                        Some(file) => file,
                                        None => return Err(Status::internal("file name empty")),
                                    };
                                    let feature_name = match filename.strip_suffix(".proto.bin") {
                                        Some(a) => a,
                                        None => {
                                            println!("malformed filename in gen/proto, skipping");
                                            continue;
                                        }
                                    };
                                    let sha = self.blob_sha(bytes.as_ref())?;
                                    features.push(Feature {
                                        name: String::from(feature_name),
                                        sha: sha.to_owned(),
                                        feature: match feature::v1beta1::Feature::decode(
                                            bytes.as_ref(),
                                        ) {
                                            Ok(d) => Some(d),
                                            Err(e) => {
                                                println!("decode error! {:?}", e);
                                                return Err(Status::internal(format!(
                                                    "decode feature from git-sync: {:?}",
                                                    e
                                                )));
                                            }
                                        },
                                    });
                                    println!(
                                        "{:} [{:?} bytes]: sha {:?}",
                                        feature_name,
                                        bytes.len(),
                                        sha,
                                    );
                                }
                            }
                        } else {
                            continue;
                        }
                    }
                },
            };
        }
        Ok(Namespace {
            name: namespace.to_owned(),
            features,
        })
    }

    // Calculates the git sha of a blob. Underlying algorithm uses
    // sha-1 with some prefixed bytes. see
    // https://stackoverflow.com/a/24283352/1849010
    fn blob_sha(&self, content: &[u8]) -> Result<String, Status> {
        let mut hasher = sha1::Sha1::new();
        let prefix = format!("blob {:}\0", content.len());
        hasher.update(prefix.as_bytes());
        hasher.update(content);
        Ok(format!("{:x}", hasher.finalize()))
    }
}
