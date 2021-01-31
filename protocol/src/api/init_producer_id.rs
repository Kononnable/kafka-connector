use super::prelude::*;

pub type InitProducerIdRequest = InitProducerIdRequest4;
pub type InitProducerIdResponse = InitProducerIdResponse4;
impl ApiCall for InitProducerIdRequest {
    type Response = InitProducerIdResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        4
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::InitProducerId
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => true,
            3 => true,
            4 => true,
            _ => true,
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
                InitProducerIdRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                InitProducerIdRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &InitProducerIdRequest0::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &InitProducerIdRequest1::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &InitProducerIdRequest2::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &InitProducerIdRequest3::try_from(self)?,
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, InitProducerIdResponse) {
        let header = HeaderResponse::deserialize(buf, false);
        let response =
            match version {
                0 => InitProducerIdResponse0::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                1 => InitProducerIdResponse1::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                2 => InitProducerIdResponse2::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                3 => InitProducerIdResponse3::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                4 => InitProducerIdResponse::deserialize(buf, Self::is_flexible_version(version)),
                _ => InitProducerIdResponse::deserialize(buf, Self::is_flexible_version(version)),
            };
        (header.correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct InitProducerIdRequest0 {
    pub transactional_id: NullableString,
    pub transaction_timeout_ms: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct InitProducerIdRequest1 {
    pub transactional_id: NullableString,
    pub transaction_timeout_ms: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct InitProducerIdRequest2 {
    pub transactional_id: NullableString,
    pub transaction_timeout_ms: Int32,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct InitProducerIdRequest3 {
    pub transactional_id: NullableString,
    pub transaction_timeout_ms: Int32,
    pub producer_id: Optional<Int64>,
    pub producer_epoch: Optional<Int16>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct InitProducerIdRequest4 {
    pub transactional_id: NullableString,
    pub transaction_timeout_ms: Int32,
    pub producer_id: Optional<Int64>,
    pub producer_epoch: Optional<Int16>,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct InitProducerIdResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct InitProducerIdResponse1 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct InitProducerIdResponse2 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct InitProducerIdResponse3 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub tag_buffer: Optional<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct InitProducerIdResponse4 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub producer_id: Int64,
    pub producer_epoch: Int16,
    pub tag_buffer: Optional<TagBuffer>,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "InitProducerIdRequest",
                0,
                "tag_buffer",
            ));
        }
        Ok(InitProducerIdRequest0 {
            transactional_id: latest.transactional_id,
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
        if latest.tag_buffer.is_some() {
            return Err(Error::OldKafkaVersion(
                "InitProducerIdRequest",
                1,
                "tag_buffer",
            ));
        }
        Ok(InitProducerIdRequest1 {
            transactional_id: latest.transactional_id,
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
            tag_buffer: latest.tag_buffer,
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
            tag_buffer: latest.tag_buffer,
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
            ..InitProducerIdResponse4::default()
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
            ..InitProducerIdResponse4::default()
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
            tag_buffer: older.tag_buffer,
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
            tag_buffer: older.tag_buffer,
        }
    }
}
