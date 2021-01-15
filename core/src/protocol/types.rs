use kafka_connector_derive::FromBytes;
use kafka_connector_derive::ToBytes;
use super::from_bytes::FromBytes;
use super::to_bytes::ToBytes;

#[derive(Debug, ToBytes, FromBytes)]
pub struct Records {
}