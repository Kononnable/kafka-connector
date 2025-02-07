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

    fn get_api_key() -> ApiKey {
        ApiKey(23)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(2)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.topics.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topics = Vec::<OffsetForLeaderTopic>::deserialize(version, bytes);
        OffsetForLeaderEpochRequest { topics }
    }
}

impl OffsetForLeaderEpochRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.topics.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for OffsetForLeaderTopic {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
        self.partitions.serialize(version, _bytes);
    }
}

impl OffsetForLeaderTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.partitions.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for OffsetForLeaderTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let partitions = Vec::<OffsetForLeaderPartition>::deserialize(version, bytes);
        OffsetForLeaderTopic { name, partitions }
    }
}

impl ToBytes for OffsetForLeaderPartition {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.partition_index.serialize(version, _bytes);
        if version >= ApiVersion(2) {
            self.current_leader_epoch.serialize(version, _bytes);
        }
        self.leader_epoch.serialize(version, _bytes);
    }
}

impl OffsetForLeaderPartition {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for OffsetForLeaderPartition {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let partition_index = i32::deserialize(version, bytes);
        let current_leader_epoch = if version >= ApiVersion(2) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let leader_epoch = i32::deserialize(version, bytes);
        OffsetForLeaderPartition {
            partition_index,
            current_leader_epoch,
            leader_epoch,
        }
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
