use super::prelude::*;

pub type AddPartitionsToTxnRequest = AddPartitionsToTxnRequest2;
pub type AddPartitionsToTxnResponse = AddPartitionsToTxnResponse2;
impl ApiCall for AddPartitionsToTxnRequest {
    type Response = AddPartitionsToTxnResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::AddPartitionsToTxn
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
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
                AddPartitionsToTxnRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                AddPartitionsToTxnRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &AddPartitionsToTxnRequest0::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &AddPartitionsToTxnRequest1::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, AddPartitionsToTxnResponse) {
        let header = HeaderResponse::deserialize(buf, false);
        let response = match version {
            0 => AddPartitionsToTxnResponse0::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            1 => AddPartitionsToTxnResponse1::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            2 => AddPartitionsToTxnResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => AddPartitionsToTxnResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (header.correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AddPartitionsToTxnRequest0 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub topics: Vec<AddPartitionsToTxnRequestTopics0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AddPartitionsToTxnRequestTopics0 {
    pub name: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AddPartitionsToTxnRequest1 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub topics: Vec<AddPartitionsToTxnRequestTopics1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AddPartitionsToTxnRequestTopics1 {
    pub name: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AddPartitionsToTxnRequest2 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub topics: Vec<AddPartitionsToTxnRequestTopics2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AddPartitionsToTxnRequestTopics2 {
    pub name: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddPartitionsToTxnResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<AddPartitionsToTxnResponseResults0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddPartitionsToTxnResponseResults0 {
    pub name: String,
    pub results: Vec<AddPartitionsToTxnResponseResultsResults0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddPartitionsToTxnResponseResultsResults0 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddPartitionsToTxnResponse1 {
    pub throttle_time_ms: Int32,
    pub results: Vec<AddPartitionsToTxnResponseResults1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddPartitionsToTxnResponseResults1 {
    pub name: String,
    pub results: Vec<AddPartitionsToTxnResponseResultsResults1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddPartitionsToTxnResponseResultsResults1 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddPartitionsToTxnResponse2 {
    pub throttle_time_ms: Int32,
    pub results: Vec<AddPartitionsToTxnResponseResults2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddPartitionsToTxnResponseResults2 {
    pub name: String,
    pub results: Vec<AddPartitionsToTxnResponseResultsResults2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddPartitionsToTxnResponseResultsResults2 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

impl TryFrom<AddPartitionsToTxnRequest2> for AddPartitionsToTxnRequest0 {
    type Error = Error;
    fn try_from(latest: AddPartitionsToTxnRequest2) -> Result<Self, Self::Error> {
        Ok(AddPartitionsToTxnRequest0 {
            transactional_id: latest.transactional_id,
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<AddPartitionsToTxnRequestTopics2> for AddPartitionsToTxnRequestTopics0 {
    type Error = Error;
    fn try_from(latest: AddPartitionsToTxnRequestTopics2) -> Result<Self, Self::Error> {
        Ok(AddPartitionsToTxnRequestTopics0 {
            name: latest.name,
            partitions: latest.partitions,
        })
    }
}

impl TryFrom<AddPartitionsToTxnRequest2> for AddPartitionsToTxnRequest1 {
    type Error = Error;
    fn try_from(latest: AddPartitionsToTxnRequest2) -> Result<Self, Self::Error> {
        Ok(AddPartitionsToTxnRequest1 {
            transactional_id: latest.transactional_id,
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<AddPartitionsToTxnRequestTopics2> for AddPartitionsToTxnRequestTopics1 {
    type Error = Error;
    fn try_from(latest: AddPartitionsToTxnRequestTopics2) -> Result<Self, Self::Error> {
        Ok(AddPartitionsToTxnRequestTopics1 {
            name: latest.name,
            partitions: latest.partitions,
        })
    }
}

impl From<AddPartitionsToTxnResponse0> for AddPartitionsToTxnResponse2 {
    fn from(older: AddPartitionsToTxnResponse0) -> Self {
        AddPartitionsToTxnResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<AddPartitionsToTxnResponseResults0> for AddPartitionsToTxnResponseResults2 {
    fn from(older: AddPartitionsToTxnResponseResults0) -> Self {
        AddPartitionsToTxnResponseResults2 {
            name: older.name,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<AddPartitionsToTxnResponseResultsResults0> for AddPartitionsToTxnResponseResultsResults2 {
    fn from(older: AddPartitionsToTxnResponseResultsResults0) -> Self {
        AddPartitionsToTxnResponseResultsResults2 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}

impl From<AddPartitionsToTxnResponse1> for AddPartitionsToTxnResponse2 {
    fn from(older: AddPartitionsToTxnResponse1) -> Self {
        AddPartitionsToTxnResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<AddPartitionsToTxnResponseResults1> for AddPartitionsToTxnResponseResults2 {
    fn from(older: AddPartitionsToTxnResponseResults1) -> Self {
        AddPartitionsToTxnResponseResults2 {
            name: older.name,
            results: older.results.into_iter().map(|el| el.into()).collect(),
        }
    }
}

impl From<AddPartitionsToTxnResponseResultsResults1> for AddPartitionsToTxnResponseResultsResults2 {
    fn from(older: AddPartitionsToTxnResponseResultsResults1) -> Self {
        AddPartitionsToTxnResponseResultsResults2 {
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}
