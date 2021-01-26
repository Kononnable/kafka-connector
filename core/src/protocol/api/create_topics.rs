use super::prelude::*;

pub type CreateTopicsRequest = CreateTopicsRequest6;
pub type CreateTopicsResponse = CreateTopicsResponse6;
pub fn serialize_create_topics_request(
    data: CreateTopicsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&CreateTopicsRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&CreateTopicsRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&CreateTopicsRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&CreateTopicsRequest3::try_from(data)?, buf),
        4 => ToBytes::serialize(&CreateTopicsRequest4::try_from(data)?, buf),
        5 => ToBytes::serialize(&CreateTopicsRequest5::try_from(data)?, buf),
        6 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_create_topics_response(version: i32, buf: &mut Bytes) -> CreateTopicsResponse {
    match version {
        0 => CreateTopicsResponse0::deserialize(buf).into(),
        1 => CreateTopicsResponse1::deserialize(buf).into(),
        2 => CreateTopicsResponse2::deserialize(buf).into(),
        3 => CreateTopicsResponse3::deserialize(buf).into(),
        4 => CreateTopicsResponse4::deserialize(buf).into(),
        5 => CreateTopicsResponse5::deserialize(buf).into(),
        6 => CreateTopicsResponse::deserialize(buf),
        _ => CreateTopicsResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequest0 {
    pub topics: Vec<CreateTopicsRequestTopics0>,
    pub timeout_ms: Int32,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopics0 {
    pub name: String,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments0>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs0>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments0 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs0 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequest1 {
    pub topics: Vec<CreateTopicsRequestTopics1>,
    pub timeout_ms: Int32,
    pub validate_only: Optional<Boolean>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopics1 {
    pub name: String,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments1>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs1>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments1 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs1 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequest2 {
    pub topics: Vec<CreateTopicsRequestTopics2>,
    pub timeout_ms: Int32,
    pub validate_only: Optional<Boolean>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopics2 {
    pub name: String,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments2>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs2>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments2 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs2 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequest3 {
    pub topics: Vec<CreateTopicsRequestTopics3>,
    pub timeout_ms: Int32,
    pub validate_only: Optional<Boolean>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopics3 {
    pub name: String,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments3>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs3>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments3 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs3 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequest4 {
    pub topics: Vec<CreateTopicsRequestTopics4>,
    pub timeout_ms: Int32,
    pub validate_only: Optional<Boolean>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopics4 {
    pub name: String,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments4>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs4>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments4 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs4 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequest5 {
    pub topics: Vec<CreateTopicsRequestTopics5>,
    pub timeout_ms: Int32,
    pub validate_only: Optional<Boolean>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopics5 {
    pub name: CompactString,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments5>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs5>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments5 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs5 {
    pub name: CompactString,
    pub value: CompactNullableString,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequest6 {
    pub topics: Vec<CreateTopicsRequestTopics6>,
    pub timeout_ms: Int32,
    pub validate_only: Optional<Boolean>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopics6 {
    pub name: CompactString,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments6>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs6>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments6 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs6 {
    pub name: CompactString,
    pub value: CompactNullableString,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponse0 {
    pub topics: Vec<CreateTopicsResponseTopics0>,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponseTopics0 {
    pub name: String,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponse1 {
    pub topics: Vec<CreateTopicsResponseTopics1>,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponseTopics1 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: Optional<NullableString>,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<CreateTopicsResponseTopics2>,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponseTopics2 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: Optional<NullableString>,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<CreateTopicsResponseTopics3>,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponseTopics3 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: Optional<NullableString>,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<CreateTopicsResponseTopics4>,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponseTopics4 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: Optional<NullableString>,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponse5 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<CreateTopicsResponseTopics5>,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponseTopics5 {
    pub name: CompactString,
    pub error_code: Int16,
    pub error_message: Optional<CompactNullableString>,
    pub num_partitions: Optional<Int32>,
    pub replication_factor: Optional<Int16>,
    pub configs: Optional<Vec<CreateTopicsResponseTopicsConfigs5>>,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponseTopicsConfigs5 {
    pub name: CompactString,
    pub value: CompactNullableString,
    pub read_only: Boolean,
    pub config_source: Int8,
    pub is_sensitive: Boolean,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponse6 {
    pub throttle_time_ms: Optional<Int32>,
    pub topics: Vec<CreateTopicsResponseTopics6>,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponseTopics6 {
    pub name: CompactString,
    pub error_code: Int16,
    pub error_message: Optional<CompactNullableString>,
    pub num_partitions: Optional<Int32>,
    pub replication_factor: Optional<Int16>,
    pub configs: Optional<Vec<CreateTopicsResponseTopicsConfigs6>>,
}

#[derive(Default, FromBytes)]
pub struct CreateTopicsResponseTopicsConfigs6 {
    pub name: CompactString,
    pub value: CompactNullableString,
    pub read_only: Boolean,
    pub config_source: Int8,
    pub is_sensitive: Boolean,
}

impl TryFrom<CreateTopicsRequest6> for CreateTopicsRequest0 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequest6) -> Result<Self, Self::Error> {
        if latest.validate_only.is_some() {
            return Err(Error::OldKafkaVersion(
                "CreateTopicsRequest",
                0,
                "validate_only",
            ));
        }
        Ok(CreateTopicsRequest0 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            timeout_ms: latest.timeout_ms,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopics6> for CreateTopicsRequestTopics0 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopics6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopics0 {
            name: latest.name.into(),
            num_partitions: latest.num_partitions,
            replication_factor: latest.replication_factor,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            configs: latest
                .configs
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopicsAssignments6> for CreateTopicsRequestTopicsAssignments0 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopicsAssignments6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopicsAssignments0 {
            partition_index: latest.partition_index,
            broker_ids: latest.broker_ids,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopicsConfigs6> for CreateTopicsRequestTopicsConfigs0 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopicsConfigs6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopicsConfigs0 {
            name: latest.name.into(),
            value: latest.value,
        })
    }
}

impl TryFrom<CreateTopicsRequest6> for CreateTopicsRequest1 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequest6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequest1 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopics6> for CreateTopicsRequestTopics1 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopics6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopics1 {
            name: latest.name.into(),
            num_partitions: latest.num_partitions,
            replication_factor: latest.replication_factor,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            configs: latest
                .configs
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopicsAssignments6> for CreateTopicsRequestTopicsAssignments1 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopicsAssignments6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopicsAssignments1 {
            partition_index: latest.partition_index,
            broker_ids: latest.broker_ids,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopicsConfigs6> for CreateTopicsRequestTopicsConfigs1 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopicsConfigs6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopicsConfigs1 {
            name: latest.name.into(),
            value: latest.value,
        })
    }
}

impl TryFrom<CreateTopicsRequest6> for CreateTopicsRequest2 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequest6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequest2 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopics6> for CreateTopicsRequestTopics2 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopics6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopics2 {
            name: latest.name.into(),
            num_partitions: latest.num_partitions,
            replication_factor: latest.replication_factor,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            configs: latest
                .configs
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopicsAssignments6> for CreateTopicsRequestTopicsAssignments2 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopicsAssignments6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopicsAssignments2 {
            partition_index: latest.partition_index,
            broker_ids: latest.broker_ids,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopicsConfigs6> for CreateTopicsRequestTopicsConfigs2 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopicsConfigs6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopicsConfigs2 {
            name: latest.name.into(),
            value: latest.value,
        })
    }
}

impl TryFrom<CreateTopicsRequest6> for CreateTopicsRequest3 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequest6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequest3 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopics6> for CreateTopicsRequestTopics3 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopics6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopics3 {
            name: latest.name.into(),
            num_partitions: latest.num_partitions,
            replication_factor: latest.replication_factor,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            configs: latest
                .configs
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopicsAssignments6> for CreateTopicsRequestTopicsAssignments3 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopicsAssignments6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopicsAssignments3 {
            partition_index: latest.partition_index,
            broker_ids: latest.broker_ids,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopicsConfigs6> for CreateTopicsRequestTopicsConfigs3 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopicsConfigs6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopicsConfigs3 {
            name: latest.name.into(),
            value: latest.value,
        })
    }
}

impl TryFrom<CreateTopicsRequest6> for CreateTopicsRequest4 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequest6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequest4 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopics6> for CreateTopicsRequestTopics4 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopics6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopics4 {
            name: latest.name.into(),
            num_partitions: latest.num_partitions,
            replication_factor: latest.replication_factor,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            configs: latest
                .configs
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopicsAssignments6> for CreateTopicsRequestTopicsAssignments4 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopicsAssignments6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopicsAssignments4 {
            partition_index: latest.partition_index,
            broker_ids: latest.broker_ids,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopicsConfigs6> for CreateTopicsRequestTopicsConfigs4 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopicsConfigs6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopicsConfigs4 {
            name: latest.name.into(),
            value: latest.value,
        })
    }
}

impl TryFrom<CreateTopicsRequest6> for CreateTopicsRequest5 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequest6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequest5 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopics6> for CreateTopicsRequestTopics5 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopics6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopics5 {
            name: latest.name,
            num_partitions: latest.num_partitions,
            replication_factor: latest.replication_factor,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            configs: latest
                .configs
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopicsAssignments6> for CreateTopicsRequestTopicsAssignments5 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopicsAssignments6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopicsAssignments5 {
            partition_index: latest.partition_index,
            broker_ids: latest.broker_ids,
        })
    }
}

impl TryFrom<CreateTopicsRequestTopicsConfigs6> for CreateTopicsRequestTopicsConfigs5 {
    type Error = Error;
    fn try_from(latest: CreateTopicsRequestTopicsConfigs6) -> Result<Self, Self::Error> {
        Ok(CreateTopicsRequestTopicsConfigs5 {
            name: latest.name,
            value: latest.value,
        })
    }
}

impl From<CreateTopicsResponse0> for CreateTopicsResponse6 {
    fn from(older: CreateTopicsResponse0) -> Self {
        CreateTopicsResponse6 {
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..CreateTopicsResponse6::default()
        }
    }
}

impl From<CreateTopicsResponseTopics0> for CreateTopicsResponseTopics6 {
    fn from(older: CreateTopicsResponseTopics0) -> Self {
        CreateTopicsResponseTopics6 {
            name: older.name.into(),
            error_code: older.error_code,
            ..CreateTopicsResponseTopics6::default()
        }
    }
}

impl From<CreateTopicsResponse1> for CreateTopicsResponse6 {
    fn from(older: CreateTopicsResponse1) -> Self {
        CreateTopicsResponse6 {
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..CreateTopicsResponse6::default()
        }
    }
}

impl From<CreateTopicsResponseTopics1> for CreateTopicsResponseTopics6 {
    fn from(older: CreateTopicsResponseTopics1) -> Self {
        CreateTopicsResponseTopics6 {
            name: older.name.into(),
            error_code: older.error_code,
            error_message: older.error_message.map(|val| val.into()),
            ..CreateTopicsResponseTopics6::default()
        }
    }
}

impl From<CreateTopicsResponse2> for CreateTopicsResponse6 {
    fn from(older: CreateTopicsResponse2) -> Self {
        CreateTopicsResponse6 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<CreateTopicsResponseTopics2> for CreateTopicsResponseTopics6 {
    fn from(older: CreateTopicsResponseTopics2) -> Self {
        CreateTopicsResponseTopics6 {
            name: older.name.into(),
            error_code: older.error_code,
            error_message: older.error_message.map(|val| val.into()),
            ..CreateTopicsResponseTopics6::default()
        }
    }
}

impl From<CreateTopicsResponse3> for CreateTopicsResponse6 {
    fn from(older: CreateTopicsResponse3) -> Self {
        CreateTopicsResponse6 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<CreateTopicsResponseTopics3> for CreateTopicsResponseTopics6 {
    fn from(older: CreateTopicsResponseTopics3) -> Self {
        CreateTopicsResponseTopics6 {
            name: older.name.into(),
            error_code: older.error_code,
            error_message: older.error_message.map(|val| val.into()),
            ..CreateTopicsResponseTopics6::default()
        }
    }
}

impl From<CreateTopicsResponse4> for CreateTopicsResponse6 {
    fn from(older: CreateTopicsResponse4) -> Self {
        CreateTopicsResponse6 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<CreateTopicsResponseTopics4> for CreateTopicsResponseTopics6 {
    fn from(older: CreateTopicsResponseTopics4) -> Self {
        CreateTopicsResponseTopics6 {
            name: older.name.into(),
            error_code: older.error_code,
            error_message: older.error_message.map(|val| val.into()),
            ..CreateTopicsResponseTopics6::default()
        }
    }
}

impl From<CreateTopicsResponse5> for CreateTopicsResponse6 {
    fn from(older: CreateTopicsResponse5) -> Self {
        CreateTopicsResponse6 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<CreateTopicsResponseTopics5> for CreateTopicsResponseTopics6 {
    fn from(older: CreateTopicsResponseTopics5) -> Self {
        CreateTopicsResponseTopics6 {
            name: older.name,
            error_code: older.error_code,
            error_message: older.error_message,
            num_partitions: older.num_partitions,
            replication_factor: older.replication_factor,
            configs: older
                .configs
                .map(|val| val.into_iter().map(|el| el.into()).collect()),
        }
    }
}

impl From<CreateTopicsResponseTopicsConfigs5> for CreateTopicsResponseTopicsConfigs6 {
    fn from(older: CreateTopicsResponseTopicsConfigs5) -> Self {
        CreateTopicsResponseTopicsConfigs6 {
            name: older.name,
            value: older.value,
            read_only: older.read_only,
            config_source: older.config_source,
            is_sensitive: older.is_sensitive,
        }
    }
}
