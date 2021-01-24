use super::prelude::*;

pub type CreatePartitionsRequest = CreatePartitionsRequest3;
pub type CreatePartitionsResponse = CreatePartitionsResponse3;
pub fn serialize_create_partitions_request(
    data: CreatePartitionsRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&CreatePartitionsRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&CreatePartitionsRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&CreatePartitionsRequest2::try_from(data)?, buf),
        4 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_create_partitions_response<T>(
    version: i32,
    buf: &mut T,
) -> CreatePartitionsResponse
where
    T: Iterator<Item = u8>,
{
    match version {
        0 => CreatePartitionsResponse0::deserialize(buf).into(),
        1 => CreatePartitionsResponse1::deserialize(buf).into(),
        2 => CreatePartitionsResponse2::deserialize(buf).into(),
        4 => CreatePartitionsResponse::deserialize(buf),
        _ => CreatePartitionsResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct CreatePartitionsRequest0 {
    pub topics: Vec<CreatePartitionsRequestTopics0>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
}

#[derive(Default, ToBytes)]
pub struct CreatePartitionsRequestTopics0 {
    pub name: String,
    pub count: Int32,
    pub assignments: Vec<CreatePartitionsRequestTopicsAssignments0>,
}

#[derive(Default, ToBytes)]
pub struct CreatePartitionsRequestTopicsAssignments0 {
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct CreatePartitionsRequest1 {
    pub topics: Vec<CreatePartitionsRequestTopics1>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
}

#[derive(Default, ToBytes)]
pub struct CreatePartitionsRequestTopics1 {
    pub name: String,
    pub count: Int32,
    pub assignments: Vec<CreatePartitionsRequestTopicsAssignments1>,
}

#[derive(Default, ToBytes)]
pub struct CreatePartitionsRequestTopicsAssignments1 {
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct CreatePartitionsRequest2 {
    pub topics: Vec<CreatePartitionsRequestTopics2>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
}

#[derive(Default, ToBytes)]
pub struct CreatePartitionsRequestTopics2 {
    pub name: CompactString,
    pub count: Int32,
    pub assignments: Vec<CreatePartitionsRequestTopicsAssignments2>,
}

#[derive(Default, ToBytes)]
pub struct CreatePartitionsRequestTopicsAssignments2 {
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct CreatePartitionsRequest3 {
    pub topics: Vec<CreatePartitionsRequestTopics3>,
    pub timeout_ms: Int32,
    pub validate_only: Boolean,
}

#[derive(Default, ToBytes)]
pub struct CreatePartitionsRequestTopics3 {
    pub name: CompactString,
    pub count: Int32,
    pub assignments: Vec<CreatePartitionsRequestTopicsAssignments3>,
}

#[derive(Default, ToBytes)]
pub struct CreatePartitionsRequestTopicsAssignments3 {
    pub broker_ids: Vec<Int32>,
}

#[derive(Default, FromBytes)]
pub struct CreatePartitionsResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreatePartitionsResponseResults0>,
}

#[derive(Default, FromBytes)]
pub struct CreatePartitionsResponseResults0 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: NullableString,
}

#[derive(Default, FromBytes)]
pub struct CreatePartitionsResponse1 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreatePartitionsResponseResults1>,
}

#[derive(Default, FromBytes)]
pub struct CreatePartitionsResponseResults1 {
    pub name: String,
    pub error_code: Int16,
    pub error_message: NullableString,
}

#[derive(Default, FromBytes)]
pub struct CreatePartitionsResponse2 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreatePartitionsResponseResults2>,
}

#[derive(Default, FromBytes)]
pub struct CreatePartitionsResponseResults2 {
    pub name: CompactString,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
}

#[derive(Default, FromBytes)]
pub struct CreatePartitionsResponse3 {
    pub throttle_time_ms: Int32,
    pub results: Vec<CreatePartitionsResponseResults3>,
}

#[derive(Default, FromBytes)]
pub struct CreatePartitionsResponseResults3 {
    pub name: CompactString,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
}

impl TryFrom<CreatePartitionsRequest3> for CreatePartitionsRequest0 {
    type Error = Error;
    fn try_from(latest: CreatePartitionsRequest3) -> Result<Self, Self::Error> {
        Ok(CreatePartitionsRequest0 {
            topics: latest
                .topics
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        })
    }
}

impl TryFrom<CreatePartitionsRequestTopics3> for CreatePartitionsRequestTopics0 {
    type Error = Error;
    fn try_from(latest: CreatePartitionsRequestTopics3) -> Result<Self, Self::Error> {
        Ok(CreatePartitionsRequestTopics0 {
            name: latest.name,
            count: latest.count,
            assignments: latest
                .assignments
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<CreatePartitionsRequestTopicsAssignments3>
    for CreatePartitionsRequestTopicsAssignments0
{
    type Error = Error;
    fn try_from(latest: CreatePartitionsRequestTopicsAssignments3) -> Result<Self, Self::Error> {
        Ok(CreatePartitionsRequestTopicsAssignments0 {
            broker_ids: latest.broker_ids.into_iter().collect(),
        })
    }
}

impl TryFrom<CreatePartitionsRequest3> for CreatePartitionsRequest1 {
    type Error = Error;
    fn try_from(latest: CreatePartitionsRequest3) -> Result<Self, Self::Error> {
        Ok(CreatePartitionsRequest1 {
            topics: latest
                .topics
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        })
    }
}

impl TryFrom<CreatePartitionsRequestTopics3> for CreatePartitionsRequestTopics1 {
    type Error = Error;
    fn try_from(latest: CreatePartitionsRequestTopics3) -> Result<Self, Self::Error> {
        Ok(CreatePartitionsRequestTopics1 {
            name: latest.name,
            count: latest.count,
            assignments: latest
                .assignments
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<CreatePartitionsRequestTopicsAssignments3>
    for CreatePartitionsRequestTopicsAssignments1
{
    type Error = Error;
    fn try_from(latest: CreatePartitionsRequestTopicsAssignments3) -> Result<Self, Self::Error> {
        Ok(CreatePartitionsRequestTopicsAssignments1 {
            broker_ids: latest.broker_ids.into_iter().collect(),
        })
    }
}

impl TryFrom<CreatePartitionsRequest3> for CreatePartitionsRequest2 {
    type Error = Error;
    fn try_from(latest: CreatePartitionsRequest3) -> Result<Self, Self::Error> {
        Ok(CreatePartitionsRequest2 {
            topics: latest
                .topics
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
            timeout_ms: latest.timeout_ms,
            validate_only: latest.validate_only,
        })
    }
}

impl TryFrom<CreatePartitionsRequestTopics3> for CreatePartitionsRequestTopics2 {
    type Error = Error;
    fn try_from(latest: CreatePartitionsRequestTopics3) -> Result<Self, Self::Error> {
        Ok(CreatePartitionsRequestTopics2 {
            name: latest.name,
            count: latest.count,
            assignments: latest
                .assignments
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<CreatePartitionsRequestTopicsAssignments3>
    for CreatePartitionsRequestTopicsAssignments2
{
    type Error = Error;
    fn try_from(latest: CreatePartitionsRequestTopicsAssignments3) -> Result<Self, Self::Error> {
        Ok(CreatePartitionsRequestTopicsAssignments2 {
            broker_ids: latest.broker_ids.into_iter().collect(),
        })
    }
}

impl From<CreatePartitionsResponse0> for CreatePartitionsResponse3 {
    fn from(older: CreatePartitionsResponse0) -> Self {
        CreatePartitionsResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<CreatePartitionsResponseResults0> for CreatePartitionsResponseResults3 {
    fn from(older: CreatePartitionsResponseResults0) -> Self {
        CreatePartitionsResponseResults3 {
            name: older.name,
            error_code: older.error_code,
            error_message: older.error_message,
        }
    }
}

impl From<CreatePartitionsResponse1> for CreatePartitionsResponse3 {
    fn from(older: CreatePartitionsResponse1) -> Self {
        CreatePartitionsResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<CreatePartitionsResponseResults1> for CreatePartitionsResponseResults3 {
    fn from(older: CreatePartitionsResponseResults1) -> Self {
        CreatePartitionsResponseResults3 {
            name: older.name,
            error_code: older.error_code,
            error_message: older.error_message,
        }
    }
}

impl From<CreatePartitionsResponse2> for CreatePartitionsResponse3 {
    fn from(older: CreatePartitionsResponse2) -> Self {
        CreatePartitionsResponse3 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<CreatePartitionsResponseResults2> for CreatePartitionsResponseResults3 {
    fn from(older: CreatePartitionsResponseResults2) -> Self {
        CreatePartitionsResponseResults3 {
            name: older.name,
            error_code: older.error_code,
            error_message: older.error_message,
        }
    }
}
