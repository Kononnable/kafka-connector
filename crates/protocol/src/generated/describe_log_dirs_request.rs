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

    fn get_api_key() -> ApiKey {
        ApiKey(35)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(1)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.topics.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topics = Option::<Vec<DescribableLogDirTopic>>::deserialize(version, bytes);
        DescribeLogDirsRequest { topics }
    }
}

impl DescribeLogDirsRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for DescribableLogDirTopic {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topic.serialize(version, _bytes)?;
        self.partition_index.serialize(version, _bytes)?;
        Ok(())
    }
}

impl DescribableLogDirTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DescribableLogDirTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topic = String::deserialize(version, bytes);
        let partition_index = Vec::<i32>::deserialize(version, bytes);
        DescribableLogDirTopic {
            topic,
            partition_index,
        }
    }
}
