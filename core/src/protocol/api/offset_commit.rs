use super::prelude::*;

pub type OffsetCommitRequest = OffsetCommitRequest8;
pub type OffsetCommitResponse = OffsetCommitResponse8;
pub fn serialize_offset_commit_request(
    data: OffsetCommitRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&OffsetCommitRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&OffsetCommitRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&OffsetCommitRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&OffsetCommitRequest3::try_from(data)?, buf),
        4 => ToBytes::serialize(&OffsetCommitRequest4::try_from(data)?, buf),
        5 => ToBytes::serialize(&OffsetCommitRequest5::try_from(data)?, buf),
        6 => ToBytes::serialize(&OffsetCommitRequest6::try_from(data)?, buf),
        7 => ToBytes::serialize(&OffsetCommitRequest7::try_from(data)?, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_offset_commit_response<T>(version: i32, buf: &mut T) -> OffsetCommitResponse
where
    T: Iterator<Item = u8>,
{
    match version {
        0 => OffsetCommitResponse0::deserialize(buf).into(),
        1 => OffsetCommitResponse1::deserialize(buf).into(),
        2 => OffsetCommitResponse2::deserialize(buf).into(),
        3 => OffsetCommitResponse3::deserialize(buf).into(),
        4 => OffsetCommitResponse4::deserialize(buf).into(),
        5 => OffsetCommitResponse5::deserialize(buf).into(),
        6 => OffsetCommitResponse6::deserialize(buf).into(),
        7 => OffsetCommitResponse7::deserialize(buf).into(),
        _ => OffsetCommitResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequest0 {
    pub group_id: String,
    pub topics: OffsetCommitRequestTopics0,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopics0 {
    pub name: String,
    pub partitions: OffsetCommitRequestTopicsPartitions0,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions0 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequest1 {
    pub group_id: String,
    pub generation_id: Optional<Int32>,
    pub member_id: Optional<String>,
    pub topics: OffsetCommitRequestTopics1,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopics1 {
    pub name: String,
    pub partitions: OffsetCommitRequestTopicsPartitions1,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions1 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub commit_timestamp: Optional<Int64>,
    pub committed_metadata: NullableString,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequest2 {
    pub group_id: String,
    pub generation_id: Optional<Int32>,
    pub member_id: Optional<String>,
    pub retention_time_ms: Optional<Int64>,
    pub topics: OffsetCommitRequestTopics2,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopics2 {
    pub name: String,
    pub partitions: OffsetCommitRequestTopicsPartitions2,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions2 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequest3 {
    pub group_id: String,
    pub generation_id: Optional<Int32>,
    pub member_id: Optional<String>,
    pub retention_time_ms: Optional<Int64>,
    pub topics: OffsetCommitRequestTopics3,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopics3 {
    pub name: String,
    pub partitions: OffsetCommitRequestTopicsPartitions3,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions3 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequest4 {
    pub group_id: String,
    pub generation_id: Optional<Int32>,
    pub member_id: Optional<String>,
    pub retention_time_ms: Optional<Int64>,
    pub topics: OffsetCommitRequestTopics4,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopics4 {
    pub name: String,
    pub partitions: OffsetCommitRequestTopicsPartitions4,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions4 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequest5 {
    pub group_id: String,
    pub generation_id: Optional<Int32>,
    pub member_id: Optional<String>,
    pub topics: OffsetCommitRequestTopics5,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopics5 {
    pub name: String,
    pub partitions: OffsetCommitRequestTopicsPartitions5,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions5 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequest6 {
    pub group_id: String,
    pub generation_id: Optional<Int32>,
    pub member_id: Optional<String>,
    pub topics: OffsetCommitRequestTopics6,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopics6 {
    pub name: String,
    pub partitions: OffsetCommitRequestTopicsPartitions6,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions6 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Optional<Int32>,
    pub committed_metadata: NullableString,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequest7 {
    pub group_id: String,
    pub generation_id: Optional<Int32>,
    pub member_id: Optional<String>,
    pub group_instance_id: Optional<NullableString>,
    pub topics: OffsetCommitRequestTopics7,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopics7 {
    pub name: String,
    pub partitions: OffsetCommitRequestTopicsPartitions7,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions7 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Optional<Int32>,
    pub committed_metadata: NullableString,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequest8 {
    pub group_id: CompactString,
    pub generation_id: Optional<Int32>,
    pub member_id: Optional<CompactString>,
    pub group_instance_id: Optional<CompactNullableString>,
    pub topics: OffsetCommitRequestTopics8,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopics8 {
    pub name: CompactString,
    pub partitions: OffsetCommitRequestTopicsPartitions8,
}

#[derive(Default, ToBytes)]
pub struct OffsetCommitRequestTopicsPartitions8 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Optional<Int32>,
    pub committed_metadata: CompactNullableString,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponse0 {
    pub topics: OffsetCommitResponseTopics0,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopics0 {
    pub name: String,
    pub partitions: OffsetCommitResponseTopicsPartitions0,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponse1 {
    pub topics: OffsetCommitResponseTopics1,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopics1 {
    pub name: String,
    pub partitions: OffsetCommitResponseTopicsPartitions1,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions1 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponse2 {
    pub topics: OffsetCommitResponseTopics2,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopics2 {
    pub name: String,
    pub partitions: OffsetCommitResponseTopicsPartitions2,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions2 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: OffsetCommitResponseTopics3,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopics3 {
    pub name: String,
    pub partitions: OffsetCommitResponseTopicsPartitions3,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions3 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: OffsetCommitResponseTopics4,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopics4 {
    pub name: String,
    pub partitions: OffsetCommitResponseTopicsPartitions4,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions4 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponse5 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: OffsetCommitResponseTopics5,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopics5 {
    pub name: String,
    pub partitions: OffsetCommitResponseTopicsPartitions5,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions5 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponse6 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: OffsetCommitResponseTopics6,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopics6 {
    pub name: String,
    pub partitions: OffsetCommitResponseTopicsPartitions6,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions6 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponse7 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: OffsetCommitResponseTopics7,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopics7 {
    pub name: String,
    pub partitions: OffsetCommitResponseTopicsPartitions7,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions7 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponse8 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: OffsetCommitResponseTopics8,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopics8 {
    pub name: CompactString,
    pub partitions: OffsetCommitResponseTopicsPartitions8,
}

#[derive(Default, FromBytes)]
pub struct OffsetCommitResponseTopicsPartitions8 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

impl TryFrom<OffsetCommitRequest8> for OffsetCommitRequest0 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequest8) -> Result<Self, Self::Error> {
        if latest.generation_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequest",
                0,
                "generation_id",
            ));
        }
        if latest.member_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequest",
                0,
                "member_id",
            ));
        }
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequest",
                0,
                "group_instance_id",
            ));
        }
        Ok(OffsetCommitRequest0 {
            group_id: latest.group_id,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics0 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopics8) -> Result<Self, Self::Error> {
        Ok(OffsetCommitRequestTopics0 {
            name: latest.name,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions0 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopicsPartitions8) -> Result<Self, Self::Error> {
        if latest.committed_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequestTopicsPartitions",
                0,
                "committed_leader_epoch",
            ));
        }
        Ok(OffsetCommitRequestTopicsPartitions0 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        })
    }
}

impl TryFrom<OffsetCommitRequest8> for OffsetCommitRequest1 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequest8) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequest",
                1,
                "group_instance_id",
            ));
        }
        Ok(OffsetCommitRequest1 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics1 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopics8) -> Result<Self, Self::Error> {
        Ok(OffsetCommitRequestTopics1 {
            name: latest.name,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions1 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopicsPartitions8) -> Result<Self, Self::Error> {
        if latest.committed_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequestTopicsPartitions",
                1,
                "committed_leader_epoch",
            ));
        }
        Ok(OffsetCommitRequestTopicsPartitions1 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
            ..OffsetCommitRequestTopicsPartitions1::default()
        })
    }
}

impl TryFrom<OffsetCommitRequest8> for OffsetCommitRequest2 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequest8) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequest",
                2,
                "group_instance_id",
            ));
        }
        Ok(OffsetCommitRequest2 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            topics: latest.topics.try_into()?,
            ..OffsetCommitRequest2::default()
        })
    }
}

impl TryFrom<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics2 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopics8) -> Result<Self, Self::Error> {
        Ok(OffsetCommitRequestTopics2 {
            name: latest.name,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions2 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopicsPartitions8) -> Result<Self, Self::Error> {
        if latest.committed_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequestTopicsPartitions",
                2,
                "committed_leader_epoch",
            ));
        }
        Ok(OffsetCommitRequestTopicsPartitions2 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        })
    }
}

impl TryFrom<OffsetCommitRequest8> for OffsetCommitRequest3 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequest8) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequest",
                3,
                "group_instance_id",
            ));
        }
        Ok(OffsetCommitRequest3 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            topics: latest.topics.try_into()?,
            ..OffsetCommitRequest3::default()
        })
    }
}

impl TryFrom<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics3 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopics8) -> Result<Self, Self::Error> {
        Ok(OffsetCommitRequestTopics3 {
            name: latest.name,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions3 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopicsPartitions8) -> Result<Self, Self::Error> {
        if latest.committed_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequestTopicsPartitions",
                3,
                "committed_leader_epoch",
            ));
        }
        Ok(OffsetCommitRequestTopicsPartitions3 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        })
    }
}

impl TryFrom<OffsetCommitRequest8> for OffsetCommitRequest4 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequest8) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequest",
                4,
                "group_instance_id",
            ));
        }
        Ok(OffsetCommitRequest4 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            topics: latest.topics.try_into()?,
            ..OffsetCommitRequest4::default()
        })
    }
}

impl TryFrom<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics4 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopics8) -> Result<Self, Self::Error> {
        Ok(OffsetCommitRequestTopics4 {
            name: latest.name,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions4 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopicsPartitions8) -> Result<Self, Self::Error> {
        if latest.committed_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequestTopicsPartitions",
                4,
                "committed_leader_epoch",
            ));
        }
        Ok(OffsetCommitRequestTopicsPartitions4 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        })
    }
}

impl TryFrom<OffsetCommitRequest8> for OffsetCommitRequest5 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequest8) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequest",
                5,
                "group_instance_id",
            ));
        }
        Ok(OffsetCommitRequest5 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics5 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopics8) -> Result<Self, Self::Error> {
        Ok(OffsetCommitRequestTopics5 {
            name: latest.name,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions5 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopicsPartitions8) -> Result<Self, Self::Error> {
        if latest.committed_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequestTopicsPartitions",
                5,
                "committed_leader_epoch",
            ));
        }
        Ok(OffsetCommitRequestTopicsPartitions5 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        })
    }
}

impl TryFrom<OffsetCommitRequest8> for OffsetCommitRequest6 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequest8) -> Result<Self, Self::Error> {
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetCommitRequest",
                6,
                "group_instance_id",
            ));
        }
        Ok(OffsetCommitRequest6 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics6 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopics8) -> Result<Self, Self::Error> {
        Ok(OffsetCommitRequestTopics6 {
            name: latest.name,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions6 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopicsPartitions8) -> Result<Self, Self::Error> {
        Ok(OffsetCommitRequestTopicsPartitions6 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_leader_epoch: latest.committed_leader_epoch,
            committed_metadata: latest.committed_metadata,
        })
    }
}

impl TryFrom<OffsetCommitRequest8> for OffsetCommitRequest7 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequest8) -> Result<Self, Self::Error> {
        Ok(OffsetCommitRequest7 {
            group_id: latest.group_id,
            generation_id: latest.generation_id,
            member_id: latest.member_id,
            group_instance_id: latest.group_instance_id,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<OffsetCommitRequestTopics8> for OffsetCommitRequestTopics7 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopics8) -> Result<Self, Self::Error> {
        Ok(OffsetCommitRequestTopics7 {
            name: latest.name,
            partitions: latest.partitions.try_into()?,
        })
    }
}

impl TryFrom<OffsetCommitRequestTopicsPartitions8> for OffsetCommitRequestTopicsPartitions7 {
    type Error = Error;
    fn try_from(latest: OffsetCommitRequestTopicsPartitions8) -> Result<Self, Self::Error> {
        Ok(OffsetCommitRequestTopicsPartitions7 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_leader_epoch: latest.committed_leader_epoch,
            committed_metadata: latest.committed_metadata,
        })
    }
}

impl From<OffsetCommitResponse0> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse0) -> Self {
        OffsetCommitResponse8 {
            topics: older.topics.into(),
            ..OffsetCommitResponse8::default()
        }
    }
}

impl From<OffsetCommitResponseTopics0> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics0) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into(),
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions0> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions0) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}

impl From<OffsetCommitResponse1> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse1) -> Self {
        OffsetCommitResponse8 {
            topics: older.topics.into(),
            ..OffsetCommitResponse8::default()
        }
    }
}

impl From<OffsetCommitResponseTopics1> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics1) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into(),
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions1> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions1) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}

impl From<OffsetCommitResponse2> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse2) -> Self {
        OffsetCommitResponse8 {
            topics: older.topics.into(),
            ..OffsetCommitResponse8::default()
        }
    }
}

impl From<OffsetCommitResponseTopics2> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics2) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into(),
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions2> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions2) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}

impl From<OffsetCommitResponse3> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse3) -> Self {
        OffsetCommitResponse8 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into(),
        }
    }
}

impl From<OffsetCommitResponseTopics3> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics3) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into(),
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions3> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions3) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}

impl From<OffsetCommitResponse4> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse4) -> Self {
        OffsetCommitResponse8 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into(),
        }
    }
}

impl From<OffsetCommitResponseTopics4> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics4) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into(),
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions4> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions4) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}

impl From<OffsetCommitResponse5> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse5) -> Self {
        OffsetCommitResponse8 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into(),
        }
    }
}

impl From<OffsetCommitResponseTopics5> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics5) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into(),
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions5> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions5) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}

impl From<OffsetCommitResponse6> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse6) -> Self {
        OffsetCommitResponse8 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into(),
        }
    }
}

impl From<OffsetCommitResponseTopics6> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics6) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into(),
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions6> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions6) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}

impl From<OffsetCommitResponse7> for OffsetCommitResponse8 {
    fn from(older: OffsetCommitResponse7) -> Self {
        OffsetCommitResponse8 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into(),
        }
    }
}

impl From<OffsetCommitResponseTopics7> for OffsetCommitResponseTopics8 {
    fn from(older: OffsetCommitResponseTopics7) -> Self {
        OffsetCommitResponseTopics8 {
            name: older.name,
            partitions: older.partitions.into(),
        }
    }
}

impl From<OffsetCommitResponseTopicsPartitions7> for OffsetCommitResponseTopicsPartitions8 {
    fn from(older: OffsetCommitResponseTopicsPartitions7) -> Self {
        OffsetCommitResponseTopicsPartitions8 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}
