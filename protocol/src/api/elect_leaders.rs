use super::prelude::*;

pub type ElectLeadersRequest = ElectLeadersRequest2;
pub type ElectLeadersResponse = ElectLeadersResponse2;
impl ApiCall for ElectLeadersRequest {
    type Response = ElectLeadersResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::ElectLeaders
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&ElectLeadersRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&ElectLeadersRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> ElectLeadersResponse {
        match version {
            0 => ElectLeadersResponse0::deserialize(buf).into(),
            1 => ElectLeadersResponse1::deserialize(buf).into(),
            2 => ElectLeadersResponse::deserialize(buf),
            _ => ElectLeadersResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct ElectLeadersRequest0 {
    pub topic_partitions: Vec<ElectLeadersRequestTopicPartitions0>,
    pub timeout_ms: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ElectLeadersRequestTopicPartitions0 {
    pub topic: String,
    pub partition_id: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ElectLeadersRequest1 {
    pub election_type: Optional<Int8>,
    pub topic_partitions: Vec<ElectLeadersRequestTopicPartitions1>,
    pub timeout_ms: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ElectLeadersRequestTopicPartitions1 {
    pub topic: String,
    pub partition_id: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ElectLeadersRequest2 {
    pub election_type: Optional<Int8>,
    pub topic_partitions: Vec<ElectLeadersRequestTopicPartitions2>,
    pub timeout_ms: Int32,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ElectLeadersRequestTopicPartitions2 {
    pub topic: CompactString,
    pub partition_id: Vec<Int32>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ElectLeadersResponse0 {
    pub throttle_time_ms: Int32,
    pub replica_election_results: Vec<ElectLeadersResponseReplicaElectionResults0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResults0 {
    pub topic: String,
    pub partition_result: Vec<ElectLeadersResponseReplicaElectionResultsPartitionResult0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResultsPartitionResult0 {
    pub partition_id: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ElectLeadersResponse1 {
    pub throttle_time_ms: Int32,
    pub error_code: Optional<Int16>,
    pub replica_election_results: Vec<ElectLeadersResponseReplicaElectionResults1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResults1 {
    pub topic: String,
    pub partition_result: Vec<ElectLeadersResponseReplicaElectionResultsPartitionResult1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResultsPartitionResult1 {
    pub partition_id: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ElectLeadersResponse2 {
    pub throttle_time_ms: Int32,
    pub error_code: Optional<Int16>,
    pub replica_election_results: Vec<ElectLeadersResponseReplicaElectionResults2>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResults2 {
    pub topic: CompactString,
    pub partition_result: Vec<ElectLeadersResponseReplicaElectionResultsPartitionResult2>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResultsPartitionResult2 {
    pub partition_id: Int32,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub tag_buffer: Optional<TagBuffer>,
}

impl TryFrom<ElectLeadersRequest2> for ElectLeadersRequest0 {
    type Error = Error;
    fn try_from(latest: ElectLeadersRequest2) -> Result<Self, Self::Error> {
        if latest.election_type.is_some() {
            return Err(Error::OldKafkaVersion(
                "ElectLeadersRequest",
                0,
                "election_type",
            ));
        }
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "ElectLeadersRequest",
                0,
                "tag_buffer",
            ));
        }
        Ok(ElectLeadersRequest0 {
            topic_partitions: latest
                .topic_partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            timeout_ms: latest.timeout_ms,
        })
    }
}

impl TryFrom<ElectLeadersRequestTopicPartitions2> for ElectLeadersRequestTopicPartitions0 {
    type Error = Error;
    fn try_from(latest: ElectLeadersRequestTopicPartitions2) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "ElectLeadersRequestTopicPartitions",
                0,
                "tag_buffer",
            ));
        }
        Ok(ElectLeadersRequestTopicPartitions0 {
            topic: latest.topic.into(),
            partition_id: latest.partition_id,
        })
    }
}

impl TryFrom<ElectLeadersRequest2> for ElectLeadersRequest1 {
    type Error = Error;
    fn try_from(latest: ElectLeadersRequest2) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "ElectLeadersRequest",
                1,
                "tag_buffer",
            ));
        }
        Ok(ElectLeadersRequest1 {
            election_type: latest.election_type,
            topic_partitions: latest
                .topic_partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            timeout_ms: latest.timeout_ms,
        })
    }
}

impl TryFrom<ElectLeadersRequestTopicPartitions2> for ElectLeadersRequestTopicPartitions1 {
    type Error = Error;
    fn try_from(latest: ElectLeadersRequestTopicPartitions2) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "ElectLeadersRequestTopicPartitions",
                1,
                "tag_buffer",
            ));
        }
        Ok(ElectLeadersRequestTopicPartitions1 {
            topic: latest.topic.into(),
            partition_id: latest.partition_id,
        })
    }
}

impl From<ElectLeadersResponse0> for ElectLeadersResponse2 {
    fn from(older: ElectLeadersResponse0) -> Self {
        ElectLeadersResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            replica_election_results: older
                .replica_election_results
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..ElectLeadersResponse2::default()
        }
    }
}

impl From<ElectLeadersResponseReplicaElectionResults0>
    for ElectLeadersResponseReplicaElectionResults2
{
    fn from(older: ElectLeadersResponseReplicaElectionResults0) -> Self {
        ElectLeadersResponseReplicaElectionResults2 {
            topic: older.topic.into(),
            partition_result: older
                .partition_result
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..ElectLeadersResponseReplicaElectionResults2::default()
        }
    }
}

impl From<ElectLeadersResponseReplicaElectionResultsPartitionResult0>
    for ElectLeadersResponseReplicaElectionResultsPartitionResult2
{
    fn from(older: ElectLeadersResponseReplicaElectionResultsPartitionResult0) -> Self {
        ElectLeadersResponseReplicaElectionResultsPartitionResult2 {
            partition_id: older.partition_id,
            error_code: older.error_code,
            error_message: older.error_message.into(),
            ..ElectLeadersResponseReplicaElectionResultsPartitionResult2::default()
        }
    }
}

impl From<ElectLeadersResponse1> for ElectLeadersResponse2 {
    fn from(older: ElectLeadersResponse1) -> Self {
        ElectLeadersResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            replica_election_results: older
                .replica_election_results
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..ElectLeadersResponse2::default()
        }
    }
}

impl From<ElectLeadersResponseReplicaElectionResults1>
    for ElectLeadersResponseReplicaElectionResults2
{
    fn from(older: ElectLeadersResponseReplicaElectionResults1) -> Self {
        ElectLeadersResponseReplicaElectionResults2 {
            topic: older.topic.into(),
            partition_result: older
                .partition_result
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..ElectLeadersResponseReplicaElectionResults2::default()
        }
    }
}

impl From<ElectLeadersResponseReplicaElectionResultsPartitionResult1>
    for ElectLeadersResponseReplicaElectionResultsPartitionResult2
{
    fn from(older: ElectLeadersResponseReplicaElectionResultsPartitionResult1) -> Self {
        ElectLeadersResponseReplicaElectionResultsPartitionResult2 {
            partition_id: older.partition_id,
            error_code: older.error_code,
            error_message: older.error_message.into(),
            ..ElectLeadersResponseReplicaElectionResultsPartitionResult2::default()
        }
    }
}
