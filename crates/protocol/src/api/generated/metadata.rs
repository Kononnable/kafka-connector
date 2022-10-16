use super::super::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct MetadataRequest<const V: u8> {
    topics: MetadataRequestTopics<V>,
    allow_auto_topic_creation: Boolean,
    include_cluster_authorized_operations: Boolean,
    include_topic_authorized_operations: Boolean,
    tag_buffer: TagBuffer,
}

#[derive(Debug, Default, Clone)]
pub struct MetadataRequestTopics<const V: u8> {
    name: String,
    tag_buffer: TagBuffer,
}

impl<const V: u8> MetadataRequest<V> {
    pub fn with_topics(&mut self, topics: MetadataRequestTopics<V>) {
        self.topics = topics;
    }

    pub fn with_allow_auto_topic_creation(&mut self, allow_auto_topic_creation: Boolean) {
        debug_assert!(V < 4, "Field not supported in this version of request");
        self.allow_auto_topic_creation = allow_auto_topic_creation;
    }

    pub fn with_include_cluster_authorized_operations(
        &mut self,
        include_cluster_authorized_operations: Boolean,
    ) {
        debug_assert!(V < 8, "Field not supported in this version of request");
        self.include_cluster_authorized_operations = include_cluster_authorized_operations;
    }

    pub fn with_include_topic_authorized_operations(
        &mut self,
        include_topic_authorized_operations: Boolean,
    ) {
        debug_assert!(V < 8, "Field not supported in this version of request");
        self.include_topic_authorized_operations = include_topic_authorized_operations;
    }

    pub fn with_tag_buffer(&mut self, tag_buffer: TagBuffer) {
        debug_assert!(V < 9, "Field not supported in this version of request");
        self.tag_buffer = tag_buffer;
    }
}

impl<const V: u8> ApiRequest for MetadataRequest<V> {
    type Response = MetadataResponse<V>;

    fn get_max_supported_version() -> u16 {
        12
    }

    fn get_api_key() -> ApiNumbers {
        ApiNumbers::Metadata
    }

    fn serialize(&self, version: u16, bytes: &mut BytesMut, correlation_id: i32, client_id: &str) {
        let is_flexible = 9 >= version;
        match is_flexible {
            true => {
                HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                    .serialize(bytes, false, 2);
            }
            false => {
                HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                    .serialize(bytes, false, 1);
            }
        };
        self.topics.serialize(bytes, is_flexible, version);
        if version >= 4 {
            self.allow_auto_topic_creation
                .serialize(bytes, is_flexible, version);
        }
        if version >= 8 {
            self.include_cluster_authorized_operations
                .serialize(bytes, is_flexible, version);
        }
        if version >= 8 {
            self.include_topic_authorized_operations
                .serialize(bytes, is_flexible, version);
        }
        if version >= 9 {
            self.tag_buffer.serialize(bytes, is_flexible, version);
        }
    }
}

impl<const V: u8> ToBytes for MetadataRequestTopics<V> {
    fn serialize(&self, bytes: &mut BytesMut, is_flexible: bool, version: u16) {
        self.name.serialize(bytes, is_flexible, version);
        if version >= 9 {
            self.tag_buffer.serialize(bytes, is_flexible, version);
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MetadataResponse<const V: u8> {
    throttle_time_ms: Int32,
    brokers: Vec<MetadataResponseBrokers<V>>,
    cluster_id: NullableString,
    controller_id: Int32,
    topics: Vec<MetadataResponseTopics<V>>,
    cluster_authorized_operations: Int32,
    tag_buffer: TagBuffer,
}

#[derive(Debug, Default, Clone)]
pub struct MetadataResponseTopics<const V: u8> {
    error_code: Int16,
    name: String,
    is_internal: Boolean,
    partitions: Vec<MetadataResponsePartitions<V>>,
    topic_authorized_operations: Int32,
    tag_buffer: TagBuffer,
}

#[derive(Debug, Default, Clone)]
pub struct MetadataResponsePartitions<const V: u8> {
    error_code: Int16,
    partition_index: Int32,
    leader_id: Int32,
    leader_epoch: Int32,
    replica_nodes: Vec<Int32>,
    isr_nodes: Vec<Int32>,
    offline_replicas: Vec<Int32>,
    tag_buffer: TagBuffer,
}

#[derive(Debug, Default, Clone)]
pub struct MetadataResponseBrokers<const V: u8> {
    node_id: Int32,
    host: String,
    port: Int32,
    rack: NullableString,
    tag_buffer: TagBuffer,
}

impl<const V: u8> MetadataResponse<V> {
    pub fn throttle_time_ms(&self) -> Int32 {
        debug_assert!(V < 3, "Field not supported in this version of response");
        self.throttle_time_ms
    }

    pub fn brokers(&self) -> &Vec<MetadataResponseBrokers<V>> {
        &self.brokers
    }

    pub fn cluster_id(&self) -> &NullableString {
        debug_assert!(V < 2, "Field not supported in this version of response");
        &self.cluster_id
    }

    pub fn controller_id(&self) -> Int32 {
        debug_assert!(V < 1, "Field not supported in this version of response");
        self.controller_id
    }

    pub fn topics(&self) -> &Vec<MetadataResponseTopics<V>> {
        &self.topics
    }

    pub fn cluster_authorized_operations(&self) -> Int32 {
        debug_assert!(V < 8, "Field not supported in this version of response");
        self.cluster_authorized_operations
    }

    pub fn tag_buffer(&self) -> TagBuffer {
        debug_assert!(V < 9, "Field not supported in this version of response");
        self.tag_buffer
    }
}

impl<const V: u8> ApiResponse for MetadataResponse<V> {
    fn deserialize(version: u16, bytes: &mut Bytes) -> (i32, Self) {
        let is_flexible = 9 >= version;
        let correlation = match is_flexible {
            true => HeaderResponse::deserialize(bytes, false, 2).correlation,
            false => HeaderResponse::deserialize(bytes, false, 1).correlation,
        };
        let throttle_time_ms = if version >= 3 {
            Int32::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        let brokers = Vec::<MetadataResponseBrokers<V>>::deserialize(bytes, is_flexible, version);
        let cluster_id = if version >= 2 {
            NullableString::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        let controller_id = if version >= 1 {
            Int32::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        let topics = Vec::<MetadataResponseTopics<V>>::deserialize(bytes, is_flexible, version);
        let cluster_authorized_operations = if version >= 8 {
            Int32::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        let tag_buffer = if version >= 9 {
            TagBuffer::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        (
            correlation,
            Self {
                throttle_time_ms,
                brokers,
                cluster_id,
                controller_id,
                topics,
                cluster_authorized_operations,
                tag_buffer,
            },
        )
    }

    fn get_general_error(&self) -> Option<ApiError> {
        None
    }
}

impl<const V: u8> MetadataResponseTopics<V> {
    pub fn error_code(&self) -> Int16 {
        self.error_code
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_internal(&self) -> Boolean {
        debug_assert!(V < 1, "Field not supported in this version of response");
        self.is_internal
    }

    pub fn partitions(&self) -> &Vec<MetadataResponsePartitions<V>> {
        &self.partitions
    }

    pub fn topic_authorized_operations(&self) -> Int32 {
        debug_assert!(V < 8, "Field not supported in this version of response");
        self.topic_authorized_operations
    }

    pub fn tag_buffer(&self) -> TagBuffer {
        debug_assert!(V < 9, "Field not supported in this version of response");
        self.tag_buffer
    }
}

impl<const V: u8> FromBytes for MetadataResponseTopics<V> {
    fn deserialize(bytes: &mut Bytes, is_flexible: bool, version: u16) -> Self {
        let error_code = Int16::deserialize(bytes, is_flexible, version);
        let name = String::deserialize(bytes, is_flexible, version);
        let is_internal = if version >= 1 {
            Boolean::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        let partitions =
            Vec::<MetadataResponsePartitions<V>>::deserialize(bytes, is_flexible, version);
        let topic_authorized_operations = if version >= 8 {
            Int32::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        let tag_buffer = if version >= 9 {
            TagBuffer::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        Self {
            error_code,
            name,
            is_internal,
            partitions,
            topic_authorized_operations,
            tag_buffer,
        }
    }
}

impl<const V: u8> MetadataResponsePartitions<V> {
    pub fn error_code(&self) -> Int16 {
        self.error_code
    }

    pub fn partition_index(&self) -> Int32 {
        self.partition_index
    }

    pub fn leader_id(&self) -> Int32 {
        self.leader_id
    }

    pub fn leader_epoch(&self) -> Int32 {
        debug_assert!(V < 7, "Field not supported in this version of response");
        self.leader_epoch
    }

    pub fn replica_nodes(&self) -> &Vec<Int32> {
        &self.replica_nodes
    }

    pub fn isr_nodes(&self) -> &Vec<Int32> {
        &self.isr_nodes
    }

    pub fn offline_replicas(&self) -> &Vec<Int32> {
        debug_assert!(V < 5, "Field not supported in this version of response");
        &self.offline_replicas
    }

    pub fn tag_buffer(&self) -> TagBuffer {
        debug_assert!(V < 9, "Field not supported in this version of response");
        self.tag_buffer
    }
}

impl<const V: u8> FromBytes for MetadataResponsePartitions<V> {
    fn deserialize(bytes: &mut Bytes, is_flexible: bool, version: u16) -> Self {
        let error_code = Int16::deserialize(bytes, is_flexible, version);
        let partition_index = Int32::deserialize(bytes, is_flexible, version);
        let leader_id = Int32::deserialize(bytes, is_flexible, version);
        let leader_epoch = if version >= 7 {
            Int32::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        let replica_nodes = Vec::<Int32>::deserialize(bytes, is_flexible, version);
        let isr_nodes = Vec::<Int32>::deserialize(bytes, is_flexible, version);
        let offline_replicas = if version >= 5 {
            Vec::<Int32>::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        let tag_buffer = if version >= 9 {
            TagBuffer::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        Self {
            error_code,
            partition_index,
            leader_id,
            leader_epoch,
            replica_nodes,
            isr_nodes,
            offline_replicas,
            tag_buffer,
        }
    }
}

impl<const V: u8> MetadataResponseBrokers<V> {
    pub fn node_id(&self) -> Int32 {
        self.node_id
    }

    pub fn host(&self) -> &String {
        &self.host
    }

    pub fn port(&self) -> Int32 {
        self.port
    }

    pub fn rack(&self) -> &NullableString {
        debug_assert!(V < 1, "Field not supported in this version of response");
        &self.rack
    }

    pub fn tag_buffer(&self) -> TagBuffer {
        debug_assert!(V < 9, "Field not supported in this version of response");
        self.tag_buffer
    }
}

impl<const V: u8> FromBytes for MetadataResponseBrokers<V> {
    fn deserialize(bytes: &mut Bytes, is_flexible: bool, version: u16) -> Self {
        let node_id = Int32::deserialize(bytes, is_flexible, version);
        let host = String::deserialize(bytes, is_flexible, version);
        let port = Int32::deserialize(bytes, is_flexible, version);
        let rack = if version >= 1 {
            NullableString::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        let tag_buffer = if version >= 9 {
            TagBuffer::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        Self {
            node_id,
            host,
            port,
            rack,
            tag_buffer,
        }
    }
}
