use super::prelude::*;

pub type OffsetCommitRequest = OffsetCommitRequest8;
pub type OffsetCommitResponse = OffsetCommitResponse8;
impl ApiCall for OffsetCommitRequest {
    type Response = OffsetCommitResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        8
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::OffsetCommit
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => false,
            5 => false,
            6 => false,
            7 => false,
            8 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                OffsetCommitRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                OffsetCommitRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &OffsetCommitRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &OffsetCommitRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &OffsetCommitRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &OffsetCommitRequest3::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(
                &OffsetCommitRequest4::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            5 => ToBytes::serialize(
                &OffsetCommitRequest5::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            6 => ToBytes::serialize(
                &OffsetCommitRequest6::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            7 => ToBytes::serialize(
                &OffsetCommitRequest7::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            8 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, OffsetCommitResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => OffsetCommitResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => OffsetCommitResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => OffsetCommitResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => OffsetCommitResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => OffsetCommitResponse4::deserialize(buf, Self::is_flexible_version(version)).into(),
            5 => OffsetCommitResponse5::deserialize(buf, Self::is_flexible_version(version)).into(),
            6 => OffsetCommitResponse6::deserialize(buf, Self::is_flexible_version(version)).into(),
            7 => OffsetCommitResponse7::deserialize(buf, Self::is_flexible_version(version)).into(),
            8 => OffsetCommitResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => OffsetCommitResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequest0 {
    pub group_id: String,
    pub topics: Vec<OffsetCommitRequestTopics0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopics0 {
    pub name: String,
    pub partitions: Vec<OffsetCommitRequestTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions0 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequest1 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub topics: Vec<OffsetCommitRequestTopics1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopics1 {
    pub name: String,
    pub partitions: Vec<OffsetCommitRequestTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions1 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub commit_timestamp: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequest2 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub retention_time_ms: Int64,
    pub topics: Vec<OffsetCommitRequestTopics2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopics2 {
    pub name: String,
    pub partitions: Vec<OffsetCommitRequestTopicsPartitions2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions2 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequest3 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub retention_time_ms: Int64,
    pub topics: Vec<OffsetCommitRequestTopics3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopics3 {
    pub name: String,
    pub partitions: Vec<OffsetCommitRequestTopicsPartitions3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions3 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequest4 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub retention_time_ms: Int64,
    pub topics: Vec<OffsetCommitRequestTopics4>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopics4 {
    pub name: String,
    pub partitions: Vec<OffsetCommitRequestTopicsPartitions4>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions4 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequest5 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub topics: Vec<OffsetCommitRequestTopics5>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopics5 {
    pub name: String,
    pub partitions: Vec<OffsetCommitRequestTopicsPartitions5>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions5 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequest6 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub topics: Vec<OffsetCommitRequestTopics6>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopics6 {
    pub name: String,
    pub partitions: Vec<OffsetCommitRequestTopicsPartitions6>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions6 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Int32,
    pub committed_metadata: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequest7 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub topics: Vec<OffsetCommitRequestTopics7>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopics7 {
    pub name: String,
    pub partitions: Vec<OffsetCommitRequestTopicsPartitions7>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions7 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Int32,
    pub committed_metadata: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequest8 {
    pub group_id: String,
    pub generation_id: Int32,
    pub member_id: String,
    pub group_instance_id: NullableString,
    pub topics: Vec<OffsetCommitRequestTopics8>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopics8 {
    pub name: String,
    pub partitions: Vec<OffsetCommitRequestTopicsPartitions8>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions8 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Int32,
    pub committed_metadata: NullableString,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponse0 {
    pub topics: Vec<OffsetCommitResponseTopics0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopics0 {
    pub name: String,
    pub partitions: Vec<OffsetCommitResponseTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponse1 {
    pub topics: Vec<OffsetCommitResponseTopics1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopics1 {
    pub name: String,
    pub partitions: Vec<OffsetCommitResponseTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions1 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponse2 {
    pub topics: Vec<OffsetCommitResponseTopics2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopics2 {
    pub name: String,
    pub partitions: Vec<OffsetCommitResponseTopicsPartitions2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions2 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponse3 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<OffsetCommitResponseTopics3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopics3 {
    pub name: String,
    pub partitions: Vec<OffsetCommitResponseTopicsPartitions3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions3 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponse4 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<OffsetCommitResponseTopics4>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopics4 {
    pub name: String,
    pub partitions: Vec<OffsetCommitResponseTopicsPartitions4>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions4 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponse5 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<OffsetCommitResponseTopics5>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopics5 {
    pub name: String,
    pub partitions: Vec<OffsetCommitResponseTopicsPartitions5>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions5 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponse6 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<OffsetCommitResponseTopics6>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopics6 {
    pub name: String,
    pub partitions: Vec<OffsetCommitResponseTopicsPartitions6>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions6 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponse7 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<OffsetCommitResponseTopics7>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopics7 {
    pub name: String,
    pub partitions: Vec<OffsetCommitResponseTopicsPartitions7>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions7 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponse8 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<OffsetCommitResponseTopics8>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopics8 {
    pub name: String,
    pub partitions: Vec<OffsetCommitResponseTopicsPartitions8>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions8 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<OffsetCommitRequest8> for OffsetCommitRequest0 {
    fn from(latest: OffsetCommitRequest8) -> OffsetCommitRequest0 {
        log::debug!("Using old api format - OffsetCommitRequest0, ignoring field generation_id");
        OffsetCommitRequest0 {
            group_id: latest.group_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics0 {
    fn from(latest: OffsetCommitRequestTopics8) -> OffsetCommitRequestTopics0 {
        OffsetCommitRequestTopics0 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions0 {
    fn from(latest: OffsetCommitRequestTopicsPartitions8) -> OffsetCommitRequestTopicsPartitions0 {
        log::debug!("Using old api format - OffsetCommitRequestTopicsPartitions0, ignoring field committed_leader_epoch");
        OffsetCommitRequestTopicsPartitions0 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        }
    }
}

impl From<OffsetCommitRequest8> for OffsetCommitRequest1 {
    fn from(latest: OffsetCommitRequest8) -> OffsetCommitRequest1 {
        OffsetCommitRequest1 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics1 {
    fn from(latest: OffsetCommitRequestTopics8) -> OffsetCommitRequestTopics1 {
        OffsetCommitRequestTopics1 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions1 {
    fn from(latest: OffsetCommitRequestTopicsPartitions8) -> OffsetCommitRequestTopicsPartitions1 {
        log::debug!("Using old api format - OffsetCommitRequestTopicsPartitions1, ignoring field committed_leader_epoch");
        OffsetCommitRequestTopicsPartitions1 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
            ..OffsetCommitRequestTopicsPartitions1::default()
        }
    }
}

impl From<OffsetCommitRequest8> for OffsetCommitRequest2 {
    fn from(latest: OffsetCommitRequest8) -> OffsetCommitRequest2 {
        OffsetCommitRequest2 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            ..OffsetCommitRequest2::default()
        }
    }
}

impl From<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics2 {
    fn from(latest: OffsetCommitRequestTopics8) -> OffsetCommitRequestTopics2 {
        OffsetCommitRequestTopics2 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions2 {
    fn from(latest: OffsetCommitRequestTopicsPartitions8) -> OffsetCommitRequestTopicsPartitions2 {
        log::debug!("Using old api format - OffsetCommitRequestTopicsPartitions2, ignoring field committed_leader_epoch");
        OffsetCommitRequestTopicsPartitions2 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        }
    }
}

impl From<OffsetCommitRequest8> for OffsetCommitRequest3 {
    fn from(latest: OffsetCommitRequest8) -> OffsetCommitRequest3 {
        OffsetCommitRequest3 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            ..OffsetCommitRequest3::default()
        }
    }
}

impl From<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics3 {
    fn from(latest: OffsetCommitRequestTopics8) -> OffsetCommitRequestTopics3 {
        OffsetCommitRequestTopics3 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions3 {
    fn from(latest: OffsetCommitRequestTopicsPartitions8) -> OffsetCommitRequestTopicsPartitions3 {
        log::debug!("Using old api format - OffsetCommitRequestTopicsPartitions3, ignoring field committed_leader_epoch");
        OffsetCommitRequestTopicsPartitions3 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        }
    }
}

impl From<OffsetCommitRequest8> for OffsetCommitRequest4 {
    fn from(latest: OffsetCommitRequest8) -> OffsetCommitRequest4 {
        OffsetCommitRequest4 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            ..OffsetCommitRequest4::default()
        }
    }
}

impl From<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics4 {
    fn from(latest: OffsetCommitRequestTopics8) -> OffsetCommitRequestTopics4 {
        OffsetCommitRequestTopics4 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions4 {
    fn from(latest: OffsetCommitRequestTopicsPartitions8) -> OffsetCommitRequestTopicsPartitions4 {
        log::debug!("Using old api format - OffsetCommitRequestTopicsPartitions4, ignoring field committed_leader_epoch");
        OffsetCommitRequestTopicsPartitions4 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        }
    }
}

impl From<OffsetCommitRequest8> for OffsetCommitRequest5 {
    fn from(latest: OffsetCommitRequest8) -> OffsetCommitRequest5 {
        OffsetCommitRequest5 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics5 {
    fn from(latest: OffsetCommitRequestTopics8) -> OffsetCommitRequestTopics5 {
        OffsetCommitRequestTopics5 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions5 {
    fn from(latest: OffsetCommitRequestTopicsPartitions8) -> OffsetCommitRequestTopicsPartitions5 {
        log::debug!("Using old api format - OffsetCommitRequestTopicsPartitions5, ignoring field committed_leader_epoch");
        OffsetCommitRequestTopicsPartitions5 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        }
    }
}

impl From<OffsetCommitRequest8> for OffsetCommitRequest6 {
    fn from(latest: OffsetCommitRequest8) -> OffsetCommitRequest6 {
        OffsetCommitRequest6 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics6 {
    fn from(latest: OffsetCommitRequestTopics8) -> OffsetCommitRequestTopics6 {
        OffsetCommitRequestTopics6 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions6 {
    fn from(latest: OffsetCommitRequestTopicsPartitions8) -> OffsetCommitRequestTopicsPartitions6 {
        OffsetCommitRequestTopicsPartitions6 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_leader_epoch: latest.committed_leader_epoch,
            committed_metadata: latest.committed_metadata,
        }
    }
}

impl From<OffsetCommitRequest8> for OffsetCommitRequest7 {
    fn from(latest: OffsetCommitRequest8) -> OffsetCommitRequest7 {
        OffsetCommitRequest7 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            group_instance_id: latest.group_instance_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics7 {
    fn from(latest: OffsetCommitRequestTopics8) -> OffsetCommitRequestTopics7 {
        OffsetCommitRequestTopics7 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions7 {
    fn from(latest: OffsetCommitRequestTopicsPartitions8) -> OffsetCommitRequestTopicsPartitions7 {
        OffsetCommitRequestTopicsPartitions7 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_leader_epoch: latest.committed_leader_epoch,
            committed_metadata: latest.committed_metadata,
        }
    }
}

impl From<OffsetCommitResponse0> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse0) -> Self {
        OffsetCommitResponse8 {
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponse8::default()
        }
    }
}

impl From<OffsetCommitResponseTopics0> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics0) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponseTopics8::default()
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions0> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions0) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..OffsetCommitResponseTopicsPartitions8::default()
        }
    }
}

impl From<OffsetCommitResponse1> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse1) -> Self {
        OffsetCommitResponse8 {
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponse8::default()
        }
    }
}

impl From<OffsetCommitResponseTopics1> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics1) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponseTopics8::default()
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions1> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions1) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..OffsetCommitResponseTopicsPartitions8::default()
        }
    }
}

impl From<OffsetCommitResponse2> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse2) -> Self {
        OffsetCommitResponse8 {
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponse8::default()
        }
    }
}

impl From<OffsetCommitResponseTopics2> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics2) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponseTopics8::default()
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions2> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions2) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..OffsetCommitResponseTopicsPartitions8::default()
        }
    }
}

impl From<OffsetCommitResponse3> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse3) -> Self {
        OffsetCommitResponse8 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponse8::default()
        }
    }
}

impl From<OffsetCommitResponseTopics3> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics3) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponseTopics8::default()
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions3> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions3) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..OffsetCommitResponseTopicsPartitions8::default()
        }
    }
}

impl From<OffsetCommitResponse4> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse4) -> Self {
        OffsetCommitResponse8 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponse8::default()
        }
    }
}

impl From<OffsetCommitResponseTopics4> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics4) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponseTopics8::default()
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions4> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions4) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..OffsetCommitResponseTopicsPartitions8::default()
        }
    }
}

impl From<OffsetCommitResponse5> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse5) -> Self {
        OffsetCommitResponse8 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponse8::default()
        }
    }
}

impl From<OffsetCommitResponseTopics5> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics5) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponseTopics8::default()
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions5> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions5) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..OffsetCommitResponseTopicsPartitions8::default()
        }
    }
}

impl From<OffsetCommitResponse6> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse6) -> Self {
        OffsetCommitResponse8 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponse8::default()
        }
    }
}

impl From<OffsetCommitResponseTopics6> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics6) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponseTopics8::default()
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions6> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions6) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..OffsetCommitResponseTopicsPartitions8::default()
        }
    }
}

impl From<OffsetCommitResponse7> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse7) -> Self {
        OffsetCommitResponse8 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponse8::default()
        }
    }
}

impl From<OffsetCommitResponseTopics7> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics7) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..OffsetCommitResponseTopics8::default()
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions7> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions7) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..OffsetCommitResponseTopicsPartitions8::default()
        }
    }
}
