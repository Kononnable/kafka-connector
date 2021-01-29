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
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&AlterReplicaLogDirsRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> AlterReplicaLogDirsResponse {
        match version {
            0 => AlterReplicaLogDirsResponse0::deserialize(buf).into(),
            1 => AlterReplicaLogDirsResponse::deserialize(buf),
            _ => AlterReplicaLogDirsResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, ToBytes)]
pub struct AlterReplicaLogDirsRequest0 {
    pub dirs: Vec<AlterReplicaLogDirsRequestDirs0>,
}

#[derive(Default, Debug, ToBytes)]
pub struct AlterReplicaLogDirsRequestDirs0 {
    pub path: String,
    pub topics: Vec<AlterReplicaLogDirsRequestDirsTopics0>,
}

#[derive(Default, Debug, ToBytes)]
pub struct AlterReplicaLogDirsRequestDirsTopics0 {
    pub name: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, ToBytes)]
pub struct AlterReplicaLogDirsRequest1 {
    pub dirs: Vec<AlterReplicaLogDirsRequestDirs1>,
}

#[derive(Default, Debug, ToBytes)]
pub struct AlterReplicaLogDirsRequestDirs1 {
    pub path: String,
    pub topics: Vec<AlterReplicaLogDirsRequestDirsTopics1>,
}

#[derive(Default, Debug, ToBytes)]
pub struct AlterReplicaLogDirsRequestDirsTopics1 {
    pub name: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, FromBytes)]
pub struct AlterReplicaLogDirsResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<AlterReplicaLogDirsResponseResults0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct AlterReplicaLogDirsResponseResults0 {
    pub topic_name: String,
    pub partitions: Vec<AlterReplicaLogDirsResponseResultsPartitions0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct AlterReplicaLogDirsResponseResultsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct AlterReplicaLogDirsResponse1 {
    pub throttle_time_ms: Int32,
    pub results: Vec<AlterReplicaLogDirsResponseResults1>,
}

#[derive(Default, Debug, FromBytes)]
pub struct AlterReplicaLogDirsResponseResults1 {
    pub topic_name: String,
    pub partitions: Vec<AlterReplicaLogDirsResponseResultsPartitions1>,
}

#[derive(Default, Debug, FromBytes)]
pub struct AlterReplicaLogDirsResponseResultsPartitions1 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

impl TryFrom<AlterReplicaLogDirsRequest1> for AlterReplicaLogDirsRequest0 {
    type Error = Error;
    fn try_from(latest: AlterReplicaLogDirsRequest1) -> Result<Self, Self::Error> {
        Ok(AlterReplicaLogDirsRequest0 {
            dirs: latest
                .dirs
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<AlterReplicaLogDirsRequestDirs1> for AlterReplicaLogDirsRequestDirs0 {
    type Error = Error;
    fn try_from(latest: AlterReplicaLogDirsRequestDirs1) -> Result<Self, Self::Error> {
        Ok(AlterReplicaLogDirsRequestDirs0 {
            path: latest.path,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<AlterReplicaLogDirsRequestDirsTopics1> for AlterReplicaLogDirsRequestDirsTopics0 {
    type Error = Error;
    fn try_from(latest: AlterReplicaLogDirsRequestDirsTopics1) -> Result<Self, Self::Error> {
        Ok(AlterReplicaLogDirsRequestDirsTopics0 {
            name: latest.name,
            partitions: latest.partitions,
        })
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
