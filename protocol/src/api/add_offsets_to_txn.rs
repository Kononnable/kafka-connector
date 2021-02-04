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
                AddOffsetsToTxnRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                AddOffsetsToTxnRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &AddOffsetsToTxnRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &AddOffsetsToTxnRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, AddOffsetsToTxnResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => AddOffsetsToTxnResponse0::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            1 => AddOffsetsToTxnResponse1::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            2 => AddOffsetsToTxnResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => AddOffsetsToTxnResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
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

impl From<AddOffsetsToTxnRequest2> for AddOffsetsToTxnRequest0 {
    fn from(latest: AddOffsetsToTxnRequest2) -> AddOffsetsToTxnRequest0 {
        AddOffsetsToTxnRequest0 {
            transactional_id: latest.transactional_id,
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            group_id: latest.group_id,
        }
    }
}

impl From<AddOffsetsToTxnRequest2> for AddOffsetsToTxnRequest1 {
    fn from(latest: AddOffsetsToTxnRequest2) -> AddOffsetsToTxnRequest1 {
        AddOffsetsToTxnRequest1 {
            transactional_id: latest.transactional_id,
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
            group_id: latest.group_id,
        }
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
