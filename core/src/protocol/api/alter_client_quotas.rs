use super::prelude::*;

pub type AlterClientQuotasRequest = AlterClientQuotasRequest0;
pub type AlterClientQuotasResponse = AlterClientQuotasResponse0;
pub fn serialize_alter_client_quotas_request(
    data: AlterClientQuotasRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        1 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_alter_client_quotas_response<T>(
    version: i32,
    buf: &mut T,
) -> AlterClientQuotasResponse
where
    T: Iterator<Item = u8>,
{
    match version {
        1 => AlterClientQuotasResponse::deserialize(buf),
        _ => AlterClientQuotasResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct AlterClientQuotasRequest0 {
    pub entries: Vec<AlterClientQuotasRequestEntries0>,
    pub validate_only: Boolean,
}

#[derive(Default, ToBytes)]
pub struct AlterClientQuotasRequestEntries0 {
    pub entity: Vec<AlterClientQuotasRequestEntriesEntity0>,
    pub ops: Vec<AlterClientQuotasRequestEntriesOps0>,
}

#[derive(Default, ToBytes)]
pub struct AlterClientQuotasRequestEntriesEntity0 {
    pub entity_type: String,
    pub entity_name: NullableString,
}

#[derive(Default, ToBytes)]
pub struct AlterClientQuotasRequestEntriesOps0 {
    pub key: String,
    pub value: Float64,
    pub remove: Boolean,
}

#[derive(Default, FromBytes)]
pub struct AlterClientQuotasResponse0 {
    pub throttle_time_ms: Int32,
    pub entries: Vec<AlterClientQuotasResponseEntries0>,
}

#[derive(Default, FromBytes)]
pub struct AlterClientQuotasResponseEntries0 {
    pub error_code: Int16,
    pub error_message: NullableString,
    pub entity: Vec<AlterClientQuotasResponseEntriesEntity0>,
}

#[derive(Default, FromBytes)]
pub struct AlterClientQuotasResponseEntriesEntity0 {
    pub entity_type: String,
    pub entity_name: NullableString,
}
