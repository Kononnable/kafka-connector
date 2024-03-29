use super::super::prelude::*;

/// Version 1 is the same as version 0.
///
/// Version 2 adds the current leader epoch to support fencing.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OffsetForLeaderEpochRequest {
    /// Each topic to get offsets for.
    pub topics: Vec<OffsetForLeaderTopic>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct OffsetForLeaderTopic {
    /// The topic name.
    pub name: String,

    /// Each partition to get offsets for.
    pub partitions: Vec<OffsetForLeaderPartition>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetForLeaderPartition {
    /// The partition index.
    pub partition_index: i32,

    /// An epoch used to fence consumers/replicas with old metadata.  If the epoch provided by the client is larger than the current epoch known to the broker, then the UNKNOWN_LEADER_EPOCH error code will be returned. If the provided epoch is smaller, then the FENCED_LEADER_EPOCH error code will be returned.
    pub current_leader_epoch: i32,

    /// The epoch to look up an offset for.
    pub leader_epoch: i32,
}

impl ApiRequest for OffsetForLeaderEpochRequest {
    type Response = super::offset_for_leader_epoch_response::OffsetForLeaderEpochResponse;

    fn get_api_key() -> i16 {
        23
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        2
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

impl OffsetForLeaderEpochRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for OffsetForLeaderTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        self.partitions.serialize(version, bytes)?;
        Ok(())
    }
}

impl OffsetForLeaderTopic {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl ToBytes for OffsetForLeaderPartition {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.partition_index.serialize(version, bytes)?;
        if version >= 2 {
            self.current_leader_epoch.serialize(version, bytes)?;
        }
        self.leader_epoch.serialize(version, bytes)?;
        Ok(())
    }
}

impl OffsetForLeaderPartition {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl Default for OffsetForLeaderPartition {
    fn default() -> Self {
        Self {
            partition_index: Default::default(),
            current_leader_epoch: -1,
            leader_epoch: Default::default(),
        }
    }
}
