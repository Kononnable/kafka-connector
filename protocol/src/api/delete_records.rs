use super::prelude::*;

pub type DeleteRecordsRequest = DeleteRecordsRequest2;
pub type DeleteRecordsResponse = DeleteRecordsResponse2;
impl ApiCall for DeleteRecordsRequest {
    type Response = DeleteRecordsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DeleteRecords
    }
    fn get_first_error(response: &DeleteRecordsResponse) -> Option<ApiError> {
        DeleteRecordsResponse::get_first_error(response)
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
                DeleteRecordsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                DeleteRecordsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &DeleteRecordsRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &DeleteRecordsRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, DeleteRecordsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response =
            match version {
                0 => DeleteRecordsResponse0::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                1 => DeleteRecordsResponse1::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                2 => DeleteRecordsResponse::deserialize(buf, Self::is_flexible_version(version)),
                _ => DeleteRecordsResponse::deserialize(buf, Self::is_flexible_version(version)),
            };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteRecordsRequest0 {
    pub topics: Vec<DeleteRecordsRequestTopics0>,
    pub timeout_ms: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteRecordsRequestTopics0 {
    pub name: String,
    pub partitions: Vec<DeleteRecordsRequestTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteRecordsRequestTopicsPartitions0 {
    pub partition_index: Int32,
    pub offset: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteRecordsRequest1 {
    pub topics: Vec<DeleteRecordsRequestTopics1>,
    pub timeout_ms: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteRecordsRequestTopics1 {
    pub name: String,
    pub partitions: Vec<DeleteRecordsRequestTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteRecordsRequestTopicsPartitions1 {
    pub partition_index: Int32,
    pub offset: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteRecordsRequest2 {
    pub topics: Vec<DeleteRecordsRequestTopics2>,
    pub timeout_ms: Int32,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteRecordsRequestTopics2 {
    pub name: String,
    pub partitions: Vec<DeleteRecordsRequestTopicsPartitions2>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteRecordsRequestTopicsPartitions2 {
    pub partition_index: Int32,
    pub offset: Int64,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteRecordsResponse0 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<DeleteRecordsResponseTopics0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteRecordsResponseTopics0 {
    pub name: String,
    pub partitions: Vec<DeleteRecordsResponseTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteRecordsResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub low_watermark: Int64,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteRecordsResponse1 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<DeleteRecordsResponseTopics1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteRecordsResponseTopics1 {
    pub name: String,
    pub partitions: Vec<DeleteRecordsResponseTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteRecordsResponseTopicsPartitions1 {
    pub partition_index: Int32,
    pub low_watermark: Int64,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteRecordsResponse2 {
    pub throttle_time_ms: Int32,
    pub topics: Vec<DeleteRecordsResponseTopics2>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteRecordsResponseTopics2 {
    pub name: String,
    pub partitions: Vec<DeleteRecordsResponseTopicsPartitions2>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteRecordsResponseTopicsPartitions2 {
    pub partition_index: Int32,
    pub low_watermark: Int64,
    pub error_code: Int16,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<DeleteRecordsRequest2> for DeleteRecordsRequest0 {
    fn from(latest: DeleteRecordsRequest2) -> DeleteRecordsRequest0 {
        DeleteRecordsRequest0 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            timeout_ms: latest.timeout_ms,
        }
    }
}

impl From<DeleteRecordsRequestTopics2> for DeleteRecordsRequestTopics0 {
    fn from(latest: DeleteRecordsRequestTopics2) -> DeleteRecordsRequestTopics0 {
        DeleteRecordsRequestTopics0 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<DeleteRecordsRequestTopicsPartitions2> for DeleteRecordsRequestTopicsPartitions0 {
    fn from(
        latest: DeleteRecordsRequestTopicsPartitions2,
    ) -> DeleteRecordsRequestTopicsPartitions0 {
        DeleteRecordsRequestTopicsPartitions0 {
            partition_index: latest.partition_index,
            offset: latest.offset,
        }
    }
}

impl From<DeleteRecordsRequest2> for DeleteRecordsRequest1 {
    fn from(latest: DeleteRecordsRequest2) -> DeleteRecordsRequest1 {
        DeleteRecordsRequest1 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            timeout_ms: latest.timeout_ms,
        }
    }
}

impl From<DeleteRecordsRequestTopics2> for DeleteRecordsRequestTopics1 {
    fn from(latest: DeleteRecordsRequestTopics2) -> DeleteRecordsRequestTopics1 {
        DeleteRecordsRequestTopics1 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<DeleteRecordsRequestTopicsPartitions2> for DeleteRecordsRequestTopicsPartitions1 {
    fn from(
        latest: DeleteRecordsRequestTopicsPartitions2,
    ) -> DeleteRecordsRequestTopicsPartitions1 {
        DeleteRecordsRequestTopicsPartitions1 {
            partition_index: latest.partition_index,
            offset: latest.offset,
        }
    }
}

impl From<DeleteRecordsResponse0> for DeleteRecordsResponse2 {
    fn from(older: DeleteRecordsResponse0) -> Self {
        DeleteRecordsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..DeleteRecordsResponse2::default()
        }
    }
}

impl From<DeleteRecordsResponseTopics0> for DeleteRecordsResponseTopics2 {
    fn from(older: DeleteRecordsResponseTopics0) -> Self {
        DeleteRecordsResponseTopics2 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..DeleteRecordsResponseTopics2::default()
        }
    }
}

impl From<DeleteRecordsResponseTopicsPartitions0> for DeleteRecordsResponseTopicsPartitions2 {
    fn from(older: DeleteRecordsResponseTopicsPartitions0) -> Self {
        DeleteRecordsResponseTopicsPartitions2 {
            partition_index: older.partition_index,
            low_watermark: older.low_watermark,
            error_code: older.error_code,
            ..DeleteRecordsResponseTopicsPartitions2::default()
        }
    }
}

impl From<DeleteRecordsResponse1> for DeleteRecordsResponse2 {
    fn from(older: DeleteRecordsResponse1) -> Self {
        DeleteRecordsResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..DeleteRecordsResponse2::default()
        }
    }
}

impl From<DeleteRecordsResponseTopics1> for DeleteRecordsResponseTopics2 {
    fn from(older: DeleteRecordsResponseTopics1) -> Self {
        DeleteRecordsResponseTopics2 {
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..DeleteRecordsResponseTopics2::default()
        }
    }
}

impl From<DeleteRecordsResponseTopicsPartitions1> for DeleteRecordsResponseTopicsPartitions2 {
    fn from(older: DeleteRecordsResponseTopicsPartitions1) -> Self {
        DeleteRecordsResponseTopicsPartitions2 {
            partition_index: older.partition_index,
            low_watermark: older.low_watermark,
            error_code: older.error_code,
            ..DeleteRecordsResponseTopicsPartitions2::default()
        }
    }
}

impl DeleteRecordsResponse2 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.topics.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DeleteRecordsResponseTopics2 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partitions.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DeleteRecordsResponseTopicsPartitions2 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
