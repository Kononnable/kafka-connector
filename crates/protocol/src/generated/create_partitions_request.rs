use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct CreatePartitionsRequest {
    /// Each topic that we want to create new partitions inside.
    pub topics: Vec<CreatePartitionsTopic>,

    /// The time in ms to wait for the partitions to be created.
    pub timeout_ms: i32,

    /// If true, then validate the request, but don't actually increase the number of partitions.
    pub validate_only: bool,
}

#[derive(Debug, Clone)]
pub struct CreatePartitionsTopic {
    /// The topic name.
    pub name: String,

    /// The new partition count.
    pub count: i32,

    /// The new partition assignments.
    pub assignments: Vec<CreatePartitionsAssignment>,
}

#[derive(Debug, Clone)]
pub struct CreatePartitionsAssignment {
    /// The assigned broker IDs.
    pub broker_ids: Vec<i32>,
}

impl ApiRequest for CreatePartitionsRequest {
    type Response = super::create_partitions_response::CreatePartitionsResponse;

    fn get_api_key() -> i16 {
        37
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        1
    }

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.topics.serialize(version, bytes);
        }
        if version >= 0 {
            self.timeout_ms.serialize(version, bytes);
        }
        if version >= 0 {
            self.validate_only.serialize(version, bytes);
        }
    }
}

impl Default for CreatePartitionsRequest {
    fn default() -> Self {
        Self {
            topics: Default::default(),
            timeout_ms: Default::default(),
            validate_only: Default::default(),
        }
    }
}

impl ToBytes for CreatePartitionsTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.count.serialize(version, bytes);
        }
        if version >= 0 {
            self.assignments.serialize(version, bytes);
        }
    }
}

impl Default for CreatePartitionsTopic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            count: Default::default(),
            assignments: Default::default(),
        }
    }
}

impl ToBytes for CreatePartitionsAssignment {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.broker_ids.serialize(version, bytes);
        }
    }
}

impl Default for CreatePartitionsAssignment {
    fn default() -> Self {
        Self {
            broker_ids: Default::default(),
        }
    }
}
