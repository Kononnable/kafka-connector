use super::prelude::*;

pub type MetadataRequest = MetadataRequest9;
pub type MetadataResponse = MetadataResponse9;
impl ApiCall for MetadataRequest {
    type Response = MetadataResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        9
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::Metadata
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => false,
            5 => false,
            6 => false,
            7 => false,
            8 => false,
            9 => true,
            _ => true,
        }
    }
    fn serialize(
        self,
        version: i16,
        buf: &mut BytesMut,
        correlation_id: i32,
        client_id: &str,
    ) -> Result<(), Error> {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                MetadataRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                MetadataRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &MetadataRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &MetadataRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &MetadataRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &MetadataRequest3::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(
                &MetadataRequest4::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            5 => ToBytes::serialize(
                &MetadataRequest5::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            6 => ToBytes::serialize(
                &MetadataRequest6::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            7 => ToBytes::serialize(
                &MetadataRequest7::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            8 => ToBytes::serialize(
                &MetadataRequest8::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            9 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, MetadataResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => MetadataResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => MetadataResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => MetadataResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => MetadataResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => MetadataResponse4::deserialize(buf, Self::is_flexible_version(version)).into(),
            5 => MetadataResponse5::deserialize(buf, Self::is_flexible_version(version)).into(),
            6 => MetadataResponse6::deserialize(buf, Self::is_flexible_version(version)).into(),
            7 => MetadataResponse7::deserialize(buf, Self::is_flexible_version(version)).into(),
            8 => MetadataResponse8::deserialize(buf, Self::is_flexible_version(version)).into(),
            9 => MetadataResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => MetadataResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequest0 {
    pub topics: Vec<MetadataRequestTopics0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequestTopics0 {
    pub name: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequest1 {
    pub topics: Vec<MetadataRequestTopics1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequestTopics1 {
    pub name: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequest2 {
    pub topics: Vec<MetadataRequestTopics2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequestTopics2 {
    pub name: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequest3 {
    pub topics: Vec<MetadataRequestTopics3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequestTopics3 {
    pub name: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequest4 {
    pub topics: Vec<MetadataRequestTopics4>,
    pub allow_auto_topic_creation: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequestTopics4 {
    pub name: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequest5 {
    pub topics: Vec<MetadataRequestTopics5>,
    pub allow_auto_topic_creation: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequestTopics5 {
    pub name: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequest6 {
    pub topics: Vec<MetadataRequestTopics6>,
    pub allow_auto_topic_creation: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequestTopics6 {
    pub name: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequest7 {
    pub topics: Vec<MetadataRequestTopics7>,
    pub allow_auto_topic_creation: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequestTopics7 {
    pub name: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequest8 {
    pub topics: Vec<MetadataRequestTopics8>,
    pub allow_auto_topic_creation: Boolean,
    pub include_cluster_authorized_operations: Boolean,
    pub include_topic_authorized_operations: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequestTopics8 {
    pub name: String,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequest9 {
    pub topics: Vec<MetadataRequestTopics9>,
    pub allow_auto_topic_creation: Boolean,
    pub include_cluster_authorized_operations: Boolean,
    pub include_topic_authorized_operations: Boolean,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct MetadataRequestTopics9 {
    pub name: String,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponse0 {
    pub brokers: Vec<MetadataResponseBrokers0>,
    pub topics: Vec<MetadataResponseTopics0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseBrokers0 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopics0 {
    pub error_code: Int16,
    pub name: String,
    pub partitions: Vec<MetadataResponseTopicsPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopicsPartitions0 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponse1 {
    pub brokers: Vec<MetadataResponseBrokers1>,
    pub controller_id: Option<Int32>,
    pub topics: Vec<MetadataResponseTopics1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseBrokers1 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Option<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopics1 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Option<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopicsPartitions1 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponse2 {
    pub brokers: Vec<MetadataResponseBrokers2>,
    pub cluster_id: Option<NullableString>,
    pub controller_id: Option<Int32>,
    pub topics: Vec<MetadataResponseTopics2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseBrokers2 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Option<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopics2 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Option<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopicsPartitions2 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponse3 {
    pub throttle_time_ms: Option<Int32>,
    pub brokers: Vec<MetadataResponseBrokers3>,
    pub cluster_id: Option<NullableString>,
    pub controller_id: Option<Int32>,
    pub topics: Vec<MetadataResponseTopics3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseBrokers3 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Option<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopics3 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Option<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopicsPartitions3 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponse4 {
    pub throttle_time_ms: Option<Int32>,
    pub brokers: Vec<MetadataResponseBrokers4>,
    pub cluster_id: Option<NullableString>,
    pub controller_id: Option<Int32>,
    pub topics: Vec<MetadataResponseTopics4>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseBrokers4 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Option<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopics4 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Option<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions4>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopicsPartitions4 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponse5 {
    pub throttle_time_ms: Option<Int32>,
    pub brokers: Vec<MetadataResponseBrokers5>,
    pub cluster_id: Option<NullableString>,
    pub controller_id: Option<Int32>,
    pub topics: Vec<MetadataResponseTopics5>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseBrokers5 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Option<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopics5 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Option<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions5>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopicsPartitions5 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
    pub offline_replicas: Option<Vec<Int32>>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponse6 {
    pub throttle_time_ms: Option<Int32>,
    pub brokers: Vec<MetadataResponseBrokers6>,
    pub cluster_id: Option<NullableString>,
    pub controller_id: Option<Int32>,
    pub topics: Vec<MetadataResponseTopics6>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseBrokers6 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Option<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopics6 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Option<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions6>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopicsPartitions6 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
    pub offline_replicas: Option<Vec<Int32>>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponse7 {
    pub throttle_time_ms: Option<Int32>,
    pub brokers: Vec<MetadataResponseBrokers7>,
    pub cluster_id: Option<NullableString>,
    pub controller_id: Option<Int32>,
    pub topics: Vec<MetadataResponseTopics7>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseBrokers7 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Option<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopics7 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Option<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions7>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopicsPartitions7 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub leader_epoch: Option<Int32>,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
    pub offline_replicas: Option<Vec<Int32>>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponse8 {
    pub throttle_time_ms: Option<Int32>,
    pub brokers: Vec<MetadataResponseBrokers8>,
    pub cluster_id: Option<NullableString>,
    pub controller_id: Option<Int32>,
    pub topics: Vec<MetadataResponseTopics8>,
    pub cluster_authorized_operations: Option<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseBrokers8 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Option<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopics8 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Option<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions8>,
    pub topic_authorized_operations: Option<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopicsPartitions8 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub leader_epoch: Option<Int32>,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
    pub offline_replicas: Option<Vec<Int32>>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponse9 {
    pub throttle_time_ms: Option<Int32>,
    pub brokers: Vec<MetadataResponseBrokers9>,
    pub cluster_id: Option<NullableString>,
    pub controller_id: Option<Int32>,
    pub topics: Vec<MetadataResponseTopics9>,
    pub cluster_authorized_operations: Option<Int32>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseBrokers9 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Option<NullableString>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopics9 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Option<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions9>,
    pub topic_authorized_operations: Option<Int32>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct MetadataResponseTopicsPartitions9 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub leader_epoch: Option<Int32>,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
    pub offline_replicas: Option<Vec<Int32>>,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<MetadataRequest9> for MetadataRequest0 {
    fn from(latest: MetadataRequest9) -> MetadataRequest0 {
        log::debug!(
            "Using old api format - MetadataRequest0, ignoring field allow_auto_topic_creation"
        );
        log::debug!("Using old api format - MetadataRequest0, ignoring field include_cluster_authorized_operations");
        log::debug!("Using old api format - MetadataRequest0, ignoring field include_topic_authorized_operations");
        MetadataRequest0 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<MetadataRequestTopics9> for MetadataRequestTopics0 {
    fn from(latest: MetadataRequestTopics9) -> MetadataRequestTopics0 {
        MetadataRequestTopics0 { name: latest.name }
    }
}

impl From<MetadataRequest9> for MetadataRequest1 {
    fn from(latest: MetadataRequest9) -> MetadataRequest1 {
        log::debug!(
            "Using old api format - MetadataRequest1, ignoring field allow_auto_topic_creation"
        );
        log::debug!("Using old api format - MetadataRequest1, ignoring field include_cluster_authorized_operations");
        log::debug!("Using old api format - MetadataRequest1, ignoring field include_topic_authorized_operations");
        MetadataRequest1 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<MetadataRequestTopics9> for MetadataRequestTopics1 {
    fn from(latest: MetadataRequestTopics9) -> MetadataRequestTopics1 {
        MetadataRequestTopics1 { name: latest.name }
    }
}

impl From<MetadataRequest9> for MetadataRequest2 {
    fn from(latest: MetadataRequest9) -> MetadataRequest2 {
        log::debug!(
            "Using old api format - MetadataRequest2, ignoring field allow_auto_topic_creation"
        );
        log::debug!("Using old api format - MetadataRequest2, ignoring field include_cluster_authorized_operations");
        log::debug!("Using old api format - MetadataRequest2, ignoring field include_topic_authorized_operations");
        MetadataRequest2 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<MetadataRequestTopics9> for MetadataRequestTopics2 {
    fn from(latest: MetadataRequestTopics9) -> MetadataRequestTopics2 {
        MetadataRequestTopics2 { name: latest.name }
    }
}

impl From<MetadataRequest9> for MetadataRequest3 {
    fn from(latest: MetadataRequest9) -> MetadataRequest3 {
        log::debug!(
            "Using old api format - MetadataRequest3, ignoring field allow_auto_topic_creation"
        );
        log::debug!("Using old api format - MetadataRequest3, ignoring field include_cluster_authorized_operations");
        log::debug!("Using old api format - MetadataRequest3, ignoring field include_topic_authorized_operations");
        MetadataRequest3 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<MetadataRequestTopics9> for MetadataRequestTopics3 {
    fn from(latest: MetadataRequestTopics9) -> MetadataRequestTopics3 {
        MetadataRequestTopics3 { name: latest.name }
    }
}

impl From<MetadataRequest9> for MetadataRequest4 {
    fn from(latest: MetadataRequest9) -> MetadataRequest4 {
        log::debug!("Using old api format - MetadataRequest4, ignoring field include_cluster_authorized_operations");
        log::debug!("Using old api format - MetadataRequest4, ignoring field include_topic_authorized_operations");
        MetadataRequest4 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            allow_auto_topic_creation: latest.allow_auto_topic_creation,
        }
    }
}

impl From<MetadataRequestTopics9> for MetadataRequestTopics4 {
    fn from(latest: MetadataRequestTopics9) -> MetadataRequestTopics4 {
        MetadataRequestTopics4 { name: latest.name }
    }
}

impl From<MetadataRequest9> for MetadataRequest5 {
    fn from(latest: MetadataRequest9) -> MetadataRequest5 {
        log::debug!("Using old api format - MetadataRequest5, ignoring field include_cluster_authorized_operations");
        log::debug!("Using old api format - MetadataRequest5, ignoring field include_topic_authorized_operations");
        MetadataRequest5 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            allow_auto_topic_creation: latest.allow_auto_topic_creation,
        }
    }
}

impl From<MetadataRequestTopics9> for MetadataRequestTopics5 {
    fn from(latest: MetadataRequestTopics9) -> MetadataRequestTopics5 {
        MetadataRequestTopics5 { name: latest.name }
    }
}

impl From<MetadataRequest9> for MetadataRequest6 {
    fn from(latest: MetadataRequest9) -> MetadataRequest6 {
        log::debug!("Using old api format - MetadataRequest6, ignoring field include_cluster_authorized_operations");
        log::debug!("Using old api format - MetadataRequest6, ignoring field include_topic_authorized_operations");
        MetadataRequest6 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            allow_auto_topic_creation: latest.allow_auto_topic_creation,
        }
    }
}

impl From<MetadataRequestTopics9> for MetadataRequestTopics6 {
    fn from(latest: MetadataRequestTopics9) -> MetadataRequestTopics6 {
        MetadataRequestTopics6 { name: latest.name }
    }
}

impl From<MetadataRequest9> for MetadataRequest7 {
    fn from(latest: MetadataRequest9) -> MetadataRequest7 {
        log::debug!("Using old api format - MetadataRequest7, ignoring field include_cluster_authorized_operations");
        log::debug!("Using old api format - MetadataRequest7, ignoring field include_topic_authorized_operations");
        MetadataRequest7 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            allow_auto_topic_creation: latest.allow_auto_topic_creation,
        }
    }
}

impl From<MetadataRequestTopics9> for MetadataRequestTopics7 {
    fn from(latest: MetadataRequestTopics9) -> MetadataRequestTopics7 {
        MetadataRequestTopics7 { name: latest.name }
    }
}

impl From<MetadataRequest9> for MetadataRequest8 {
    fn from(latest: MetadataRequest9) -> MetadataRequest8 {
        MetadataRequest8 {
            topics: latest.topics.into_iter().map(|ele| ele.into()).collect(),
            allow_auto_topic_creation: latest.allow_auto_topic_creation,
            include_cluster_authorized_operations: latest.include_cluster_authorized_operations,
            include_topic_authorized_operations: latest.include_topic_authorized_operations,
        }
    }
}

impl From<MetadataRequestTopics9> for MetadataRequestTopics8 {
    fn from(latest: MetadataRequestTopics9) -> MetadataRequestTopics8 {
        MetadataRequestTopics8 { name: latest.name }
    }
}

impl From<MetadataResponse0> for MetadataResponse9 {
    fn from(older: MetadataResponse0) -> Self {
        MetadataResponse9 {
            brokers: older.brokers.into_iter().map(|el| el.into()).collect(),
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponse9::default()
        }
    }
}

impl From<MetadataResponseBrokers0> for MetadataResponseBrokers9 {
    fn from(older: MetadataResponseBrokers0) -> Self {
        MetadataResponseBrokers9 {
            node_id: older.node_id,
            host: older.host,
            port: older.port,
            ..MetadataResponseBrokers9::default()
        }
    }
}

impl From<MetadataResponseTopics0> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics0) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponseTopics9::default()
        }
    }
}

impl From<MetadataResponseTopicsPartitions0> for MetadataResponseTopicsPartitions9 {
    fn from(older: MetadataResponseTopicsPartitions0) -> Self {
        MetadataResponseTopicsPartitions9 {
            error_code: older.error_code,
            partition_index: older.partition_index,
            leader_id: older.leader_id,
            replica_nodes: older.replica_nodes,
            isr_nodes: older.isr_nodes,
            ..MetadataResponseTopicsPartitions9::default()
        }
    }
}

impl From<MetadataResponse1> for MetadataResponse9 {
    fn from(older: MetadataResponse1) -> Self {
        MetadataResponse9 {
            brokers: older.brokers.into_iter().map(|el| el.into()).collect(),
            controller_id: older.controller_id,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponse9::default()
        }
    }
}

impl From<MetadataResponseBrokers1> for MetadataResponseBrokers9 {
    fn from(older: MetadataResponseBrokers1) -> Self {
        MetadataResponseBrokers9 {
            node_id: older.node_id,
            host: older.host,
            port: older.port,
            rack: older.rack,
            ..MetadataResponseBrokers9::default()
        }
    }
}

impl From<MetadataResponseTopics1> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics1) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name,
            is_internal: older.is_internal,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponseTopics9::default()
        }
    }
}

impl From<MetadataResponseTopicsPartitions1> for MetadataResponseTopicsPartitions9 {
    fn from(older: MetadataResponseTopicsPartitions1) -> Self {
        MetadataResponseTopicsPartitions9 {
            error_code: older.error_code,
            partition_index: older.partition_index,
            leader_id: older.leader_id,
            replica_nodes: older.replica_nodes,
            isr_nodes: older.isr_nodes,
            ..MetadataResponseTopicsPartitions9::default()
        }
    }
}

impl From<MetadataResponse2> for MetadataResponse9 {
    fn from(older: MetadataResponse2) -> Self {
        MetadataResponse9 {
            brokers: older.brokers.into_iter().map(|el| el.into()).collect(),
            cluster_id: older.cluster_id,
            controller_id: older.controller_id,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponse9::default()
        }
    }
}

impl From<MetadataResponseBrokers2> for MetadataResponseBrokers9 {
    fn from(older: MetadataResponseBrokers2) -> Self {
        MetadataResponseBrokers9 {
            node_id: older.node_id,
            host: older.host,
            port: older.port,
            rack: older.rack,
            ..MetadataResponseBrokers9::default()
        }
    }
}

impl From<MetadataResponseTopics2> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics2) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name,
            is_internal: older.is_internal,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponseTopics9::default()
        }
    }
}

impl From<MetadataResponseTopicsPartitions2> for MetadataResponseTopicsPartitions9 {
    fn from(older: MetadataResponseTopicsPartitions2) -> Self {
        MetadataResponseTopicsPartitions9 {
            error_code: older.error_code,
            partition_index: older.partition_index,
            leader_id: older.leader_id,
            replica_nodes: older.replica_nodes,
            isr_nodes: older.isr_nodes,
            ..MetadataResponseTopicsPartitions9::default()
        }
    }
}

impl From<MetadataResponse3> for MetadataResponse9 {
    fn from(older: MetadataResponse3) -> Self {
        MetadataResponse9 {
            throttle_time_ms: older.throttle_time_ms,
            brokers: older.brokers.into_iter().map(|el| el.into()).collect(),
            cluster_id: older.cluster_id,
            controller_id: older.controller_id,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponse9::default()
        }
    }
}

impl From<MetadataResponseBrokers3> for MetadataResponseBrokers9 {
    fn from(older: MetadataResponseBrokers3) -> Self {
        MetadataResponseBrokers9 {
            node_id: older.node_id,
            host: older.host,
            port: older.port,
            rack: older.rack,
            ..MetadataResponseBrokers9::default()
        }
    }
}

impl From<MetadataResponseTopics3> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics3) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name,
            is_internal: older.is_internal,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponseTopics9::default()
        }
    }
}

impl From<MetadataResponseTopicsPartitions3> for MetadataResponseTopicsPartitions9 {
    fn from(older: MetadataResponseTopicsPartitions3) -> Self {
        MetadataResponseTopicsPartitions9 {
            error_code: older.error_code,
            partition_index: older.partition_index,
            leader_id: older.leader_id,
            replica_nodes: older.replica_nodes,
            isr_nodes: older.isr_nodes,
            ..MetadataResponseTopicsPartitions9::default()
        }
    }
}

impl From<MetadataResponse4> for MetadataResponse9 {
    fn from(older: MetadataResponse4) -> Self {
        MetadataResponse9 {
            throttle_time_ms: older.throttle_time_ms,
            brokers: older.brokers.into_iter().map(|el| el.into()).collect(),
            cluster_id: older.cluster_id,
            controller_id: older.controller_id,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponse9::default()
        }
    }
}

impl From<MetadataResponseBrokers4> for MetadataResponseBrokers9 {
    fn from(older: MetadataResponseBrokers4) -> Self {
        MetadataResponseBrokers9 {
            node_id: older.node_id,
            host: older.host,
            port: older.port,
            rack: older.rack,
            ..MetadataResponseBrokers9::default()
        }
    }
}

impl From<MetadataResponseTopics4> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics4) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name,
            is_internal: older.is_internal,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponseTopics9::default()
        }
    }
}

impl From<MetadataResponseTopicsPartitions4> for MetadataResponseTopicsPartitions9 {
    fn from(older: MetadataResponseTopicsPartitions4) -> Self {
        MetadataResponseTopicsPartitions9 {
            error_code: older.error_code,
            partition_index: older.partition_index,
            leader_id: older.leader_id,
            replica_nodes: older.replica_nodes,
            isr_nodes: older.isr_nodes,
            ..MetadataResponseTopicsPartitions9::default()
        }
    }
}

impl From<MetadataResponse5> for MetadataResponse9 {
    fn from(older: MetadataResponse5) -> Self {
        MetadataResponse9 {
            throttle_time_ms: older.throttle_time_ms,
            brokers: older.brokers.into_iter().map(|el| el.into()).collect(),
            cluster_id: older.cluster_id,
            controller_id: older.controller_id,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponse9::default()
        }
    }
}

impl From<MetadataResponseBrokers5> for MetadataResponseBrokers9 {
    fn from(older: MetadataResponseBrokers5) -> Self {
        MetadataResponseBrokers9 {
            node_id: older.node_id,
            host: older.host,
            port: older.port,
            rack: older.rack,
            ..MetadataResponseBrokers9::default()
        }
    }
}

impl From<MetadataResponseTopics5> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics5) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name,
            is_internal: older.is_internal,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponseTopics9::default()
        }
    }
}

impl From<MetadataResponseTopicsPartitions5> for MetadataResponseTopicsPartitions9 {
    fn from(older: MetadataResponseTopicsPartitions5) -> Self {
        MetadataResponseTopicsPartitions9 {
            error_code: older.error_code,
            partition_index: older.partition_index,
            leader_id: older.leader_id,
            replica_nodes: older.replica_nodes,
            isr_nodes: older.isr_nodes,
            offline_replicas: older.offline_replicas,
            ..MetadataResponseTopicsPartitions9::default()
        }
    }
}

impl From<MetadataResponse6> for MetadataResponse9 {
    fn from(older: MetadataResponse6) -> Self {
        MetadataResponse9 {
            throttle_time_ms: older.throttle_time_ms,
            brokers: older.brokers.into_iter().map(|el| el.into()).collect(),
            cluster_id: older.cluster_id,
            controller_id: older.controller_id,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponse9::default()
        }
    }
}

impl From<MetadataResponseBrokers6> for MetadataResponseBrokers9 {
    fn from(older: MetadataResponseBrokers6) -> Self {
        MetadataResponseBrokers9 {
            node_id: older.node_id,
            host: older.host,
            port: older.port,
            rack: older.rack,
            ..MetadataResponseBrokers9::default()
        }
    }
}

impl From<MetadataResponseTopics6> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics6) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name,
            is_internal: older.is_internal,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponseTopics9::default()
        }
    }
}

impl From<MetadataResponseTopicsPartitions6> for MetadataResponseTopicsPartitions9 {
    fn from(older: MetadataResponseTopicsPartitions6) -> Self {
        MetadataResponseTopicsPartitions9 {
            error_code: older.error_code,
            partition_index: older.partition_index,
            leader_id: older.leader_id,
            replica_nodes: older.replica_nodes,
            isr_nodes: older.isr_nodes,
            offline_replicas: older.offline_replicas,
            ..MetadataResponseTopicsPartitions9::default()
        }
    }
}

impl From<MetadataResponse7> for MetadataResponse9 {
    fn from(older: MetadataResponse7) -> Self {
        MetadataResponse9 {
            throttle_time_ms: older.throttle_time_ms,
            brokers: older.brokers.into_iter().map(|el| el.into()).collect(),
            cluster_id: older.cluster_id,
            controller_id: older.controller_id,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponse9::default()
        }
    }
}

impl From<MetadataResponseBrokers7> for MetadataResponseBrokers9 {
    fn from(older: MetadataResponseBrokers7) -> Self {
        MetadataResponseBrokers9 {
            node_id: older.node_id,
            host: older.host,
            port: older.port,
            rack: older.rack,
            ..MetadataResponseBrokers9::default()
        }
    }
}

impl From<MetadataResponseTopics7> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics7) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name,
            is_internal: older.is_internal,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            ..MetadataResponseTopics9::default()
        }
    }
}

impl From<MetadataResponseTopicsPartitions7> for MetadataResponseTopicsPartitions9 {
    fn from(older: MetadataResponseTopicsPartitions7) -> Self {
        MetadataResponseTopicsPartitions9 {
            error_code: older.error_code,
            partition_index: older.partition_index,
            leader_id: older.leader_id,
            leader_epoch: older.leader_epoch,
            replica_nodes: older.replica_nodes,
            isr_nodes: older.isr_nodes,
            offline_replicas: older.offline_replicas,
            ..MetadataResponseTopicsPartitions9::default()
        }
    }
}

impl From<MetadataResponse8> for MetadataResponse9 {
    fn from(older: MetadataResponse8) -> Self {
        MetadataResponse9 {
            throttle_time_ms: older.throttle_time_ms,
            brokers: older.brokers.into_iter().map(|el| el.into()).collect(),
            cluster_id: older.cluster_id,
            controller_id: older.controller_id,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            cluster_authorized_operations: older.cluster_authorized_operations,
            ..MetadataResponse9::default()
        }
    }
}

impl From<MetadataResponseBrokers8> for MetadataResponseBrokers9 {
    fn from(older: MetadataResponseBrokers8) -> Self {
        MetadataResponseBrokers9 {
            node_id: older.node_id,
            host: older.host,
            port: older.port,
            rack: older.rack,
            ..MetadataResponseBrokers9::default()
        }
    }
}

impl From<MetadataResponseTopics8> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics8) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name,
            is_internal: older.is_internal,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            topic_authorized_operations: older.topic_authorized_operations,
            ..MetadataResponseTopics9::default()
        }
    }
}

impl From<MetadataResponseTopicsPartitions8> for MetadataResponseTopicsPartitions9 {
    fn from(older: MetadataResponseTopicsPartitions8) -> Self {
        MetadataResponseTopicsPartitions9 {
            error_code: older.error_code,
            partition_index: older.partition_index,
            leader_id: older.leader_id,
            leader_epoch: older.leader_epoch,
            replica_nodes: older.replica_nodes,
            isr_nodes: older.isr_nodes,
            offline_replicas: older.offline_replicas,
            ..MetadataResponseTopicsPartitions9::default()
        }
    }
}
