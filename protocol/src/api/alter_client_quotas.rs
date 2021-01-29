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
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> AlterClientQuotasResponse {
        match version {
            0 => AlterClientQuotasResponse::deserialize(buf),
            _ => AlterClientQuotasResponse::deserialize(buf),
        }
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
