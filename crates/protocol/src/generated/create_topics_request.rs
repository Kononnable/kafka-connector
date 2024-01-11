use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct CreateTopicsRequest {
    pub topics: Vec<CreatableTopic>,
    pub timeout_ms: i32,
    pub validate_only: bool,
}

#[derive(Debug, Default, Clone)]
pub struct CreatableTopic {
    pub name: String,
    pub num_partitions: i32,
    pub replication_factor: i16,
    pub assignments: Vec<CreatableReplicaAssignment>,
    pub configs: Vec<CreateableTopicConfig>,
}

#[derive(Debug, Default, Clone)]
pub struct CreatableReplicaAssignment {
    pub partition_index: i32,
    pub broker_ids: Vec<i32>,
}

#[derive(Debug, Default, Clone)]
pub struct CreateableTopicConfig {
    pub name: String,
    pub value: String,
}

impl ApiRequest for CreateTopicsRequest {
    type Response = super::create_topics_response::CreateTopicsResponse;

    fn get_api_key() -> i16 {
        19
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        3
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
        if version >= 1 {
            self.validate_only.serialize(version, bytes);
        }
    }
}

impl ToBytes for CreatableTopic {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.num_partitions.serialize(version, bytes);
        }
        if version >= 0 {
            self.replication_factor.serialize(version, bytes);
        }
        if version >= 0 {
            self.assignments.serialize(version, bytes);
        }
        if version >= 0 {
            self.configs.serialize(version, bytes);
        }
    }
}

impl ToBytes for CreatableReplicaAssignment {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.partition_index.serialize(version, bytes);
        }
        if version >= 0 {
            self.broker_ids.serialize(version, bytes);
        }
    }
}

impl ToBytes for CreateableTopicConfig {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.name.serialize(version, bytes);
        }
        if version >= 0 {
            self.value.serialize(version, bytes);
        }
    }
}
