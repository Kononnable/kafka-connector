use super::prelude::*;

pub type AddOffsetsToTxnRequest = AddOffsetsToTxnRequest2;
pub type AddOffsetsToTxnResponse = AddOffsetsToTxnResponse2;
impl ApiCall for AddOffsetsToTxnRequest {
    type Response = AddOffsetsToTxnResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        2
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::AddOffsetsToTxn
    }
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&AddOffsetsToTxnRequest0::try_from(self)?, buf),
            1 => ToBytes::serialize(&AddOffsetsToTxnRequest1::try_from(self)?, buf),
            2 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> AddOffsetsToTxnResponse {
        match version {
            0 => AddOffsetsToTxnResponse0::deserialize(buf).into(),
            1 => AddOffsetsToTxnResponse1::deserialize(buf).into(),
            2 => AddOffsetsToTxnResponse::deserialize(buf),
            _ => AddOffsetsToTxnResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AddOffsetsToTxnRequest0 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub group_id: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AddOffsetsToTxnRequest1 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub group_id: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AddOffsetsToTxnRequest2 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub group_id: String,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddOffsetsToTxnResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddOffsetsToTxnResponse1 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AddOffsetsToTxnResponse2 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
}

impl TryFrom<AddOffsetsToTxnRequest2> for AddOffsetsToTxnRequest0 {
    type Error = Error;
    fn try_from(latest: AddOffsetsToTxnRequest2) -> Result<Self, Self::Error> {
        Ok(AddOffsetsToTxnRequest0 {
            transactional_id: latest.transactional_id,
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            group_id: latest.group_id,
        })
    }
}

impl TryFrom<AddOffsetsToTxnRequest2> for AddOffsetsToTxnRequest1 {
    type Error = Error;
    fn try_from(latest: AddOffsetsToTxnRequest2) -> Result<Self, Self::Error> {
        Ok(AddOffsetsToTxnRequest1 {
            transactional_id: latest.transactional_id,
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            group_id: latest.group_id,
        })
    }
}

impl From<AddOffsetsToTxnResponse0> for AddOffsetsToTxnResponse2 {
    fn from(older: AddOffsetsToTxnResponse0) -> Self {
        AddOffsetsToTxnResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
        }
    }
}

impl From<AddOffsetsToTxnResponse1> for AddOffsetsToTxnResponse2 {
    fn from(older: AddOffsetsToTxnResponse1) -> Self {
        AddOffsetsToTxnResponse2 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
        }
    }
}
