use super::prelude::*;

pub type AddPartitionsToTxnRequest = AddPartitionsToTxnRequest2;
pub type AddPartitionsToTxnResponse = AddPartitionsToTxnResponse2;
pub fn serialize_add_partitions_to_txn_request(
    data: AddPartitionsToTxnRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&AddPartitionsToTxnRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&AddPartitionsToTxnRequest1::try_from(data)?, buf),
        3 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_add_partitions_to_txn_response<T>(
    version: i32,
    buf: &mut T,
) -> AddPartitionsToTxnResponse
where
    T: Iterator<Item = u8>,
{
    match version {
        0 => AddPartitionsToTxnResponse0::deserialize(buf).into(),
        1 => AddPartitionsToTxnResponse1::deserialize(buf).into(),
        3 => AddPartitionsToTxnResponse::deserialize(buf),
        _ => AddPartitionsToTxnResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct AddPartitionsToTxnRequest0 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub topics: Vec<AddPartitionsToTxnRequestTopics0>,
}

#[derive(Default, ToBytes)]
pub struct AddPartitionsToTxnRequestTopics0 {
    pub name: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct AddPartitionsToTxnRequest1 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub topics: Vec<AddPartitionsToTxnRequestTopics1>,
}

#[derive(Default, ToBytes)]
pub struct AddPartitionsToTxnRequestTopics1 {
    pub name: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct AddPartitionsToTxnRequest2 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub topics: Vec<AddPartitionsToTxnRequestTopics2>,
}

#[derive(Default, ToBytes)]
pub struct AddPartitionsToTxnRequestTopics2 {
    pub name: String,
    pub partitions: Vec<Int32>,
}

#[derive(Default, FromBytes)]
pub struct AddPartitionsToTxnResponse0 {
    pub throttle_time_ms: Int32,
    pub results: Vec<AddPartitionsToTxnResponseResults0>,
}

#[derive(Default, FromBytes)]
pub struct AddPartitionsToTxnResponseResults0 {
    pub name: String,
    pub results: Vec<AddPartitionsToTxnResponseResultsResults0>,
}

#[derive(Default, FromBytes)]
pub struct AddPartitionsToTxnResponseResultsResults0 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct AddPartitionsToTxnResponse1 {
    pub throttle_time_ms: Int32,
    pub results: Vec<AddPartitionsToTxnResponseResults1>,
}

#[derive(Default, FromBytes)]
pub struct AddPartitionsToTxnResponseResults1 {
    pub name: String,
    pub results: Vec<AddPartitionsToTxnResponseResultsResults1>,
}

#[derive(Default, FromBytes)]
pub struct AddPartitionsToTxnResponseResultsResults1 {
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct AddPartitionsToTxnResponse2 {
    pub throttle_time_ms: Int32,
    pub results: Vec<AddPartitionsToTxnResponseResults2>,
}

#[derive(Default, FromBytes)]
pub struct AddPartitionsToTxnResponseResults2 {
    pub name: String,
    pub results: Vec<AddPartitionsToTxnResponseResultsResults2>,
}

#[derive(Default, FromBytes)]
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
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<AddPartitionsToTxnRequestTopics2> for AddPartitionsToTxnRequestTopics0 {
    type Error = Error;
    fn try_from(latest: AddPartitionsToTxnRequestTopics2) -> Result<Self, Self::Error> {
        Ok(AddPartitionsToTxnRequestTopics0 {
            name: latest.name,
            partitions: latest.partitions.into_iter().collect(),
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
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<AddPartitionsToTxnRequestTopics2> for AddPartitionsToTxnRequestTopics1 {
    type Error = Error;
    fn try_from(latest: AddPartitionsToTxnRequestTopics2) -> Result<Self, Self::Error> {
        Ok(AddPartitionsToTxnRequestTopics1 {
            name: latest.name,
            partitions: latest.partitions.into_iter().collect(),
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
