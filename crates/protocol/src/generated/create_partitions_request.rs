use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct CreatePartitionsRequest {
    pub topics: Vec<CreatePartitionsTopic>,
    pub timeout_ms: i32,
    pub validate_only: bool,
}

#[derive(Debug, Default, Clone)]
pub struct CreatePartitionsTopic {
    pub name: String,
    pub count: i32,
    pub assignments: Vec<CreatePartitionsAssignment>,
}

#[derive(Debug, Default, Clone)]
pub struct CreatePartitionsAssignment {
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

impl ToBytes for CreatePartitionsAssignment {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.broker_ids.serialize(version, bytes);
        }
    }
}
