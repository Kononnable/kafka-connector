use super::prelude::*;

pub type DeleteTopicsRequest = DeleteTopicsRequest5;
pub type DeleteTopicsResponse = DeleteTopicsResponse5;
pub fn serialize_delete_topics_request(
    data: DeleteTopicsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&DeleteTopicsRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&DeleteTopicsRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&DeleteTopicsRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&DeleteTopicsRequest3::try_from(data)?, buf),
        4 => ToBytes::serialize(&DeleteTopicsRequest4::try_from(data)?, buf),
        5 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_delete_topics_response(version: i32, buf: &mut Bytes) -> DeleteTopicsResponse {
    match version {
        0 => DeleteTopicsResponse0::deserialize(buf).into(),
        1 => DeleteTopicsResponse1::deserialize(buf).into(),
        2 => DeleteTopicsResponse2::deserialize(buf).into(),
        3 => DeleteTopicsResponse3::deserialize(buf).into(),
        4 => DeleteTopicsResponse4::deserialize(buf).into(),
        5 => DeleteTopicsResponse::deserialize(buf),
        _ => DeleteTopicsResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct DeleteTopicsRequest0 {
    pub topic_names: Vec<String>,
    pub timeout_ms: Int32,
}

#[derive(Default, ToBytes)]
pub struct DeleteTopicsRequest1 {
    pub topic_names: Vec<String>,
    pub timeout_ms: Int32,
}

#[derive(Default, ToBytes)]
pub struct DeleteTopicsRequest2 {
    pub topic_names: Vec<String>,
    pub timeout_ms: Int32,
}

#[derive(Default, ToBytes)]
pub struct DeleteTopicsRequest3 {
    pub topic_names: Vec<String>,
    pub timeout_ms: Int32,
}

#[derive(Default, ToBytes)]
pub struct DeleteTopicsRequest4 {
    pub topic_names: Vec<CompactString>,
    pub timeout_ms: Int32,
}

#[derive(Default, ToBytes)]
pub struct DeleteTopicsRequest5 {
    pub topic_names: Vec<CompactString>,
    pub timeout_ms: Int32,
}

#[derive(Default, FromBytes)]
pub struct DeleteTopicsResponse0 {
    pub responses: Vec<DeleteTopicsResponseResponses0>,
}

#[derive(Default, FromBytes)]
pub struct DeleteTopicsResponseResponses0 {
    pub name: String,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct DeleteTopicsResponse1 {
    pub throttle_time_ms: Optional<Int32>,
    pub responses: Vec<DeleteTopicsResponseResponses1>,
}

#[derive(Default, FromBytes)]
pub struct DeleteTopicsResponseResponses1 {
    pub name: String,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct DeleteTopicsResponse2 {
    pub throttle_time_ms: Optional<Int32>,
    pub responses: Vec<DeleteTopicsResponseResponses2>,
}

#[derive(Default, FromBytes)]
pub struct DeleteTopicsResponseResponses2 {
    pub name: String,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct DeleteTopicsResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub responses: Vec<DeleteTopicsResponseResponses3>,
}

#[derive(Default, FromBytes)]
pub struct DeleteTopicsResponseResponses3 {
    pub name: String,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct DeleteTopicsResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub responses: Vec<DeleteTopicsResponseResponses4>,
}

#[derive(Default, FromBytes)]
pub struct DeleteTopicsResponseResponses4 {
    pub name: CompactString,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct DeleteTopicsResponse5 {
    pub throttle_time_ms: Optional<Int32>,
    pub responses: Vec<DeleteTopicsResponseResponses5>,
}

#[derive(Default, FromBytes)]
pub struct DeleteTopicsResponseResponses5 {
    pub name: CompactString,
    pub error_code: Int16,
    pub error_message: Optional<CompactNullableString>,
}

impl TryFrom<DeleteTopicsRequest5> for DeleteTopicsRequest0 {
    type Error = Error;
    fn try_from(latest: DeleteTopicsRequest5) -> Result<Self, Self::Error> {
        Ok(DeleteTopicsRequest0 {
            topic_names: latest.topic_names.into_iter().collect(),
            timeout_ms: latest.timeout_ms,
        })
    }
}

impl TryFrom<DeleteTopicsRequest5> for DeleteTopicsRequest1 {
    type Error = Error;
    fn try_from(latest: DeleteTopicsRequest5) -> Result<Self, Self::Error> {
        Ok(DeleteTopicsRequest1 {
            topic_names: latest.topic_names.into_iter().collect(),
            timeout_ms: latest.timeout_ms,
        })
    }
}

impl TryFrom<DeleteTopicsRequest5> for DeleteTopicsRequest2 {
    type Error = Error;
    fn try_from(latest: DeleteTopicsRequest5) -> Result<Self, Self::Error> {
        Ok(DeleteTopicsRequest2 {
            topic_names: latest.topic_names.into_iter().collect(),
            timeout_ms: latest.timeout_ms,
        })
    }
}

impl TryFrom<DeleteTopicsRequest5> for DeleteTopicsRequest3 {
    type Error = Error;
    fn try_from(latest: DeleteTopicsRequest5) -> Result<Self, Self::Error> {
        Ok(DeleteTopicsRequest3 {
            topic_names: latest.topic_names.into_iter().collect(),
            timeout_ms: latest.timeout_ms,
        })
    }
}

impl TryFrom<DeleteTopicsRequest5> for DeleteTopicsRequest4 {
    type Error = Error;
    fn try_from(latest: DeleteTopicsRequest5) -> Result<Self, Self::Error> {
        Ok(DeleteTopicsRequest4 {
            topic_names: latest.topic_names,
            timeout_ms: latest.timeout_ms,
        })
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
        }
    }
}

impl From<DeleteTopicsResponseResponses4> for DeleteTopicsResponseResponses5 {
    fn from(older: DeleteTopicsResponseResponses4) -> Self {
        DeleteTopicsResponseResponses5 {
            name: older.name,
            error_code: older.error_code,
            ..DeleteTopicsResponseResponses5::default()
        }
    }
}
