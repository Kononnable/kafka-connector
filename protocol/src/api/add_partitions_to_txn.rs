use super::prelude::*;
pub type AddPartitionsToTxnRequest = AddPartitionsToTxnRequest0;
impl ApiCall for AddPartitionsToTxnRequest0 {
    type Response = AddPartitionsToTxnResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::AddPartitionsToTxn
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(_version: u16) -> bool {
        false
    }
    fn serialize(&self, version: u16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 2),
            false => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 1),
        }
        ToBytes::serialize(self, buf, Self::is_flexible_version(version), version);
    }
    fn deserialize_response(version: u16, buf: &mut Bytes) -> (i32, Self::Response) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse::deserialize(buf, false, 2).correlation,
            false => HeaderResponse::deserialize(buf, false, 1).correlation,
        };
        let response =
            Self::Response::deserialize(buf, Self::is_flexible_version(version), version);
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AddPartitionsToTxnRequest0 {
    #[min_version = 0]
    pub transactional_id: String,
    #[min_version = 0]
    pub producer_id: Int64,
    #[min_version = 0]
    pub producer_epoch: Int16,
    #[min_version = 0]
    pub topics: Vec<AddPartitionsToTxnRequestTopics0>,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AddPartitionsToTxnRequestTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partitions: Vec<Int32>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddPartitionsToTxnResponse0 {
    #[min_version = 0]
    pub throttle_time_ms: Int32,
    #[min_version = 0]
    pub results: Vec<AddPartitionsToTxnResponseResults0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddPartitionsToTxnResponseResults0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub results: Vec<AddPartitionsToTxnResponseResultsResults0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddPartitionsToTxnResponseResultsResults0 {
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub error_code: Int16,
}

impl AddPartitionsToTxnResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.results.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl AddPartitionsToTxnResponseResults0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.results.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl AddPartitionsToTxnResponseResultsResults0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
