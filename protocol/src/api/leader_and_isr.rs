use super::prelude::*;
pub type LeaderAndIsrRequest = LeaderAndIsrRequest1;
impl ApiCall for LeaderAndIsrRequest0 {
    type Response = LeaderAndIsrResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        1
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::LeaderAndIsr
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
pub struct LeaderAndIsrRequest0 {
    #[min_version = 0]
    pub controller_id: Int32,
    #[min_version = 0]
    pub controller_epoch: Int32,
    #[min_version = 0]
    pub ungrouped_partition_states: Vec<LeaderAndIsrRequestUngroupedPartitionStates0>,
    #[min_version = 0]
    pub live_leaders: Vec<LeaderAndIsrRequestLiveLeaders0>,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestUngroupedPartitionStates0 {
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
    #[min_version = 1]
    pub is_new: Boolean,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestLiveLeaders0 {
    #[min_version = 0]
    pub broker_id: Int32,
    #[min_version = 0]
    pub host_name: String,
    #[min_version = 0]
    pub port: Int32,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponse0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub partition_errors: Vec<LeaderAndIsrResponsePartitionErrors0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponsePartitionErrors0 {
    #[min_version = 0]
    pub topic_name: String,
    #[min_version = 0]
    pub partition_index: Int32,
    #[min_version = 0]
    pub error_code: Int16,
}

impl LeaderAndIsrResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partition_errors.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl LeaderAndIsrResponsePartitionErrors0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
impl ApiCall for LeaderAndIsrRequest1 {
    type Response = LeaderAndIsrResponse1;
    fn get_min_supported_version() -> u16 {
        2
    }
    fn get_max_supported_version() -> u16 {
        4
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::LeaderAndIsr
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 4
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
pub struct LeaderAndIsrRequest1 {
    #[min_version = 2]
    pub controller_id: Int32,
    #[min_version = 2]
    pub controller_epoch: Int32,
    #[min_version = 2]
    pub broker_epoch: Int64,
    #[min_version = 2]
    pub topic_states: Vec<LeaderAndIsrRequestTopicStates1>,
    #[min_version = 2]
    pub live_leaders: Vec<LeaderAndIsrRequestLiveLeaders1>,
    #[min_version = 4]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestTopicStates1 {
    #[min_version = 2]
    pub topic_name: String,
    #[min_version = 2]
    pub partition_states: Vec<LeaderAndIsrRequestTopicStatesPartitionStates1>,
    #[min_version = 4]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestTopicStatesPartitionStates1 {
    #[min_version = 2]
    pub partition_index: Int32,
    #[min_version = 2]
    pub controller_epoch: Int32,
    #[min_version = 2]
    pub leader: Int32,
    #[min_version = 2]
    pub leader_epoch: Int32,
    #[min_version = 2]
    pub isr: Vec<Int32>,
    #[min_version = 2]
    pub zk_version: Int32,
    #[min_version = 2]
    pub replicas: Vec<Int32>,
    #[min_version = 3]
    pub adding_replicas: Vec<Int32>,
    #[min_version = 3]
    pub removing_replicas: Vec<Int32>,
    #[min_version = 2]
    pub is_new: Boolean,
    #[min_version = 4]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestLiveLeaders1 {
    #[min_version = 2]
    pub broker_id: Int32,
    #[min_version = 2]
    pub host_name: String,
    #[min_version = 2]
    pub port: Int32,
    #[min_version = 4]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponse1 {
    #[min_version = 2]
    pub error_code: Int16,
    #[min_version = 2]
    pub partition_errors: Vec<LeaderAndIsrResponsePartitionErrors1>,
    #[min_version = 4]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponsePartitionErrors1 {
    #[min_version = 2]
    pub topic_name: String,
    #[min_version = 2]
    pub partition_index: Int32,
    #[min_version = 2]
    pub error_code: Int16,
    #[min_version = 4]
    pub tag_buffer: Option<TagBuffer>,
}

impl LeaderAndIsrResponse1 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partition_errors.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl LeaderAndIsrResponsePartitionErrors1 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
