use super::prelude::*;

pub type StopReplicaRequest = StopReplicaRequest3;
pub type StopReplicaResponse = StopReplicaResponse3;
pub fn serialize_stop_replica_request(
    data: StopReplicaRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&StopReplicaRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&StopReplicaRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&StopReplicaRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_stop_replica_response(version: i32, buf: &mut Bytes) -> StopReplicaResponse {
    match version {
        0 => StopReplicaResponse0::deserialize(buf).into(),
        1 => StopReplicaResponse1::deserialize(buf).into(),
        2 => StopReplicaResponse2::deserialize(buf).into(),
        3 => StopReplicaResponse::deserialize(buf),
        _ => StopReplicaResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct StopReplicaRequest0 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub delete_partitions: Boolean,
    pub ungrouped_partitions: Vec<StopReplicaRequestUngroupedPartitions0>,
}

#[derive(Default, ToBytes)]
pub struct StopReplicaRequestUngroupedPartitions0 {
    pub topic_name: String,
    pub partition_index: Int32,
}

#[derive(Default, ToBytes)]
pub struct StopReplicaRequest1 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Optional<Int64>,
    pub delete_partitions: Boolean,
    pub topics: Optional<Vec<StopReplicaRequestTopics1>>,
}

#[derive(Default, ToBytes)]
pub struct StopReplicaRequestTopics1 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct StopReplicaRequest2 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Optional<Int64>,
    pub delete_partitions: Boolean,
    pub topics: Optional<Vec<StopReplicaRequestTopics2>>,
}

#[derive(Default, ToBytes)]
pub struct StopReplicaRequestTopics2 {
    pub name: CompactString,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct StopReplicaRequest3 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Optional<Int64>,
    pub topic_states: Optional<Vec<StopReplicaRequestTopicStates3>>,
}

#[derive(Default, ToBytes)]
pub struct StopReplicaRequestTopicStates3 {
    pub topic_name: CompactString,
    pub partition_states: Vec<StopReplicaRequestTopicStatesPartitionStates3>,
}

#[derive(Default, ToBytes)]
pub struct StopReplicaRequestTopicStatesPartitionStates3 {
    pub partition_index: Int32,
    pub leader_epoch: Int32,
    pub delete_partition: Boolean,
}

#[derive(Default, FromBytes)]
pub struct StopReplicaResponse0 {
    pub error_code: Int16,
    pub partition_errors: Vec<StopReplicaResponsePartitionErrors0>,
}

#[derive(Default, FromBytes)]
pub struct StopReplicaResponsePartitionErrors0 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct StopReplicaResponse1 {
    pub error_code: Int16,
    pub partition_errors: Vec<StopReplicaResponsePartitionErrors1>,
}

#[derive(Default, FromBytes)]
pub struct StopReplicaResponsePartitionErrors1 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct StopReplicaResponse2 {
    pub error_code: Int16,
    pub partition_errors: Vec<StopReplicaResponsePartitionErrors2>,
}

#[derive(Default, FromBytes)]
pub struct StopReplicaResponsePartitionErrors2 {
    pub topic_name: CompactString,
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct StopReplicaResponse3 {
    pub error_code: Int16,
    pub partition_errors: Vec<StopReplicaResponsePartitionErrors3>,
}

#[derive(Default, FromBytes)]
pub struct StopReplicaResponsePartitionErrors3 {
    pub topic_name: CompactString,
    pub partition_index: Int32,
    pub error_code: Int16,
}

impl TryFrom<StopReplicaRequest3> for StopReplicaRequest0 {
    type Error = Error;
    fn try_from(latest: StopReplicaRequest3) -> Result<Self, Self::Error> {
        if latest.broker_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "StopReplicaRequest",
                0,
                "broker_epoch",
            ));
        }
        if latest.topic_states.is_some() {
            return Err(Error::OldKafkaVersion(
                "StopReplicaRequest",
                0,
                "topic_states",
            ));
        }
        Ok(StopReplicaRequest0 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            ..StopReplicaRequest0::default()
        })
    }
}

impl TryFrom<StopReplicaRequest3> for StopReplicaRequest1 {
    type Error = Error;
    fn try_from(latest: StopReplicaRequest3) -> Result<Self, Self::Error> {
        if latest.topic_states.is_some() {
            return Err(Error::OldKafkaVersion(
                "StopReplicaRequest",
                1,
                "topic_states",
            ));
        }
        Ok(StopReplicaRequest1 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            broker_epoch: latest.broker_epoch,
            ..StopReplicaRequest1::default()
        })
    }
}

impl TryFrom<StopReplicaRequest3> for StopReplicaRequest2 {
    type Error = Error;
    fn try_from(latest: StopReplicaRequest3) -> Result<Self, Self::Error> {
        if latest.topic_states.is_some() {
            return Err(Error::OldKafkaVersion(
                "StopReplicaRequest",
                2,
                "topic_states",
            ));
        }
        Ok(StopReplicaRequest2 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            broker_epoch: latest.broker_epoch,
            ..StopReplicaRequest2::default()
        })
    }
}

impl From<StopReplicaResponse0> for StopReplicaResponse3 {
    fn from(older: StopReplicaResponse0) -> Self {
        StopReplicaResponse3 {
            error_code: older.error_code,
            partition_errors: older
                .partition_errors
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<StopReplicaResponsePartitionErrors0> for StopReplicaResponsePartitionErrors3 {
    fn from(older: StopReplicaResponsePartitionErrors0) -> Self {
        StopReplicaResponsePartitionErrors3 {
            topic_name: older.topic_name.into(),
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}

impl From<StopReplicaResponse1> for StopReplicaResponse3 {
    fn from(older: StopReplicaResponse1) -> Self {
        StopReplicaResponse3 {
            error_code: older.error_code,
            partition_errors: older
                .partition_errors
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<StopReplicaResponsePartitionErrors1> for StopReplicaResponsePartitionErrors3 {
    fn from(older: StopReplicaResponsePartitionErrors1) -> Self {
        StopReplicaResponsePartitionErrors3 {
            topic_name: older.topic_name.into(),
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}

impl From<StopReplicaResponse2> for StopReplicaResponse3 {
    fn from(older: StopReplicaResponse2) -> Self {
        StopReplicaResponse3 {
            error_code: older.error_code,
            partition_errors: older
                .partition_errors
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<StopReplicaResponsePartitionErrors2> for StopReplicaResponsePartitionErrors3 {
    fn from(older: StopReplicaResponsePartitionErrors2) -> Self {
        StopReplicaResponsePartitionErrors3 {
            topic_name: older.topic_name,
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}
