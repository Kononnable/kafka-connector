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
    fn get_first_error(response: &StopReplicaResponse) -> Option<ApiError> {
        StopReplicaResponse::get_first_error(response)
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => true,
            3 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                StopReplicaRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                StopReplicaRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &StopReplicaRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &StopReplicaRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &StopReplicaRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, StopReplicaResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => StopReplicaResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => StopReplicaResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => StopReplicaResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => StopReplicaResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => StopReplicaResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
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
    pub broker_epoch: Int64,
    pub delete_partitions: Boolean,
    pub topics: Vec<StopReplicaRequestTopics1>,
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
    pub broker_epoch: Int64,
    pub delete_partitions: Boolean,
    pub topics: Vec<StopReplicaRequestTopics2>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct StopReplicaRequestTopics2 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct StopReplicaRequest3 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Int64,
    pub topic_states: Vec<StopReplicaRequestTopicStates3>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct StopReplicaRequestTopicStates3 {
    pub topic_name: String,
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
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponsePartitionErrors2 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponse3 {
    pub error_code: Int16,
    pub partition_errors: Vec<StopReplicaResponsePartitionErrors3>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct StopReplicaResponsePartitionErrors3 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<StopReplicaRequest3> for StopReplicaRequest0 {
    fn from(latest: StopReplicaRequest3) -> StopReplicaRequest0 {
        log::debug!("Using old api format - StopReplicaRequest0, ignoring field broker_epoch");
        log::debug!("Using old api format - StopReplicaRequest0, ignoring field topic_states");
        StopReplicaRequest0 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            ..StopReplicaRequest0::default()
        }
    }
}

impl From<StopReplicaRequest3> for StopReplicaRequest1 {
    fn from(latest: StopReplicaRequest3) -> StopReplicaRequest1 {
        log::debug!("Using old api format - StopReplicaRequest1, ignoring field topic_states");
        StopReplicaRequest1 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            broker_epoch: latest.broker_epoch,
            ..StopReplicaRequest1::default()
        }
    }
}

impl From<StopReplicaRequest3> for StopReplicaRequest2 {
    fn from(latest: StopReplicaRequest3) -> StopReplicaRequest2 {
        log::debug!("Using old api format - StopReplicaRequest2, ignoring field topic_states");
        StopReplicaRequest2 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            broker_epoch: latest.broker_epoch,
            tag_buffer: latest.tag_buffer,
            ..StopReplicaRequest2::default()
        }
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
            topic_name: older.topic_name,
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
            topic_name: older.topic_name,
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

impl StopReplicaResponse3 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partition_errors.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl StopReplicaResponsePartitionErrors3 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
