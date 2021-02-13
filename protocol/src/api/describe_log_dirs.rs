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
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                DescribeLogDirsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                DescribeLogDirsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &DescribeLogDirsRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &DescribeLogDirsRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, DescribeLogDirsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => DescribeLogDirsResponse0::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            1 => DescribeLogDirsResponse1::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            2 => DescribeLogDirsResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => DescribeLogDirsResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
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
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeLogDirsRequestTopics2 {
    pub topic: String,
    pub partition_index: Vec<Int32>,
    pub tag_buffer: TagBuffer,
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
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResults2 {
    pub error_code: Int16,
    pub log_dir: String,
    pub topics: Vec<DescribeLogDirsResponseResultsTopics2>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResultsTopics2 {
    pub name: String,
    pub partitions: Vec<DescribeLogDirsResponseResultsTopicsPartitions2>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeLogDirsResponseResultsTopicsPartitions2 {
    pub partition_index: Int32,
    pub partition_size: Int64,
    pub offset_lag: Int64,
    pub is_future_key: Boolean,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<DescribeLogDirsRequest2> for DescribeLogDirsRequest0 {
    fn from(latest: DescribeLogDirsRequest2) -> DescribeLogDirsRequest0 {
        DescribeLogDirsRequest0 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<DescribeLogDirsRequestTopics2> for DescribeLogDirsRequestTopics0 {
    fn from(latest: DescribeLogDirsRequestTopics2) -> DescribeLogDirsRequestTopics0 {
        DescribeLogDirsRequestTopics0 {
            topic: latest.topic,
            partition_index: latest.partition_index,
        }
    }
}

impl From<DescribeLogDirsRequest2> for DescribeLogDirsRequest1 {
    fn from(latest: DescribeLogDirsRequest2) -> DescribeLogDirsRequest1 {
        DescribeLogDirsRequest1 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<DescribeLogDirsRequestTopics2> for DescribeLogDirsRequestTopics1 {
    fn from(latest: DescribeLogDirsRequestTopics2) -> DescribeLogDirsRequestTopics1 {
        DescribeLogDirsRequestTopics1 {
            topic: latest.topic,
            partition_index: latest.partition_index,
        }
    }
}

impl From<DescribeLogDirsResponse0> for DescribeLogDirsResponse2 {
    fn from(older: DescribeLogDirsResponse0) -> Self {
        DescribeLogDirsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
            ..DescribeLogDirsResponse2::default()
        }
    }
}

impl From<DescribeLogDirsResponseResults0> for DescribeLogDirsResponseResults2 {
    fn from(older: DescribeLogDirsResponseResults0) -> Self {
        DescribeLogDirsResponseResults2 {
            error_code: older.error_code,
            log_dir: older.log_dir,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..DescribeLogDirsResponseResults2::default()
        }
    }
}

impl From<DescribeLogDirsResponseResultsTopics0> for DescribeLogDirsResponseResultsTopics2 {
    fn from(older: DescribeLogDirsResponseResultsTopics0) -> Self {
        DescribeLogDirsResponseResultsTopics2 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..DescribeLogDirsResponseResultsTopics2::default()
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
            ..DescribeLogDirsResponseResultsTopicsPartitions2::default()
        }
    }
}

impl From<DescribeLogDirsResponse1> for DescribeLogDirsResponse2 {
    fn from(older: DescribeLogDirsResponse1) -> Self {
        DescribeLogDirsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
            ..DescribeLogDirsResponse2::default()
        }
    }
}

impl From<DescribeLogDirsResponseResults1> for DescribeLogDirsResponseResults2 {
    fn from(older: DescribeLogDirsResponseResults1) -> Self {
        DescribeLogDirsResponseResults2 {
            error_code: older.error_code,
            log_dir: older.log_dir,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..DescribeLogDirsResponseResults2::default()
        }
    }
}

impl From<DescribeLogDirsResponseResultsTopics1> for DescribeLogDirsResponseResultsTopics2 {
    fn from(older: DescribeLogDirsResponseResultsTopics1) -> Self {
        DescribeLogDirsResponseResultsTopics2 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..DescribeLogDirsResponseResultsTopics2::default()
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
            ..DescribeLogDirsResponseResultsTopicsPartitions2::default()
        }
    }
}
