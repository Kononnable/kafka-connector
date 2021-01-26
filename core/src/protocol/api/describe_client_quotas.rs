use super::prelude::*;

pub type DescribeClientQuotasRequest = DescribeClientQuotasRequest0;
pub type DescribeClientQuotasResponse = DescribeClientQuotasResponse0;
pub fn serialize_describe_client_quotas_request(
    data: DescribeClientQuotasRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_describe_client_quotas_response(
    version: i32,
    buf: &mut Bytes,
) -> DescribeClientQuotasResponse {
    match version {
        0 => DescribeClientQuotasResponse::deserialize(buf),
        _ => DescribeClientQuotasResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct DescribeClientQuotasRequest0 {
    pub components: Vec<DescribeClientQuotasRequestComponents0>,
    pub strict: Boolean,
}

#[derive(Default, ToBytes)]
pub struct DescribeClientQuotasRequestComponents0 {
    pub entity_type: String,
    pub match_type: Int8,
    pub match_: NullableString,
}

#[derive(Default, FromBytes)]
pub struct DescribeClientQuotasResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub entries: Vec<DescribeClientQuotasResponseEntries0>,
}

#[derive(Default, FromBytes)]
pub struct DescribeClientQuotasResponseEntries0 {
    pub entity: Vec<DescribeClientQuotasResponseEntriesEntity0>,
    pub values: Vec<DescribeClientQuotasResponseEntriesValues0>,
}

#[derive(Default, FromBytes)]
pub struct DescribeClientQuotasResponseEntriesEntity0 {
    pub entity_type: String,
    pub entity_name: NullableString,
}

#[derive(Default, FromBytes)]
pub struct DescribeClientQuotasResponseEntriesValues0 {
    pub key: String,
    pub value: Float64,
}
