use super::prelude::*;

pub type CreateTopicsRequest = CreateTopicsRequest6;
pub type CreateTopicsResponse = CreateTopicsResponse6;
impl ApiCall for CreateTopicsRequest {
    type Response = CreateTopicsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        6
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::CreateTopics
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => false,
            5 => true,
            6 => true,
            _ => true,
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
                CreateTopicsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                CreateTopicsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &CreateTopicsRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &CreateTopicsRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &CreateTopicsRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &CreateTopicsRequest3::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(
                &CreateTopicsRequest4::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            5 => ToBytes::serialize(
                &CreateTopicsRequest5::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            6 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, CreateTopicsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => CreateTopicsResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => CreateTopicsResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => CreateTopicsResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => CreateTopicsResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => CreateTopicsResponse4::deserialize(buf, Self::is_flexible_version(version)).into(),
            5 => CreateTopicsResponse5::deserialize(buf, Self::is_flexible_version(version)).into(),
            6 => CreateTopicsResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => CreateTopicsResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequest0 {
    pub topics: Vec<CreateTopicsRequestTopics0>,
    pub timeout_ms: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopics0 {
    pub name: String,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments0>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments0 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs0 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequest1 {
    pub topics: Vec<CreateTopicsRequestTopics1>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopics1 {
    pub name: String,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments1>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments1 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs1 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequest2 {
    pub topics: Vec<CreateTopicsRequestTopics2>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopics2 {
    pub name: String,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments2>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments2 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs2 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequest3 {
    pub topics: Vec<CreateTopicsRequestTopics3>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopics3 {
    pub name: String,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments3>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments3 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs3 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequest4 {
    pub topics: Vec<CreateTopicsRequestTopics4>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopics4 {
    pub name: String,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments4>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs4>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments4 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs4 {
    pub name: String,
    pub value: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequest5 {
    pub topics: Vec<CreateTopicsRequestTopics5>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopics5 {
    pub name: String,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments5>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs5>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments5 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs5 {
    pub name: String,
    pub value: NullableString,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequest6 {
    pub topics: Vec<CreateTopicsRequestTopics6>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopics6 {
    pub name: String,
    pub num_partitions: Int32,
    pub replication_factor: Int16,
    pub assignments: Vec<CreateTopicsRequestTopicsAssignments6>,
    pub configs: Vec<CreateTopicsRequestTopicsConfigs6>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsAssignments6 {
    pub partition_index: Int32,
    pub broker_ids: Vec<Int32>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreateTopicsRequestTopicsConfigs6 {
    pub name: String,
    pub value: NullableString,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponse0 {
    pub topics: Vec<CreateTopicsResponseTopics0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponseTopics0 {
    pub name: String,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponse1 {
    pub topics: Vec<CreateTopicsResponseTopics1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponseTopics1 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: Option<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponse2 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<CreateTopicsResponseTopics2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponseTopics2 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: Option<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponse3 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<CreateTopicsResponseTopics3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponseTopics3 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: Option<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponse4 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<CreateTopicsResponseTopics4>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponseTopics4 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: Option<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponse5 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<CreateTopicsResponseTopics5>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponseTopics5 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: Option<NullableString>,
    pub num_partitions: Option<Int32>,
    pub replication_factor: Option<Int16>,
    pub configs: Option<Vec<CreateTopicsResponseTopicsConfigs5>>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponseTopicsConfigs5 {
    pub name: String,
    pub value: NullableString,
    pub read_only: Boolean,
    pub config_source: Int8,
    pub is_sensitive: Boolean,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponse6 {
    pub throttle_time_ms: Option<Int32>,
    pub topics: Vec<CreateTopicsResponseTopics6>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponseTopics6 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: Option<NullableString>,
    pub num_partitions: Option<Int32>,
    pub replication_factor: Option<Int16>,
    pub configs: Option<Vec<CreateTopicsResponseTopicsConfigs6>>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreateTopicsResponseTopicsConfigs6 {
    pub name: String,
    pub value: NullableString,
    pub read_only: Boolean,
    pub config_source: Int8,
    pub is_sensitive: Boolean,
    pub tag_buffer: TagBuffer,
}

impl From<CreateTopicsRequest6> for CreateTopicsRequest0 {
    fn from(latest: CreateTopicsRequest6) -> CreateTopicsRequest0 {
        log::debug!("Using old api format - CreateTopicsRequest0, ignoring field validate_only");
        CreateTopicsRequest0 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            timeout_ms: latest.timeout_ms,
        }
    }
}

impl From<CreateTopicsRequestTopics6> for CreateTopicsRequestTopics0 {
    fn from(latest: CreateTopicsRequestTopics6) -> CreateTopicsRequestTopics0 {
        CreateTopicsRequestTopics0 {
            name: latest.name,
            num_partitions: latest.num_partitions,
            replication_factor: latest.replication_factor,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            configs: latest.configs.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<CreateTopicsRequestTopicsAssignments6> for CreateTopicsRequestTopicsAssignments0 {
    fn from(
        latest: CreateTopicsRequestTopicsAssignments6,
    ) -> CreateTopicsRequestTopicsAssignments0 {
        CreateTopicsRequestTopicsAssignments0 {
            partition_index: latest.partition_index,
            broker_ids: latest.broker_ids,
        }
    }
}

impl From<CreateTopicsRequestTopicsConfigs6> for CreateTopicsRequestTopicsConfigs0 {
    fn from(latest: CreateTopicsRequestTopicsConfigs6) -> CreateTopicsRequestTopicsConfigs0 {
        CreateTopicsRequestTopicsConfigs0 {
            name: latest.name,
            value: latest.value,
        }
    }
}

impl From<CreateTopicsRequest6> for CreateTopicsRequest1 {
    fn from(latest: CreateTopicsRequest6) -> CreateTopicsRequest1 {
        CreateTopicsRequest1 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        }
    }
}

impl From<CreateTopicsRequestTopics6> for CreateTopicsRequestTopics1 {
    fn from(latest: CreateTopicsRequestTopics6) -> CreateTopicsRequestTopics1 {
        CreateTopicsRequestTopics1 {
            name: latest.name,
            num_partitions: latest.num_partitions,
            replication_factor: latest.replication_factor,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            configs: latest.configs.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<CreateTopicsRequestTopicsAssignments6> for CreateTopicsRequestTopicsAssignments1 {
    fn from(
        latest: CreateTopicsRequestTopicsAssignments6,
    ) -> CreateTopicsRequestTopicsAssignments1 {
        CreateTopicsRequestTopicsAssignments1 {
            partition_index: latest.partition_index,
            broker_ids: latest.broker_ids,
        }
    }
}

impl From<CreateTopicsRequestTopicsConfigs6> for CreateTopicsRequestTopicsConfigs1 {
    fn from(latest: CreateTopicsRequestTopicsConfigs6) -> CreateTopicsRequestTopicsConfigs1 {
        CreateTopicsRequestTopicsConfigs1 {
            name: latest.name,
            value: latest.value,
        }
    }
}

impl From<CreateTopicsRequest6> for CreateTopicsRequest2 {
    fn from(latest: CreateTopicsRequest6) -> CreateTopicsRequest2 {
        CreateTopicsRequest2 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        }
    }
}

impl From<CreateTopicsRequestTopics6> for CreateTopicsRequestTopics2 {
    fn from(latest: CreateTopicsRequestTopics6) -> CreateTopicsRequestTopics2 {
        CreateTopicsRequestTopics2 {
            name: latest.name,
            num_partitions: latest.num_partitions,
            replication_factor: latest.replication_factor,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            configs: latest.configs.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<CreateTopicsRequestTopicsAssignments6> for CreateTopicsRequestTopicsAssignments2 {
    fn from(
        latest: CreateTopicsRequestTopicsAssignments6,
    ) -> CreateTopicsRequestTopicsAssignments2 {
        CreateTopicsRequestTopicsAssignments2 {
            partition_index: latest.partition_index,
            broker_ids: latest.broker_ids,
        }
    }
}

impl From<CreateTopicsRequestTopicsConfigs6> for CreateTopicsRequestTopicsConfigs2 {
    fn from(latest: CreateTopicsRequestTopicsConfigs6) -> CreateTopicsRequestTopicsConfigs2 {
        CreateTopicsRequestTopicsConfigs2 {
            name: latest.name,
            value: latest.value,
        }
    }
}

impl From<CreateTopicsRequest6> for CreateTopicsRequest3 {
    fn from(latest: CreateTopicsRequest6) -> CreateTopicsRequest3 {
        CreateTopicsRequest3 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        }
    }
}

impl From<CreateTopicsRequestTopics6> for CreateTopicsRequestTopics3 {
    fn from(latest: CreateTopicsRequestTopics6) -> CreateTopicsRequestTopics3 {
        CreateTopicsRequestTopics3 {
            name: latest.name,
            num_partitions: latest.num_partitions,
            replication_factor: latest.replication_factor,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            configs: latest.configs.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<CreateTopicsRequestTopicsAssignments6> for CreateTopicsRequestTopicsAssignments3 {
    fn from(
        latest: CreateTopicsRequestTopicsAssignments6,
    ) -> CreateTopicsRequestTopicsAssignments3 {
        CreateTopicsRequestTopicsAssignments3 {
            partition_index: latest.partition_index,
            broker_ids: latest.broker_ids,
        }
    }
}

impl From<CreateTopicsRequestTopicsConfigs6> for CreateTopicsRequestTopicsConfigs3 {
    fn from(latest: CreateTopicsRequestTopicsConfigs6) -> CreateTopicsRequestTopicsConfigs3 {
        CreateTopicsRequestTopicsConfigs3 {
            name: latest.name,
            value: latest.value,
        }
    }
}

impl From<CreateTopicsRequest6> for CreateTopicsRequest4 {
    fn from(latest: CreateTopicsRequest6) -> CreateTopicsRequest4 {
        CreateTopicsRequest4 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        }
    }
}

impl From<CreateTopicsRequestTopics6> for CreateTopicsRequestTopics4 {
    fn from(latest: CreateTopicsRequestTopics6) -> CreateTopicsRequestTopics4 {
        CreateTopicsRequestTopics4 {
            name: latest.name,
            num_partitions: latest.num_partitions,
            replication_factor: latest.replication_factor,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            configs: latest.configs.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<CreateTopicsRequestTopicsAssignments6> for CreateTopicsRequestTopicsAssignments4 {
    fn from(
        latest: CreateTopicsRequestTopicsAssignments6,
    ) -> CreateTopicsRequestTopicsAssignments4 {
        CreateTopicsRequestTopicsAssignments4 {
            partition_index: latest.partition_index,
            broker_ids: latest.broker_ids,
        }
    }
}

impl From<CreateTopicsRequestTopicsConfigs6> for CreateTopicsRequestTopicsConfigs4 {
    fn from(latest: CreateTopicsRequestTopicsConfigs6) -> CreateTopicsRequestTopicsConfigs4 {
        CreateTopicsRequestTopicsConfigs4 {
            name: latest.name,
            value: latest.value,
        }
    }
}

impl From<CreateTopicsRequest6> for CreateTopicsRequest5 {
    fn from(latest: CreateTopicsRequest6) -> CreateTopicsRequest5 {
        CreateTopicsRequest5 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
            tag_buffer: latest.tag_buffer,
        }
    }
}

impl From<CreateTopicsRequestTopics6> for CreateTopicsRequestTopics5 {
    fn from(latest: CreateTopicsRequestTopics6) -> CreateTopicsRequestTopics5 {
        CreateTopicsRequestTopics5 {
            name: latest.name,
            num_partitions: latest.num_partitions,
            replication_factor: latest.replication_factor,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            configs: latest.configs.into_iter().map(|ele| ele.into()).collect(),
            tag_buffer: latest.tag_buffer,
        }
    }
}

impl From<CreateTopicsRequestTopicsAssignments6> for CreateTopicsRequestTopicsAssignments5 {
    fn from(
        latest: CreateTopicsRequestTopicsAssignments6,
    ) -> CreateTopicsRequestTopicsAssignments5 {
        CreateTopicsRequestTopicsAssignments5 {
            partition_index: latest.partition_index,
            broker_ids: latest.broker_ids,
            tag_buffer: latest.tag_buffer,
        }
    }
}

impl From<CreateTopicsRequestTopicsConfigs6> for CreateTopicsRequestTopicsConfigs5 {
    fn from(latest: CreateTopicsRequestTopicsConfigs6) -> CreateTopicsRequestTopicsConfigs5 {
        CreateTopicsRequestTopicsConfigs5 {
            name: latest.name,
            value: latest.value,
            tag_buffer: latest.tag_buffer,
        }
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
            name: older.name,
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
            name: older.name,
            error_code: older.error_code,
            error_message: older.error_message,
            ..CreateTopicsResponseTopics6::default()
        }
    }
}

impl From<CreateTopicsResponse2> for CreateTopicsResponse6 {
    fn from(older: CreateTopicsResponse2) -> Self {
        CreateTopicsResponse6 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..CreateTopicsResponse6::default()
        }
    }
}

impl From<CreateTopicsResponseTopics2> for CreateTopicsResponseTopics6 {
    fn from(older: CreateTopicsResponseTopics2) -> Self {
        CreateTopicsResponseTopics6 {
            name: older.name,
            error_code: older.error_code,
            error_message: older.error_message,
            ..CreateTopicsResponseTopics6::default()
        }
    }
}

impl From<CreateTopicsResponse3> for CreateTopicsResponse6 {
    fn from(older: CreateTopicsResponse3) -> Self {
        CreateTopicsResponse6 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..CreateTopicsResponse6::default()
        }
    }
}

impl From<CreateTopicsResponseTopics3> for CreateTopicsResponseTopics6 {
    fn from(older: CreateTopicsResponseTopics3) -> Self {
        CreateTopicsResponseTopics6 {
            name: older.name,
            error_code: older.error_code,
            error_message: older.error_message,
            ..CreateTopicsResponseTopics6::default()
        }
    }
}

impl From<CreateTopicsResponse4> for CreateTopicsResponse6 {
    fn from(older: CreateTopicsResponse4) -> Self {
        CreateTopicsResponse6 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..CreateTopicsResponse6::default()
        }
    }
}

impl From<CreateTopicsResponseTopics4> for CreateTopicsResponseTopics6 {
    fn from(older: CreateTopicsResponseTopics4) -> Self {
        CreateTopicsResponseTopics6 {
            name: older.name,
            error_code: older.error_code,
            error_message: older.error_message,
            ..CreateTopicsResponseTopics6::default()
        }
    }
}

impl From<CreateTopicsResponse5> for CreateTopicsResponse6 {
    fn from(older: CreateTopicsResponse5) -> Self {
        CreateTopicsResponse6 {
            throttle_time_ms: older.throttle_time_ms,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            tag_buffer: older.tag_buffer,
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
            tag_buffer: older.tag_buffer,
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
            tag_buffer: older.tag_buffer,
        }
    }
}
