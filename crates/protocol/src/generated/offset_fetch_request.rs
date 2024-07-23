use super::super::prelude::*;

/// Starting in version 1, the broker supports fetching offsets from the internal __consumer_offsets topic.
///
/// Starting in version 2, the request can contain a null topics array to indicate that offsets
/// for all topics should be fetched.
///
/// Version 3, 4, and 5 are the same as version 2.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OffsetFetchRequest {
    /// The group to fetch offsets for.
    pub group_id: String,

    /// Each topic we would like to fetch offsets for, or null to fetch offsets for all topics.
    pub topics: Option<Vec<OffsetFetchRequestTopic>>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct OffsetFetchRequestTopic {
    pub name: String,

    /// The partition indexes we would like to fetch offsets for.
    pub partition_indexes: Vec<i32>,
}

impl ApiRequest for OffsetFetchRequest {
    type Response = super::offset_fetch_response::OffsetFetchResponse;

    fn get_api_key() -> i16 {
        9
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        5
    }

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &RequestHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.group_id.serialize(version, bytes)?;
        self.topics.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let group_id = String::deserialize(version, bytes);
        let topics = Option::<Vec<OffsetFetchRequestTopic>>::deserialize(version, bytes);
        OffsetFetchRequest { group_id, topics }
    }
}

impl OffsetFetchRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.topics.is_none() {
            return Err(SerializationError::NullValue(
                "topics",
                _version,
                "OffsetFetchRequest",
            ));
        }
        Ok(())
    }
}

impl ToBytes for OffsetFetchRequestTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.partition_indexes.serialize(version, bytes)?;
        Ok(())
    }
}

impl OffsetFetchRequestTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for OffsetFetchRequestTopic {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partition_indexes = Vec::<i32>::deserialize(version, bytes);
        OffsetFetchRequestTopic {
            name,
            partition_indexes,
        }
    }
}
