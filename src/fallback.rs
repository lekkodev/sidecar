use std::fs::{read, read_dir};

use tonic::Status;

use crate::gen::lekko::backend::v1beta1::{RepositoryKey, GetRepositoryContentsResponse, Namespace};

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
    pub fn load(&self, repo_key: RepositoryKey, namespaces: &[String]) -> Result<GetRepositoryContentsResponse, Status> {
        println!(
            "Fallback: load {:?} {:?} {:?}",
            self.repo_path, repo_key, namespaces
        );
        let mut ns_results = vec![];
        for namespace in namespaces {
            ns_results.push(self.load_namespace(namespace)?);
        }
        Ok(GetRepositoryContentsResponse {
            commit_sha: "".to_string(),
            namespaces: ns_results,
        })
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
                Ok(p) => {
                    println!("Name: {}", p.path().display());
                    match p.file_type() {
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
                                        println!(
                                            "file name {:?} has {:?} bytes",
                                            p.file_name(),
                                            bytes.len()
                                        );
                                        let filename = match p.file_name().to_str() {
                                            Some(file) => file,
                                            None => return Err(Status::internal("file name empty")),
                                        };
                                        let feature_name = 
                                        let s = p.file_name().to_str().unwrap().split_once(".");
                                    }
                                }
                            } else {
                                continue;
                            }
                        }
                    }
                }
            };
        }
        Ok(Namespace { name: namespace.to_owned(), features })
    }
}
