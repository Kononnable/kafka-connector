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

    fn get_api_key() -> ApiKey {
        ApiKey(9)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(5)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.group_id.serialize(version, _bytes);
        self.topics.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let group_id = String::deserialize(version, bytes);
        let topics = Option::<Vec<OffsetFetchRequestTopic>>::deserialize(version, bytes);
        OffsetFetchRequest { group_id, topics }
    }
}

impl OffsetFetchRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.topics.iter().flatten() {
            item.validate_fields(_version)?;
        }
        if self.topics.is_none() && !_version.0 < 2 {
            return Err(SerializationError::NullValue(
                "topics",
                *_version,
                "OffsetFetchRequest",
            ));
        }
        Ok(())
    }
}

impl ToBytes for OffsetFetchRequestTopic {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
        self.partition_indexes.serialize(version, _bytes);
    }
}

impl OffsetFetchRequestTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for OffsetFetchRequestTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partition_indexes = Vec::<i32>::deserialize(version, bytes);
        OffsetFetchRequestTopic {
            name,
            partition_indexes,
        }
    }
}
