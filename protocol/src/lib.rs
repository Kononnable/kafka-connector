use api::ApiNumbers;
use api_error::ApiError;
use bytes::{Bytes, BytesMut};

pub mod api;
pub mod api_error;
pub mod custom_types;
pub mod from_bytes;
pub mod to_bytes;

pub trait ApiCall: Clone {
    type Response;

    fn get_min_supported_version() -> i16;
    fn get_max_supported_version() -> i16;
    fn get_api_key() -> ApiNumbers;
    fn get_first_error(response: &Self::Response) -> Option<ApiError>;
    fn serialize(self, version: i16, bytes: &mut BytesMut, correlation_id: i32, client_id: &str);
    fn deserialize_response(version: i16, bytes: &mut Bytes) -> (i32, Self::Response);
    fn is_flexible_version(version: i16) -> bool;
}
