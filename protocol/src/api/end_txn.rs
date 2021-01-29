use super::prelude::*;

pub type EndTxnRequest = EndTxnRequest2;
pub type EndTxnResponse = EndTxnResponse2;
impl ApiCall for EndTxnRequest {
    type Response = EndTxnResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::EndTxn
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&EndTxnRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&EndTxnRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> EndTxnResponse {
        match version {
            0 => EndTxnResponse0::deserialize(buf).into(),
            1 => EndTxnResponse1::deserialize(buf).into(),
            2 => EndTxnResponse::deserialize(buf),
            _ => EndTxnResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, ToBytes)]
pub struct EndTxnRequest0 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub committed: Boolean,
}

#[derive(Default, Debug, ToBytes)]
pub struct EndTxnRequest1 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub committed: Boolean,
}

#[derive(Default, Debug, ToBytes)]
pub struct EndTxnRequest2 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub committed: Boolean,
}

#[derive(Default, Debug, FromBytes)]
pub struct EndTxnResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct EndTxnResponse1 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct EndTxnResponse2 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
}

impl TryFrom<EndTxnRequest2> for EndTxnRequest0 {
    type Error = Error;
    fn try_from(latest: EndTxnRequest2) -> Result<Self, Self::Error> {
        Ok(EndTxnRequest0 {
            transactional_id: latest.transactional_id,
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            committed: latest.committed,
        })
    }
}

impl TryFrom<EndTxnRequest2> for EndTxnRequest1 {
    type Error = Error;
    fn try_from(latest: EndTxnRequest2) -> Result<Self, Self::Error> {
        Ok(EndTxnRequest1 {
            transactional_id: latest.transactional_id,
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            committed: latest.committed,
        })
    }
}

impl From<EndTxnResponse0> for EndTxnResponse2 {
    fn from(older: EndTxnResponse0) -> Self {
        EndTxnResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
        }
    }
}

impl From<EndTxnResponse1> for EndTxnResponse2 {
    fn from(older: EndTxnResponse1) -> Self {
        EndTxnResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
        }
    }
}
