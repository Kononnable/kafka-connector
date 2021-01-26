use super::prelude::*;

pub type TxnOffsetCommitRequest = TxnOffsetCommitRequest3;
pub type TxnOffsetCommitResponse = TxnOffsetCommitResponse3;
pub fn serialize_txn_offset_commit_request(
    data: TxnOffsetCommitRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&TxnOffsetCommitRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&TxnOffsetCommitRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&TxnOffsetCommitRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_txn_offset_commit_response(
    version: i32,
    buf: &mut Bytes,
) -> TxnOffsetCommitResponse {
    match version {
        0 => TxnOffsetCommitResponse0::deserialize(buf).into(),
        1 => TxnOffsetCommitResponse1::deserialize(buf).into(),
        2 => TxnOffsetCommitResponse2::deserialize(buf).into(),
        3 => TxnOffsetCommitResponse::deserialize(buf),
        _ => TxnOffsetCommitResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct TxnOffsetCommitRequest0 {
    pub transactional_id: String,
    pub group_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub topics: Vec<TxnOffsetCommitRequestTopics0>,
}

#[derive(Default, ToBytes)]
pub struct TxnOffsetCommitRequestTopics0 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitRequestTopicsPartitions0>,
}

#[derive(Default, ToBytes)]
pub struct TxnOffsetCommitRequestTopicsPartitions0 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, ToBytes)]
pub struct TxnOffsetCommitRequest1 {
    pub transactional_id: String,
    pub group_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub topics: Vec<TxnOffsetCommitRequestTopics1>,
}

#[derive(Default, ToBytes)]
pub struct TxnOffsetCommitRequestTopics1 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitRequestTopicsPartitions1>,
}

#[derive(Default, ToBytes)]
pub struct TxnOffsetCommitRequestTopicsPartitions1 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_metadata: NullableString,
}

#[derive(Default, ToBytes)]
pub struct TxnOffsetCommitRequest2 {
    pub transactional_id: String,
    pub group_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub topics: Vec<TxnOffsetCommitRequestTopics2>,
}

#[derive(Default, ToBytes)]
pub struct TxnOffsetCommitRequestTopics2 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitRequestTopicsPartitions2>,
}

#[derive(Default, ToBytes)]
pub struct TxnOffsetCommitRequestTopicsPartitions2 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Optional<Int32>,
    pub committed_metadata: NullableString,
}

#[derive(Default, ToBytes)]
pub struct TxnOffsetCommitRequest3 {
    pub transactional_id: CompactString,
    pub group_id: CompactString,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub generation_id: Optional<Int32>,
    pub member_id: Optional<CompactString>,
    pub group_instance_id: Optional<CompactNullableString>,
    pub topics: Vec<TxnOffsetCommitRequestTopics3>,
}

#[derive(Default, ToBytes)]
pub struct TxnOffsetCommitRequestTopics3 {
    pub name: CompactString,
    pub partitions: Vec<TxnOffsetCommitRequestTopicsPartitions3>,
}

#[derive(Default, ToBytes)]
pub struct TxnOffsetCommitRequestTopicsPartitions3 {
    pub partition_index: Int32,
    pub committed_offset: Int64,
    pub committed_leader_epoch: Optional<Int32>,
    pub committed_metadata: CompactNullableString,
}

#[derive(Default, FromBytes)]
pub struct TxnOffsetCommitResponse0 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<TxnOffsetCommitResponseTopics0>,
}

#[derive(Default, FromBytes)]
pub struct TxnOffsetCommitResponseTopics0 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitResponseTopicsPartitions0>,
}

#[derive(Default, FromBytes)]
pub struct TxnOffsetCommitResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct TxnOffsetCommitResponse1 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<TxnOffsetCommitResponseTopics1>,
}

#[derive(Default, FromBytes)]
pub struct TxnOffsetCommitResponseTopics1 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitResponseTopicsPartitions1>,
}

#[derive(Default, FromBytes)]
pub struct TxnOffsetCommitResponseTopicsPartitions1 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct TxnOffsetCommitResponse2 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<TxnOffsetCommitResponseTopics2>,
}

#[derive(Default, FromBytes)]
pub struct TxnOffsetCommitResponseTopics2 {
    pub name: String,
    pub partitions: Vec<TxnOffsetCommitResponseTopicsPartitions2>,
}

#[derive(Default, FromBytes)]
pub struct TxnOffsetCommitResponseTopicsPartitions2 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct TxnOffsetCommitResponse3 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<TxnOffsetCommitResponseTopics3>,
}

#[derive(Default, FromBytes)]
pub struct TxnOffsetCommitResponseTopics3 {
    pub name: CompactString,
    pub partitions: Vec<TxnOffsetCommitResponseTopicsPartitions3>,
}

#[derive(Default, FromBytes)]
pub struct TxnOffsetCommitResponseTopicsPartitions3 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

impl TryFrom<TxnOffsetCommitRequest3> for TxnOffsetCommitRequest0 {
    type Error = Error;
    fn try_from(latest: TxnOffsetCommitRequest3) -> Result<Self, Self::Error> {
        if latest.generation_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "TxnOffsetCommitRequest",
                0,
                "generation_id",
            ));
        }
        if latest.member_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "TxnOffsetCommitRequest",
                0,
                "member_id",
            ));
        }
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "TxnOffsetCommitRequest",
                0,
                "group_instance_id",
            ));
        }
        Ok(TxnOffsetCommitRequest0 {
            transactional_id: latest.transactional_id.into(),
            group_id: latest.group_id.into(),
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<TxnOffsetCommitRequestTopics3> for TxnOffsetCommitRequestTopics0 {
    type Error = Error;
    fn try_from(latest: TxnOffsetCommitRequestTopics3) -> Result<Self, Self::Error> {
        Ok(TxnOffsetCommitRequestTopics0 {
            name: latest.name.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<TxnOffsetCommitRequestTopicsPartitions3> for TxnOffsetCommitRequestTopicsPartitions0 {
    type Error = Error;
    fn try_from(latest: TxnOffsetCommitRequestTopicsPartitions3) -> Result<Self, Self::Error> {
        if latest.committed_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "TxnOffsetCommitRequestTopicsPartitions",
                0,
                "committed_leader_epoch",
            ));
        }
        Ok(TxnOffsetCommitRequestTopicsPartitions0 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        })
    }
}

impl TryFrom<TxnOffsetCommitRequest3> for TxnOffsetCommitRequest1 {
    type Error = Error;
    fn try_from(latest: TxnOffsetCommitRequest3) -> Result<Self, Self::Error> {
        if latest.generation_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "TxnOffsetCommitRequest",
                1,
                "generation_id",
            ));
        }
        if latest.member_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "TxnOffsetCommitRequest",
                1,
                "member_id",
            ));
        }
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "TxnOffsetCommitRequest",
                1,
                "group_instance_id",
            ));
        }
        Ok(TxnOffsetCommitRequest1 {
            transactional_id: latest.transactional_id.into(),
            group_id: latest.group_id.into(),
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<TxnOffsetCommitRequestTopics3> for TxnOffsetCommitRequestTopics1 {
    type Error = Error;
    fn try_from(latest: TxnOffsetCommitRequestTopics3) -> Result<Self, Self::Error> {
        Ok(TxnOffsetCommitRequestTopics1 {
            name: latest.name.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<TxnOffsetCommitRequestTopicsPartitions3> for TxnOffsetCommitRequestTopicsPartitions1 {
    type Error = Error;
    fn try_from(latest: TxnOffsetCommitRequestTopicsPartitions3) -> Result<Self, Self::Error> {
        if latest.committed_leader_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "TxnOffsetCommitRequestTopicsPartitions",
                1,
                "committed_leader_epoch",
            ));
        }
        Ok(TxnOffsetCommitRequestTopicsPartitions1 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_metadata: latest.committed_metadata,
        })
    }
}

impl TryFrom<TxnOffsetCommitRequest3> for TxnOffsetCommitRequest2 {
    type Error = Error;
    fn try_from(latest: TxnOffsetCommitRequest3) -> Result<Self, Self::Error> {
        if latest.generation_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "TxnOffsetCommitRequest",
                2,
                "generation_id",
            ));
        }
        if latest.member_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "TxnOffsetCommitRequest",
                2,
                "member_id",
            ));
        }
        if latest.group_instance_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "TxnOffsetCommitRequest",
                2,
                "group_instance_id",
            ));
        }
        Ok(TxnOffsetCommitRequest2 {
            transactional_id: latest.transactional_id.into(),
            group_id: latest.group_id.into(),
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<TxnOffsetCommitRequestTopics3> for TxnOffsetCommitRequestTopics2 {
    type Error = Error;
    fn try_from(latest: TxnOffsetCommitRequestTopics3) -> Result<Self, Self::Error> {
        Ok(TxnOffsetCommitRequestTopics2 {
            name: latest.name.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<TxnOffsetCommitRequestTopicsPartitions3> for TxnOffsetCommitRequestTopicsPartitions2 {
    type Error = Error;
    fn try_from(latest: TxnOffsetCommitRequestTopicsPartitions3) -> Result<Self, Self::Error> {
        Ok(TxnOffsetCommitRequestTopicsPartitions2 {
            partition_index: latest.partition_index,
            committed_offset: latest.committed_offset,
            committed_leader_epoch: latest.committed_leader_epoch,
            committed_metadata: latest.committed_metadata,
        })
    }
}

impl From<TxnOffsetCommitResponse0> for TxnOffsetCommitResponse3 {
    fn from(older: TxnOffsetCommitResponse0) -> Self {
        TxnOffsetCommitResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<TxnOffsetCommitResponseTopics0> for TxnOffsetCommitResponseTopics3 {
    fn from(older: TxnOffsetCommitResponseTopics0) -> Self {
        TxnOffsetCommitResponseTopics3 {
            name: older.name.into(),
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<TxnOffsetCommitResponseTopicsPartitions0> for TxnOffsetCommitResponseTopicsPartitions3 {
    fn from(older: TxnOffsetCommitResponseTopicsPartitions0) -> Self {
        TxnOffsetCommitResponseTopicsPartitions3 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}

impl From<TxnOffsetCommitResponse1> for TxnOffsetCommitResponse3 {
    fn from(older: TxnOffsetCommitResponse1) -> Self {
        TxnOffsetCommitResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<TxnOffsetCommitResponseTopics1> for TxnOffsetCommitResponseTopics3 {
    fn from(older: TxnOffsetCommitResponseTopics1) -> Self {
        TxnOffsetCommitResponseTopics3 {
            name: older.name.into(),
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<TxnOffsetCommitResponseTopicsPartitions1> for TxnOffsetCommitResponseTopicsPartitions3 {
    fn from(older: TxnOffsetCommitResponseTopicsPartitions1) -> Self {
        TxnOffsetCommitResponseTopicsPartitions3 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}

impl From<TxnOffsetCommitResponse2> for TxnOffsetCommitResponse3 {
    fn from(older: TxnOffsetCommitResponse2) -> Self {
        TxnOffsetCommitResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<TxnOffsetCommitResponseTopics2> for TxnOffsetCommitResponseTopics3 {
    fn from(older: TxnOffsetCommitResponseTopics2) -> Self {
        TxnOffsetCommitResponseTopics3 {
            name: older.name.into(),
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<TxnOffsetCommitResponseTopicsPartitions2> for TxnOffsetCommitResponseTopicsPartitions3 {
    fn from(older: TxnOffsetCommitResponseTopicsPartitions2) -> Self {
        TxnOffsetCommitResponseTopicsPartitions3 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}
