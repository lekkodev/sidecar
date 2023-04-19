use prost::{DecodeError, Message};
use prost_types::Any;

use crate::gen::mod_cli::lekko::backend::v1beta1::RepositoryKey;
use crate::gen::mod_sdk::lekko::client::v1beta1::RepositoryKey as PublicRepositoryKey;

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

// Convert repository key from the publicly defined protobuf message to the private one.
pub fn convert_repo_key(rk: &PublicRepositoryKey) -> RepositoryKey {
    RepositoryKey {
        owner_name: rk.owner_name.to_owned(),
        repo_name: rk.repo_name.to_owned(),
    }
}
