use crate::records::base_types::{VarInt, VarIntBytes, VarIntVec, VarLong};
use crate::records::header::Header;
use crate::{ApiVersion, FromBytes, ToBytes};
use bytes::{BufMut, BytesMut};

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Record {
    pub timestamp_delta: VarLong,
    /// Will be calculated automatically
    pub offset_delta: VarInt,
    pub key: VarIntBytes,
    pub value: VarIntBytes,
    pub headers: VarIntVec<Header>,
}

impl FromBytes for Record {
    fn deserialize(_version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let _length: VarInt = FromBytes::deserialize(ApiVersion(0), bytes);
        let attributes: i8 = FromBytes::deserialize(ApiVersion(0), bytes);
        debug_assert_eq!(
            attributes, 0,
            "Kafka record attributes are reserved, but not utilized in any known version"
        );
        let timestamp_delta = FromBytes::deserialize(ApiVersion(0), bytes);
        let offset_delta = FromBytes::deserialize(ApiVersion(0), bytes);
        let key = FromBytes::deserialize(ApiVersion(0), bytes);
        let value = FromBytes::deserialize(ApiVersion(0), bytes);
        let headers = FromBytes::deserialize(ApiVersion(0), bytes);
        Record {
            timestamp_delta,
            offset_delta,
            key,
            value,
            headers,
        }
    }
}
impl ToBytes for Record {
    fn serialize(&self, _version: ApiVersion, bytes: &mut BytesMut) {
        let mut buffer = bytes.split_off(bytes.len());

        // reserve max space for length (VarInt)
        buffer.put_slice(&[0; 5]);
        let mut length_buf = buffer.split();
        length_buf.clear();

        0_i8.serialize(ApiVersion(0), &mut buffer);
        self.timestamp_delta.serialize(ApiVersion(0), &mut buffer);
        self.offset_delta.serialize(ApiVersion(0), &mut buffer);
        self.key.serialize(ApiVersion(0), &mut buffer);
        self.value.serialize(ApiVersion(0), &mut buffer);
        self.headers.serialize(ApiVersion(0), &mut buffer);

        let length = VarInt(buffer.len() as i32);
        length.serialize(ApiVersion(0), &mut length_buf);

        // Write in the proper field order, second unsplit will likely degrade to extend_from_slice internally
        bytes.unsplit(length_buf);
        bytes.unsplit(buffer);
    }
}
