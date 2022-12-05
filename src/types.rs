use prost::{DecodeError, Message};
use prost_types::Any;
use tonic::metadata::{Ascii, MetadataValue};

use crate::gen::lekko::backend::v1beta1::RepositoryKey;

// Key that the lekko api key is stored under in rpc headers.
pub const APIKEY: &str = "apikey";

// Contains all parameters needed to fetch a feature.
#[derive(Debug)]
pub struct FeatureRequestParams {
    // The API key that allows Lekko to identify the team that the feature is under.
    pub api_key: MetadataValue<Ascii>,
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
