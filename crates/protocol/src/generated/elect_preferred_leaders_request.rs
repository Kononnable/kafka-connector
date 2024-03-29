use super::super::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ElectPreferredLeadersRequest {
    /// The topic partitions to elect the preferred leader of.
    pub topic_partitions: Option<Vec<TopicPartitions>>,

    /// The time in ms to wait for the election to complete.
    pub timeout_ms: i32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TopicPartitions {
    /// The name of a topic.
    pub topic: String,

    /// The partitions of this topic whose preferred leader should be elected
    pub partition_id: Vec<i32>,
}

impl ApiRequest for ElectPreferredLeadersRequest {
    type Response = super::elect_preferred_leaders_response::ElectPreferredLeadersResponse;

    fn get_api_key() -> i16 {
        43
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        0
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
        self.topic_partitions.serialize(version, bytes)?;
        self.timeout_ms.serialize(version, bytes)?;
        Ok(())
    }
}

impl ElectPreferredLeadersRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.topic_partitions.is_none() {
            return Err(SerializationError::NullValue(
                "topic_partitions",
                _version,
                "ElectPreferredLeadersRequest",
            ));
        }
        Ok(())
    }
}

impl Default for ElectPreferredLeadersRequest {
    fn default() -> Self {
        Self {
            topic_partitions: Default::default(),
            timeout_ms: 60000,
        }
    }
}

impl ToBytes for TopicPartitions {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topic.serialize(version, bytes)?;
        self.partition_id.serialize(version, bytes)?;
        Ok(())
    }
}

impl TopicPartitions {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}
