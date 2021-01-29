use super::prelude::*;

pub type DescribeLogDirsRequest = DescribeLogDirsRequest2;
pub type DescribeLogDirsResponse = DescribeLogDirsResponse2;
impl ApiCall for DescribeLogDirsRequest {
    type Response = DescribeLogDirsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DescribeLogDirs
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&DescribeLogDirsRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&DescribeLogDirsRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> DescribeLogDirsResponse {
        match version {
            0 => DescribeLogDirsResponse0::deserialize(buf).into(),
            1 => DescribeLogDirsResponse1::deserialize(buf).into(),
            2 => DescribeLogDirsResponse::deserialize(buf),
            _ => DescribeLogDirsResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeLogDirsRequest0 {
    pub topics: Vec<DescribeLogDirsRequestTopics0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeLogDirsRequestTopics0 {
    pub topic: String,
    pub partition_index: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeLogDirsRequest1 {
    pub topics: Vec<DescribeLogDirsRequestTopics1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeLogDirsRequestTopics1 {
    pub topic: String,
    pub partition_index: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeLogDirsRequest2 {
    pub topics: Vec<DescribeLogDirsRequestTopics2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeLogDirsRequestTopics2 {
    pub topic: CompactString,
    pub partition_index: Vec<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<DescribeLogDirsResponseResults0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResults0 {
    pub error_code: Int16,
    pub log_dir: String,
    pub topics: Vec<DescribeLogDirsResponseResultsTopics0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResultsTopics0 {
    pub name: String,
    pub partitions: Vec<DescribeLogDirsResponseResultsTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResultsTopicsPartitions0 {
    pub partition_index: Int32,
    pub partition_size: Int64,
    pub offset_lag: Int64,
    pub is_future_key: Boolean,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponse1 {
    pub throttle_time_ms: Int32,
    pub results: Vec<DescribeLogDirsResponseResults1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResults1 {
    pub error_code: Int16,
    pub log_dir: String,
    pub topics: Vec<DescribeLogDirsResponseResultsTopics1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResultsTopics1 {
    pub name: String,
    pub partitions: Vec<DescribeLogDirsResponseResultsTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResultsTopicsPartitions1 {
    pub partition_index: Int32,
    pub partition_size: Int64,
    pub offset_lag: Int64,
    pub is_future_key: Boolean,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponse2 {
    pub throttle_time_ms: Int32,
    pub results: Vec<DescribeLogDirsResponseResults2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResults2 {
    pub error_code: Int16,
    pub log_dir: CompactString,
    pub topics: Vec<DescribeLogDirsResponseResultsTopics2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResultsTopics2 {
    pub name: CompactString,
    pub partitions: Vec<DescribeLogDirsResponseResultsTopicsPartitions2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResultsTopicsPartitions2 {
    pub partition_index: Int32,
    pub partition_size: Int64,
    pub offset_lag: Int64,
    pub is_future_key: Boolean,
}

impl TryFrom<DescribeLogDirsRequest2> for DescribeLogDirsRequest0 {
    type Error = Error;
    fn try_from(latest: DescribeLogDirsRequest2) -> Result<Self, Self::Error> {
        Ok(DescribeLogDirsRequest0 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<DescribeLogDirsRequestTopics2> for DescribeLogDirsRequestTopics0 {
    type Error = Error;
    fn try_from(latest: DescribeLogDirsRequestTopics2) -> Result<Self, Self::Error> {
        Ok(DescribeLogDirsRequestTopics0 {
            topic: latest.topic.into(),
            partition_index: latest.partition_index,
        })
    }
}

impl TryFrom<DescribeLogDirsRequest2> for DescribeLogDirsRequest1 {
    type Error = Error;
    fn try_from(latest: DescribeLogDirsRequest2) -> Result<Self, Self::Error> {
        Ok(DescribeLogDirsRequest1 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<DescribeLogDirsRequestTopics2> for DescribeLogDirsRequestTopics1 {
    type Error = Error;
    fn try_from(latest: DescribeLogDirsRequestTopics2) -> Result<Self, Self::Error> {
        Ok(DescribeLogDirsRequestTopics1 {
            topic: latest.topic.into(),
            partition_index: latest.partition_index,
        })
    }
}

impl From<DescribeLogDirsResponse0> for DescribeLogDirsResponse2 {
    fn from(older: DescribeLogDirsResponse0) -> Self {
        DescribeLogDirsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeLogDirsResponseResults0> for DescribeLogDirsResponseResults2 {
    fn from(older: DescribeLogDirsResponseResults0) -> Self {
        DescribeLogDirsResponseResults2 {
            error_code: older.error_code,
            log_dir: older.log_dir.into(),
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeLogDirsResponseResultsTopics0> for DescribeLogDirsResponseResultsTopics2 {
    fn from(older: DescribeLogDirsResponseResultsTopics0) -> Self {
        DescribeLogDirsResponseResultsTopics2 {
            name: older.name.into(),
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeLogDirsResponseResultsTopicsPartitions0>
    for DescribeLogDirsResponseResultsTopicsPartitions2
{
    fn from(older: DescribeLogDirsResponseResultsTopicsPartitions0) -> Self {
        DescribeLogDirsResponseResultsTopicsPartitions2 {
            partition_index: older.partition_index,
            partition_size: older.partition_size,
            offset_lag: older.offset_lag,
            is_future_key: older.is_future_key,
        }
    }
}

impl From<DescribeLogDirsResponse1> for DescribeLogDirsResponse2 {
    fn from(older: DescribeLogDirsResponse1) -> Self {
        DescribeLogDirsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeLogDirsResponseResults1> for DescribeLogDirsResponseResults2 {
    fn from(older: DescribeLogDirsResponseResults1) -> Self {
        DescribeLogDirsResponseResults2 {
            error_code: older.error_code,
            log_dir: older.log_dir.into(),
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeLogDirsResponseResultsTopics1> for DescribeLogDirsResponseResultsTopics2 {
    fn from(older: DescribeLogDirsResponseResultsTopics1) -> Self {
        DescribeLogDirsResponseResultsTopics2 {
            name: older.name.into(),
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<DescribeLogDirsResponseResultsTopicsPartitions1>
    for DescribeLogDirsResponseResultsTopicsPartitions2
{
    fn from(older: DescribeLogDirsResponseResultsTopicsPartitions1) -> Self {
        DescribeLogDirsResponseResultsTopicsPartitions2 {
            partition_index: older.partition_index,
            partition_size: older.partition_size,
            offset_lag: older.offset_lag,
            is_future_key: older.is_future_key,
        }
    }
}
