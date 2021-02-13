use super::prelude::*;

pub type CreatePartitionsRequest = CreatePartitionsRequest3;
pub type CreatePartitionsResponse = CreatePartitionsResponse3;
impl ApiCall for CreatePartitionsRequest {
    type Response = CreatePartitionsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::CreatePartitions
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => true,
            3 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                CreatePartitionsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                CreatePartitionsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &CreatePartitionsRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &CreatePartitionsRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &CreatePartitionsRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, CreatePartitionsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => CreatePartitionsResponse0::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            1 => CreatePartitionsResponse1::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            2 => CreatePartitionsResponse2::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            3 => CreatePartitionsResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => CreatePartitionsResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequest0 {
    pub topics: Vec<CreatePartitionsRequestTopics0>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequestTopics0 {
    pub name: String,
    pub count: Int32,
    pub assignments: Vec<CreatePartitionsRequestTopicsAssignments0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequestTopicsAssignments0 {
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequest1 {
    pub topics: Vec<CreatePartitionsRequestTopics1>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequestTopics1 {
    pub name: String,
    pub count: Int32,
    pub assignments: Vec<CreatePartitionsRequestTopicsAssignments1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequestTopicsAssignments1 {
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequest2 {
    pub topics: Vec<CreatePartitionsRequestTopics2>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequestTopics2 {
    pub name: String,
    pub count: Int32,
    pub assignments: Vec<CreatePartitionsRequestTopicsAssignments2>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequestTopicsAssignments2 {
    pub broker_ids: Vec<Int32>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequest3 {
    pub topics: Vec<CreatePartitionsRequestTopics3>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequestTopics3 {
    pub name: String,
    pub count: Int32,
    pub assignments: Vec<CreatePartitionsRequestTopicsAssignments3>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct CreatePartitionsRequestTopicsAssignments3 {
    pub broker_ids: Vec<Int32>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreatePartitionsResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreatePartitionsResponseResults0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreatePartitionsResponseResults0 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: NullableString,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreatePartitionsResponse1 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreatePartitionsResponseResults1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreatePartitionsResponseResults1 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: NullableString,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreatePartitionsResponse2 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreatePartitionsResponseResults2>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreatePartitionsResponseResults2 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreatePartitionsResponse3 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreatePartitionsResponseResults3>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct CreatePartitionsResponseResults3 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<CreatePartitionsRequest3> for CreatePartitionsRequest0 {
    fn from(latest: CreatePartitionsRequest3) -> CreatePartitionsRequest0 {
        CreatePartitionsRequest0 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        }
    }
}

impl From<CreatePartitionsRequestTopics3> for CreatePartitionsRequestTopics0 {
    fn from(latest: CreatePartitionsRequestTopics3) -> CreatePartitionsRequestTopics0 {
        CreatePartitionsRequestTopics0 {
            name: latest.name,
            count: latest.count,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<CreatePartitionsRequestTopicsAssignments3> for CreatePartitionsRequestTopicsAssignments0 {
    fn from(
        latest: CreatePartitionsRequestTopicsAssignments3,
    ) -> CreatePartitionsRequestTopicsAssignments0 {
        CreatePartitionsRequestTopicsAssignments0 {
            broker_ids: latest.broker_ids,
        }
    }
}

impl From<CreatePartitionsRequest3> for CreatePartitionsRequest1 {
    fn from(latest: CreatePartitionsRequest3) -> CreatePartitionsRequest1 {
        CreatePartitionsRequest1 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        }
    }
}

impl From<CreatePartitionsRequestTopics3> for CreatePartitionsRequestTopics1 {
    fn from(latest: CreatePartitionsRequestTopics3) -> CreatePartitionsRequestTopics1 {
        CreatePartitionsRequestTopics1 {
            name: latest.name,
            count: latest.count,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<CreatePartitionsRequestTopicsAssignments3> for CreatePartitionsRequestTopicsAssignments1 {
    fn from(
        latest: CreatePartitionsRequestTopicsAssignments3,
    ) -> CreatePartitionsRequestTopicsAssignments1 {
        CreatePartitionsRequestTopicsAssignments1 {
            broker_ids: latest.broker_ids,
        }
    }
}

impl From<CreatePartitionsRequest3> for CreatePartitionsRequest2 {
    fn from(latest: CreatePartitionsRequest3) -> CreatePartitionsRequest2 {
        CreatePartitionsRequest2 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
            tag_buffer: latest.tag_buffer,
        }
    }
}

impl From<CreatePartitionsRequestTopics3> for CreatePartitionsRequestTopics2 {
    fn from(latest: CreatePartitionsRequestTopics3) -> CreatePartitionsRequestTopics2 {
        CreatePartitionsRequestTopics2 {
            name: latest.name,
            count: latest.count,
            assignments: latest
                .assignments
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            tag_buffer: latest.tag_buffer,
        }
    }
}

impl From<CreatePartitionsRequestTopicsAssignments3> for CreatePartitionsRequestTopicsAssignments2 {
    fn from(
        latest: CreatePartitionsRequestTopicsAssignments3,
    ) -> CreatePartitionsRequestTopicsAssignments2 {
        CreatePartitionsRequestTopicsAssignments2 {
            broker_ids: latest.broker_ids,
            tag_buffer: latest.tag_buffer,
        }
    }
}

impl From<CreatePartitionsResponse0> for CreatePartitionsResponse3 {
    fn from(older: CreatePartitionsResponse0) -> Self {
        CreatePartitionsResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
            ..CreatePartitionsResponse3::default()
        }
    }
}

impl From<CreatePartitionsResponseResults0> for CreatePartitionsResponseResults3 {
    fn from(older: CreatePartitionsResponseResults0) -> Self {
        CreatePartitionsResponseResults3 {
            name: older.name,
            error_code: older.error_code,
            error_message: older.error_message,
            ..CreatePartitionsResponseResults3::default()
        }
    }
}

impl From<CreatePartitionsResponse1> for CreatePartitionsResponse3 {
    fn from(older: CreatePartitionsResponse1) -> Self {
        CreatePartitionsResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
            ..CreatePartitionsResponse3::default()
        }
    }
}

impl From<CreatePartitionsResponseResults1> for CreatePartitionsResponseResults3 {
    fn from(older: CreatePartitionsResponseResults1) -> Self {
        CreatePartitionsResponseResults3 {
            name: older.name,
            error_code: older.error_code,
            error_message: older.error_message,
            ..CreatePartitionsResponseResults3::default()
        }
    }
}

impl From<CreatePartitionsResponse2> for CreatePartitionsResponse3 {
    fn from(older: CreatePartitionsResponse2) -> Self {
        CreatePartitionsResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
            tag_buffer: older.tag_buffer,
        }
    }
}

impl From<CreatePartitionsResponseResults2> for CreatePartitionsResponseResults3 {
    fn from(older: CreatePartitionsResponseResults2) -> Self {
        CreatePartitionsResponseResults3 {
            name: older.name,
            error_code: older.error_code,
            error_message: older.error_message,
            tag_buffer: older.tag_buffer,
        }
    }
}
