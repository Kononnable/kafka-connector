use super::prelude::*;
pub type MetadataRequest = MetadataRequest0;
impl ApiCall for MetadataRequest0 {
    type Response = MetadataResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        9
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::Metadata
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 9
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
    fn deserialize_request(version: u16, buf: &mut Bytes) -> (OwnedHeaderRequest, Self) {
        let header = match Self::is_flexible_version(version) {
            true => OwnedHeaderRequest::deserialize(buf, false, 2),
            false => OwnedHeaderRequest::deserialize(buf, false, 1),
        };
        let request = Self::deserialize(buf, Self::is_flexible_version(version), version);
        (header, request)
    }
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct MetadataRequest0 {
    #[min_version = 0]
    pub topics: Vec<MetadataRequestTopics0>,
    #[min_version = 4]
    pub allow_auto_topic_creation: Boolean,
    #[min_version = 8]
    pub include_cluster_authorized_operations: Boolean,
    #[min_version = 8]
    pub include_topic_authorized_operations: Boolean,
    #[min_version = 9]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct MetadataRequestTopics0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 9]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponse0 {
    #[min_version = 3]
    pub throttle_time_ms: Option<Int32>,
    #[min_version = 0]
    pub brokers: Vec<MetadataResponseBrokers0>,
    #[min_version = 2]
    pub cluster_id: Option<NullableString>,
    #[min_version = 1]
    pub controller_id: Option<Int32>,
    #[min_version = 0]
    pub topics: Vec<MetadataResponseTopics0>,
    #[min_version = 8]
    pub cluster_authorized_operations: Option<Int32>,
    #[min_version = 9]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseBrokers0 {
    #[min_version = 0]
    pub node_id: Int32,
    #[min_version = 0]
    pub host: String,
    #[min_version = 0]
    pub port: Int32,
    #[min_version = 1]
    pub rack: Option<NullableString>,
    #[min_version = 9]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopics0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub name: String,
    #[min_version = 1]
    pub is_internal: Option<Boolean>,
    #[min_version = 0]
    pub partitions: Vec<MetadataResponseTopicsPartitions0>,
    #[min_version = 8]
    pub topic_authorized_operations: Option<Int32>,
    #[min_version = 9]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopicsPartitions0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub leader_id: Int32,
    #[min_version = 7]
    pub leader_epoch: Option<Int32>,
    #[min_version = 0]
    pub replica_nodes: Vec<Int32>,
    #[min_version = 0]
    pub isr_nodes: Vec<Int32>,
    #[min_version = 5]
    pub offline_replicas: Option<Vec<Int32>>,
    #[min_version = 9]
    pub tag_buffer: Option<TagBuffer>,
}

impl MetadataResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
