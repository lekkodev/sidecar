use prost::{DecodeError, Message};
use prost_types::Any;
use tonic::metadata::{Ascii, MetadataValue};

use crate::gen::cli::lekko::backend::v1beta1::RepositoryKey;
use crate::gen::sdk::lekko::client::v1beta1::RepositoryKey as PublicRepositoryKey;

// Key that the lekko api key is stored under in rpc headers.
pub const APIKEY: &str = "apikey";

// Contains all parameters needed to fetch a feature.
#[derive(Debug, Clone)]
pub struct FeatureRequestParams {
    pub rk: RepositoryKey,
    pub namespace: String,
    pub feature: String,
}

// from_any converts the Any message to the given type. This method should be used
// when the caller expects the message to contain the given type.
pub fn from_any<T>(message: &Any) -> Result<T, DecodeError>
where
    T: Message + Default,
{
    T::decode(message.value.as_slice())
}

// for testing.
pub fn to_any<T>(message: &T) -> Any
where
    T: Message,
{
    Any {
        type_url: String::from(""),
        value: message.encode_to_vec(),
    }
}

// Copies repository key from the publicly defined protobuf message into a private one.
pub fn convert_repo_key(rk: &PublicRepositoryKey) -> RepositoryKey {
    RepositoryKey {
        owner_name: rk.owner_name.clone(),
        repo_name: rk.repo_name.clone(),
    }
}

// Mode represents the running mode of the sidecar.
//
// Default implies waiting for a Register call, fetching from a bootstrap,
// and evaluating locally while polling for updates.
//
// Static fetches from the bootstrap and always evaluates against those values. No
// connection is made to Lekko services.
#[derive(clap::ValueEnum, Clone, Default, Debug)]
pub enum Mode {
    #[default]
    Default,
    Static,
}

#[derive(Clone)]
pub struct ConnectionCredentials {
    pub repo_key: RepositoryKey,
    pub api_key: MetadataValue<Ascii>,
    pub session_key: String,
}

pub fn add_api_key<T: Message>(m: T, api_key: MetadataValue<Ascii>) -> tonic::Request<T> {
    let mut r = tonic::Request::new(m);
    r.metadata_mut().append(APIKEY, api_key);
    r
}

// If we have a message that may have an api key, override it only if
// we have an apikey, and one isn't contained in the request.
pub fn override_api_key<T: Message>(
    mut r: tonic::Request<T>,
    conn_creds_opt: &Option<ConnectionCredentials>,
) -> tonic::Request<T> {
    if let (Some(_), Some(cc)) = (r.metadata().get(APIKEY), conn_creds_opt) {
        r.metadata_mut().append(APIKEY, cc.api_key.clone());
    }
    r
}

pub fn get_owner_and_repo(path: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = path.trim_end_matches(".git").split('/').collect();
    if parts.len() >= 2 {
        let repo_name = parts[parts.len() - 1].to_owned();
        let sub_parts: Vec<&str> = parts[parts.len() - 2].split(':').collect();
        Some((sub_parts[sub_parts.len() - 1].to_owned(), repo_name))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::types::get_owner_and_repo;

    #[test]
    fn test_get_owner_and_repo_http() {
        match get_owner_and_repo("https://github.com/lekkodev/example") {
            Some(tuple) => assert_eq!(tuple, (String::from("lekkodev"), String::from("example"))),
            None => panic!("owner and repo should exist"),
        }

        match get_owner_and_repo("https://github.com/lekkodev/example.git") {
            Some(tuple) => assert_eq!(tuple, (String::from("lekkodev"), String::from("example"))),
            None => panic!("owner and repo should exist with .git suffix"),
        }
    }

    #[test]
    fn test_get_owner_and_repo_ssh() {
        match get_owner_and_repo("git@github.com:lekkodev/example.git") {
            Some(tuple) => assert_eq!(tuple, (String::from("lekkodev"), String::from("example"))),
            None => panic!("owner and repo should exist"),
        }

        match get_owner_and_repo("git@github.com:lekkodev/example") {
            Some(tuple) => assert_eq!(tuple, (String::from("lekkodev"), String::from("example"))),
            None => panic!("owner and repo should exist without .git suffix"),
        }
    }

    #[test]
    fn test_get_owner_and_repo_invalid() {
        match get_owner_and_repo("") {
            None => (),
            _ => panic!("empty string is invalid"),
        }

        match get_owner_and_repo("lekkodev-example") {
            None => (),
            _ => panic!("invalid url"),
        }
    }
}
