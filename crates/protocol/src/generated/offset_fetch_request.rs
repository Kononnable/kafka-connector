use super::super::prelude::*;

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
        if self.group_id != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "group_id",
                _version,
                "OffsetFetchRequest",
            ));
        }
        if self.topics.is_some() && self.topics != Some(Vec::<OffsetFetchRequestTopic>::default()) {
            return Err(SerializationError::NonIgnorableFieldSet(
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
        if self.name != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "name",
                _version,
                "OffsetFetchRequestTopic",
            ));
        }
        if self.partition_indexes != Vec::<i32>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "partition_indexes",
                _version,
                "OffsetFetchRequestTopic",
            ));
        }
        Ok(())
    }
}
