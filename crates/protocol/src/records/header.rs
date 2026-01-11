use crate::records::base_types::{VarIntBytes, VarIntString};
use crate::{ApiVersion, FromBytes, ToBytes};
use bytes::BytesMut;

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Header {
    pub header_key: VarIntString,
    pub value: VarIntBytes,
}

impl FromBytes for Header {
    fn deserialize(_version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let header_key = FromBytes::deserialize(ApiVersion(0), bytes);
        let value = FromBytes::deserialize(ApiVersion(0), bytes);
        Self { header_key, value }
    }
}
impl ToBytes for Header {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.header_key.serialize(version, _bytes);
        self.value.serialize(version, _bytes);
    }
}
