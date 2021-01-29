use super::prelude::*;

pub type InitProducerIdRequest = InitProducerIdRequest4;
pub type InitProducerIdResponse = InitProducerIdResponse4;
pub fn serialize_init_producer_id_request(
    data: InitProducerIdRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&InitProducerIdRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&InitProducerIdRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&InitProducerIdRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&InitProducerIdRequest3::try_from(data)?, buf),
        4 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_init_producer_id_response(
    version: i32,
    buf: &mut Bytes,
) -> InitProducerIdResponse {
    match version {
        0 => InitProducerIdResponse0::deserialize(buf).into(),
        1 => InitProducerIdResponse1::deserialize(buf).into(),
        2 => InitProducerIdResponse2::deserialize(buf).into(),
        3 => InitProducerIdResponse3::deserialize(buf).into(),
        4 => InitProducerIdResponse::deserialize(buf),
        _ => InitProducerIdResponse::deserialize(buf),
    }
}

#[derive(Default, Debug, ToBytes)]
pub struct InitProducerIdRequest0 {
    pub transactional_id: NullableString,
    pub transaction_timeout_ms: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct InitProducerIdRequest1 {
    pub transactional_id: NullableString,
    pub transaction_timeout_ms: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct InitProducerIdRequest2 {
    pub transactional_id: CompactNullableString,
    pub transaction_timeout_ms: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct InitProducerIdRequest3 {
    pub transactional_id: CompactNullableString,
    pub transaction_timeout_ms: Int32,
    pub producer_id: Optional<Int64>,
    pub producer_epoch: Optional<Int16>,
}

#[derive(Default, Debug, ToBytes)]
pub struct InitProducerIdRequest4 {
    pub transactional_id: CompactNullableString,
    pub transaction_timeout_ms: Int32,
    pub producer_id: Optional<Int64>,
    pub producer_epoch: Optional<Int16>,
}

#[derive(Default, Debug, FromBytes)]
pub struct InitProducerIdResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct InitProducerIdResponse1 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct InitProducerIdResponse2 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct InitProducerIdResponse3 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct InitProducerIdResponse4 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
}

impl TryFrom<InitProducerIdRequest4> for InitProducerIdRequest0 {
    type Error = Error;
    fn try_from(latest: InitProducerIdRequest4) -> Result<Self, Self::Error> {
        if latest.producer_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "InitProducerIdRequest",
                0,
                "producer_id",
            ));
        }
        if latest.producer_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "InitProducerIdRequest",
                0,
                "producer_epoch",
            ));
        }
        Ok(InitProducerIdRequest0 {
            transactional_id: latest.transactional_id.into(),
            transaction_timeout_ms: latest.transaction_timeout_ms,
        })
    }
}

impl TryFrom<InitProducerIdRequest4> for InitProducerIdRequest1 {
    type Error = Error;
    fn try_from(latest: InitProducerIdRequest4) -> Result<Self, Self::Error> {
        if latest.producer_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "InitProducerIdRequest",
                1,
                "producer_id",
            ));
        }
        if latest.producer_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "InitProducerIdRequest",
                1,
                "producer_epoch",
            ));
        }
        Ok(InitProducerIdRequest1 {
            transactional_id: latest.transactional_id.into(),
            transaction_timeout_ms: latest.transaction_timeout_ms,
        })
    }
}

impl TryFrom<InitProducerIdRequest4> for InitProducerIdRequest2 {
    type Error = Error;
    fn try_from(latest: InitProducerIdRequest4) -> Result<Self, Self::Error> {
        if latest.producer_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "InitProducerIdRequest",
                2,
                "producer_id",
            ));
        }
        if latest.producer_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "InitProducerIdRequest",
                2,
                "producer_epoch",
            ));
        }
        Ok(InitProducerIdRequest2 {
            transactional_id: latest.transactional_id,
            transaction_timeout_ms: latest.transaction_timeout_ms,
        })
    }
}

impl TryFrom<InitProducerIdRequest4> for InitProducerIdRequest3 {
    type Error = Error;
    fn try_from(latest: InitProducerIdRequest4) -> Result<Self, Self::Error> {
        Ok(InitProducerIdRequest3 {
            transactional_id: latest.transactional_id,
            transaction_timeout_ms: latest.transaction_timeout_ms,
            producer_id: latest.producer_id,
            producer_epoch: latest.producer_epoch,
        })
    }
}

impl From<InitProducerIdResponse0> for InitProducerIdResponse4 {
    fn from(older: InitProducerIdResponse0) -> Self {
        InitProducerIdResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            producer_id: older.producer_id,
            producer_epoch: older.producer_epoch,
        }
    }
}

impl From<InitProducerIdResponse1> for InitProducerIdResponse4 {
    fn from(older: InitProducerIdResponse1) -> Self {
        InitProducerIdResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            producer_id: older.producer_id,
            producer_epoch: older.producer_epoch,
        }
    }
}

impl From<InitProducerIdResponse2> for InitProducerIdResponse4 {
    fn from(older: InitProducerIdResponse2) -> Self {
        InitProducerIdResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            producer_id: older.producer_id,
            producer_epoch: older.producer_epoch,
        }
    }
}

impl From<InitProducerIdResponse3> for InitProducerIdResponse4 {
    fn from(older: InitProducerIdResponse3) -> Self {
        InitProducerIdResponse4 {
            throttle_time_ms: older.throttle_time_ms,
            error_code: older.error_code,
            producer_id: older.producer_id,
            producer_epoch: older.producer_epoch,
        }
    }
}
