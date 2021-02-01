use crate::custom_types::{zig_zag_string::ZigZagString, zig_zag_vec::ZigZagVec};
use crate::{from_bytes::FromBytes, to_bytes::ToBytes};
use bytes::{Bytes};

use kafka_connector_derive::FromBytes;
use kafka_connector_derive::ToBytes;

#[derive(Debug, Default, ToBytes, FromBytes, Clone)]
pub struct Header {
    pub key: ZigZagString,
    pub value: ZigZagVec<u8>,
}
