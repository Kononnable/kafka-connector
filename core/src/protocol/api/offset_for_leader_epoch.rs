use super::prelude::*;

pub type OffsetForLeaderEpochRequest = OffsetForLeaderEpochRequest3;
pub type OffsetForLeaderEpochResponse = OffsetForLeaderEpochResponse3;
pub fn serialize_offset_for_leader_epoch_request(
    data: OffsetForLeaderEpochRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&OffsetForLeaderEpochRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&OffsetForLeaderEpochRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&OffsetForLeaderEpochRequest2::try_from(data)?, buf),
        4 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_offset_for_leader_epoch_response<T>(
    version: i32,
    buf: &mut T,
) -> OffsetForLeaderEpochResponse
where
    T: Iterator<Item = u8>,
{
    match version {
        0 => OffsetForLeaderEpochResponse0::deserialize(buf).into(),
        1 => OffsetForLeaderEpochResponse1::deserialize(buf).into(),
        2 => OffsetForLeaderEpochResponse2::deserialize(buf).into(),
        4 => OffsetForLeaderEpochResponse::deserialize(buf),
        _ => OffsetForLeaderEpochResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct OffsetForLeaderEpochRequest0 {
    pub topics: Vec<OffsetForLeaderEpochRequestTopics0>,
}

#[derive(Default, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopics0 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochRequestTopicsPartitions0>,
}

#[derive(Default, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopicsPartitions0 {
    pub partition: Int32,
    pub leader_epoch: Int32,
}

#[derive(Default, ToBytes)]
pub struct OffsetForLeaderEpochRequest1 {
    pub topics: Vec<OffsetForLeaderEpochRequestTopics1>,
}

#[derive(Default, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopics1 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochRequestTopicsPartitions1>,
}

#[derive(Default, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopicsPartitions1 {
    pub partition: Int32,
    pub leader_epoch: Int32,
}

#[derive(Default, ToBytes)]
pub struct OffsetForLeaderEpochRequest2 {
    pub topics: Vec<OffsetForLeaderEpochRequestTopics2>,
}

#[derive(Default, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopics2 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochRequestTopicsPartitions2>,
}

#[derive(Default, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopicsPartitions2 {
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub leader_epoch: Int32,
}

#[derive(Default, ToBytes)]
pub struct OffsetForLeaderEpochRequest3 {
    pub replica_id: Optional<Int32>,
    pub topics: Vec<OffsetForLeaderEpochRequestTopics3>,
}

#[derive(Default, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopics3 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochRequestTopicsPartitions3>,
}

#[derive(Default, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopicsPartitions3 {
    pub partition: Int32,
    pub current_leader_epoch: Optional<Int32>,
    pub leader_epoch: Int32,
}

#[derive(Default, FromBytes)]
pub struct OffsetForLeaderEpochResponse0 {
    pub topics: Vec<OffsetForLeaderEpochResponseTopics0>,
}

#[derive(Default, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopics0 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochResponseTopicsPartitions0>,
}

#[derive(Default, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopicsPartitions0 {
    pub error_code: Int16,
    pub partition: Int32,
    pub end_offset: Int64,
}

#[derive(Default, FromBytes)]
pub struct OffsetForLeaderEpochResponse1 {
    pub topics: Vec<OffsetForLeaderEpochResponseTopics1>,
}

#[derive(Default, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopics1 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochResponseTopicsPartitions1>,
}

#[derive(Default, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopicsPartitions1 {
    pub error_code: Int16,
    pub partition: Int32,
    pub leader_epoch: Optional<Int32>,
    pub end_offset: Int64,
}

#[derive(Default, FromBytes)]
pub struct OffsetForLeaderEpochResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<OffsetForLeaderEpochResponseTopics2>,
}

#[derive(Default, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopics2 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochResponseTopicsPartitions2>,
}

#[derive(Default, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopicsPartitions2 {
    pub error_code: Int16,
    pub partition: Int32,
    pub leader_epoch: Optional<Int32>,
    pub end_offset: Int64,
}

#[derive(Default, FromBytes)]
pub struct OffsetForLeaderEpochResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<OffsetForLeaderEpochResponseTopics3>,
}

#[derive(Default, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopics3 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochResponseTopicsPartitions3>,
}

#[derive(Default, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopicsPartitions3 {
    pub error_code: Int16,
    pub partition: Int32,
    pub leader_epoch: Optional<Int32>,
    pub end_offset: Int64,
}

impl TryFrom<OffsetForLeaderEpochRequest3> for OffsetForLeaderEpochRequest0 {
    type Error = Error;
    fn try_from(latest: OffsetForLeaderEpochRequest3) -> Result<Self, Self::Error> {
        if latest.replica_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetForLeaderEpochRequest",
                0,
                "replica_id",
            ));
        }
        Ok(OffsetForLeaderEpochRequest0 {
            topics: latest
                .topics
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<OffsetForLeaderEpochRequestTopics3> for OffsetForLeaderEpochRequestTopics0 {
    type Error = Error;
    fn try_from(latest: OffsetForLeaderEpochRequestTopics3) -> Result<Self, Self::Error> {
        Ok(OffsetForLeaderEpochRequestTopics0 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<OffsetForLeaderEpochRequestTopicsPartitions3>
    for OffsetForLeaderEpochRequestTopicsPartitions0
{
    type Error = Error;
    fn try_from(latest: OffsetForLeaderEpochRequestTopicsPartitions3) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetForLeaderEpochRequestTopicsPartitions",
                0,
                "current_leader_epoch",
            ));
        }
        Ok(OffsetForLeaderEpochRequestTopicsPartitions0 {
            partition: latest.partition,
            leader_epoch: latest.leader_epoch,
        })
    }
}

impl TryFrom<OffsetForLeaderEpochRequest3> for OffsetForLeaderEpochRequest1 {
    type Error = Error;
    fn try_from(latest: OffsetForLeaderEpochRequest3) -> Result<Self, Self::Error> {
        if latest.replica_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetForLeaderEpochRequest",
                1,
                "replica_id",
            ));
        }
        Ok(OffsetForLeaderEpochRequest1 {
            topics: latest
                .topics
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<OffsetForLeaderEpochRequestTopics3> for OffsetForLeaderEpochRequestTopics1 {
    type Error = Error;
    fn try_from(latest: OffsetForLeaderEpochRequestTopics3) -> Result<Self, Self::Error> {
        Ok(OffsetForLeaderEpochRequestTopics1 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<OffsetForLeaderEpochRequestTopicsPartitions3>
    for OffsetForLeaderEpochRequestTopicsPartitions1
{
    type Error = Error;
    fn try_from(latest: OffsetForLeaderEpochRequestTopicsPartitions3) -> Result<Self, Self::Error> {
        if latest.current_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetForLeaderEpochRequestTopicsPartitions",
                1,
                "current_leader_epoch",
            ));
        }
        Ok(OffsetForLeaderEpochRequestTopicsPartitions1 {
            partition: latest.partition,
            leader_epoch: latest.leader_epoch,
        })
    }
}

impl TryFrom<OffsetForLeaderEpochRequest3> for OffsetForLeaderEpochRequest2 {
    type Error = Error;
    fn try_from(latest: OffsetForLeaderEpochRequest3) -> Result<Self, Self::Error> {
        if latest.replica_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "OffsetForLeaderEpochRequest",
                2,
                "replica_id",
            ));
        }
        Ok(OffsetForLeaderEpochRequest2 {
            topics: latest
                .topics
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<OffsetForLeaderEpochRequestTopics3> for OffsetForLeaderEpochRequestTopics2 {
    type Error = Error;
    fn try_from(latest: OffsetForLeaderEpochRequestTopics3) -> Result<Self, Self::Error> {
        Ok(OffsetForLeaderEpochRequestTopics2 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<OffsetForLeaderEpochRequestTopicsPartitions3>
    for OffsetForLeaderEpochRequestTopicsPartitions2
{
    type Error = Error;
    fn try_from(latest: OffsetForLeaderEpochRequestTopicsPartitions3) -> Result<Self, Self::Error> {
        Ok(OffsetForLeaderEpochRequestTopicsPartitions2 {
            partition: latest.partition,
            current_leader_epoch: latest.current_leader_epoch.map(|val| val),
            leader_epoch: latest.leader_epoch,
        })
    }
}

impl From<OffsetForLeaderEpochResponse0> for OffsetForLeaderEpochResponse3 {
    fn from(older: OffsetForLeaderEpochResponse0) -> Self {
        OffsetForLeaderEpochResponse3 {
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..OffsetForLeaderEpochResponse3::default()
        }
    }
}

impl From<OffsetForLeaderEpochResponseTopics0> for OffsetForLeaderEpochResponseTopics3 {
    fn from(older: OffsetForLeaderEpochResponseTopics0) -> Self {
        OffsetForLeaderEpochResponseTopics3 {
            topic: older.topic,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<OffsetForLeaderEpochResponseTopicsPartitions0>
    for OffsetForLeaderEpochResponseTopicsPartitions3
{
    fn from(older: OffsetForLeaderEpochResponseTopicsPartitions0) -> Self {
        OffsetForLeaderEpochResponseTopicsPartitions3 {
            error_code: older.error_code,
            partition: older.partition,
            end_offset: older.end_offset,
            ..OffsetForLeaderEpochResponseTopicsPartitions3::default()
        }
    }
}

impl From<OffsetForLeaderEpochResponse1> for OffsetForLeaderEpochResponse3 {
    fn from(older: OffsetForLeaderEpochResponse1) -> Self {
        OffsetForLeaderEpochResponse3 {
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..OffsetForLeaderEpochResponse3::default()
        }
    }
}

impl From<OffsetForLeaderEpochResponseTopics1> for OffsetForLeaderEpochResponseTopics3 {
    fn from(older: OffsetForLeaderEpochResponseTopics1) -> Self {
        OffsetForLeaderEpochResponseTopics3 {
            topic: older.topic,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<OffsetForLeaderEpochResponseTopicsPartitions1>
    for OffsetForLeaderEpochResponseTopicsPartitions3
{
    fn from(older: OffsetForLeaderEpochResponseTopicsPartitions1) -> Self {
        OffsetForLeaderEpochResponseTopicsPartitions3 {
            error_code: older.error_code,
            partition: older.partition,
            leader_epoch: older.leader_epoch.map(|val| val),
            end_offset: older.end_offset,
        }
    }
}

impl From<OffsetForLeaderEpochResponse2> for OffsetForLeaderEpochResponse3 {
    fn from(older: OffsetForLeaderEpochResponse2) -> Self {
        OffsetForLeaderEpochResponse3 {
            throttle_time_ms: older.throttle_time_ms.map(|val| val),
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<OffsetForLeaderEpochResponseTopics2> for OffsetForLeaderEpochResponseTopics3 {
    fn from(older: OffsetForLeaderEpochResponseTopics2) -> Self {
        OffsetForLeaderEpochResponseTopics3 {
            topic: older.topic,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<OffsetForLeaderEpochResponseTopicsPartitions2>
    for OffsetForLeaderEpochResponseTopicsPartitions3
{
    fn from(older: OffsetForLeaderEpochResponseTopicsPartitions2) -> Self {
        OffsetForLeaderEpochResponseTopicsPartitions3 {
            error_code: older.error_code,
            partition: older.partition,
            leader_epoch: older.leader_epoch.map(|val| val),
            end_offset: older.end_offset,
        }
    }
}
