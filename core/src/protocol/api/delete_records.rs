use super::prelude::*;

pub type DeleteRecordsRequest = DeleteRecordsRequest2;
pub type DeleteRecordsResponse = DeleteRecordsResponse2;
pub fn serialize_delete_records_request(
    data: DeleteRecordsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&DeleteRecordsRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&DeleteRecordsRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_delete_records_response(version: i32, buf: &mut Bytes) -> DeleteRecordsResponse {
    match version {
        0 => DeleteRecordsResponse0::deserialize(buf).into(),
        1 => DeleteRecordsResponse1::deserialize(buf).into(),
        2 => DeleteRecordsResponse::deserialize(buf),
        _ => DeleteRecordsResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct DeleteRecordsRequest0 {
    pub topics: Vec<DeleteRecordsRequestTopics0>,
    pub timeout_ms: Int32,
}

#[derive(Default, ToBytes)]
pub struct DeleteRecordsRequestTopics0 {
    pub name: String,
    pub partitions: Vec<DeleteRecordsRequestTopicsPartitions0>,
}

#[derive(Default, ToBytes)]
pub struct DeleteRecordsRequestTopicsPartitions0 {
    pub partition_index: Int32,
    pub offset: Int64,
}

#[derive(Default, ToBytes)]
pub struct DeleteRecordsRequest1 {
    pub topics: Vec<DeleteRecordsRequestTopics1>,
    pub timeout_ms: Int32,
}

#[derive(Default, ToBytes)]
pub struct DeleteRecordsRequestTopics1 {
    pub name: String,
    pub partitions: Vec<DeleteRecordsRequestTopicsPartitions1>,
}

#[derive(Default, ToBytes)]
pub struct DeleteRecordsRequestTopicsPartitions1 {
    pub partition_index: Int32,
    pub offset: Int64,
}

#[derive(Default, ToBytes)]
pub struct DeleteRecordsRequest2 {
    pub topics: Vec<DeleteRecordsRequestTopics2>,
    pub timeout_ms: Int32,
}

#[derive(Default, ToBytes)]
pub struct DeleteRecordsRequestTopics2 {
    pub name: CompactString,
    pub partitions: Vec<DeleteRecordsRequestTopicsPartitions2>,
}

#[derive(Default, ToBytes)]
pub struct DeleteRecordsRequestTopicsPartitions2 {
    pub partition_index: Int32,
    pub offset: Int64,
}

#[derive(Default, FromBytes)]
pub struct DeleteRecordsResponse0 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<DeleteRecordsResponseTopics0>,
}

#[derive(Default, FromBytes)]
pub struct DeleteRecordsResponseTopics0 {
    pub name: String,
    pub partitions: Vec<DeleteRecordsResponseTopicsPartitions0>,
}

#[derive(Default, FromBytes)]
pub struct DeleteRecordsResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub low_watermark: Int64,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct DeleteRecordsResponse1 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<DeleteRecordsResponseTopics1>,
}

#[derive(Default, FromBytes)]
pub struct DeleteRecordsResponseTopics1 {
    pub name: String,
    pub partitions: Vec<DeleteRecordsResponseTopicsPartitions1>,
}

#[derive(Default, FromBytes)]
pub struct DeleteRecordsResponseTopicsPartitions1 {
    pub partition_index: Int32,
    pub low_watermark: Int64,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct DeleteRecordsResponse2 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<DeleteRecordsResponseTopics2>,
}

#[derive(Default, FromBytes)]
pub struct DeleteRecordsResponseTopics2 {
    pub name: CompactString,
    pub partitions: Vec<DeleteRecordsResponseTopicsPartitions2>,
}

#[derive(Default, FromBytes)]
pub struct DeleteRecordsResponseTopicsPartitions2 {
    pub partition_index: Int32,
    pub low_watermark: Int64,
    pub error_code: Int16,
}

impl TryFrom<DeleteRecordsRequest2> for DeleteRecordsRequest0 {
    type Error = Error;
    fn try_from(latest: DeleteRecordsRequest2) -> Result<Self, Self::Error> {
        Ok(DeleteRecordsRequest0 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            timeout_ms: latest.timeout_ms,
        })
    }
}

impl TryFrom<DeleteRecordsRequestTopics2> for DeleteRecordsRequestTopics0 {
    type Error = Error;
    fn try_from(latest: DeleteRecordsRequestTopics2) -> Result<Self, Self::Error> {
        Ok(DeleteRecordsRequestTopics0 {
            name: latest.name.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<DeleteRecordsRequestTopicsPartitions2> for DeleteRecordsRequestTopicsPartitions0 {
    type Error = Error;
    fn try_from(latest: DeleteRecordsRequestTopicsPartitions2) -> Result<Self, Self::Error> {
        Ok(DeleteRecordsRequestTopicsPartitions0 {
            partition_index: latest.partition_index,
            offset: latest.offset,
        })
    }
}

impl TryFrom<DeleteRecordsRequest2> for DeleteRecordsRequest1 {
    type Error = Error;
    fn try_from(latest: DeleteRecordsRequest2) -> Result<Self, Self::Error> {
        Ok(DeleteRecordsRequest1 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            timeout_ms: latest.timeout_ms,
        })
    }
}

impl TryFrom<DeleteRecordsRequestTopics2> for DeleteRecordsRequestTopics1 {
    type Error = Error;
    fn try_from(latest: DeleteRecordsRequestTopics2) -> Result<Self, Self::Error> {
        Ok(DeleteRecordsRequestTopics1 {
            name: latest.name.into(),
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<DeleteRecordsRequestTopicsPartitions2> for DeleteRecordsRequestTopicsPartitions1 {
    type Error = Error;
    fn try_from(latest: DeleteRecordsRequestTopicsPartitions2) -> Result<Self, Self::Error> {
        Ok(DeleteRecordsRequestTopicsPartitions1 {
            partition_index: latest.partition_index,
            offset: latest.offset,
        })
    }
}

impl From<DeleteRecordsResponse0> for DeleteRecordsResponse2 {
    fn from(older: DeleteRecordsResponse0) -> Self {
        DeleteRecordsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DeleteRecordsResponseTopics0> for DeleteRecordsResponseTopics2 {
    fn from(older: DeleteRecordsResponseTopics0) -> Self {
        DeleteRecordsResponseTopics2 {
            name: older.name.into(),
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DeleteRecordsResponseTopicsPartitions0> for DeleteRecordsResponseTopicsPartitions2 {
    fn from(older: DeleteRecordsResponseTopicsPartitions0) -> Self {
        DeleteRecordsResponseTopicsPartitions2 {
            partition_index: older.partition_index,
            low_watermark: older.low_watermark,
            error_code: older.error_code,
        }
    }
}

impl From<DeleteRecordsResponse1> for DeleteRecordsResponse2 {
    fn from(older: DeleteRecordsResponse1) -> Self {
        DeleteRecordsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DeleteRecordsResponseTopics1> for DeleteRecordsResponseTopics2 {
    fn from(older: DeleteRecordsResponseTopics1) -> Self {
        DeleteRecordsResponseTopics2 {
            name: older.name.into(),
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DeleteRecordsResponseTopicsPartitions1> for DeleteRecordsResponseTopicsPartitions2 {
    fn from(older: DeleteRecordsResponseTopicsPartitions1) -> Self {
        DeleteRecordsResponseTopicsPartitions2 {
            partition_index: older.partition_index,
            low_watermark: older.low_watermark,
            error_code: older.error_code,
        }
    }
}
