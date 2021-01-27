use super::prelude::*;

pub type ElectLeadersRequest = ElectLeadersRequest2;
pub type ElectLeadersResponse = ElectLeadersResponse2;
pub fn serialize_elect_leaders_request(
    data: ElectLeadersRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&ElectLeadersRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&ElectLeadersRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_elect_leaders_response(version: i32, buf: &mut Bytes) -> ElectLeadersResponse {
    match version {
        0 => ElectLeadersResponse0::deserialize(buf).into(),
        1 => ElectLeadersResponse1::deserialize(buf).into(),
        2 => ElectLeadersResponse::deserialize(buf),
        _ => ElectLeadersResponse::deserialize(buf),
    }
}

#[derive(Default, Debug, ToBytes)]
pub struct ElectLeadersRequest0 {
    pub topic_partitions: Vec<ElectLeadersRequestTopicPartitions0>,
    pub timeout_ms: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct ElectLeadersRequestTopicPartitions0 {
    pub topic: String,
    pub partition_id: Vec<Int32>,
}

#[derive(Default, Debug, ToBytes)]
pub struct ElectLeadersRequest1 {
    pub election_type: Optional<Int8>,
    pub topic_partitions: Vec<ElectLeadersRequestTopicPartitions1>,
    pub timeout_ms: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct ElectLeadersRequestTopicPartitions1 {
    pub topic: String,
    pub partition_id: Vec<Int32>,
}

#[derive(Default, Debug, ToBytes)]
pub struct ElectLeadersRequest2 {
    pub election_type: Optional<Int8>,
    pub topic_partitions: Vec<ElectLeadersRequestTopicPartitions2>,
    pub timeout_ms: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct ElectLeadersRequestTopicPartitions2 {
    pub topic: CompactString,
    pub partition_id: Vec<Int32>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ElectLeadersResponse0 {
    pub throttle_time_ms: Int32,
    pub replica_election_results: Vec<ElectLeadersResponseReplicaElectionResults0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResults0 {
    pub topic: String,
    pub partition_result: Vec<ElectLeadersResponseReplicaElectionResultsPartitionResult0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResultsPartitionResult0 {
    pub partition_id: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
}

#[derive(Default, Debug, FromBytes)]
pub struct ElectLeadersResponse1 {
    pub throttle_time_ms: Int32,
    pub error_code: Optional<Int16>,
    pub replica_election_results: Vec<ElectLeadersResponseReplicaElectionResults1>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResults1 {
    pub topic: String,
    pub partition_result: Vec<ElectLeadersResponseReplicaElectionResultsPartitionResult1>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResultsPartitionResult1 {
    pub partition_id: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
}

#[derive(Default, Debug, FromBytes)]
pub struct ElectLeadersResponse2 {
    pub throttle_time_ms: Int32,
    pub error_code: Optional<Int16>,
    pub replica_election_results: Vec<ElectLeadersResponseReplicaElectionResults2>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResults2 {
    pub topic: CompactString,
    pub partition_result: Vec<ElectLeadersResponseReplicaElectionResultsPartitionResult2>,
}

#[derive(Default, Debug, FromBytes)]
pub struct ElectLeadersResponseReplicaElectionResultsPartitionResult2 {
    pub partition_id: Int32,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
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
        Ok(ElectLeadersRequestTopicPartitions0 {
            topic: latest.topic.into(),
            partition_id: latest.partition_id,
        })
    }
}

impl TryFrom<ElectLeadersRequest2> for ElectLeadersRequest1 {
    type Error = Error;
    fn try_from(latest: ElectLeadersRequest2) -> Result<Self, Self::Error> {
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
        }
    }
}
