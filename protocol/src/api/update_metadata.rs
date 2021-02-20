use super::prelude::*;
pub type UpdateMetadataRequest = UpdateMetadataRequest1;
impl ApiCall for UpdateMetadataRequest0 {
    type Response = UpdateMetadataResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        4
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::UpdateMetadata
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
pub struct UpdateMetadataRequest0 {
    #[min_version = 0]
    pub controller_id: Int32,
    #[min_version = 0]
    pub controller_epoch: Int32,
    #[min_version = 0]
    pub ungrouped_partition_states: Vec<UpdateMetadataRequestUngroupedPartitionStates0>,
    #[min_version = 0]
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers0>,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestUngroupedPartitionStates0 {
    #[min_version = 0]
    pub topic_name: String,
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub controller_epoch: Int32,
    #[min_version = 0]
    pub leader: Int32,
    #[min_version = 0]
    pub leader_epoch: Int32,
    #[min_version = 0]
    pub isr: Vec<Int32>,
    #[min_version = 0]
    pub zk_version: Int32,
    #[min_version = 0]
    pub replicas: Vec<Int32>,
    #[min_version = 4]
    pub offline_replicas: Vec<Int32>,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers0 {
    #[min_version = 0]
    pub id: Int32,
    #[min_version = 1]
    pub endpoints: Vec<UpdateMetadataRequestLiveBrokersEndpoints0>,
    #[min_version = 2]
    pub rack: NullableString,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints0 {
    #[min_version = 1]
    pub port: Int32,
    #[min_version = 1]
    pub host: String,
    #[min_version = 3]
    pub listener: String,
    #[min_version = 1]
    pub security_protocol: Int16,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct UpdateMetadataResponse0 {
    #[min_version = 0]
    pub error_code: Int16,
}

impl UpdateMetadataResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
impl ApiCall for UpdateMetadataRequest1 {
    type Response = UpdateMetadataResponse1;
    fn get_min_supported_version() -> u16 {
        5
    }
    fn get_max_supported_version() -> u16 {
        6
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::UpdateMetadata
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 6
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
pub struct UpdateMetadataRequest1 {
    #[min_version = 5]
    pub controller_id: Int32,
    #[min_version = 5]
    pub controller_epoch: Int32,
    #[min_version = 5]
    pub broker_epoch: Int64,
    #[min_version = 5]
    pub topic_states: Vec<UpdateMetadataRequestTopicStates1>,
    #[min_version = 5]
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers1>,
    #[min_version = 6]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestTopicStates1 {
    #[min_version = 5]
    pub topic_name: String,
    #[min_version = 5]
    pub partition_states: Vec<UpdateMetadataRequestTopicStatesPartitionStates1>,
    #[min_version = 6]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestTopicStatesPartitionStates1 {
    #[min_version = 5]
    pub partition_index: Int32,
    #[min_version = 5]
    pub controller_epoch: Int32,
    #[min_version = 5]
    pub leader: Int32,
    #[min_version = 5]
    pub leader_epoch: Int32,
    #[min_version = 5]
    pub isr: Vec<Int32>,
    #[min_version = 5]
    pub zk_version: Int32,
    #[min_version = 5]
    pub replicas: Vec<Int32>,
    #[min_version = 5]
    pub offline_replicas: Vec<Int32>,
    #[min_version = 6]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers1 {
    #[min_version = 5]
    pub id: Int32,
    #[min_version = 5]
    pub endpoints: Vec<UpdateMetadataRequestLiveBrokersEndpoints1>,
    #[min_version = 5]
    pub rack: NullableString,
    #[min_version = 6]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints1 {
    #[min_version = 5]
    pub port: Int32,
    #[min_version = 5]
    pub host: String,
    #[min_version = 5]
    pub listener: String,
    #[min_version = 5]
    pub security_protocol: Int16,
    #[min_version = 6]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct UpdateMetadataResponse1 {
    #[min_version = 5]
    pub error_code: Int16,
    #[min_version = 6]
    pub tag_buffer: Option<TagBuffer>,
}

impl UpdateMetadataResponse1 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
