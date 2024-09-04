pub use crate::{from_bytes::FromBytes, to_bytes::ToBytes};
pub use bytes::BytesMut;
pub use indexmap::{IndexMap, IndexSet};
pub use std::fmt::Debug;
use std::ops::Deref;
use thiserror::Error as DeriveError;

pub trait ApiRequest: Clone + Debug + Default {
    type Response: ApiResponse;

    fn get_api_key() -> ApiKey;
    fn get_min_supported_version() -> ApiVersion;
    fn get_max_supported_version() -> ApiVersion;
    fn serialize(
        &self,
        version: ApiVersion,
        bytes: &mut BytesMut,
    ) -> Result<(), SerializationError>;

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self;
}

pub trait ApiResponse: Clone + Debug + Default {
    type Request: ApiRequest;

    fn get_api_key() -> ApiKey;
    fn get_min_supported_version() -> ApiVersion;
    fn get_max_supported_version() -> ApiVersion;
    fn serialize(
        &self,
        version: ApiVersion,
        bytes: &mut BytesMut,
    ) -> Result<(), SerializationError>;

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self;
}

#[non_exhaustive]
#[derive(Clone, Debug, DeriveError)]
pub enum SerializationError {
    #[error("Field {0} is required in {1} api version of {2}")]
    NullValue(&'static str, i16, &'static str),
    #[error("Field {0} has value set, but it does not exist in {1} api version of {2}")]
    NonIgnorableFieldSet(&'static str, i16, &'static str),
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub struct ApiVersion(pub i16);
impl Deref for ApiVersion {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone)]
pub struct ApiKey(pub i16);

impl Deref for ApiKey {
    type Target = i16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
