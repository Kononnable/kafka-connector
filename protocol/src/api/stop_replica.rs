use super::prelude::*;

pub type StopReplicaRequest = StopReplicaRequest3;
pub type StopReplicaResponse = StopReplicaResponse3;
impl ApiCall for StopReplicaRequest {
    type Response = StopReplicaResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::StopReplica
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&StopReplicaRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&StopReplicaRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&StopReplicaRequest2::try_from(self)?, buf),
            3 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> StopReplicaResponse {
        match version {
            0 => StopReplicaResponse0::deserialize(buf).into(),
            1 => StopReplicaResponse1::deserialize(buf).into(),
            2 => StopReplicaResponse2::deserialize(buf).into(),
            3 => StopReplicaResponse::deserialize(buf),
            _ => StopReplicaResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct StopReplicaRequest0 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub delete_partitions: Boolean,
    pub ungrouped_partitions: Vec<StopReplicaRequestUngroupedPartitions0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct StopReplicaRequestUngroupedPartitions0 {
    pub topic_name: String,
    pub partition_index: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct StopReplicaRequest1 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Optional<Int64>,
    pub delete_partitions: Boolean,
    pub topics: Optional<Vec<StopReplicaRequestTopics1>>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct StopReplicaRequestTopics1 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct StopReplicaRequest2 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Optional<Int64>,
    pub delete_partitions: Boolean,
    pub topics: Optional<Vec<StopReplicaRequestTopics2>>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct StopReplicaRequestTopics2 {
    pub name: CompactString,
    pub partition_indexes: Vec<Int32>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct StopReplicaRequest3 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Optional<Int64>,
    pub topic_states: Optional<Vec<StopReplicaRequestTopicStates3>>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct StopReplicaRequestTopicStates3 {
    pub topic_name: CompactString,
    pub partition_states: Vec<StopReplicaRequestTopicStatesPartitionStates3>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct StopReplicaRequestTopicStatesPartitionStates3 {
    pub partition_index: Int32,
    pub leader_epoch: Int32,
    pub delete_partition: Boolean,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponse0 {
    pub error_code: Int16,
    pub partition_errors: Vec<StopReplicaResponsePartitionErrors0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponsePartitionErrors0 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponse1 {
    pub error_code: Int16,
    pub partition_errors: Vec<StopReplicaResponsePartitionErrors1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponsePartitionErrors1 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponse2 {
    pub error_code: Int16,
    pub partition_errors: Vec<StopReplicaResponsePartitionErrors2>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponsePartitionErrors2 {
    pub topic_name: CompactString,
    pub partition_index: Int32,
    pub error_code: Int16,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponse3 {
    pub error_code: Int16,
    pub partition_errors: Vec<StopReplicaResponsePartitionErrors3>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponsePartitionErrors3 {
    pub topic_name: CompactString,
    pub partition_index: Int32,
    pub error_code: Int16,
    pub tag_buffer: Optional<TagBuffer>,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "StopReplicaRequest",
                0,
                "tag_buffer",
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "StopReplicaRequest",
                1,
                "tag_buffer",
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
            tag_buffer: latest.tag_buffer,
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
            ..StopReplicaResponse3::default()
        }
    }
}

impl From<StopReplicaResponsePartitionErrors0> for StopReplicaResponsePartitionErrors3 {
    fn from(older: StopReplicaResponsePartitionErrors0) -> Self {
        StopReplicaResponsePartitionErrors3 {
            topic_name: older.topic_name.into(),
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..StopReplicaResponsePartitionErrors3::default()
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
            ..StopReplicaResponse3::default()
        }
    }
}

impl From<StopReplicaResponsePartitionErrors1> for StopReplicaResponsePartitionErrors3 {
    fn from(older: StopReplicaResponsePartitionErrors1) -> Self {
        StopReplicaResponsePartitionErrors3 {
            topic_name: older.topic_name.into(),
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..StopReplicaResponsePartitionErrors3::default()
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
            tag_buffer: older.tag_buffer,
        }
    }
}

impl From<StopReplicaResponsePartitionErrors2> for StopReplicaResponsePartitionErrors3 {
    fn from(older: StopReplicaResponsePartitionErrors2) -> Self {
        StopReplicaResponsePartitionErrors3 {
            topic_name: older.topic_name,
            partition_index: older.partition_index,
            error_code: older.error_code,
            tag_buffer: older.tag_buffer,
        }
    }
}
