use api::ApiNumbers;
use bytes::{Bytes, BytesMut};
use error::Error;

pub mod api;
pub mod custom_types;
pub mod error;
pub mod from_bytes;
pub mod optional;
pub mod to_bytes;

pub trait ApiCall {
    type Response;

    fn get_min_supported_version() -> i16;
    fn get_max_supported_version() -> i16;
    fn get_api_key() -> ApiNumbers;
    fn serialize(self, version: i16, bytes: &mut BytesMut) -> Result<(), Error>;
    fn deserialize_response(version: i16, bytes: &mut Bytes) -> Self::Response;
}
