use super::prelude::*;

pub type WriteTxnMarkersRequest = WriteTxnMarkersRequest0;
pub type WriteTxnMarkersResponse = WriteTxnMarkersResponse0;
impl ApiCall for WriteTxnMarkersRequest {
    type Response = WriteTxnMarkersResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::WriteTxnMarkers
    }
    fn get_first_error(response: &WriteTxnMarkersResponse) -> Option<ApiError> {
        WriteTxnMarkersResponse::get_first_error(response)
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            _ => false,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                WriteTxnMarkersRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                WriteTxnMarkersRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, WriteTxnMarkersResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => WriteTxnMarkersResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => WriteTxnMarkersResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct WriteTxnMarkersRequest0 {
    pub markers: Vec<WriteTxnMarkersRequestMarkers0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct WriteTxnMarkersRequestMarkers0 {
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub transaction_result: Boolean,
    pub topics: Vec<WriteTxnMarkersRequestMarkersTopics0>,
    pub coordinator_epoch: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct WriteTxnMarkersRequestMarkersTopics0 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct WriteTxnMarkersResponse0 {
    pub markers: Vec<WriteTxnMarkersResponseMarkers0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct WriteTxnMarkersResponseMarkers0 {
    pub producer_id: Int64,
    pub topics: Vec<WriteTxnMarkersResponseMarkersTopics0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct WriteTxnMarkersResponseMarkersTopics0 {
    pub name: String,
    pub partitions: Vec<WriteTxnMarkersResponseMarkersTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct WriteTxnMarkersResponseMarkersTopicsPartitions0 {
    pub partition_index: Int32,
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
