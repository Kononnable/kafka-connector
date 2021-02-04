use super::prelude::*;

pub type OffsetFetchRequest = OffsetFetchRequest7;
pub type OffsetFetchResponse = OffsetFetchResponse7;
impl ApiCall for OffsetFetchRequest {
    type Response = OffsetFetchResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        7
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::OffsetFetch
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => false,
            5 => false,
            6 => true,
            7 => true,
            _ => true,
        }
    }
    fn serialize(
        self,
        version: i16,
        buf: &mut BytesMut,
        correlation_id: i32,
        client_id: &str,
    ) -> Result<(), Error> {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                OffsetFetchRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                OffsetFetchRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &OffsetFetchRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &OffsetFetchRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &OffsetFetchRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &OffsetFetchRequest3::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(
                &OffsetFetchRequest4::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            5 => ToBytes::serialize(
                &OffsetFetchRequest5::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            6 => ToBytes::serialize(
                &OffsetFetchRequest6::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            7 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, OffsetFetchResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => OffsetFetchResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => OffsetFetchResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => OffsetFetchResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => OffsetFetchResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => OffsetFetchResponse4::deserialize(buf, Self::is_flexible_version(version)).into(),
            5 => OffsetFetchResponse5::deserialize(buf, Self::is_flexible_version(version)).into(),
            6 => OffsetFetchResponse6::deserialize(buf, Self::is_flexible_version(version)).into(),
            7 => OffsetFetchResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => OffsetFetchResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequest0 {
    pub group_id: String,
    pub topics: Vec<OffsetFetchRequestTopics0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequestTopics0 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequest1 {
    pub group_id: String,
    pub topics: Vec<OffsetFetchRequestTopics1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequestTopics1 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequest2 {
    pub group_id: String,
    pub topics: Vec<OffsetFetchRequestTopics2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequestTopics2 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequest3 {
    pub group_id: String,
    pub topics: Vec<OffsetFetchRequestTopics3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequestTopics3 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequest4 {
    pub group_id: String,
    pub topics: Vec<OffsetFetchRequestTopics4>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequestTopics4 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequest5 {
    pub group_id: String,
    pub topics: Vec<OffsetFetchRequestTopics5>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequestTopics5 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequest6 {
    pub group_id: String,
    pub topics: Vec<OffsetFetchRequestTopics6>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequestTopics6 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequest7 {
    pub group_id: String,
    pub topics: Vec<OffsetFetchRequestTopics7>,
    pub require_stable: Boolean,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequestTopics7 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponse0 {
    pub topics: Vec<OffsetFetchResponseTopics0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopics0 {
    pub name: String,
    pub partitions: Vec<OffsetFetchResponseTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub metadata: NullableString,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponse1 {
    pub topics: Vec<OffsetFetchResponseTopics1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopics1 {
    pub name: String,
    pub partitions: Vec<OffsetFetchResponseTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions1 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub metadata: NullableString,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponse2 {
    pub topics: Vec<OffsetFetchResponseTopics2>,
    pub error_code: Option<Int16>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopics2 {
    pub name: String,
    pub partitions: Vec<OffsetFetchResponseTopicsPartitions2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions2 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub metadata: NullableString,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponse3 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<OffsetFetchResponseTopics3>,
    pub error_code: Option<Int16>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopics3 {
    pub name: String,
    pub partitions: Vec<OffsetFetchResponseTopicsPartitions3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions3 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub metadata: NullableString,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponse4 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<OffsetFetchResponseTopics4>,
    pub error_code: Option<Int16>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopics4 {
    pub name: String,
    pub partitions: Vec<OffsetFetchResponseTopicsPartitions4>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions4 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub metadata: NullableString,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponse5 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<OffsetFetchResponseTopics5>,
    pub error_code: Option<Int16>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopics5 {
    pub name: String,
    pub partitions: Vec<OffsetFetchResponseTopicsPartitions5>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions5 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Option<Int32>,
    pub metadata: NullableString,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponse6 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<OffsetFetchResponseTopics6>,
    pub error_code: Option<Int16>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopics6 {
    pub name: String,
    pub partitions: Vec<OffsetFetchResponseTopicsPartitions6>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions6 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Option<Int32>,
    pub metadata: NullableString,
    pub error_code: Int16,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponse7 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<OffsetFetchResponseTopics7>,
    pub error_code: Option<Int16>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopics7 {
    pub name: String,
    pub partitions: Vec<OffsetFetchResponseTopicsPartitions7>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions7 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Option<Int32>,
    pub metadata: NullableString,
    pub error_code: Int16,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<OffsetFetchRequest7> for OffsetFetchRequest0 {
    fn from(latest: OffsetFetchRequest7) -> OffsetFetchRequest0 {
        log::debug!("Using old api format - OffsetFetchRequest0, ignoring field require_stable");
        OffsetFetchRequest0 {
            group_id: latest.group_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics0 {
    fn from(latest: OffsetFetchRequestTopics7) -> OffsetFetchRequestTopics0 {
        OffsetFetchRequestTopics0 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
        }
    }
}

impl From<OffsetFetchRequest7> for OffsetFetchRequest1 {
    fn from(latest: OffsetFetchRequest7) -> OffsetFetchRequest1 {
        log::debug!("Using old api format - OffsetFetchRequest1, ignoring field require_stable");
        OffsetFetchRequest1 {
            group_id: latest.group_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics1 {
    fn from(latest: OffsetFetchRequestTopics7) -> OffsetFetchRequestTopics1 {
        OffsetFetchRequestTopics1 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
        }
    }
}

impl From<OffsetFetchRequest7> for OffsetFetchRequest2 {
    fn from(latest: OffsetFetchRequest7) -> OffsetFetchRequest2 {
        log::debug!("Using old api format - OffsetFetchRequest2, ignoring field require_stable");
        OffsetFetchRequest2 {
            group_id: latest.group_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics2 {
    fn from(latest: OffsetFetchRequestTopics7) -> OffsetFetchRequestTopics2 {
        OffsetFetchRequestTopics2 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
        }
    }
}

impl From<OffsetFetchRequest7> for OffsetFetchRequest3 {
    fn from(latest: OffsetFetchRequest7) -> OffsetFetchRequest3 {
        log::debug!("Using old api format - OffsetFetchRequest3, ignoring field require_stable");
        OffsetFetchRequest3 {
            group_id: latest.group_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics3 {
    fn from(latest: OffsetFetchRequestTopics7) -> OffsetFetchRequestTopics3 {
        OffsetFetchRequestTopics3 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
        }
    }
}

impl From<OffsetFetchRequest7> for OffsetFetchRequest4 {
    fn from(latest: OffsetFetchRequest7) -> OffsetFetchRequest4 {
        log::debug!("Using old api format - OffsetFetchRequest4, ignoring field require_stable");
        OffsetFetchRequest4 {
            group_id: latest.group_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics4 {
    fn from(latest: OffsetFetchRequestTopics7) -> OffsetFetchRequestTopics4 {
        OffsetFetchRequestTopics4 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
        }
    }
}

impl From<OffsetFetchRequest7> for OffsetFetchRequest5 {
    fn from(latest: OffsetFetchRequest7) -> OffsetFetchRequest5 {
        log::debug!("Using old api format - OffsetFetchRequest5, ignoring field require_stable");
        OffsetFetchRequest5 {
            group_id: latest.group_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics5 {
    fn from(latest: OffsetFetchRequestTopics7) -> OffsetFetchRequestTopics5 {
        OffsetFetchRequestTopics5 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
        }
    }
}

impl From<OffsetFetchRequest7> for OffsetFetchRequest6 {
    fn from(latest: OffsetFetchRequest7) -> OffsetFetchRequest6 {
        log::debug!("Using old api format - OffsetFetchRequest6, ignoring field require_stable");
        OffsetFetchRequest6 {
            group_id: latest.group_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            tag_buffer: latest.tag_buffer,
        }
    }
}

impl From<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics6 {
    fn from(latest: OffsetFetchRequestTopics7) -> OffsetFetchRequestTopics6 {
        OffsetFetchRequestTopics6 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
            tag_buffer: latest.tag_buffer,
        }
    }
}

impl From<OffsetFetchResponse0> for OffsetFetchResponse7 {
    fn from(older: OffsetFetchResponse0) -> Self {
        OffsetFetchResponse7 {
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..OffsetFetchResponse7::default()
        }
    }
}

impl From<OffsetFetchResponseTopics0> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics0) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetFetchResponseTopics7::default()
        }
    }
}

impl From<OffsetFetchResponseTopicsPartitions0> for OffsetFetchResponseTopicsPartitions7 {
    fn from(older: OffsetFetchResponseTopicsPartitions0) -> Self {
        OffsetFetchResponseTopicsPartitions7 {
            partition_index: older.partition_index,
            committed_offset: older.committed_offset,
            metadata: older.metadata,
            error_code: older.error_code,
            ..OffsetFetchResponseTopicsPartitions7::default()
        }
    }
}

impl From<OffsetFetchResponse1> for OffsetFetchResponse7 {
    fn from(older: OffsetFetchResponse1) -> Self {
        OffsetFetchResponse7 {
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..OffsetFetchResponse7::default()
        }
    }
}

impl From<OffsetFetchResponseTopics1> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics1) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetFetchResponseTopics7::default()
        }
    }
}

impl From<OffsetFetchResponseTopicsPartitions1> for OffsetFetchResponseTopicsPartitions7 {
    fn from(older: OffsetFetchResponseTopicsPartitions1) -> Self {
        OffsetFetchResponseTopicsPartitions7 {
            partition_index: older.partition_index,
            committed_offset: older.committed_offset,
            metadata: older.metadata,
            error_code: older.error_code,
            ..OffsetFetchResponseTopicsPartitions7::default()
        }
    }
}

impl From<OffsetFetchResponse2> for OffsetFetchResponse7 {
    fn from(older: OffsetFetchResponse2) -> Self {
        OffsetFetchResponse7 {
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            error_code: older.error_code,
            ..OffsetFetchResponse7::default()
        }
    }
}

impl From<OffsetFetchResponseTopics2> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics2) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetFetchResponseTopics7::default()
        }
    }
}

impl From<OffsetFetchResponseTopicsPartitions2> for OffsetFetchResponseTopicsPartitions7 {
    fn from(older: OffsetFetchResponseTopicsPartitions2) -> Self {
        OffsetFetchResponseTopicsPartitions7 {
            partition_index: older.partition_index,
            committed_offset: older.committed_offset,
            metadata: older.metadata,
            error_code: older.error_code,
            ..OffsetFetchResponseTopicsPartitions7::default()
        }
    }
}

impl From<OffsetFetchResponse3> for OffsetFetchResponse7 {
    fn from(older: OffsetFetchResponse3) -> Self {
        OffsetFetchResponse7 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            error_code: older.error_code,
            ..OffsetFetchResponse7::default()
        }
    }
}

impl From<OffsetFetchResponseTopics3> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics3) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetFetchResponseTopics7::default()
        }
    }
}

impl From<OffsetFetchResponseTopicsPartitions3> for OffsetFetchResponseTopicsPartitions7 {
    fn from(older: OffsetFetchResponseTopicsPartitions3) -> Self {
        OffsetFetchResponseTopicsPartitions7 {
            partition_index: older.partition_index,
            committed_offset: older.committed_offset,
            metadata: older.metadata,
            error_code: older.error_code,
            ..OffsetFetchResponseTopicsPartitions7::default()
        }
    }
}

impl From<OffsetFetchResponse4> for OffsetFetchResponse7 {
    fn from(older: OffsetFetchResponse4) -> Self {
        OffsetFetchResponse7 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            error_code: older.error_code,
            ..OffsetFetchResponse7::default()
        }
    }
}

impl From<OffsetFetchResponseTopics4> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics4) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetFetchResponseTopics7::default()
        }
    }
}

impl From<OffsetFetchResponseTopicsPartitions4> for OffsetFetchResponseTopicsPartitions7 {
    fn from(older: OffsetFetchResponseTopicsPartitions4) -> Self {
        OffsetFetchResponseTopicsPartitions7 {
            partition_index: older.partition_index,
            committed_offset: older.committed_offset,
            metadata: older.metadata,
            error_code: older.error_code,
            ..OffsetFetchResponseTopicsPartitions7::default()
        }
    }
}

impl From<OffsetFetchResponse5> for OffsetFetchResponse7 {
    fn from(older: OffsetFetchResponse5) -> Self {
        OffsetFetchResponse7 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            error_code: older.error_code,
            ..OffsetFetchResponse7::default()
        }
    }
}

impl From<OffsetFetchResponseTopics5> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics5) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetFetchResponseTopics7::default()
        }
    }
}

impl From<OffsetFetchResponseTopicsPartitions5> for OffsetFetchResponseTopicsPartitions7 {
    fn from(older: OffsetFetchResponseTopicsPartitions5) -> Self {
        OffsetFetchResponseTopicsPartitions7 {
            partition_index: older.partition_index,
            committed_offset: older.committed_offset,
            committed_leader_epoch: older.committed_leader_epoch,
            metadata: older.metadata,
            error_code: older.error_code,
            ..OffsetFetchResponseTopicsPartitions7::default()
        }
    }
}

impl From<OffsetFetchResponse6> for OffsetFetchResponse7 {
    fn from(older: OffsetFetchResponse6) -> Self {
        OffsetFetchResponse7 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            error_code: older.error_code,
            tag_buffer: older.tag_buffer,
        }
    }
}

impl From<OffsetFetchResponseTopics6> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics6) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            tag_buffer: older.tag_buffer,
        }
    }
}

impl From<OffsetFetchResponseTopicsPartitions6> for OffsetFetchResponseTopicsPartitions7 {
    fn from(older: OffsetFetchResponseTopicsPartitions6) -> Self {
        OffsetFetchResponseTopicsPartitions7 {
            partition_index: older.partition_index,
            committed_offset: older.committed_offset,
            committed_leader_epoch: older.committed_leader_epoch,
            metadata: older.metadata,
            error_code: older.error_code,
            tag_buffer: older.tag_buffer,
        }
    }
}
