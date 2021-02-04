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
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => false,
            5 => false,
            _ => false,
        }
    }
    fn serialize(
        self,
        version: i16,
        buf: &mut BytesMut,
        correlation_id: i32,
        client_id: &str,
    ) -> Result<(), Error> {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                ListOffsetsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                ListOffsetsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &ListOffsetsRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &ListOffsetsRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &ListOffsetsRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &ListOffsetsRequest3::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(
                &ListOffsetsRequest4::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            5 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, ListOffsetsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => ListOffsetsResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => ListOffsetsResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => ListOffsetsResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => ListOffsetsResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => ListOffsetsResponse4::deserialize(buf, Self::is_flexible_version(version)).into(),
            5 => ListOffsetsResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => ListOffsetsResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
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
    pub isolation_level: Int8,
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
    pub isolation_level: Int8,
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
    pub isolation_level: Int8,
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
    pub current_leader_epoch: Int32,
    pub timestamp: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ListOffsetsRequest5 {
    pub replica_id: Int32,
    pub isolation_level: Int8,
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
    pub current_leader_epoch: Int32,
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
    pub timestamp: Option<Int64>,
    pub offset: Option<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponse2 {
    pub throttle_time_ms: Option<Int32>,
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
    pub timestamp: Option<Int64>,
    pub offset: Option<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponse3 {
    pub throttle_time_ms: Option<Int32>,
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
    pub timestamp: Option<Int64>,
    pub offset: Option<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponse4 {
    pub throttle_time_ms: Option<Int32>,
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
    pub timestamp: Option<Int64>,
    pub offset: Option<Int64>,
    pub leader_epoch: Option<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ListOffsetsResponse5 {
    pub throttle_time_ms: Option<Int32>,
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
    pub timestamp: Option<Int64>,
    pub offset: Option<Int64>,
    pub leader_epoch: Option<Int32>,
}

impl From<ListOffsetsRequest5> for ListOffsetsRequest0 {
    fn from(latest: ListOffsetsRequest5) -> ListOffsetsRequest0 {
        log::debug!("Using old api format - ListOffsetsRequest0, ignoring field isolation_level");
        ListOffsetsRequest0 {
            replica_id: latest.replica_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<ListOffsetsRequestTopics5> for ListOffsetsRequestTopics0 {
    fn from(latest: ListOffsetsRequestTopics5) -> ListOffsetsRequestTopics0 {
        ListOffsetsRequestTopics0 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<ListOffsetsRequestTopicsPartitions5> for ListOffsetsRequestTopicsPartitions0 {
    fn from(latest: ListOffsetsRequestTopicsPartitions5) -> ListOffsetsRequestTopicsPartitions0 {
        log::debug!("Using old api format - ListOffsetsRequestTopicsPartitions0, ignoring field current_leader_epoch");
        ListOffsetsRequestTopicsPartitions0 {
            partition_index: latest.partition_index,
            timestamp: latest.timestamp,
            ..ListOffsetsRequestTopicsPartitions0::default()
        }
    }
}

impl From<ListOffsetsRequest5> for ListOffsetsRequest1 {
    fn from(latest: ListOffsetsRequest5) -> ListOffsetsRequest1 {
        log::debug!("Using old api format - ListOffsetsRequest1, ignoring field isolation_level");
        ListOffsetsRequest1 {
            replica_id: latest.replica_id,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<ListOffsetsRequestTopics5> for ListOffsetsRequestTopics1 {
    fn from(latest: ListOffsetsRequestTopics5) -> ListOffsetsRequestTopics1 {
        ListOffsetsRequestTopics1 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<ListOffsetsRequestTopicsPartitions5> for ListOffsetsRequestTopicsPartitions1 {
    fn from(latest: ListOffsetsRequestTopicsPartitions5) -> ListOffsetsRequestTopicsPartitions1 {
        log::debug!("Using old api format - ListOffsetsRequestTopicsPartitions1, ignoring field current_leader_epoch");
        ListOffsetsRequestTopicsPartitions1 {
            partition_index: latest.partition_index,
            timestamp: latest.timestamp,
        }
    }
}

impl From<ListOffsetsRequest5> for ListOffsetsRequest2 {
    fn from(latest: ListOffsetsRequest5) -> ListOffsetsRequest2 {
        ListOffsetsRequest2 {
            replica_id: latest.replica_id,
            isolation_level: latest.isolation_level,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<ListOffsetsRequestTopics5> for ListOffsetsRequestTopics2 {
    fn from(latest: ListOffsetsRequestTopics5) -> ListOffsetsRequestTopics2 {
        ListOffsetsRequestTopics2 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<ListOffsetsRequestTopicsPartitions5> for ListOffsetsRequestTopicsPartitions2 {
    fn from(latest: ListOffsetsRequestTopicsPartitions5) -> ListOffsetsRequestTopicsPartitions2 {
        log::debug!("Using old api format - ListOffsetsRequestTopicsPartitions2, ignoring field current_leader_epoch");
        ListOffsetsRequestTopicsPartitions2 {
            partition_index: latest.partition_index,
            timestamp: latest.timestamp,
        }
    }
}

impl From<ListOffsetsRequest5> for ListOffsetsRequest3 {
    fn from(latest: ListOffsetsRequest5) -> ListOffsetsRequest3 {
        ListOffsetsRequest3 {
            replica_id: latest.replica_id,
            isolation_level: latest.isolation_level,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<ListOffsetsRequestTopics5> for ListOffsetsRequestTopics3 {
    fn from(latest: ListOffsetsRequestTopics5) -> ListOffsetsRequestTopics3 {
        ListOffsetsRequestTopics3 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<ListOffsetsRequestTopicsPartitions5> for ListOffsetsRequestTopicsPartitions3 {
    fn from(latest: ListOffsetsRequestTopicsPartitions5) -> ListOffsetsRequestTopicsPartitions3 {
        log::debug!("Using old api format - ListOffsetsRequestTopicsPartitions3, ignoring field current_leader_epoch");
        ListOffsetsRequestTopicsPartitions3 {
            partition_index: latest.partition_index,
            timestamp: latest.timestamp,
        }
    }
}

impl From<ListOffsetsRequest5> for ListOffsetsRequest4 {
    fn from(latest: ListOffsetsRequest5) -> ListOffsetsRequest4 {
        ListOffsetsRequest4 {
            replica_id: latest.replica_id,
            isolation_level: latest.isolation_level,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<ListOffsetsRequestTopics5> for ListOffsetsRequestTopics4 {
    fn from(latest: ListOffsetsRequestTopics5) -> ListOffsetsRequestTopics4 {
        ListOffsetsRequestTopics4 {
            name: latest.name,
            partitions: latest
                .partitions
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<ListOffsetsRequestTopicsPartitions5> for ListOffsetsRequestTopicsPartitions4 {
    fn from(latest: ListOffsetsRequestTopicsPartitions5) -> ListOffsetsRequestTopicsPartitions4 {
        ListOffsetsRequestTopicsPartitions4 {
            partition_index: latest.partition_index,
            current_leader_epoch: latest.current_leader_epoch,
            timestamp: latest.timestamp,
        }
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
