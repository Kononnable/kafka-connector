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
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&OffsetFetchRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&OffsetFetchRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&OffsetFetchRequest2::try_from(self)?, buf),
            3 => ToBytes::serialize(&OffsetFetchRequest3::try_from(self)?, buf),
            4 => ToBytes::serialize(&OffsetFetchRequest4::try_from(self)?, buf),
            5 => ToBytes::serialize(&OffsetFetchRequest5::try_from(self)?, buf),
            6 => ToBytes::serialize(&OffsetFetchRequest6::try_from(self)?, buf),
            7 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> OffsetFetchResponse {
        match version {
            0 => OffsetFetchResponse0::deserialize(buf).into(),
            1 => OffsetFetchResponse1::deserialize(buf).into(),
            2 => OffsetFetchResponse2::deserialize(buf).into(),
            3 => OffsetFetchResponse3::deserialize(buf).into(),
            4 => OffsetFetchResponse4::deserialize(buf).into(),
            5 => OffsetFetchResponse5::deserialize(buf).into(),
            6 => OffsetFetchResponse6::deserialize(buf).into(),
            7 => OffsetFetchResponse::deserialize(buf),
            _ => OffsetFetchResponse::deserialize(buf),
        }
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
    pub group_id: CompactString,
    pub topics: Vec<OffsetFetchRequestTopics6>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequestTopics6 {
    pub name: CompactString,
    pub partition_indexes: Vec<Int32>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequest7 {
    pub group_id: CompactString,
    pub topics: Vec<OffsetFetchRequestTopics7>,
    pub require_stable: Optional<Boolean>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetFetchRequestTopics7 {
    pub name: CompactString,
    pub partition_indexes: Vec<Int32>,
    pub tag_buffer: Optional<TagBuffer>,
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
    pub error_code: Optional<Int16>,
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
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<OffsetFetchResponseTopics3>,
    pub error_code: Optional<Int16>,
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
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<OffsetFetchResponseTopics4>,
    pub error_code: Optional<Int16>,
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
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<OffsetFetchResponseTopics5>,
    pub error_code: Optional<Int16>,
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
    pub committed_leader_epoch: Optional<Int32>,
    pub metadata: NullableString,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponse6 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<OffsetFetchResponseTopics6>,
    pub error_code: Optional<Int16>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopics6 {
    pub name: CompactString,
    pub partitions: Vec<OffsetFetchResponseTopicsPartitions6>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions6 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Optional<Int32>,
    pub metadata: CompactNullableString,
    pub error_code: Int16,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponse7 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<OffsetFetchResponseTopics7>,
    pub error_code: Optional<Int16>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopics7 {
    pub name: CompactString,
    pub partitions: Vec<OffsetFetchResponseTopicsPartitions7>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetFetchResponseTopicsPartitions7 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Optional<Int32>,
    pub metadata: CompactNullableString,
    pub error_code: Int16,
    pub tag_buffer: Optional<TagBuffer>,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequest",
                0,
                "tag_buffer",
            ));
        }
        Ok(OffsetFetchRequest0 {
            group_id: latest.group_id.into(),
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics0 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequestTopics",
                0,
                "tag_buffer",
            ));
        }
        Ok(OffsetFetchRequestTopics0 {
            name: latest.name.into(),
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequest",
                1,
                "tag_buffer",
            ));
        }
        Ok(OffsetFetchRequest1 {
            group_id: latest.group_id.into(),
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics1 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequestTopics",
                1,
                "tag_buffer",
            ));
        }
        Ok(OffsetFetchRequestTopics1 {
            name: latest.name.into(),
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequest",
                2,
                "tag_buffer",
            ));
        }
        Ok(OffsetFetchRequest2 {
            group_id: latest.group_id.into(),
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics2 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequestTopics",
                2,
                "tag_buffer",
            ));
        }
        Ok(OffsetFetchRequestTopics2 {
            name: latest.name.into(),
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequest",
                3,
                "tag_buffer",
            ));
        }
        Ok(OffsetFetchRequest3 {
            group_id: latest.group_id.into(),
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics3 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequestTopics",
                3,
                "tag_buffer",
            ));
        }
        Ok(OffsetFetchRequestTopics3 {
            name: latest.name.into(),
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequest",
                4,
                "tag_buffer",
            ));
        }
        Ok(OffsetFetchRequest4 {
            group_id: latest.group_id.into(),
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics4 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequestTopics",
                4,
                "tag_buffer",
            ));
        }
        Ok(OffsetFetchRequestTopics4 {
            name: latest.name.into(),
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequest",
                5,
                "tag_buffer",
            ));
        }
        Ok(OffsetFetchRequest5 {
            group_id: latest.group_id.into(),
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics5 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetFetchRequestTopics",
                5,
                "tag_buffer",
            ));
        }
        Ok(OffsetFetchRequestTopics5 {
            name: latest.name.into(),
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
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            tag_buffer: latest.tag_buffer,
        })
    }
}

impl TryFrom<OffsetFetchRequestTopics7> for OffsetFetchRequestTopics6 {
    type Error = Error;
    fn try_from(latest: OffsetFetchRequestTopics7) -> Result<Self, Self::Error> {
        Ok(OffsetFetchRequestTopics6 {
            name: latest.name,
            partition_indexes: latest.partition_indexes,
            tag_buffer: latest.tag_buffer,
        })
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
            name: older.name.into(),
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
            metadata: older.metadata.into(),
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
            name: older.name.into(),
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
            metadata: older.metadata.into(),
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
            name: older.name.into(),
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
            metadata: older.metadata.into(),
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
            name: older.name.into(),
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
            metadata: older.metadata.into(),
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
            name: older.name.into(),
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
            metadata: older.metadata.into(),
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
            name: older.name.into(),
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
            metadata: older.metadata.into(),
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
