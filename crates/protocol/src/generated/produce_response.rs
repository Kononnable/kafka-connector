use super::super::prelude::*;

/// Version 1 added the throttle time.
///
/// Version 2 added the log append time.
///
/// Version 3 is the same as version 2.
///
/// Version 4 added KAFKA_STORAGE_ERROR as a possible error code.
///
/// Version 5 added LogStartOffset to filter out spurious
/// OutOfOrderSequenceExceptions on the client.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProduceResponse {
    /// Each produce response
    pub responses: Vec<TopicProduceResponse>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct TopicProduceResponse {
    /// The topic name
    pub name: String,

    /// Each partition that we produced to within the topic.
    pub partitions: Vec<PartitionProduceResponse>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionProduceResponse {
    /// The partition index.
    pub partition_index: i32,

    /// The error code, or 0 if there was no error.
    pub error_code: i16,

    /// The base offset.
    pub base_offset: i64,

    /// The timestamp returned by broker after appending the messages. If CreateTime is used for the topic, the timestamp will be -1.  If LogAppendTime is used for the topic, the timestamp will be the broker local time when the messages are appended.
    pub log_append_time_ms: i64,

    /// The log start offset.
    pub log_start_offset: i64,
}

impl ApiResponse for ProduceResponse {
    type Request = super::produce_request::ProduceRequest;

    fn get_api_key() -> ApiKey {
        ApiKey(0)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(7)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.responses.serialize(version, _bytes)?;
        if version >= ApiVersion(1) {
            self.throttle_time_ms.serialize(version, _bytes)?;
        }
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let responses = Vec::<TopicProduceResponse>::deserialize(version, bytes);
        let throttle_time_ms = if version >= ApiVersion(1) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        ProduceResponse {
            responses,
            throttle_time_ms,
        }
    }
}

impl ProduceResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for TopicProduceResponse {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, _bytes)?;
        self.partitions.serialize(version, _bytes)?;
        Ok(())
    }
}

impl TopicProduceResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for TopicProduceResponse {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<PartitionProduceResponse>::deserialize(version, bytes);
        TopicProduceResponse { name, partitions }
    }
}

impl ToBytes for PartitionProduceResponse {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, _bytes)?;
        self.error_code.serialize(version, _bytes)?;
        self.base_offset.serialize(version, _bytes)?;
        if version >= ApiVersion(2) {
            self.log_append_time_ms.serialize(version, _bytes)?;
        }
        if version >= ApiVersion(5) {
            self.log_start_offset.serialize(version, _bytes)?;
        }
        Ok(())
    }
}

impl PartitionProduceResponse {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for PartitionProduceResponse {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        let base_offset = i64::deserialize(version, bytes);
        let log_append_time_ms = if version >= ApiVersion(2) {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let log_start_offset = if version >= ApiVersion(5) {
            i64::deserialize(version, bytes)
        } else {
            Default::default()
        };
        PartitionProduceResponse {
            partition_index,
            error_code,
            base_offset,
            log_append_time_ms,
            log_start_offset,
        }
    }
}

impl Default for PartitionProduceResponse {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            error_code: Default::default(),
            base_offset: Default::default(),
            log_append_time_ms: -1,
            log_start_offset: -1,
        }
    }
}
