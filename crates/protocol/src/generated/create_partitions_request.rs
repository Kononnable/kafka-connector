use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CreatePartitionsRequest {
    /// Each topic that we want to create new partitions inside.
    pub topics: Vec<CreatePartitionsTopic>,

    /// The time in ms to wait for the partitions to be created.
    pub timeout_ms: i32,

    /// If true, then validate the request, but don't actually increase the number of partitions.
    pub validate_only: bool,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CreatePartitionsTopic {
    /// The topic name.
    pub name: String,

    /// The new partition count.
    pub count: i32,

    /// The new partition assignments.
    pub assignments: Option<Vec<CreatePartitionsAssignment>>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CreatePartitionsAssignment {
    /// The assigned broker IDs.
    pub broker_ids: Vec<i32>,
}

impl ApiRequest for CreatePartitionsRequest {
    type Response = super::create_partitions_response::CreatePartitionsResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(37)
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
        self.topics.serialize(version, _bytes);
        self.timeout_ms.serialize(version, _bytes);
        self.validate_only.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let topics = Vec::<CreatePartitionsTopic>::deserialize(version, bytes);
        let timeout_ms = i32::deserialize(version, bytes);
        let validate_only = bool::deserialize(version, bytes);
        CreatePartitionsRequest {
            topics,
            timeout_ms,
            validate_only,
        }
    }
}

impl CreatePartitionsRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.topics.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for CreatePartitionsTopic {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.name.serialize(version, _bytes);
        self.count.serialize(version, _bytes);
        self.assignments.serialize(version, _bytes);
    }
}

impl CreatePartitionsTopic {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.assignments.iter().flatten() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl FromBytes for CreatePartitionsTopic {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        let count = i32::deserialize(version, bytes);
        let assignments = Option::<Vec<CreatePartitionsAssignment>>::deserialize(version, bytes);
        CreatePartitionsTopic {
            name,
            count,
            assignments,
        }
    }
}

impl ToBytes for CreatePartitionsAssignment {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.broker_ids.serialize(version, _bytes);
    }
}

impl CreatePartitionsAssignment {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for CreatePartitionsAssignment {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let broker_ids = Vec::<i32>::deserialize(version, bytes);
        CreatePartitionsAssignment { broker_ids }
    }
}
