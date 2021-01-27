use super::prelude::*;

pub type AddOffsetsToTxnRequest = AddOffsetsToTxnRequest2;
pub type AddOffsetsToTxnResponse = AddOffsetsToTxnResponse2;
pub fn serialize_add_offsets_to_txn_request(
    data: AddOffsetsToTxnRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&AddOffsetsToTxnRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&AddOffsetsToTxnRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_add_offsets_to_txn_response(
    version: i32,
    buf: &mut Bytes,
) -> AddOffsetsToTxnResponse {
    match version {
        0 => AddOffsetsToTxnResponse0::deserialize(buf).into(),
        1 => AddOffsetsToTxnResponse1::deserialize(buf).into(),
        2 => AddOffsetsToTxnResponse::deserialize(buf),
        _ => AddOffsetsToTxnResponse::deserialize(buf),
    }
}

#[derive(Default, Debug, ToBytes)]
pub struct AddOffsetsToTxnRequest0 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub group_id: String,
}

#[derive(Default, Debug, ToBytes)]
pub struct AddOffsetsToTxnRequest1 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub group_id: String,
}

#[derive(Default, Debug, ToBytes)]
pub struct AddOffsetsToTxnRequest2 {
    pub transactional_id: String,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub group_id: String,
}

#[derive(Default, Debug, FromBytes)]
pub struct AddOffsetsToTxnResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct AddOffsetsToTxnResponse1 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, FromBytes)]
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
