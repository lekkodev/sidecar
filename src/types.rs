use prost::{DecodeError, Message};
use prost_types::Any;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::Request;

use crate::gen::mod_cli::lekko::backend::v1beta1::RepositoryKey;
use crate::gen::mod_sdk::lekko::client::v1beta1::RepositoryKey as PublicRepositoryKey;

// Key that the lekko api key is stored under in rpc headers.
pub const APIKEY: &str = "apikey";

// Mode represents the running mode of the sidecar.
//
// Default implies polling for updates from remote while
// and evaluating locally while polling for updates. Initialization
// can be optionally boostrapped from a volume of the configuration repo.
//
// Static fetches from the bootstrap and always evaluates against those values. No
// connection is made to Lekko services.
#[derive(clap::ValueEnum, Clone, Copy, Default, Debug)]
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

pub fn add_api_key<T: Message>(m: T, api_key: MetadataValue<Ascii>) -> Request<T> {
    let mut r = Request::new(m);
    r.metadata_mut().append(APIKEY, api_key);
    r
}
