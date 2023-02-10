use std::{
    fs::{read, read_dir, read_to_string},
    path::Path,
};

use prost::Message;
use sha1::Digest;
use tonic::Status;
use yaml_rust::YamlLoader;

use crate::gen::lekko::{
    backend::v1beta1::{Feature, GetRepositoryContentsResponse, Namespace},
    feature,
};

// Helps the sidecar bootsrap feature flags using an on-disk clone of
// the config repo.
pub struct Bootstrap {
    // Path to the directory on disk that contains the .git folder.
    repo_path: String,
    // Path to the directory on disk that contains the repo contents (lekko.root.yaml).
    // It is assumed that the contents are in repo_path, which
    // is the case for most local clones of a git repo. If using git-sync,
    // we assume that the repo contents are in a subsirectory called 'contents'.
    contents_path: Option<String>,
}

impl Bootstrap {
    pub fn new(repo_path: String) -> Self {
        Self {
            repo_path,
            contents_path: None,
        }
    }

    pub fn load(&mut self) -> Result<GetRepositoryContentsResponse, Status> {
        self.validate()?;
        let commit_sha = self.git_commit_sha()?;
        let ns_names = self.find_namespace_names()?;
        let namespaces: Vec<Namespace> =
            match ns_names.iter().map(|ns| self.load_namespace(ns)).collect() {
                Ok(nsr) => nsr,
                Err(e) => return Err(e),
            };
        println!("git-sync commit sha {commit_sha:}");
        Ok(GetRepositoryContentsResponse {
            commit_sha,
            namespaces,
        })
    }

    // Check to see if the repo exists
    fn validate(&mut self) -> Result<(), Status> {
        let git_dir_path = format!("{:}/.git", self.repo_path);
        let md = match std::fs::metadata(Path::new(&git_dir_path)) {
            Ok(m) => m,
            Err(e) => {
                return Err(Status::internal(format!(
                    "path {git_dir_path:} does not exist: {e:?}",
                )))
            }
        };

        if !md.is_dir() {
            return Err(Status::internal(format!(
                "path {git_dir_path:} is not a directory",
            )));
        }
        let default_contents_path = self.repo_path.to_owned();
        let default_root_yaml_path = format!("{default_contents_path:}/lekko.root.yaml");
        if Path::new(&default_root_yaml_path).exists() {
            self.contents_path = Some(default_contents_path);
            return Ok(());
        }
        let git_sync_contents_path = format!("{:}/contents", self.repo_path);
        let git_sync_root_yaml_path = format!("{git_sync_contents_path:}/lekko.root.yaml");
        if !Path::new(&git_sync_root_yaml_path).exists() {
            return Err(Status::internal(format!(
                "paths {default_root_yaml_path:} or {git_sync_root_yaml_path:} do not exist",
            )));
        }
        self.contents_path = Some(git_sync_contents_path);
        Ok(())
    }

    // find namespaces contained in the repo by inspecting lekko.root.yaml.
    fn find_namespace_names(&self) -> Result<Vec<String>, Status> {
        let lekko_root_path = format!(
            "{:}/lekko.root.yaml",
            self.contents_path.to_owned().unwrap()
        );
        let yaml = match read_to_string(&lekko_root_path) {
            Ok(contents) => match YamlLoader::load_from_str(contents.as_ref()) {
                Ok(docs) => docs[0].to_owned(),
                Err(e) => {
                    return Err(Status::internal(format!(
                        "failed to parse lekko yaml: {e:?}",
                    )))
                }
            },
            Err(e) => {
                return Err(Status::internal(format!(
                    "failed to read lekko yaml from {lekko_root_path:?}: {e:?}",
                )))
            }
        };
        yaml["namespaces"]
            .clone()
            .into_iter()
            .map(|elem| match elem.as_str() {
                Some(s) => Ok(s.to_owned()),
                None => Err(Status::internal("unknown namespace")),
            })
            .collect()
    }

    fn load_namespace(&self, namespace: &str) -> Result<Namespace, Status> {
        let ns_path = format!(
            "{}/{namespace}/gen/proto",
            self.contents_path.to_owned().unwrap(),
        );
        let paths = read_dir(ns_path).unwrap();

        let mut features = vec![];

        for path in paths {
            let dir_entry = match path {
                Err(e) => {
                    return Err(Status::invalid_argument(format!(
                        "failed to read dir content: {e:?}",
                    )))
                }
                Ok(p) => p,
            };
            let ft = match dir_entry.file_type() {
                Err(e) => {
                    return Err(Status::invalid_argument(format!(
                        "failed to get file type {e:?}",
                    )))
                }
                Ok(ft) => ft,
            };
            if ft.is_file() {
                match read(dir_entry.path()) {
                    Err(e) => return Err(Status::internal(format!("failed to read path: {e:?}"))),
                    Ok(bytes) => {
                        let filename = dir_entry.file_name();
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
                        let sha = self.git_hash_object(bytes.as_ref())?;
                        features.push(Feature {
                            name: String::from(feature_name),
                            sha: sha.to_owned(),
                            feature: match feature::v1beta1::Feature::decode(bytes.as_ref()) {
                                Ok(d) => Some(d),
                                Err(e) => {
                                    println!("decode error! {e:?}");
                                    return Err(Status::internal(format!(
                                        "decode feature from git-sync: {e:?}",
                                    )));
                                }
                            },
                        });
                        println!("{feature_name:} [{:?} bytes]: sha {sha:}", bytes.len());
                    }
                }
            } else {
                continue;
            }
        }
        Ok(Namespace {
            name: namespace.to_owned(),
            features,
        })
    }

    // Calculates the git sha of a blob. Underlying algorithm uses
    // sha-1 with some prefixed bytes. see
    // https://stackoverflow.com/a/24283352/1849010
    fn git_hash_object(&self, content: &[u8]) -> Result<String, Status> {
        let mut hasher = sha1::Sha1::new();
        let prefix = format!("blob {:}\0", content.len());
        hasher.update(prefix.as_bytes());
        hasher.update(content);
        Ok(format!("{:x}", hasher.finalize()))
    }

    // Determines the git commit sha of HEAD.
    fn git_commit_sha(&self) -> Result<String, Status> {
        let repo = match git_repository::open(Path::new(&self.repo_path)) {
            Ok(r) => r,
            Err(e) => return Err(Status::internal(format!("failed to open repo: {e:?}"))),
        };
        let commit_sha = match repo.head_id() {
            Ok(id) => id.to_string(),
            Err(e) => return Err(Status::internal(format!("failed rev parse: {e:?}"))),
        };
        Ok(commit_sha)
    }
}
