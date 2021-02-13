use super::prelude::*;

pub type TxnOffsetCommitRequest = TxnOffsetCommitRequest3;
pub type TxnOffsetCommitResponse = TxnOffsetCommitResponse3;
impl ApiCall for TxnOffsetCommitRequest {
    type Response = TxnOffsetCommitResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::TxnOffsetCommit
    }
    fn get_first_error(response: &TxnOffsetCommitResponse) -> Option<ApiError> {
        TxnOffsetCommitResponse::get_first_error(response)
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                TxnOffsetCommitRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                TxnOffsetCommitRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &TxnOffsetCommitRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &TxnOffsetCommitRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &TxnOffsetCommitRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, TxnOffsetCommitResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => TxnOffsetCommitResponse0::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            1 => TxnOffsetCommitResponse1::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            2 => TxnOffsetCommitResponse2::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            3 => TxnOffsetCommitResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => TxnOffsetCommitResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct TxnOffsetCommitRequest0 {
    pub transactional_id: String,
    pub group_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub topics: Vec<TxnOffsetCommitRequestTopics0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct TxnOffsetCommitRequestTopics0 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitRequestTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct TxnOffsetCommitRequestTopicsPartitions0 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct TxnOffsetCommitRequest1 {
    pub transactional_id: String,
    pub group_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub topics: Vec<TxnOffsetCommitRequestTopics1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct TxnOffsetCommitRequestTopics1 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitRequestTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct TxnOffsetCommitRequestTopicsPartitions1 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct TxnOffsetCommitRequest2 {
    pub transactional_id: String,
    pub group_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub topics: Vec<TxnOffsetCommitRequestTopics2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct TxnOffsetCommitRequestTopics2 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitRequestTopicsPartitions2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct TxnOffsetCommitRequestTopicsPartitions2 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Int32,
    pub committed_metadata: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct TxnOffsetCommitRequest3 {
    pub transactional_id: String,
    pub group_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub generation_id: Int32,
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub topics: Vec<TxnOffsetCommitRequestTopics3>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct TxnOffsetCommitRequestTopics3 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitRequestTopicsPartitions3>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct TxnOffsetCommitRequestTopicsPartitions3 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Int32,
    pub committed_metadata: NullableString,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct TxnOffsetCommitResponse0 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<TxnOffsetCommitResponseTopics0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct TxnOffsetCommitResponseTopics0 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitResponseTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct TxnOffsetCommitResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct TxnOffsetCommitResponse1 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<TxnOffsetCommitResponseTopics1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct TxnOffsetCommitResponseTopics1 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitResponseTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct TxnOffsetCommitResponseTopicsPartitions1 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct TxnOffsetCommitResponse2 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<TxnOffsetCommitResponseTopics2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct TxnOffsetCommitResponseTopics2 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitResponseTopicsPartitions2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct TxnOffsetCommitResponseTopicsPartitions2 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct TxnOffsetCommitResponse3 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<TxnOffsetCommitResponseTopics3>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct TxnOffsetCommitResponseTopics3 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitResponseTopicsPartitions3>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct TxnOffsetCommitResponseTopicsPartitions3 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<TxnOffsetCommitRequest3> for TxnOffsetCommitRequest0 {
    fn from(latest: TxnOffsetCommitRequest3) -> TxnOffsetCommitRequest0 {
        log::debug!("Using old api format - TxnOffsetCommitRequest0, ignoring field generation_id");
        TxnOffsetCommitRequest0 {
            transactional_id: latest.transactional_id,
            group_id: latest.group_id,
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<TxnOffsetCommitRequestTopics3> for TxnOffsetCommitRequestTopics0 {
    fn from(latest: TxnOffsetCommitRequestTopics3) -> TxnOffsetCommitRequestTopics0 {
        TxnOffsetCommitRequestTopics0 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<TxnOffsetCommitRequestTopicsPartitions3> for TxnOffsetCommitRequestTopicsPartitions0 {
    fn from(
        latest: TxnOffsetCommitRequestTopicsPartitions3,
    ) -> TxnOffsetCommitRequestTopicsPartitions0 {
        log::debug!("Using old api format - TxnOffsetCommitRequestTopicsPartitions0, ignoring field committed_leader_epoch");
        TxnOffsetCommitRequestTopicsPartitions0 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        }
    }
}

impl From<TxnOffsetCommitRequest3> for TxnOffsetCommitRequest1 {
    fn from(latest: TxnOffsetCommitRequest3) -> TxnOffsetCommitRequest1 {
        log::debug!("Using old api format - TxnOffsetCommitRequest1, ignoring field generation_id");
        TxnOffsetCommitRequest1 {
            transactional_id: latest.transactional_id,
            group_id: latest.group_id,
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<TxnOffsetCommitRequestTopics3> for TxnOffsetCommitRequestTopics1 {
    fn from(latest: TxnOffsetCommitRequestTopics3) -> TxnOffsetCommitRequestTopics1 {
        TxnOffsetCommitRequestTopics1 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<TxnOffsetCommitRequestTopicsPartitions3> for TxnOffsetCommitRequestTopicsPartitions1 {
    fn from(
        latest: TxnOffsetCommitRequestTopicsPartitions3,
    ) -> TxnOffsetCommitRequestTopicsPartitions1 {
        log::debug!("Using old api format - TxnOffsetCommitRequestTopicsPartitions1, ignoring field committed_leader_epoch");
        TxnOffsetCommitRequestTopicsPartitions1 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        }
    }
}

impl From<TxnOffsetCommitRequest3> for TxnOffsetCommitRequest2 {
    fn from(latest: TxnOffsetCommitRequest3) -> TxnOffsetCommitRequest2 {
        log::debug!("Using old api format - TxnOffsetCommitRequest2, ignoring field generation_id");
        TxnOffsetCommitRequest2 {
            transactional_id: latest.transactional_id,
            group_id: latest.group_id,
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<TxnOffsetCommitRequestTopics3> for TxnOffsetCommitRequestTopics2 {
    fn from(latest: TxnOffsetCommitRequestTopics3) -> TxnOffsetCommitRequestTopics2 {
        TxnOffsetCommitRequestTopics2 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<TxnOffsetCommitRequestTopicsPartitions3> for TxnOffsetCommitRequestTopicsPartitions2 {
    fn from(
        latest: TxnOffsetCommitRequestTopicsPartitions3,
    ) -> TxnOffsetCommitRequestTopicsPartitions2 {
        TxnOffsetCommitRequestTopicsPartitions2 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_leader_epoch: latest.committed_leader_epoch,
            committed_metadata: latest.committed_metadata,
        }
    }
}

impl From<TxnOffsetCommitResponse0> for TxnOffsetCommitResponse3 {
    fn from(older: TxnOffsetCommitResponse0) -> Self {
        TxnOffsetCommitResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..TxnOffsetCommitResponse3::default()
        }
    }
}

impl From<TxnOffsetCommitResponseTopics0> for TxnOffsetCommitResponseTopics3 {
    fn from(older: TxnOffsetCommitResponseTopics0) -> Self {
        TxnOffsetCommitResponseTopics3 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..TxnOffsetCommitResponseTopics3::default()
        }
    }
}

impl From<TxnOffsetCommitResponseTopicsPartitions0> for TxnOffsetCommitResponseTopicsPartitions3 {
    fn from(older: TxnOffsetCommitResponseTopicsPartitions0) -> Self {
        TxnOffsetCommitResponseTopicsPartitions3 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..TxnOffsetCommitResponseTopicsPartitions3::default()
        }
    }
}

impl From<TxnOffsetCommitResponse1> for TxnOffsetCommitResponse3 {
    fn from(older: TxnOffsetCommitResponse1) -> Self {
        TxnOffsetCommitResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..TxnOffsetCommitResponse3::default()
        }
    }
}

impl From<TxnOffsetCommitResponseTopics1> for TxnOffsetCommitResponseTopics3 {
    fn from(older: TxnOffsetCommitResponseTopics1) -> Self {
        TxnOffsetCommitResponseTopics3 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..TxnOffsetCommitResponseTopics3::default()
        }
    }
}

impl From<TxnOffsetCommitResponseTopicsPartitions1> for TxnOffsetCommitResponseTopicsPartitions3 {
    fn from(older: TxnOffsetCommitResponseTopicsPartitions1) -> Self {
        TxnOffsetCommitResponseTopicsPartitions3 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..TxnOffsetCommitResponseTopicsPartitions3::default()
        }
    }
}

impl From<TxnOffsetCommitResponse2> for TxnOffsetCommitResponse3 {
    fn from(older: TxnOffsetCommitResponse2) -> Self {
        TxnOffsetCommitResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..TxnOffsetCommitResponse3::default()
        }
    }
}

impl From<TxnOffsetCommitResponseTopics2> for TxnOffsetCommitResponseTopics3 {
    fn from(older: TxnOffsetCommitResponseTopics2) -> Self {
        TxnOffsetCommitResponseTopics3 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..TxnOffsetCommitResponseTopics3::default()
        }
    }
}

impl From<TxnOffsetCommitResponseTopicsPartitions2> for TxnOffsetCommitResponseTopicsPartitions3 {
    fn from(older: TxnOffsetCommitResponseTopicsPartitions2) -> Self {
        TxnOffsetCommitResponseTopicsPartitions3 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..TxnOffsetCommitResponseTopicsPartitions3::default()
        }
    }
}

impl TxnOffsetCommitResponse3 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.topics.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl TxnOffsetCommitResponseTopics3 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partitions.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl TxnOffsetCommitResponseTopicsPartitions3 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
