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

    fn get_api_key() -> ApiKey {
        ApiKey(43)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.topic_partitions.serialize(version, _bytes)?;
        self.timeout_ms.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topic_partitions = Option::<Vec<TopicPartitions>>::deserialize(version, bytes);
        let timeout_ms = i32::deserialize(version, bytes);
        ElectPreferredLeadersRequest {
            topic_partitions,
            timeout_ms,
        }
    }
}

impl ElectPreferredLeadersRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        if self.topic_partitions.is_none() {
            return Err(SerializationError::NullValue(
                "topic_partitions",
                *_version,
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
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.topic.serialize(version, _bytes)?;
        self.partition_id.serialize(version, _bytes)?;
        Ok(())
    }
}

impl TopicPartitions {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for TopicPartitions {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topic = String::deserialize(version, bytes);
        let partition_id = Vec::<i32>::deserialize(version, bytes);
        TopicPartitions {
            topic,
            partition_id,
        }
    }
}
