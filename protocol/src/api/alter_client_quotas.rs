use super::prelude::*;

pub type AlterClientQuotasRequest = AlterClientQuotasRequest0;
pub type AlterClientQuotasResponse = AlterClientQuotasResponse0;
impl ApiCall for AlterClientQuotasRequest {
    type Response = AlterClientQuotasResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::AlterClientQuotas
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
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
                AlterClientQuotasRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                AlterClientQuotasRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, AlterClientQuotasResponse) {
        let header = HeaderResponse::deserialize(buf, false);
        let response = match version {
            0 => AlterClientQuotasResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => AlterClientQuotasResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (header.correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterClientQuotasRequest0 {
    pub entries: Vec<AlterClientQuotasRequestEntries0>,
    pub validate_only: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterClientQuotasRequestEntries0 {
    pub entity: Vec<AlterClientQuotasRequestEntriesEntity0>,
    pub ops: Vec<AlterClientQuotasRequestEntriesOps0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterClientQuotasRequestEntriesEntity0 {
    pub entity_type: String,
    pub entity_name: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterClientQuotasRequestEntriesOps0 {
    pub key: String,
    pub value: Float64,
    pub remove: Boolean,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterClientQuotasResponse0 {
    pub throttle_time_ms: Int32,
    pub entries: Vec<AlterClientQuotasResponseEntries0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterClientQuotasResponseEntries0 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub entity: Vec<AlterClientQuotasResponseEntriesEntity0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterClientQuotasResponseEntriesEntity0 {
    pub entity_type: String,
    pub entity_name: NullableString,
}
