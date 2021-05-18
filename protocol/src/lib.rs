use api::{header::OwnedHeaderRequest, ApiNumbers};
use api_error::ApiError;
use bytes::{Bytes, BytesMut};

pub mod api;
pub mod api_error;
pub mod custom_types;
pub mod from_bytes;
pub mod to_bytes;

pub trait ApiCall: Clone {
    type Response: Default;

    fn get_min_supported_version() -> u16;
    fn get_max_supported_version() -> u16;
    fn get_api_key() -> ApiNumbers;
    fn get_first_error(response: &Self::Response) -> Option<ApiError>;
    fn serialize(&self, version: u16, bytes: &mut BytesMut, correlation_id: i32, client_id: &str);
    fn deserialize_request(version: u16, bytes: &mut Bytes) -> (OwnedHeaderRequest, Self);
    fn deserialize_response(version: u16, bytes: &mut Bytes) -> (i32, Self::Response);
    fn is_flexible_version(version: u16) -> bool;
}
