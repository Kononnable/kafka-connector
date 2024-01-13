pub use crate::from_bytes::FromBytes;
pub use crate::generated::request_header::RequestHeader;
pub use crate::generated::response_header::ResponseHeader;
pub use crate::to_bytes::ToBytes;
pub use bytes::{Bytes, BytesMut};
pub use std::fmt::Debug;

use thiserror::Error as DeriveError;
pub trait ApiRequest: Clone + Debug + Default {
    type Response: ApiResponse;

    fn get_api_key() -> i16;
    fn get_min_supported_version() -> i16;
    fn get_max_supported_version() -> i16;
    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &RequestHeader,
    ) -> Result<(), SerializationError>;
}

pub trait ApiResponse: Clone + Debug + Default {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self);
}

#[non_exhaustive]
#[derive(Clone, Debug, DeriveError)]
pub enum SerializationError {
    #[error("Field {0} is required in {1} api version of {2}")]
    NullValue(&'static str, i16, &'static str),
}
