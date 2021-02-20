use super::prelude::*;
pub type ElectLeadersRequest = ElectLeadersRequest0;
impl ApiCall for ElectLeadersRequest0 {
    type Response = ElectLeadersResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::ElectLeaders
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
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct ElectLeadersRequest0 {
    #[min_version = 1]
    pub election_type: Int8,
    #[min_version = 0]
    pub topic_partitions: Vec<ElectLeadersRequestTopicPartitions0>,
    #[min_version = 0]
    pub timeout_ms: Int32,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct ElectLeadersRequestTopicPartitions0 {
    #[min_version = 0]
    pub topic: String,
    #[min_version = 0]
    pub partition_id: Vec<Int32>,
    #[min_version = 2]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ElectLeadersResponse0 {
    #[min_version = 0]
    pub throttle_time_ms: Int32,
    #[min_version = 1]
    pub error_code: Option<Int16>,
    #[min_version = 0]
    pub replica_election_results: Vec<ElectLeadersResponseReplicaElectionResults0>,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResults0 {
    #[min_version = 0]
    pub topic: String,
    #[min_version = 0]
    pub partition_result: Vec<ElectLeadersResponseReplicaElectionResultsPartitionResult0>,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResultsPartitionResult0 {
    #[min_version = 0]
    pub partition_id: Int32,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub error_message: NullableString,
    #[min_version = 2]
    pub tag_buffer: Option<TagBuffer>,
}

impl ElectLeadersResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.replica_election_results.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl ElectLeadersResponseReplicaElectionResults0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partition_result.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl ElectLeadersResponseReplicaElectionResultsPartitionResult0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
