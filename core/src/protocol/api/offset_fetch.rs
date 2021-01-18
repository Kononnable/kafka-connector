use super::prelude::*;

pub type OffsetFetchRequest = OffsetFetchRequest7;
pub type OffsetFetchResponse = OffsetFetchResponse7;
pub fn serialize_offset_fetch_request(
    data: OffsetFetchRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&OffsetFetchRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&OffsetFetchRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&OffsetFetchRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&OffsetFetchRequest3::try_from(data)?, buf),
        4 => ToBytes::serialize(&OffsetFetchRequest4::try_from(data)?, buf),
        5 => ToBytes::serialize(&OffsetFetchRequest5::try_from(data)?, buf),
        6 => ToBytes::serialize(&OffsetFetchRequest6::try_from(data)?, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_offset_fetch_response<T>(version: i32, buf: &mut T) -> OffsetFetchResponse
where
    T: Iterator<Item = u8>,
{
    match version {
        0 => OffsetFetchResponse0::deserialize(buf).into(),
        1 => OffsetFetchResponse1::deserialize(buf).into(),
        2 => OffsetFetchResponse2::deserialize(buf).into(),
        3 => OffsetFetchResponse3::deserialize(buf).into(),
        4 => OffsetFetchResponse4::deserialize(buf).into(),
        5 => OffsetFetchResponse5::deserialize(buf).into(),
        6 => OffsetFetchResponse6::deserialize(buf).into(),
        _ => OffsetFetchResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequest0 {
    pub group_id: String,
    pub topics: OffsetFetchRequestTopics0,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequestTopics0 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequest1 {
    pub group_id: String,
    pub topics: OffsetFetchRequestTopics1,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequestTopics1 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequest2 {
    pub group_id: String,
    pub topics: OffsetFetchRequestTopics2,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequestTopics2 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequest3 {
    pub group_id: String,
    pub topics: OffsetFetchRequestTopics3,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequestTopics3 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequest4 {
    pub group_id: String,
    pub topics: OffsetFetchRequestTopics4,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequestTopics4 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequest5 {
    pub group_id: String,
    pub topics: OffsetFetchRequestTopics5,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequestTopics5 {
    pub name: String,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequest6 {
    pub group_id: CompactString,
    pub topics: OffsetFetchRequestTopics6,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequestTopics6 {
    pub name: CompactString,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequest7 {
    pub group_id: CompactString,
    pub topics: OffsetFetchRequestTopics7,
    pub require_stable: Optional<Boolean>,
}

#[derive(Default, ToBytes)]
pub struct OffsetFetchRequestTopics7 {
    pub name: CompactString,
    pub partition_indexes: Vec<Int32>,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponse0 {
    pub topics: OffsetFetchResponseTopics0,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopics0 {
    pub name: String,
    pub partitions: OffsetFetchResponseTopicsPartitions0,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub metadata: NullableString,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponse1 {
    pub topics: OffsetFetchResponseTopics1,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopics1 {
    pub name: String,
    pub partitions: OffsetFetchResponseTopicsPartitions1,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions1 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub metadata: NullableString,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponse2 {
    pub topics: OffsetFetchResponseTopics2,
    pub error_code: Optional<Int16>,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopics2 {
    pub name: String,
    pub partitions: OffsetFetchResponseTopicsPartitions2,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions2 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub metadata: NullableString,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: OffsetFetchResponseTopics3,
    pub error_code: Optional<Int16>,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopics3 {
    pub name: String,
    pub partitions: OffsetFetchResponseTopicsPartitions3,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions3 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub metadata: NullableString,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: OffsetFetchResponseTopics4,
    pub error_code: Optional<Int16>,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopics4 {
    pub name: String,
    pub partitions: OffsetFetchResponseTopicsPartitions4,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions4 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub metadata: NullableString,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponse5 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: OffsetFetchResponseTopics5,
    pub error_code: Optional<Int16>,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopics5 {
    pub name: String,
    pub partitions: OffsetFetchResponseTopicsPartitions5,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions5 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Optional<Int32>,
    pub metadata: NullableString,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponse6 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: OffsetFetchResponseTopics6,
    pub error_code: Optional<Int16>,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopics6 {
    pub name: CompactString,
    pub partitions: OffsetFetchResponseTopicsPartitions6,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions6 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Optional<Int32>,
    pub metadata: CompactNullableString,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponse7 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: OffsetFetchResponseTopics7,
    pub error_code: Optional<Int16>,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopics7 {
    pub name: CompactString,
    pub partitions: OffsetFetchResponseTopicsPartitions7,
}

#[derive(Default, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions7 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Optional<Int32>,
    pub metadata: CompactNullableString,
    pub error_code: Int16,
}

impl TryFrom<OffsetFetchRequest7> for OffsetFetchRequest0 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequest7) -> Result<Self, Self::Error> {
        if latest.require_stable.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequest",
                0,
                "require_stable",
            ));
        }
        Ok(OffsetFetchRequest0 {
            group_id: latest.group_id,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics0 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        Ok(OffsetFetchRequestTopics0 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
        })
    }
}

impl TryFrom<OffsetFetchRequest7> for OffsetFetchRequest1 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequest7) -> Result<Self, Self::Error> {
        if latest.require_stable.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequest",
                1,
                "require_stable",
            ));
        }
        Ok(OffsetFetchRequest1 {
            group_id: latest.group_id,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics1 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        Ok(OffsetFetchRequestTopics1 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
        })
    }
}

impl TryFrom<OffsetFetchRequest7> for OffsetFetchRequest2 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequest7) -> Result<Self, Self::Error> {
        if latest.require_stable.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequest",
                2,
                "require_stable",
            ));
        }
        Ok(OffsetFetchRequest2 {
            group_id: latest.group_id,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics2 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        Ok(OffsetFetchRequestTopics2 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
        })
    }
}

impl TryFrom<OffsetFetchRequest7> for OffsetFetchRequest3 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequest7) -> Result<Self, Self::Error> {
        if latest.require_stable.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequest",
                3,
                "require_stable",
            ));
        }
        Ok(OffsetFetchRequest3 {
            group_id: latest.group_id,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics3 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        Ok(OffsetFetchRequestTopics3 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
        })
    }
}

impl TryFrom<OffsetFetchRequest7> for OffsetFetchRequest4 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequest7) -> Result<Self, Self::Error> {
        if latest.require_stable.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequest",
                4,
                "require_stable",
            ));
        }
        Ok(OffsetFetchRequest4 {
            group_id: latest.group_id,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics4 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        Ok(OffsetFetchRequestTopics4 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
        })
    }
}

impl TryFrom<OffsetFetchRequest7> for OffsetFetchRequest5 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequest7) -> Result<Self, Self::Error> {
        if latest.require_stable.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequest",
                5,
                "require_stable",
            ));
        }
        Ok(OffsetFetchRequest5 {
            group_id: latest.group_id,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics5 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        Ok(OffsetFetchRequestTopics5 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
        })
    }
}

impl TryFrom<OffsetFetchRequest7> for OffsetFetchRequest6 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequest7) -> Result<Self, Self::Error> {
        if latest.require_stable.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequest",
                6,
                "require_stable",
            ));
        }
        Ok(OffsetFetchRequest6 {
            group_id: latest.group_id,
            topics: latest.topics.try_into()?,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics6 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        Ok(OffsetFetchRequestTopics6 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
        })
    }
}

impl From<OffsetFetchResponse0> for OffsetFetchResponse7 {
    fn from(older: OffsetFetchResponse0) -> Self {
        OffsetFetchResponse7 {
            topics: older.topics.into(),
            ..OffsetFetchResponse7::default()
        }
    }
}

impl From<OffsetFetchResponseTopics0> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics0) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into(),
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
            topics: older.topics.into(),
            ..OffsetFetchResponse7::default()
        }
    }
}

impl From<OffsetFetchResponseTopics1> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics1) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into(),
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
            topics: older.topics.into(),
            error_code: older.error_code,
            ..OffsetFetchResponse7::default()
        }
    }
}

impl From<OffsetFetchResponseTopics2> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics2) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into(),
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
            topics: older.topics.into(),
            error_code: older.error_code,
        }
    }
}

impl From<OffsetFetchResponseTopics3> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics3) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into(),
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
            topics: older.topics.into(),
            error_code: older.error_code,
        }
    }
}

impl From<OffsetFetchResponseTopics4> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics4) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into(),
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
            topics: older.topics.into(),
            error_code: older.error_code,
        }
    }
}

impl From<OffsetFetchResponseTopics5> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics5) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into(),
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
        }
    }
}

impl From<OffsetFetchResponse6> for OffsetFetchResponse7 {
    fn from(older: OffsetFetchResponse6) -> Self {
        OffsetFetchResponse7 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into(),
            error_code: older.error_code,
        }
    }
}

impl From<OffsetFetchResponseTopics6> for OffsetFetchResponseTopics7 {
    fn from(older: OffsetFetchResponseTopics6) -> Self {
        OffsetFetchResponseTopics7 {
            name: older.name,
            partitions: older.partitions.into(),
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
        }
    }
}
