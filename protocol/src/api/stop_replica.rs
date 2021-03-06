use super::prelude::*;
pub type StopReplicaRequest = StopReplicaRequest2;
impl ApiCall for StopReplicaRequest0 {
    type Response = StopReplicaResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::StopReplica
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
pub struct StopReplicaRequest0 {
    #[min_version = 0]
    pub controller_id: Int32,
    #[min_version = 0]
    pub controller_epoch: Int32,
    #[min_version = 0]
    pub delete_partitions: Boolean,
    #[min_version = 0]
    pub ungrouped_partitions: Vec<StopReplicaRequestUngroupedPartitions0>,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct StopReplicaRequestUngroupedPartitions0 {
    #[min_version = 0]
    pub topic_name: String,
    #[min_version = 0]
    pub partition_index: Int32,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponse0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub partition_errors: Vec<StopReplicaResponsePartitionErrors0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponsePartitionErrors0 {
    #[min_version = 0]
    pub topic_name: String,
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub error_code: Int16,
}

impl StopReplicaResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        if self.error_code != 0 {
            return Some(self.error_code.into());
        }
        None
    }
}
impl ApiCall for StopReplicaRequest1 {
    type Response = StopReplicaResponse1;
    fn get_min_supported_version() -> u16 {
        1
    }
    fn get_max_supported_version() -> u16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::StopReplica
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 2
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
pub struct StopReplicaRequest1 {
    #[min_version = 1]
    pub controller_id: Int32,
    #[min_version = 1]
    pub controller_epoch: Int32,
    #[min_version = 1]
    pub broker_epoch: Int64,
    #[min_version = 1]
    pub delete_partitions: Boolean,
    #[min_version = 1]
    pub topics: Vec<StopReplicaRequestTopics1>,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct StopReplicaRequestTopics1 {
    #[min_version = 1]
    pub name: String,
    #[min_version = 1]
    pub partition_indexes: Vec<Int32>,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponse1 {
    #[min_version = 1]
    pub error_code: Int16,
    #[min_version = 1]
    pub partition_errors: Vec<StopReplicaResponsePartitionErrors1>,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponsePartitionErrors1 {
    #[min_version = 1]
    pub topic_name: String,
    #[min_version = 1]
    pub partition_index: Int32,
    #[min_version = 1]
    pub error_code: Int16,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}

impl StopReplicaResponse1 {
    fn get_first_error(&self) -> Option<ApiError> {
        if self.error_code != 0 {
            return Some(self.error_code.into());
        }
        None
    }
}
impl ApiCall for StopReplicaRequest2 {
    type Response = StopReplicaResponse2;
    fn get_min_supported_version() -> u16 {
        3
    }
    fn get_max_supported_version() -> u16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::StopReplica
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(_version: u16) -> bool {
        true
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
pub struct StopReplicaRequest2 {
    #[min_version = 3]
    pub controller_id: Int32,
    #[min_version = 3]
    pub controller_epoch: Int32,
    #[min_version = 3]
    pub broker_epoch: Int64,
    #[min_version = 3]
    pub topic_states: Vec<StopReplicaRequestTopicStates2>,
    #[min_version = 3]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct StopReplicaRequestTopicStates2 {
    #[min_version = 3]
    pub topic_name: String,
    #[min_version = 3]
    pub partition_states: Vec<StopReplicaRequestTopicStatesPartitionStates2>,
    #[min_version = 3]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct StopReplicaRequestTopicStatesPartitionStates2 {
    #[min_version = 3]
    pub partition_index: Int32,
    #[min_version = 3]
    pub leader_epoch: Int32,
    #[min_version = 3]
    pub delete_partition: Boolean,
    #[min_version = 3]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponse2 {
    #[min_version = 3]
    pub error_code: Int16,
    #[min_version = 3]
    pub partition_errors: Vec<StopReplicaResponsePartitionErrors2>,
    #[min_version = 3]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponsePartitionErrors2 {
    #[min_version = 3]
    pub topic_name: String,
    #[min_version = 3]
    pub partition_index: Int32,
    #[min_version = 3]
    pub error_code: Int16,
    #[min_version = 3]
    pub tag_buffer: TagBuffer,
}

impl StopReplicaResponse2 {
    fn get_first_error(&self) -> Option<ApiError> {
        if self.error_code != 0 {
            return Some(self.error_code.into());
        }
        None
    }
}
