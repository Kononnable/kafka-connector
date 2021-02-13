use super::prelude::*;

pub type OffsetForLeaderEpochRequest = OffsetForLeaderEpochRequest3;
pub type OffsetForLeaderEpochResponse = OffsetForLeaderEpochResponse3;
impl ApiCall for OffsetForLeaderEpochRequest {
    type Response = OffsetForLeaderEpochResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::OffsetForLeaderEpoch
    }
    fn get_first_error(response: &OffsetForLeaderEpochResponse) -> Option<ApiError> {
        OffsetForLeaderEpochResponse::get_first_error(response)
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            _ => false,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                OffsetForLeaderEpochRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                OffsetForLeaderEpochRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &OffsetForLeaderEpochRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &OffsetForLeaderEpochRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &OffsetForLeaderEpochRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, OffsetForLeaderEpochResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => {
                OffsetForLeaderEpochResponse0::deserialize(buf, Self::is_flexible_version(version))
                    .into()
            }
            1 => {
                OffsetForLeaderEpochResponse1::deserialize(buf, Self::is_flexible_version(version))
                    .into()
            }
            2 => {
                OffsetForLeaderEpochResponse2::deserialize(buf, Self::is_flexible_version(version))
                    .into()
            }
            3 => OffsetForLeaderEpochResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => OffsetForLeaderEpochResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequest0 {
    pub topics: Vec<OffsetForLeaderEpochRequestTopics0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopics0 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochRequestTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopicsPartitions0 {
    pub partition: Int32,
    pub leader_epoch: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequest1 {
    pub topics: Vec<OffsetForLeaderEpochRequestTopics1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopics1 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochRequestTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopicsPartitions1 {
    pub partition: Int32,
    pub leader_epoch: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequest2 {
    pub topics: Vec<OffsetForLeaderEpochRequestTopics2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopics2 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochRequestTopicsPartitions2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopicsPartitions2 {
    pub partition: Int32,
    pub current_leader_epoch: Int32,
    pub leader_epoch: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequest3 {
    pub replica_id: Int32,
    pub topics: Vec<OffsetForLeaderEpochRequestTopics3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopics3 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochRequestTopicsPartitions3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct OffsetForLeaderEpochRequestTopicsPartitions3 {
    pub partition: Int32,
    pub current_leader_epoch: Int32,
    pub leader_epoch: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponse0 {
    pub topics: Vec<OffsetForLeaderEpochResponseTopics0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopics0 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochResponseTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopicsPartitions0 {
    pub error_code: Int16,
    pub partition: Int32,
    pub end_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponse1 {
    pub topics: Vec<OffsetForLeaderEpochResponseTopics1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopics1 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochResponseTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopicsPartitions1 {
    pub error_code: Int16,
    pub partition: Int32,
    pub leader_epoch: Option<Int32>,
    pub end_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponse2 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<OffsetForLeaderEpochResponseTopics2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopics2 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochResponseTopicsPartitions2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopicsPartitions2 {
    pub error_code: Int16,
    pub partition: Int32,
    pub leader_epoch: Option<Int32>,
    pub end_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponse3 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<OffsetForLeaderEpochResponseTopics3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopics3 {
    pub topic: String,
    pub partitions: Vec<OffsetForLeaderEpochResponseTopicsPartitions3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct OffsetForLeaderEpochResponseTopicsPartitions3 {
    pub error_code: Int16,
    pub partition: Int32,
    pub leader_epoch: Option<Int32>,
    pub end_offset: Int64,
}

impl From<OffsetForLeaderEpochRequest3> for OffsetForLeaderEpochRequest0 {
    fn from(latest: OffsetForLeaderEpochRequest3) -> OffsetForLeaderEpochRequest0 {
        log::debug!(
            "Using old api format - OffsetForLeaderEpochRequest0, ignoring field replica_id"
        );
        OffsetForLeaderEpochRequest0 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetForLeaderEpochRequestTopics3> for OffsetForLeaderEpochRequestTopics0 {
    fn from(latest: OffsetForLeaderEpochRequestTopics3) -> OffsetForLeaderEpochRequestTopics0 {
        OffsetForLeaderEpochRequestTopics0 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<OffsetForLeaderEpochRequestTopicsPartitions3>
    for OffsetForLeaderEpochRequestTopicsPartitions0
{
    fn from(
        latest: OffsetForLeaderEpochRequestTopicsPartitions3,
    ) -> OffsetForLeaderEpochRequestTopicsPartitions0 {
        log::debug!("Using old api format - OffsetForLeaderEpochRequestTopicsPartitions0, ignoring field current_leader_epoch");
        OffsetForLeaderEpochRequestTopicsPartitions0 {
            partition: latest.partition,
            leader_epoch: latest.leader_epoch,
        }
    }
}

impl From<OffsetForLeaderEpochRequest3> for OffsetForLeaderEpochRequest1 {
    fn from(latest: OffsetForLeaderEpochRequest3) -> OffsetForLeaderEpochRequest1 {
        log::debug!(
            "Using old api format - OffsetForLeaderEpochRequest1, ignoring field replica_id"
        );
        OffsetForLeaderEpochRequest1 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetForLeaderEpochRequestTopics3> for OffsetForLeaderEpochRequestTopics1 {
    fn from(latest: OffsetForLeaderEpochRequestTopics3) -> OffsetForLeaderEpochRequestTopics1 {
        OffsetForLeaderEpochRequestTopics1 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<OffsetForLeaderEpochRequestTopicsPartitions3>
    for OffsetForLeaderEpochRequestTopicsPartitions1
{
    fn from(
        latest: OffsetForLeaderEpochRequestTopicsPartitions3,
    ) -> OffsetForLeaderEpochRequestTopicsPartitions1 {
        log::debug!("Using old api format - OffsetForLeaderEpochRequestTopicsPartitions1, ignoring field current_leader_epoch");
        OffsetForLeaderEpochRequestTopicsPartitions1 {
            partition: latest.partition,
            leader_epoch: latest.leader_epoch,
        }
    }
}

impl From<OffsetForLeaderEpochRequest3> for OffsetForLeaderEpochRequest2 {
    fn from(latest: OffsetForLeaderEpochRequest3) -> OffsetForLeaderEpochRequest2 {
        log::debug!(
            "Using old api format - OffsetForLeaderEpochRequest2, ignoring field replica_id"
        );
        OffsetForLeaderEpochRequest2 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<OffsetForLeaderEpochRequestTopics3> for OffsetForLeaderEpochRequestTopics2 {
    fn from(latest: OffsetForLeaderEpochRequestTopics3) -> OffsetForLeaderEpochRequestTopics2 {
        OffsetForLeaderEpochRequestTopics2 {
            topic: latest.topic,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<OffsetForLeaderEpochRequestTopicsPartitions3>
    for OffsetForLeaderEpochRequestTopicsPartitions2
{
    fn from(
        latest: OffsetForLeaderEpochRequestTopicsPartitions3,
    ) -> OffsetForLeaderEpochRequestTopicsPartitions2 {
        OffsetForLeaderEpochRequestTopicsPartitions2 {
            partition: latest.partition,
            current_leader_epoch: latest.current_leader_epoch,
            leader_epoch: latest.leader_epoch,
        }
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
            leader_epoch: older.leader_epoch,
            end_offset: older.end_offset,
        }
    }
}

impl From<OffsetForLeaderEpochResponse2> for OffsetForLeaderEpochResponse3 {
    fn from(older: OffsetForLeaderEpochResponse2) -> Self {
        OffsetForLeaderEpochResponse3 {
            throttle_time_ms: older.throttle_time_ms,
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
            leader_epoch: older.leader_epoch,
            end_offset: older.end_offset,
        }
    }
}

impl OffsetForLeaderEpochResponse3 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.topics.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl OffsetForLeaderEpochResponseTopics3 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partitions.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl OffsetForLeaderEpochResponseTopicsPartitions3 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
