use super::prelude::*;
pub type ControlledShutdownRequest = ControlledShutdownRequest0;
impl ApiCall for ControlledShutdownRequest0 {
    type Response = ControlledShutdownResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::ControlledShutdown
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 3
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
pub struct ControlledShutdownRequest0 {
    #[min_version = 0]
    pub broker_id: Int32,
    #[min_version = 2]
    pub broker_epoch: Int64,
    #[min_version = 3]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ControlledShutdownResponse0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub remaining_partitions: Vec<ControlledShutdownResponseRemainingPartitions0>,
    #[min_version = 3]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ControlledShutdownResponseRemainingPartitions0 {
    #[min_version = 0]
    pub topic_name: String,
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 3]
    pub tag_buffer: Option<TagBuffer>,
}

impl ControlledShutdownResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.remaining_partitions.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl ControlledShutdownResponseRemainingPartitions0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
