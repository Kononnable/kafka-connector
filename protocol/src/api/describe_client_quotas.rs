use super::prelude::*;

pub type DescribeClientQuotasRequest = DescribeClientQuotasRequest0;
pub type DescribeClientQuotasResponse = DescribeClientQuotasResponse0;
impl ApiCall for DescribeClientQuotasRequest {
    type Response = DescribeClientQuotasResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DescribeClientQuotas
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
                DescribeClientQuotasRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                DescribeClientQuotasRequest::get_api_key(),
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
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, DescribeClientQuotasResponse) {
        let header = HeaderResponse::deserialize(buf, false);
        let response = match version {
            0 => DescribeClientQuotasResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => DescribeClientQuotasResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (header.correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeClientQuotasRequest0 {
    pub components: Vec<DescribeClientQuotasRequestComponents0>,
    pub strict: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct DescribeClientQuotasRequestComponents0 {
    pub entity_type: String,
    pub match_type: Int8,
    pub match_: NullableString,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeClientQuotasResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub entries: Vec<DescribeClientQuotasResponseEntries0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeClientQuotasResponseEntries0 {
    pub entity: Vec<DescribeClientQuotasResponseEntriesEntity0>,
    pub values: Vec<DescribeClientQuotasResponseEntriesValues0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeClientQuotasResponseEntriesEntity0 {
    pub entity_type: String,
    pub entity_name: NullableString,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeClientQuotasResponseEntriesValues0 {
    pub key: String,
    pub value: Float64,
}
