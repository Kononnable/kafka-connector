use super::prelude::*;

pub type MetadataRequest = MetadataRequest9;
pub type MetadataResponse = MetadataResponse9;
pub fn serialize_metadata_request(
    data: MetadataRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&MetadataRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&MetadataRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&MetadataRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&MetadataRequest3::try_from(data)?, buf),
        4 => ToBytes::serialize(&MetadataRequest4::try_from(data)?, buf),
        5 => ToBytes::serialize(&MetadataRequest5::try_from(data)?, buf),
        6 => ToBytes::serialize(&MetadataRequest6::try_from(data)?, buf),
        7 => ToBytes::serialize(&MetadataRequest7::try_from(data)?, buf),
        8 => ToBytes::serialize(&MetadataRequest8::try_from(data)?, buf),
        9 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_metadata_response(version: i32, buf: &mut Bytes) -> MetadataResponse {
    match version {
        0 => MetadataResponse0::deserialize(buf).into(),
        1 => MetadataResponse1::deserialize(buf).into(),
        2 => MetadataResponse2::deserialize(buf).into(),
        3 => MetadataResponse3::deserialize(buf).into(),
        4 => MetadataResponse4::deserialize(buf).into(),
        5 => MetadataResponse5::deserialize(buf).into(),
        6 => MetadataResponse6::deserialize(buf).into(),
        7 => MetadataResponse7::deserialize(buf).into(),
        8 => MetadataResponse8::deserialize(buf).into(),
        9 => MetadataResponse::deserialize(buf),
        _ => MetadataResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct MetadataRequest0 {
    pub topics: Vec<MetadataRequestTopics0>,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequestTopics0 {
    pub name: String,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequest1 {
    pub topics: Vec<MetadataRequestTopics1>,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequestTopics1 {
    pub name: String,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequest2 {
    pub topics: Vec<MetadataRequestTopics2>,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequestTopics2 {
    pub name: String,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequest3 {
    pub topics: Vec<MetadataRequestTopics3>,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequestTopics3 {
    pub name: String,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequest4 {
    pub topics: Vec<MetadataRequestTopics4>,
    pub allow_auto_topic_creation: Optional<Boolean>,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequestTopics4 {
    pub name: String,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequest5 {
    pub topics: Vec<MetadataRequestTopics5>,
    pub allow_auto_topic_creation: Optional<Boolean>,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequestTopics5 {
    pub name: String,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequest6 {
    pub topics: Vec<MetadataRequestTopics6>,
    pub allow_auto_topic_creation: Optional<Boolean>,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequestTopics6 {
    pub name: String,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequest7 {
    pub topics: Vec<MetadataRequestTopics7>,
    pub allow_auto_topic_creation: Optional<Boolean>,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequestTopics7 {
    pub name: String,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequest8 {
    pub topics: Vec<MetadataRequestTopics8>,
    pub allow_auto_topic_creation: Optional<Boolean>,
    pub include_cluster_authorized_operations: Optional<Boolean>,
    pub include_topic_authorized_operations: Optional<Boolean>,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequestTopics8 {
    pub name: String,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequest9 {
    pub topics: Vec<MetadataRequestTopics9>,
    pub allow_auto_topic_creation: Optional<Boolean>,
    pub include_cluster_authorized_operations: Optional<Boolean>,
    pub include_topic_authorized_operations: Optional<Boolean>,
}

#[derive(Default, ToBytes)]
pub struct MetadataRequestTopics9 {
    pub name: CompactString,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponse0 {
    pub brokers: Vec<MetadataResponseBrokers0>,
    pub topics: Vec<MetadataResponseTopics0>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseBrokers0 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopics0 {
    pub error_code: Int16,
    pub name: String,
    pub partitions: Vec<MetadataResponseTopicsPartitions0>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopicsPartitions0 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponse1 {
    pub brokers: Vec<MetadataResponseBrokers1>,
    pub controller_id: Optional<Int32>,
    pub topics: Vec<MetadataResponseTopics1>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseBrokers1 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Optional<NullableString>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopics1 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Optional<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions1>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopicsPartitions1 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponse2 {
    pub brokers: Vec<MetadataResponseBrokers2>,
    pub cluster_id: Optional<NullableString>,
    pub controller_id: Optional<Int32>,
    pub topics: Vec<MetadataResponseTopics2>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseBrokers2 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Optional<NullableString>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopics2 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Optional<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions2>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopicsPartitions2 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponse3 {
    pub throttle_time_ms: Optional<Int32>,
    pub brokers: Vec<MetadataResponseBrokers3>,
    pub cluster_id: Optional<NullableString>,
    pub controller_id: Optional<Int32>,
    pub topics: Vec<MetadataResponseTopics3>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseBrokers3 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Optional<NullableString>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopics3 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Optional<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions3>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopicsPartitions3 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponse4 {
    pub throttle_time_ms: Optional<Int32>,
    pub brokers: Vec<MetadataResponseBrokers4>,
    pub cluster_id: Optional<NullableString>,
    pub controller_id: Optional<Int32>,
    pub topics: Vec<MetadataResponseTopics4>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseBrokers4 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Optional<NullableString>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopics4 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Optional<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions4>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopicsPartitions4 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponse5 {
    pub throttle_time_ms: Optional<Int32>,
    pub brokers: Vec<MetadataResponseBrokers5>,
    pub cluster_id: Optional<NullableString>,
    pub controller_id: Optional<Int32>,
    pub topics: Vec<MetadataResponseTopics5>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseBrokers5 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Optional<NullableString>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopics5 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Optional<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions5>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopicsPartitions5 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
    pub offline_replicas: Optional<Vec<Int32>>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponse6 {
    pub throttle_time_ms: Optional<Int32>,
    pub brokers: Vec<MetadataResponseBrokers6>,
    pub cluster_id: Optional<NullableString>,
    pub controller_id: Optional<Int32>,
    pub topics: Vec<MetadataResponseTopics6>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseBrokers6 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Optional<NullableString>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopics6 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Optional<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions6>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopicsPartitions6 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
    pub offline_replicas: Optional<Vec<Int32>>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponse7 {
    pub throttle_time_ms: Optional<Int32>,
    pub brokers: Vec<MetadataResponseBrokers7>,
    pub cluster_id: Optional<NullableString>,
    pub controller_id: Optional<Int32>,
    pub topics: Vec<MetadataResponseTopics7>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseBrokers7 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Optional<NullableString>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopics7 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Optional<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions7>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopicsPartitions7 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub leader_epoch: Optional<Int32>,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
    pub offline_replicas: Optional<Vec<Int32>>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponse8 {
    pub throttle_time_ms: Optional<Int32>,
    pub brokers: Vec<MetadataResponseBrokers8>,
    pub cluster_id: Optional<NullableString>,
    pub controller_id: Optional<Int32>,
    pub topics: Vec<MetadataResponseTopics8>,
    pub cluster_authorized_operations: Optional<Int32>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseBrokers8 {
    pub node_id: Int32,
    pub host: String,
    pub port: Int32,
    pub rack: Optional<NullableString>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopics8 {
    pub error_code: Int16,
    pub name: String,
    pub is_internal: Optional<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions8>,
    pub topic_authorized_operations: Optional<Int32>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopicsPartitions8 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub leader_epoch: Optional<Int32>,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
    pub offline_replicas: Optional<Vec<Int32>>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponse9 {
    pub throttle_time_ms: Optional<Int32>,
    pub brokers: Vec<MetadataResponseBrokers9>,
    pub cluster_id: Optional<CompactNullableString>,
    pub controller_id: Optional<Int32>,
    pub topics: Vec<MetadataResponseTopics9>,
    pub cluster_authorized_operations: Optional<Int32>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseBrokers9 {
    pub node_id: Int32,
    pub host: CompactString,
    pub port: Int32,
    pub rack: Optional<CompactNullableString>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopics9 {
    pub error_code: Int16,
    pub name: CompactString,
    pub is_internal: Optional<Boolean>,
    pub partitions: Vec<MetadataResponseTopicsPartitions9>,
    pub topic_authorized_operations: Optional<Int32>,
}

#[derive(Default, FromBytes)]
pub struct MetadataResponseTopicsPartitions9 {
    pub error_code: Int16,
    pub partition_index: Int32,
    pub leader_id: Int32,
    pub leader_epoch: Optional<Int32>,
    pub replica_nodes: Vec<Int32>,
    pub isr_nodes: Vec<Int32>,
    pub offline_replicas: Optional<Vec<Int32>>,
}

impl TryFrom<MetadataRequest9> for MetadataRequest0 {
    type Error = Error;
    fn try_from(latest: MetadataRequest9) -> Result<Self, Self::Error> {
        if latest.allow_auto_topic_creation.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                0,
                "allow_auto_topic_creation",
            ));
        }
        if latest.include_cluster_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                0,
                "include_cluster_authorized_operations",
            ));
        }
        if latest.include_topic_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                0,
                "include_topic_authorized_operations",
            ));
        }
        Ok(MetadataRequest0 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<MetadataRequestTopics9> for MetadataRequestTopics0 {
    type Error = Error;
    fn try_from(latest: MetadataRequestTopics9) -> Result<Self, Self::Error> {
        Ok(MetadataRequestTopics0 {
            name: latest.name.into(),
        })
    }
}

impl TryFrom<MetadataRequest9> for MetadataRequest1 {
    type Error = Error;
    fn try_from(latest: MetadataRequest9) -> Result<Self, Self::Error> {
        if latest.allow_auto_topic_creation.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                1,
                "allow_auto_topic_creation",
            ));
        }
        if latest.include_cluster_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                1,
                "include_cluster_authorized_operations",
            ));
        }
        if latest.include_topic_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                1,
                "include_topic_authorized_operations",
            ));
        }
        Ok(MetadataRequest1 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<MetadataRequestTopics9> for MetadataRequestTopics1 {
    type Error = Error;
    fn try_from(latest: MetadataRequestTopics9) -> Result<Self, Self::Error> {
        Ok(MetadataRequestTopics1 {
            name: latest.name.into(),
        })
    }
}

impl TryFrom<MetadataRequest9> for MetadataRequest2 {
    type Error = Error;
    fn try_from(latest: MetadataRequest9) -> Result<Self, Self::Error> {
        if latest.allow_auto_topic_creation.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                2,
                "allow_auto_topic_creation",
            ));
        }
        if latest.include_cluster_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                2,
                "include_cluster_authorized_operations",
            ));
        }
        if latest.include_topic_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                2,
                "include_topic_authorized_operations",
            ));
        }
        Ok(MetadataRequest2 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<MetadataRequestTopics9> for MetadataRequestTopics2 {
    type Error = Error;
    fn try_from(latest: MetadataRequestTopics9) -> Result<Self, Self::Error> {
        Ok(MetadataRequestTopics2 {
            name: latest.name.into(),
        })
    }
}

impl TryFrom<MetadataRequest9> for MetadataRequest3 {
    type Error = Error;
    fn try_from(latest: MetadataRequest9) -> Result<Self, Self::Error> {
        if latest.allow_auto_topic_creation.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                3,
                "allow_auto_topic_creation",
            ));
        }
        if latest.include_cluster_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                3,
                "include_cluster_authorized_operations",
            ));
        }
        if latest.include_topic_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                3,
                "include_topic_authorized_operations",
            ));
        }
        Ok(MetadataRequest3 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<MetadataRequestTopics9> for MetadataRequestTopics3 {
    type Error = Error;
    fn try_from(latest: MetadataRequestTopics9) -> Result<Self, Self::Error> {
        Ok(MetadataRequestTopics3 {
            name: latest.name.into(),
        })
    }
}

impl TryFrom<MetadataRequest9> for MetadataRequest4 {
    type Error = Error;
    fn try_from(latest: MetadataRequest9) -> Result<Self, Self::Error> {
        if latest.include_cluster_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                4,
                "include_cluster_authorized_operations",
            ));
        }
        if latest.include_topic_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                4,
                "include_topic_authorized_operations",
            ));
        }
        Ok(MetadataRequest4 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            allow_auto_topic_creation: latest.allow_auto_topic_creation,
        })
    }
}

impl TryFrom<MetadataRequestTopics9> for MetadataRequestTopics4 {
    type Error = Error;
    fn try_from(latest: MetadataRequestTopics9) -> Result<Self, Self::Error> {
        Ok(MetadataRequestTopics4 {
            name: latest.name.into(),
        })
    }
}

impl TryFrom<MetadataRequest9> for MetadataRequest5 {
    type Error = Error;
    fn try_from(latest: MetadataRequest9) -> Result<Self, Self::Error> {
        if latest.include_cluster_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                5,
                "include_cluster_authorized_operations",
            ));
        }
        if latest.include_topic_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                5,
                "include_topic_authorized_operations",
            ));
        }
        Ok(MetadataRequest5 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            allow_auto_topic_creation: latest.allow_auto_topic_creation,
        })
    }
}

impl TryFrom<MetadataRequestTopics9> for MetadataRequestTopics5 {
    type Error = Error;
    fn try_from(latest: MetadataRequestTopics9) -> Result<Self, Self::Error> {
        Ok(MetadataRequestTopics5 {
            name: latest.name.into(),
        })
    }
}

impl TryFrom<MetadataRequest9> for MetadataRequest6 {
    type Error = Error;
    fn try_from(latest: MetadataRequest9) -> Result<Self, Self::Error> {
        if latest.include_cluster_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                6,
                "include_cluster_authorized_operations",
            ));
        }
        if latest.include_topic_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                6,
                "include_topic_authorized_operations",
            ));
        }
        Ok(MetadataRequest6 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            allow_auto_topic_creation: latest.allow_auto_topic_creation,
        })
    }
}

impl TryFrom<MetadataRequestTopics9> for MetadataRequestTopics6 {
    type Error = Error;
    fn try_from(latest: MetadataRequestTopics9) -> Result<Self, Self::Error> {
        Ok(MetadataRequestTopics6 {
            name: latest.name.into(),
        })
    }
}

impl TryFrom<MetadataRequest9> for MetadataRequest7 {
    type Error = Error;
    fn try_from(latest: MetadataRequest9) -> Result<Self, Self::Error> {
        if latest.include_cluster_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                7,
                "include_cluster_authorized_operations",
            ));
        }
        if latest.include_topic_authorized_operations.is_some() {
            return Err(Error::OldKafkaVersion(
                "MetadataRequest",
                7,
                "include_topic_authorized_operations",
            ));
        }
        Ok(MetadataRequest7 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            allow_auto_topic_creation: latest.allow_auto_topic_creation,
        })
    }
}

impl TryFrom<MetadataRequestTopics9> for MetadataRequestTopics7 {
    type Error = Error;
    fn try_from(latest: MetadataRequestTopics9) -> Result<Self, Self::Error> {
        Ok(MetadataRequestTopics7 {
            name: latest.name.into(),
        })
    }
}

impl TryFrom<MetadataRequest9> for MetadataRequest8 {
    type Error = Error;
    fn try_from(latest: MetadataRequest9) -> Result<Self, Self::Error> {
        Ok(MetadataRequest8 {
            topics: latest
                .topics
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            allow_auto_topic_creation: latest.allow_auto_topic_creation,
            include_cluster_authorized_operations: latest.include_cluster_authorized_operations,
            include_topic_authorized_operations: latest.include_topic_authorized_operations,
        })
    }
}

impl TryFrom<MetadataRequestTopics9> for MetadataRequestTopics8 {
    type Error = Error;
    fn try_from(latest: MetadataRequestTopics9) -> Result<Self, Self::Error> {
        Ok(MetadataRequestTopics8 {
            name: latest.name.into(),
        })
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
            host: older.host.into(),
            port: older.port,
            ..MetadataResponseBrokers9::default()
        }
    }
}

impl From<MetadataResponseTopics0> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics0) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name.into(),
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
            host: older.host.into(),
            port: older.port,
            rack: older.rack.map(|val| val.into()),
        }
    }
}

impl From<MetadataResponseTopics1> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics1) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name.into(),
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
            cluster_id: older.cluster_id.map(|val| val.into()),
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
            host: older.host.into(),
            port: older.port,
            rack: older.rack.map(|val| val.into()),
        }
    }
}

impl From<MetadataResponseTopics2> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics2) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name.into(),
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
            cluster_id: older.cluster_id.map(|val| val.into()),
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
            host: older.host.into(),
            port: older.port,
            rack: older.rack.map(|val| val.into()),
        }
    }
}

impl From<MetadataResponseTopics3> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics3) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name.into(),
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
            cluster_id: older.cluster_id.map(|val| val.into()),
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
            host: older.host.into(),
            port: older.port,
            rack: older.rack.map(|val| val.into()),
        }
    }
}

impl From<MetadataResponseTopics4> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics4) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name.into(),
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
            cluster_id: older.cluster_id.map(|val| val.into()),
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
            host: older.host.into(),
            port: older.port,
            rack: older.rack.map(|val| val.into()),
        }
    }
}

impl From<MetadataResponseTopics5> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics5) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name.into(),
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
            cluster_id: older.cluster_id.map(|val| val.into()),
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
            host: older.host.into(),
            port: older.port,
            rack: older.rack.map(|val| val.into()),
        }
    }
}

impl From<MetadataResponseTopics6> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics6) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name.into(),
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
            cluster_id: older.cluster_id.map(|val| val.into()),
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
            host: older.host.into(),
            port: older.port,
            rack: older.rack.map(|val| val.into()),
        }
    }
}

impl From<MetadataResponseTopics7> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics7) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name.into(),
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
        }
    }
}

impl From<MetadataResponse8> for MetadataResponse9 {
    fn from(older: MetadataResponse8) -> Self {
        MetadataResponse9 {
            throttle_time_ms: older.throttle_time_ms,
            brokers: older.brokers.into_iter().map(|el| el.into()).collect(),
            cluster_id: older.cluster_id.map(|val| val.into()),
            controller_id: older.controller_id,
            topics: older.topics.into_iter().map(|el| el.into()).collect(),
            cluster_authorized_operations: older.cluster_authorized_operations,
        }
    }
}

impl From<MetadataResponseBrokers8> for MetadataResponseBrokers9 {
    fn from(older: MetadataResponseBrokers8) -> Self {
        MetadataResponseBrokers9 {
            node_id: older.node_id,
            host: older.host.into(),
            port: older.port,
            rack: older.rack.map(|val| val.into()),
        }
    }
}

impl From<MetadataResponseTopics8> for MetadataResponseTopics9 {
    fn from(older: MetadataResponseTopics8) -> Self {
        MetadataResponseTopics9 {
            error_code: older.error_code,
            name: older.name.into(),
            is_internal: older.is_internal,
            partitions: older.partitions.into_iter().map(|el| el.into()).collect(),
            topic_authorized_operations: older.topic_authorized_operations,
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
        }
    }
}
