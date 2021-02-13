use super::prelude::*;

pub type DeleteTopicsRequest = DeleteTopicsRequest5;
pub type DeleteTopicsResponse = DeleteTopicsResponse5;
impl ApiCall for DeleteTopicsRequest {
    type Response = DeleteTopicsResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        5
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DeleteTopics
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => true,
            5 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                DeleteTopicsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                DeleteTopicsRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &DeleteTopicsRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &DeleteTopicsRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &DeleteTopicsRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &DeleteTopicsRequest3::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(
                &DeleteTopicsRequest4::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            5 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, DeleteTopicsResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => DeleteTopicsResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => DeleteTopicsResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => DeleteTopicsResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => DeleteTopicsResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => DeleteTopicsResponse4::deserialize(buf, Self::is_flexible_version(version)).into(),
            5 => DeleteTopicsResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => DeleteTopicsResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteTopicsRequest0 {
    pub topic_names: Vec<String>,
    pub timeout_ms: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteTopicsRequest1 {
    pub topic_names: Vec<String>,
    pub timeout_ms: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteTopicsRequest2 {
    pub topic_names: Vec<String>,
    pub timeout_ms: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteTopicsRequest3 {
    pub topic_names: Vec<String>,
    pub timeout_ms: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteTopicsRequest4 {
    pub topic_names: Vec<String>,
    pub timeout_ms: Int32,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DeleteTopicsRequest5 {
    pub topic_names: Vec<String>,
    pub timeout_ms: Int32,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteTopicsResponse0 {
    pub responses: Vec<DeleteTopicsResponseResponses0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteTopicsResponseResponses0 {
    pub name: String,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteTopicsResponse1 {
    pub throttle_time_ms: Option<Int32>,
    pub responses: Vec<DeleteTopicsResponseResponses1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteTopicsResponseResponses1 {
    pub name: String,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteTopicsResponse2 {
    pub throttle_time_ms: Option<Int32>,
    pub responses: Vec<DeleteTopicsResponseResponses2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteTopicsResponseResponses2 {
    pub name: String,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteTopicsResponse3 {
    pub throttle_time_ms: Option<Int32>,
    pub responses: Vec<DeleteTopicsResponseResponses3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteTopicsResponseResponses3 {
    pub name: String,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteTopicsResponse4 {
    pub throttle_time_ms: Option<Int32>,
    pub responses: Vec<DeleteTopicsResponseResponses4>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteTopicsResponseResponses4 {
    pub name: String,
    pub error_code: Int16,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteTopicsResponse5 {
    pub throttle_time_ms: Option<Int32>,
    pub responses: Vec<DeleteTopicsResponseResponses5>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DeleteTopicsResponseResponses5 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: Option<NullableString>,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<DeleteTopicsRequest5> for DeleteTopicsRequest0 {
    fn from(latest: DeleteTopicsRequest5) -> DeleteTopicsRequest0 {
        DeleteTopicsRequest0 {
            topic_names: latest.topic_names,
            timeout_ms: latest.timeout_ms,
        }
    }
}

impl From<DeleteTopicsRequest5> for DeleteTopicsRequest1 {
    fn from(latest: DeleteTopicsRequest5) -> DeleteTopicsRequest1 {
        DeleteTopicsRequest1 {
            topic_names: latest.topic_names,
            timeout_ms: latest.timeout_ms,
        }
    }
}

impl From<DeleteTopicsRequest5> for DeleteTopicsRequest2 {
    fn from(latest: DeleteTopicsRequest5) -> DeleteTopicsRequest2 {
        DeleteTopicsRequest2 {
            topic_names: latest.topic_names,
            timeout_ms: latest.timeout_ms,
        }
    }
}

impl From<DeleteTopicsRequest5> for DeleteTopicsRequest3 {
    fn from(latest: DeleteTopicsRequest5) -> DeleteTopicsRequest3 {
        DeleteTopicsRequest3 {
            topic_names: latest.topic_names,
            timeout_ms: latest.timeout_ms,
        }
    }
}

impl From<DeleteTopicsRequest5> for DeleteTopicsRequest4 {
    fn from(latest: DeleteTopicsRequest5) -> DeleteTopicsRequest4 {
        DeleteTopicsRequest4 {
            topic_names: latest.topic_names,
            timeout_ms: latest.timeout_ms,
            tag_buffer: latest.tag_buffer,
        }
    }
}

impl From<DeleteTopicsResponse0> for DeleteTopicsResponse5 {
    fn from(older: DeleteTopicsResponse0) -> Self {
        DeleteTopicsResponse5 {
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            ..DeleteTopicsResponse5::default()
        }
    }
}

impl From<DeleteTopicsResponseResponses0> for DeleteTopicsResponseResponses5 {
    fn from(older: DeleteTopicsResponseResponses0) -> Self {
        DeleteTopicsResponseResponses5 {
            name: older.name,
            error_code: older.error_code,
            ..DeleteTopicsResponseResponses5::default()
        }
    }
}

impl From<DeleteTopicsResponse1> for DeleteTopicsResponse5 {
    fn from(older: DeleteTopicsResponse1) -> Self {
        DeleteTopicsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            ..DeleteTopicsResponse5::default()
        }
    }
}

impl From<DeleteTopicsResponseResponses1> for DeleteTopicsResponseResponses5 {
    fn from(older: DeleteTopicsResponseResponses1) -> Self {
        DeleteTopicsResponseResponses5 {
            name: older.name,
            error_code: older.error_code,
            ..DeleteTopicsResponseResponses5::default()
        }
    }
}

impl From<DeleteTopicsResponse2> for DeleteTopicsResponse5 {
    fn from(older: DeleteTopicsResponse2) -> Self {
        DeleteTopicsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            ..DeleteTopicsResponse5::default()
        }
    }
}

impl From<DeleteTopicsResponseResponses2> for DeleteTopicsResponseResponses5 {
    fn from(older: DeleteTopicsResponseResponses2) -> Self {
        DeleteTopicsResponseResponses5 {
            name: older.name,
            error_code: older.error_code,
            ..DeleteTopicsResponseResponses5::default()
        }
    }
}

impl From<DeleteTopicsResponse3> for DeleteTopicsResponse5 {
    fn from(older: DeleteTopicsResponse3) -> Self {
        DeleteTopicsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            ..DeleteTopicsResponse5::default()
        }
    }
}

impl From<DeleteTopicsResponseResponses3> for DeleteTopicsResponseResponses5 {
    fn from(older: DeleteTopicsResponseResponses3) -> Self {
        DeleteTopicsResponseResponses5 {
            name: older.name,
            error_code: older.error_code,
            ..DeleteTopicsResponseResponses5::default()
        }
    }
}

impl From<DeleteTopicsResponse4> for DeleteTopicsResponse5 {
    fn from(older: DeleteTopicsResponse4) -> Self {
        DeleteTopicsResponse5 {
            throttle_time_ms: older.throttle_time_ms,
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            tag_buffer: older.tag_buffer,
        }
    }
}

impl From<DeleteTopicsResponseResponses4> for DeleteTopicsResponseResponses5 {
    fn from(older: DeleteTopicsResponseResponses4) -> Self {
        DeleteTopicsResponseResponses5 {
            name: older.name,
            error_code: older.error_code,
            tag_buffer: older.tag_buffer,
            ..DeleteTopicsResponseResponses5::default()
        }
    }
}
