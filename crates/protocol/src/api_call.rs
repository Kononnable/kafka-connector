use std::fmt::Debug;

use bytes::{Bytes, BytesMut};

use crate::{api_error::ApiError, api_numbers::ApiNumbers};

pub trait ApiRequest: Clone + Debug {
    type Response: ApiResponse;

    fn get_max_supported_version() -> u16;
    fn get_api_key() -> ApiNumbers;
    fn serialize(&self, version: u16, bytes: &mut BytesMut, correlation_id: i32, client_id: &str);
}

pub trait ApiResponse: Debug {
    fn get_general_error(&self) -> Option<ApiError>;
    fn deserialize(version: u16, bytes: &mut Bytes) -> Self;
}
