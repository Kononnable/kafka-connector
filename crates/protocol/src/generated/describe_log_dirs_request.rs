use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DescribeLogDirsRequest {
    /// Each topic that we want to describe log directories for, or null for all topics.
    pub topics: Option<Vec<DescribableLogDirTopic>>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DescribableLogDirTopic {
    /// The topic name
    pub topic: String,

    /// The partition indxes.
    pub partition_index: Vec<i32>,
}

impl ApiRequest for DescribeLogDirsRequest {
    type Response = super::describe_log_dirs_response::DescribeLogDirsResponse;

    fn get_api_key() -> i16 {
        35
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        1
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
        self.topics.serialize(version, bytes)?;
        Ok(())
    }
}

impl DescribeLogDirsRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.topics.is_none() {
            return Err(SerializationError::NullValue(
                "topics",
                _version,
                "DescribeLogDirsRequest",
            ));
        }
        Ok(())
    }
}

impl ToBytes for DescribableLogDirTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topic.serialize(version, bytes)?;
        self.partition_index.serialize(version, bytes)?;
        Ok(())
    }
}

impl DescribableLogDirTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
