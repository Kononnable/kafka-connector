use super::prelude::*;
pub type WriteTxnMarkersRequest = WriteTxnMarkersRequest0;
impl ApiCall for WriteTxnMarkersRequest0 {
    type Response = WriteTxnMarkersResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::WriteTxnMarkers
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(_version: u16) -> bool {
        false
    }
    fn serialize(self, version: u16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 2),
            false => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 1),
        }
        ToBytes::serialize(&self, buf, Self::is_flexible_version(version), version);
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
pub struct WriteTxnMarkersRequest0 {
    #[min_version = 0]
    pub markers: Vec<WriteTxnMarkersRequestMarkers0>,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct WriteTxnMarkersRequestMarkers0 {
    #[min_version = 0]
    pub producer_id: Int64,
    #[min_version = 0]
    pub producer_epoch: Int16,
    #[min_version = 0]
    pub transaction_result: Boolean,
    #[min_version = 0]
    pub topics: Vec<WriteTxnMarkersRequestMarkersTopics0>,
    #[min_version = 0]
    pub coordinator_epoch: Int32,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct WriteTxnMarkersRequestMarkersTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partition_indexes: Vec<Int32>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct WriteTxnMarkersResponse0 {
    #[min_version = 0]
    pub markers: Vec<WriteTxnMarkersResponseMarkers0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct WriteTxnMarkersResponseMarkers0 {
    #[min_version = 0]
    pub producer_id: Int64,
    #[min_version = 0]
    pub topics: Vec<WriteTxnMarkersResponseMarkersTopics0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct WriteTxnMarkersResponseMarkersTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub partitions: Vec<WriteTxnMarkersResponseMarkersTopicsPartitions0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct WriteTxnMarkersResponseMarkersTopicsPartitions0 {
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub error_code: Int16,
}

impl WriteTxnMarkersResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.markers.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl WriteTxnMarkersResponseMarkers0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.topics.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl WriteTxnMarkersResponseMarkersTopics0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partitions.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl WriteTxnMarkersResponseMarkersTopicsPartitions0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
