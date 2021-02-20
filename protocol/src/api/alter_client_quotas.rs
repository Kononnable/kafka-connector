use super::prelude::*;
pub type AlterClientQuotasRequest = AlterClientQuotasRequest0;
impl ApiCall for AlterClientQuotasRequest0 {
    type Response = AlterClientQuotasResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::AlterClientQuotas
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(_version: u16) -> bool {
        false
    }
    fn serialize(&self, version: u16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 2),
            false => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 1),
        }
        ToBytes::serialize(self, buf, Self::is_flexible_version(version), version);
    }
    fn deserialize_response(version: u16, buf: &mut Bytes) -> (i32, Self::Response) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse::deserialize(buf, false, 2).correlation,
            false => HeaderResponse::deserialize(buf, false, 1).correlation,
        };
        let response =
            Self::Response::deserialize(buf, Self::is_flexible_version(version), version);
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterClientQuotasRequest0 {
    #[min_version = 0]
    pub entries: Vec<AlterClientQuotasRequestEntries0>,
    #[min_version = 0]
    pub validate_only: Boolean,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterClientQuotasRequestEntries0 {
    #[min_version = 0]
    pub entity: Vec<AlterClientQuotasRequestEntriesEntity0>,
    #[min_version = 0]
    pub ops: Vec<AlterClientQuotasRequestEntriesOps0>,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterClientQuotasRequestEntriesEntity0 {
    #[min_version = 0]
    pub entity_type: String,
    #[min_version = 0]
    pub entity_name: NullableString,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterClientQuotasRequestEntriesOps0 {
    #[min_version = 0]
    pub key: String,
    #[min_version = 0]
    pub value: Float64,
    #[min_version = 0]
    pub remove: Boolean,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterClientQuotasResponse0 {
    #[min_version = 0]
    pub throttle_time_ms: Int32,
    #[min_version = 0]
    pub entries: Vec<AlterClientQuotasResponseEntries0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterClientQuotasResponseEntries0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub error_message: NullableString,
    #[min_version = 0]
    pub entity: Vec<AlterClientQuotasResponseEntriesEntity0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterClientQuotasResponseEntriesEntity0 {
    #[min_version = 0]
    pub entity_type: String,
    #[min_version = 0]
    pub entity_name: NullableString,
}

impl AlterClientQuotasResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.entries.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl AlterClientQuotasResponseEntries0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.entity.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl AlterClientQuotasResponseEntriesEntity0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
