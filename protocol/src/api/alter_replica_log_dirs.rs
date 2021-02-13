use super::prelude::*;

pub type AlterReplicaLogDirsRequest = AlterReplicaLogDirsRequest1;
pub type AlterReplicaLogDirsResponse = AlterReplicaLogDirsResponse1;
impl ApiCall for AlterReplicaLogDirsRequest {
    type Response = AlterReplicaLogDirsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        1
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::AlterReplicaLogDirs
    }
    fn get_first_error(response: &AlterReplicaLogDirsResponse) -> Option<ApiError> {
        AlterReplicaLogDirsResponse::get_first_error(response)
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            _ => false,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                AlterReplicaLogDirsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                AlterReplicaLogDirsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &AlterReplicaLogDirsRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, AlterReplicaLogDirsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => AlterReplicaLogDirsResponse0::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            1 => AlterReplicaLogDirsResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => AlterReplicaLogDirsResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterReplicaLogDirsRequest0 {
    pub dirs: Vec<AlterReplicaLogDirsRequestDirs0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterReplicaLogDirsRequestDirs0 {
    pub path: String,
    pub topics: Vec<AlterReplicaLogDirsRequestDirsTopics0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterReplicaLogDirsRequestDirsTopics0 {
    pub name: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterReplicaLogDirsRequest1 {
    pub dirs: Vec<AlterReplicaLogDirsRequestDirs1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterReplicaLogDirsRequestDirs1 {
    pub path: String,
    pub topics: Vec<AlterReplicaLogDirsRequestDirsTopics1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterReplicaLogDirsRequestDirsTopics1 {
    pub name: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterReplicaLogDirsResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<AlterReplicaLogDirsResponseResults0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterReplicaLogDirsResponseResults0 {
    pub topic_name: String,
    pub partitions: Vec<AlterReplicaLogDirsResponseResultsPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterReplicaLogDirsResponseResultsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterReplicaLogDirsResponse1 {
    pub throttle_time_ms: Int32,
    pub results: Vec<AlterReplicaLogDirsResponseResults1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterReplicaLogDirsResponseResults1 {
    pub topic_name: String,
    pub partitions: Vec<AlterReplicaLogDirsResponseResultsPartitions1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterReplicaLogDirsResponseResultsPartitions1 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

impl From<AlterReplicaLogDirsRequest1> for AlterReplicaLogDirsRequest0 {
    fn from(latest: AlterReplicaLogDirsRequest1) -> AlterReplicaLogDirsRequest0 {
        AlterReplicaLogDirsRequest0 {
            dirs: latest.dirs.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<AlterReplicaLogDirsRequestDirs1> for AlterReplicaLogDirsRequestDirs0 {
    fn from(latest: AlterReplicaLogDirsRequestDirs1) -> AlterReplicaLogDirsRequestDirs0 {
        AlterReplicaLogDirsRequestDirs0 {
            path: latest.path,
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<AlterReplicaLogDirsRequestDirsTopics1> for AlterReplicaLogDirsRequestDirsTopics0 {
    fn from(
        latest: AlterReplicaLogDirsRequestDirsTopics1,
    ) -> AlterReplicaLogDirsRequestDirsTopics0 {
        AlterReplicaLogDirsRequestDirsTopics0 {
            name: latest.name,
            partitions: latest.partitions,
        }
    }
}

impl From<AlterReplicaLogDirsResponse0> for AlterReplicaLogDirsResponse1 {
    fn from(older: AlterReplicaLogDirsResponse0) -> Self {
        AlterReplicaLogDirsResponse1 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<AlterReplicaLogDirsResponseResults0> for AlterReplicaLogDirsResponseResults1 {
    fn from(older: AlterReplicaLogDirsResponseResults0) -> Self {
        AlterReplicaLogDirsResponseResults1 {
            topic_name: older.topic_name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<AlterReplicaLogDirsResponseResultsPartitions0>
    for AlterReplicaLogDirsResponseResultsPartitions1
{
    fn from(older: AlterReplicaLogDirsResponseResultsPartitions0) -> Self {
        AlterReplicaLogDirsResponseResultsPartitions1 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}

impl AlterReplicaLogDirsResponse1 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.results.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl AlterReplicaLogDirsResponseResults1 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partitions.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl AlterReplicaLogDirsResponseResultsPartitions1 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
