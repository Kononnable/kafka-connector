use super::prelude::*;

pub type ListOffsetsRequest = ListOffsetsRequest5;
pub type ListOffsetsResponse = ListOffsetsResponse5;
impl ApiCall for ListOffsetsRequest {
    type Response = ListOffsetsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        5
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::ListOffsets
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&ListOffsetsRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&ListOffsetsRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&ListOffsetsRequest2::try_from(self)?, buf),
            3 => ToBytes::serialize(&ListOffsetsRequest3::try_from(self)?, buf),
            4 => ToBytes::serialize(&ListOffsetsRequest4::try_from(self)?, buf),
            5 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> ListOffsetsResponse {
        match version {
            0 => ListOffsetsResponse0::deserialize(buf).into(),
            1 => ListOffsetsResponse1::deserialize(buf).into(),
            2 => ListOffsetsResponse2::deserialize(buf).into(),
            3 => ListOffsetsResponse3::deserialize(buf).into(),
            4 => ListOffsetsResponse4::deserialize(buf).into(),
            5 => ListOffsetsResponse::deserialize(buf),
            _ => ListOffsetsResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequest0 {
    pub replica_id: Int32,
    pub topics: Vec<ListOffsetsRequestTopics0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequestTopics0 {
    pub name: String,
    pub partitions: Vec<ListOffsetsRequestTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequestTopicsPartitions0 {
    pub partition_index: Int32,
    pub timestamp: Int64,
    pub max_num_offsets: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequest1 {
    pub replica_id: Int32,
    pub topics: Vec<ListOffsetsRequestTopics1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequestTopics1 {
    pub name: String,
    pub partitions: Vec<ListOffsetsRequestTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequestTopicsPartitions1 {
    pub partition_index: Int32,
    pub timestamp: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequest2 {
    pub replica_id: Int32,
    pub isolation_level: Optional<Int8>,
    pub topics: Vec<ListOffsetsRequestTopics2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequestTopics2 {
    pub name: String,
    pub partitions: Vec<ListOffsetsRequestTopicsPartitions2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequestTopicsPartitions2 {
    pub partition_index: Int32,
    pub timestamp: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequest3 {
    pub replica_id: Int32,
    pub isolation_level: Optional<Int8>,
    pub topics: Vec<ListOffsetsRequestTopics3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequestTopics3 {
    pub name: String,
    pub partitions: Vec<ListOffsetsRequestTopicsPartitions3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequestTopicsPartitions3 {
    pub partition_index: Int32,
    pub timestamp: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequest4 {
    pub replica_id: Int32,
    pub isolation_level: Optional<Int8>,
    pub topics: Vec<ListOffsetsRequestTopics4>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequestTopics4 {
    pub name: String,
    pub partitions: Vec<ListOffsetsRequestTopicsPartitions4>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequestTopicsPartitions4 {
    pub partition_index: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub timestamp: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequest5 {
    pub replica_id: Int32,
    pub isolation_level: Optional<Int8>,
    pub topics: Vec<ListOffsetsRequestTopics5>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequestTopics5 {
    pub name: String,
    pub partitions: Vec<ListOffsetsRequestTopicsPartitions5>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequestTopicsPartitions5 {
    pub partition_index: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub timestamp: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponse0 {
    pub topics: Vec<ListOffsetsResponseTopics0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopics0 {
    pub name: String,
    pub partitions: Vec<ListOffsetsResponseTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub old_style_offsets: Vec<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponse1 {
    pub topics: Vec<ListOffsetsResponseTopics1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopics1 {
    pub name: String,
    pub partitions: Vec<ListOffsetsResponseTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopicsPartitions1 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub timestamp: Optional<Int64>,
    pub offset: Optional<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<ListOffsetsResponseTopics2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopics2 {
    pub name: String,
    pub partitions: Vec<ListOffsetsResponseTopicsPartitions2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopicsPartitions2 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub timestamp: Optional<Int64>,
    pub offset: Optional<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<ListOffsetsResponseTopics3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopics3 {
    pub name: String,
    pub partitions: Vec<ListOffsetsResponseTopicsPartitions3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopicsPartitions3 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub timestamp: Optional<Int64>,
    pub offset: Optional<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<ListOffsetsResponseTopics4>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopics4 {
    pub name: String,
    pub partitions: Vec<ListOffsetsResponseTopicsPartitions4>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopicsPartitions4 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub timestamp: Optional<Int64>,
    pub offset: Optional<Int64>,
    pub leader_epoch: Optional<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponse5 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<ListOffsetsResponseTopics5>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopics5 {
    pub name: String,
    pub partitions: Vec<ListOffsetsResponseTopicsPartitions5>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponseTopicsPartitions5 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub timestamp: Optional<Int64>,
    pub offset: Optional<Int64>,
    pub leader_epoch: Optional<Int32>,
}

impl TryFrom<ListOffsetsRequest5> for ListOffsetsRequest0 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequest5) -> Result<Self, Self::Error> {
        if latest.isolation_level.is_some() {
            return Err(Error::OldKafkaVersion(
                "ListOffsetsRequest",
                0,
                "isolation_level",
            ));
        }
        Ok(ListOffsetsRequest0 {
            replica_id: latest.replica_id,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ListOffsetsRequestTopics5> for ListOffsetsRequestTopics0 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequestTopics5) -> Result<Self, Self::Error> {
        Ok(ListOffsetsRequestTopics0 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ListOffsetsRequestTopicsPartitions5> for ListOffsetsRequestTopicsPartitions0 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequestTopicsPartitions5) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "ListOffsetsRequestTopicsPartitions",
                0,
                "current_leader_epoch",
            ));
        }
        Ok(ListOffsetsRequestTopicsPartitions0 {
            partition_index: latest.partition_index,
            timestamp: latest.timestamp,
            ..ListOffsetsRequestTopicsPartitions0::default()
        })
    }
}

impl TryFrom<ListOffsetsRequest5> for ListOffsetsRequest1 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequest5) -> Result<Self, Self::Error> {
        if latest.isolation_level.is_some() {
            return Err(Error::OldKafkaVersion(
                "ListOffsetsRequest",
                1,
                "isolation_level",
            ));
        }
        Ok(ListOffsetsRequest1 {
            replica_id: latest.replica_id,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ListOffsetsRequestTopics5> for ListOffsetsRequestTopics1 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequestTopics5) -> Result<Self, Self::Error> {
        Ok(ListOffsetsRequestTopics1 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ListOffsetsRequestTopicsPartitions5> for ListOffsetsRequestTopicsPartitions1 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequestTopicsPartitions5) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "ListOffsetsRequestTopicsPartitions",
                1,
                "current_leader_epoch",
            ));
        }
        Ok(ListOffsetsRequestTopicsPartitions1 {
            partition_index: latest.partition_index,
            timestamp: latest.timestamp,
        })
    }
}

impl TryFrom<ListOffsetsRequest5> for ListOffsetsRequest2 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequest5) -> Result<Self, Self::Error> {
        Ok(ListOffsetsRequest2 {
            replica_id: latest.replica_id,
            isolation_level: latest.isolation_level,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ListOffsetsRequestTopics5> for ListOffsetsRequestTopics2 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequestTopics5) -> Result<Self, Self::Error> {
        Ok(ListOffsetsRequestTopics2 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ListOffsetsRequestTopicsPartitions5> for ListOffsetsRequestTopicsPartitions2 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequestTopicsPartitions5) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "ListOffsetsRequestTopicsPartitions",
                2,
                "current_leader_epoch",
            ));
        }
        Ok(ListOffsetsRequestTopicsPartitions2 {
            partition_index: latest.partition_index,
            timestamp: latest.timestamp,
        })
    }
}

impl TryFrom<ListOffsetsRequest5> for ListOffsetsRequest3 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequest5) -> Result<Self, Self::Error> {
        Ok(ListOffsetsRequest3 {
            replica_id: latest.replica_id,
            isolation_level: latest.isolation_level,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ListOffsetsRequestTopics5> for ListOffsetsRequestTopics3 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequestTopics5) -> Result<Self, Self::Error> {
        Ok(ListOffsetsRequestTopics3 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ListOffsetsRequestTopicsPartitions5> for ListOffsetsRequestTopicsPartitions3 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequestTopicsPartitions5) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "ListOffsetsRequestTopicsPartitions",
                3,
                "current_leader_epoch",
            ));
        }
        Ok(ListOffsetsRequestTopicsPartitions3 {
            partition_index: latest.partition_index,
            timestamp: latest.timestamp,
        })
    }
}

impl TryFrom<ListOffsetsRequest5> for ListOffsetsRequest4 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequest5) -> Result<Self, Self::Error> {
        Ok(ListOffsetsRequest4 {
            replica_id: latest.replica_id,
            isolation_level: latest.isolation_level,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ListOffsetsRequestTopics5> for ListOffsetsRequestTopics4 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequestTopics5) -> Result<Self, Self::Error> {
        Ok(ListOffsetsRequestTopics4 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ListOffsetsRequestTopicsPartitions5> for ListOffsetsRequestTopicsPartitions4 {
    type Error = Error;
    fn try_from(latest: ListOffsetsRequestTopicsPartitions5) -> Result<Self, Self::Error> {
        Ok(ListOffsetsRequestTopicsPartitions4 {
            partition_index: latest.partition_index,
            current_leader_epoch: latest.current_leader_epoch,
            timestamp: latest.timestamp,
        })
    }
}

impl From<ListOffsetsResponse0> for ListOffsetsResponse5 {
    fn from(older: ListOffsetsResponse0) -> Self {
        ListOffsetsResponse5 {
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..ListOffsetsResponse5::default()
        }
    }
}

impl From<ListOffsetsResponseTopics0> for ListOffsetsResponseTopics5 {
    fn from(older: ListOffsetsResponseTopics0) -> Self {
        ListOffsetsResponseTopics5 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<ListOffsetsResponseTopicsPartitions0> for ListOffsetsResponseTopicsPartitions5 {
    fn from(older: ListOffsetsResponseTopicsPartitions0) -> Self {
        ListOffsetsResponseTopicsPartitions5 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..ListOffsetsResponseTopicsPartitions5::default()
        }
    }
}

impl From<ListOffsetsResponse1> for ListOffsetsResponse5 {
    fn from(older: ListOffsetsResponse1) -> Self {
        ListOffsetsResponse5 {
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..ListOffsetsResponse5::default()
        }
    }
}

impl From<ListOffsetsResponseTopics1> for ListOffsetsResponseTopics5 {
    fn from(older: ListOffsetsResponseTopics1) -> Self {
        ListOffsetsResponseTopics5 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<ListOffsetsResponseTopicsPartitions1> for ListOffsetsResponseTopicsPartitions5 {
    fn from(older: ListOffsetsResponseTopicsPartitions1) -> Self {
        ListOffsetsResponseTopicsPartitions5 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            timestamp: older.timestamp,
            offset: older.offset,
            ..ListOffsetsResponseTopicsPartitions5::default()
        }
    }
}

impl From<ListOffsetsResponse2> for ListOffsetsResponse5 {
    fn from(older: ListOffsetsResponse2) -> Self {
        ListOffsetsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<ListOffsetsResponseTopics2> for ListOffsetsResponseTopics5 {
    fn from(older: ListOffsetsResponseTopics2) -> Self {
        ListOffsetsResponseTopics5 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<ListOffsetsResponseTopicsPartitions2> for ListOffsetsResponseTopicsPartitions5 {
    fn from(older: ListOffsetsResponseTopicsPartitions2) -> Self {
        ListOffsetsResponseTopicsPartitions5 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            timestamp: older.timestamp,
            offset: older.offset,
            ..ListOffsetsResponseTopicsPartitions5::default()
        }
    }
}

impl From<ListOffsetsResponse3> for ListOffsetsResponse5 {
    fn from(older: ListOffsetsResponse3) -> Self {
        ListOffsetsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<ListOffsetsResponseTopics3> for ListOffsetsResponseTopics5 {
    fn from(older: ListOffsetsResponseTopics3) -> Self {
        ListOffsetsResponseTopics5 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<ListOffsetsResponseTopicsPartitions3> for ListOffsetsResponseTopicsPartitions5 {
    fn from(older: ListOffsetsResponseTopicsPartitions3) -> Self {
        ListOffsetsResponseTopicsPartitions5 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            timestamp: older.timestamp,
            offset: older.offset,
            ..ListOffsetsResponseTopicsPartitions5::default()
        }
    }
}

impl From<ListOffsetsResponse4> for ListOffsetsResponse5 {
    fn from(older: ListOffsetsResponse4) -> Self {
        ListOffsetsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<ListOffsetsResponseTopics4> for ListOffsetsResponseTopics5 {
    fn from(older: ListOffsetsResponseTopics4) -> Self {
        ListOffsetsResponseTopics5 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<ListOffsetsResponseTopicsPartitions4> for ListOffsetsResponseTopicsPartitions5 {
    fn from(older: ListOffsetsResponseTopicsPartitions4) -> Self {
        ListOffsetsResponseTopicsPartitions5 {
            partition_index: older.partition_index,
            error_code: older.error_code,
            timestamp: older.timestamp,
            offset: older.offset,
            leader_epoch: older.leader_epoch,
        }
    }
}
