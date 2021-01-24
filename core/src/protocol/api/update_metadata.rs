use super::prelude::*;

pub type UpdateMetadataRequest = UpdateMetadataRequest6;
pub type UpdateMetadataResponse = UpdateMetadataResponse6;
pub fn serialize_update_metadata_request(
    data: UpdateMetadataRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&UpdateMetadataRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&UpdateMetadataRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&UpdateMetadataRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&UpdateMetadataRequest3::try_from(data)?, buf),
        4 => ToBytes::serialize(&UpdateMetadataRequest4::try_from(data)?, buf),
        5 => ToBytes::serialize(&UpdateMetadataRequest5::try_from(data)?, buf),
        7 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_update_metadata_response<T>(version: i32, buf: &mut T) -> UpdateMetadataResponse
where
    T: Iterator<Item = u8>,
{
    match version {
        0 => UpdateMetadataResponse0::deserialize(buf).into(),
        1 => UpdateMetadataResponse1::deserialize(buf).into(),
        2 => UpdateMetadataResponse2::deserialize(buf).into(),
        3 => UpdateMetadataResponse3::deserialize(buf).into(),
        4 => UpdateMetadataResponse4::deserialize(buf).into(),
        5 => UpdateMetadataResponse5::deserialize(buf).into(),
        7 => UpdateMetadataResponse::deserialize(buf),
        _ => UpdateMetadataResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequest0 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<UpdateMetadataRequestUngroupedPartitionStates0>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers0>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestUngroupedPartitionStates0 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers0 {
    pub id: Int32,
    pub v0_host: String,
    pub v0_port: Int32,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequest1 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<UpdateMetadataRequestUngroupedPartitionStates1>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers1>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestUngroupedPartitionStates1 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers1 {
    pub id: Int32,
    pub endpoints: Optional<Vec<UpdateMetadataRequestLiveBrokersEndpoints1>>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints1 {
    pub port: Int32,
    pub host: String,
    pub security_protocol: Int16,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequest2 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<UpdateMetadataRequestUngroupedPartitionStates2>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers2>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestUngroupedPartitionStates2 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers2 {
    pub id: Int32,
    pub endpoints: Optional<Vec<UpdateMetadataRequestLiveBrokersEndpoints2>>,
    pub rack: Optional<NullableString>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints2 {
    pub port: Int32,
    pub host: String,
    pub security_protocol: Int16,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequest3 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<UpdateMetadataRequestUngroupedPartitionStates3>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers3>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestUngroupedPartitionStates3 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers3 {
    pub id: Int32,
    pub endpoints: Optional<Vec<UpdateMetadataRequestLiveBrokersEndpoints3>>,
    pub rack: Optional<NullableString>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints3 {
    pub port: Int32,
    pub host: String,
    pub listener: Optional<String>,
    pub security_protocol: Int16,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequest4 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<UpdateMetadataRequestUngroupedPartitionStates4>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers4>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestUngroupedPartitionStates4 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
    pub offline_replicas: Optional<Vec<Int32>>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers4 {
    pub id: Int32,
    pub endpoints: Optional<Vec<UpdateMetadataRequestLiveBrokersEndpoints4>>,
    pub rack: Optional<NullableString>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints4 {
    pub port: Int32,
    pub host: String,
    pub listener: Optional<String>,
    pub security_protocol: Int16,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequest5 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Optional<Int64>,
    pub topic_states: Optional<Vec<UpdateMetadataRequestTopicStates5>>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers5>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestTopicStates5 {
    pub topic_name: String,
    pub partition_states: Vec<UpdateMetadataRequestTopicStatesPartitionStates5>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestTopicStatesPartitionStates5 {
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
    pub offline_replicas: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers5 {
    pub id: Int32,
    pub endpoints: Optional<Vec<UpdateMetadataRequestLiveBrokersEndpoints5>>,
    pub rack: Optional<NullableString>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints5 {
    pub port: Int32,
    pub host: String,
    pub listener: Optional<String>,
    pub security_protocol: Int16,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequest6 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Optional<Int64>,
    pub topic_states: Optional<Vec<UpdateMetadataRequestTopicStates6>>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers6>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestTopicStates6 {
    pub topic_name: CompactString,
    pub partition_states: Vec<UpdateMetadataRequestTopicStatesPartitionStates6>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestTopicStatesPartitionStates6 {
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
    pub offline_replicas: Vec<Int32>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers6 {
    pub id: Int32,
    pub endpoints: Optional<Vec<UpdateMetadataRequestLiveBrokersEndpoints6>>,
    pub rack: Optional<CompactNullableString>,
}

#[derive(Default, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints6 {
    pub port: Int32,
    pub host: CompactString,
    pub listener: Optional<CompactString>,
    pub security_protocol: Int16,
}

#[derive(Default, FromBytes)]
pub struct UpdateMetadataResponse0 {
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct UpdateMetadataResponse1 {
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct UpdateMetadataResponse2 {
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct UpdateMetadataResponse3 {
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct UpdateMetadataResponse4 {
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct UpdateMetadataResponse5 {
    pub error_code: Int16,
}

#[derive(Default, FromBytes)]
pub struct UpdateMetadataResponse6 {
    pub error_code: Int16,
}

impl TryFrom<UpdateMetadataRequest6> for UpdateMetadataRequest0 {
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequest6) -> Result<Self, Self::Error> {
        if latest.broker_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequest",
                0,
                "broker_epoch",
            ));
        }
        if latest.topic_states.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequest",
                0,
                "topic_states",
            ));
        }
        Ok(UpdateMetadataRequest0 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_brokers: latest
                .live_brokers
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
            ..UpdateMetadataRequest0::default()
        })
    }
}

impl TryFrom<UpdateMetadataRequestLiveBrokers6> for UpdateMetadataRequestLiveBrokers0 {
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequestLiveBrokers6) -> Result<Self, Self::Error> {
        if latest.endpoints.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequestLiveBrokers",
                0,
                "endpoints",
            ));
        }
        if latest.rack.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequestLiveBrokers",
                0,
                "rack",
            ));
        }
        Ok(UpdateMetadataRequestLiveBrokers0 {
            id: latest.id,
            ..UpdateMetadataRequestLiveBrokers0::default()
        })
    }
}

impl TryFrom<UpdateMetadataRequest6> for UpdateMetadataRequest1 {
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequest6) -> Result<Self, Self::Error> {
        if latest.broker_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequest",
                1,
                "broker_epoch",
            ));
        }
        if latest.topic_states.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequest",
                1,
                "topic_states",
            ));
        }
        Ok(UpdateMetadataRequest1 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_brokers: latest
                .live_brokers
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
            ..UpdateMetadataRequest1::default()
        })
    }
}

impl TryFrom<UpdateMetadataRequestLiveBrokers6> for UpdateMetadataRequestLiveBrokers1 {
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequestLiveBrokers6) -> Result<Self, Self::Error> {
        if latest.rack.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequestLiveBrokers",
                1,
                "rack",
            ));
        }
        Ok(UpdateMetadataRequestLiveBrokers1 {
            id: latest.id,
            endpoints: latest
                .endpoints
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
        })
    }
}

impl TryFrom<UpdateMetadataRequestLiveBrokersEndpoints6>
    for UpdateMetadataRequestLiveBrokersEndpoints1
{
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequestLiveBrokersEndpoints6) -> Result<Self, Self::Error> {
        if latest.listener.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequestLiveBrokersEndpoints",
                1,
                "listener",
            ));
        }
        Ok(UpdateMetadataRequestLiveBrokersEndpoints1 {
            port: latest.port,
            host: latest.host,
            security_protocol: latest.security_protocol,
        })
    }
}

impl TryFrom<UpdateMetadataRequest6> for UpdateMetadataRequest2 {
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequest6) -> Result<Self, Self::Error> {
        if latest.broker_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequest",
                2,
                "broker_epoch",
            ));
        }
        if latest.topic_states.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequest",
                2,
                "topic_states",
            ));
        }
        Ok(UpdateMetadataRequest2 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_brokers: latest
                .live_brokers
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
            ..UpdateMetadataRequest2::default()
        })
    }
}

impl TryFrom<UpdateMetadataRequestLiveBrokers6> for UpdateMetadataRequestLiveBrokers2 {
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequestLiveBrokers6) -> Result<Self, Self::Error> {
        Ok(UpdateMetadataRequestLiveBrokers2 {
            id: latest.id,
            endpoints: latest
                .endpoints
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
            rack: latest.rack.map(|val| val),
        })
    }
}

impl TryFrom<UpdateMetadataRequestLiveBrokersEndpoints6>
    for UpdateMetadataRequestLiveBrokersEndpoints2
{
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequestLiveBrokersEndpoints6) -> Result<Self, Self::Error> {
        if latest.listener.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequestLiveBrokersEndpoints",
                2,
                "listener",
            ));
        }
        Ok(UpdateMetadataRequestLiveBrokersEndpoints2 {
            port: latest.port,
            host: latest.host,
            security_protocol: latest.security_protocol,
        })
    }
}

impl TryFrom<UpdateMetadataRequest6> for UpdateMetadataRequest3 {
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequest6) -> Result<Self, Self::Error> {
        if latest.broker_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequest",
                3,
                "broker_epoch",
            ));
        }
        if latest.topic_states.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequest",
                3,
                "topic_states",
            ));
        }
        Ok(UpdateMetadataRequest3 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_brokers: latest
                .live_brokers
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
            ..UpdateMetadataRequest3::default()
        })
    }
}

impl TryFrom<UpdateMetadataRequestLiveBrokers6> for UpdateMetadataRequestLiveBrokers3 {
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequestLiveBrokers6) -> Result<Self, Self::Error> {
        Ok(UpdateMetadataRequestLiveBrokers3 {
            id: latest.id,
            endpoints: latest
                .endpoints
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
            rack: latest.rack.map(|val| val),
        })
    }
}

impl TryFrom<UpdateMetadataRequestLiveBrokersEndpoints6>
    for UpdateMetadataRequestLiveBrokersEndpoints3
{
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequestLiveBrokersEndpoints6) -> Result<Self, Self::Error> {
        Ok(UpdateMetadataRequestLiveBrokersEndpoints3 {
            port: latest.port,
            host: latest.host,
            listener: latest.listener.map(|val| val),
            security_protocol: latest.security_protocol,
        })
    }
}

impl TryFrom<UpdateMetadataRequest6> for UpdateMetadataRequest4 {
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequest6) -> Result<Self, Self::Error> {
        if latest.broker_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequest",
                4,
                "broker_epoch",
            ));
        }
        if latest.topic_states.is_some() {
            return Err(Error::OldKafkaVersion(
                "UpdateMetadataRequest",
                4,
                "topic_states",
            ));
        }
        Ok(UpdateMetadataRequest4 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_brokers: latest
                .live_brokers
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
            ..UpdateMetadataRequest4::default()
        })
    }
}

impl TryFrom<UpdateMetadataRequestLiveBrokers6> for UpdateMetadataRequestLiveBrokers4 {
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequestLiveBrokers6) -> Result<Self, Self::Error> {
        Ok(UpdateMetadataRequestLiveBrokers4 {
            id: latest.id,
            endpoints: latest
                .endpoints
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
            rack: latest.rack.map(|val| val),
        })
    }
}

impl TryFrom<UpdateMetadataRequestLiveBrokersEndpoints6>
    for UpdateMetadataRequestLiveBrokersEndpoints4
{
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequestLiveBrokersEndpoints6) -> Result<Self, Self::Error> {
        Ok(UpdateMetadataRequestLiveBrokersEndpoints4 {
            port: latest.port,
            host: latest.host,
            listener: latest.listener.map(|val| val),
            security_protocol: latest.security_protocol,
        })
    }
}

impl TryFrom<UpdateMetadataRequest6> for UpdateMetadataRequest5 {
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequest6) -> Result<Self, Self::Error> {
        Ok(UpdateMetadataRequest5 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            broker_epoch: latest.broker_epoch.map(|val| val),
            topic_states: latest
                .topic_states
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
            live_brokers: latest
                .live_brokers
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<UpdateMetadataRequestTopicStates6> for UpdateMetadataRequestTopicStates5 {
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequestTopicStates6) -> Result<Self, Self::Error> {
        Ok(UpdateMetadataRequestTopicStates5 {
            topic_name: latest.topic_name,
            partition_states: latest
                .partition_states
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<UpdateMetadataRequestTopicStatesPartitionStates6>
    for UpdateMetadataRequestTopicStatesPartitionStates5
{
    type Error = Error;
    fn try_from(
        latest: UpdateMetadataRequestTopicStatesPartitionStates6,
    ) -> Result<Self, Self::Error> {
        Ok(UpdateMetadataRequestTopicStatesPartitionStates5 {
            partition_index: latest.partition_index,
            controller_epoch: latest.controller_epoch,
            leader: latest.leader,
            leader_epoch: latest.leader_epoch,
            isr: latest.isr.into_iter().collect(),
            zk_version: latest.zk_version,
            replicas: latest.replicas.into_iter().collect(),
            offline_replicas: latest.offline_replicas.into_iter().collect(),
        })
    }
}

impl TryFrom<UpdateMetadataRequestLiveBrokers6> for UpdateMetadataRequestLiveBrokers5 {
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequestLiveBrokers6) -> Result<Self, Self::Error> {
        Ok(UpdateMetadataRequestLiveBrokers5 {
            id: latest.id,
            endpoints: latest
                .endpoints
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
            rack: latest.rack.map(|val| val),
        })
    }
}

impl TryFrom<UpdateMetadataRequestLiveBrokersEndpoints6>
    for UpdateMetadataRequestLiveBrokersEndpoints5
{
    type Error = Error;
    fn try_from(latest: UpdateMetadataRequestLiveBrokersEndpoints6) -> Result<Self, Self::Error> {
        Ok(UpdateMetadataRequestLiveBrokersEndpoints5 {
            port: latest.port,
            host: latest.host,
            listener: latest.listener.map(|val| val),
            security_protocol: latest.security_protocol,
        })
    }
}

impl From<UpdateMetadataResponse0> for UpdateMetadataResponse6 {
    fn from(older: UpdateMetadataResponse0) -> Self {
        UpdateMetadataResponse6 {
            error_code: older.error_code,
        }
    }
}

impl From<UpdateMetadataResponse1> for UpdateMetadataResponse6 {
    fn from(older: UpdateMetadataResponse1) -> Self {
        UpdateMetadataResponse6 {
            error_code: older.error_code,
        }
    }
}

impl From<UpdateMetadataResponse2> for UpdateMetadataResponse6 {
    fn from(older: UpdateMetadataResponse2) -> Self {
        UpdateMetadataResponse6 {
            error_code: older.error_code,
        }
    }
}

impl From<UpdateMetadataResponse3> for UpdateMetadataResponse6 {
    fn from(older: UpdateMetadataResponse3) -> Self {
        UpdateMetadataResponse6 {
            error_code: older.error_code,
        }
    }
}

impl From<UpdateMetadataResponse4> for UpdateMetadataResponse6 {
    fn from(older: UpdateMetadataResponse4) -> Self {
        UpdateMetadataResponse6 {
            error_code: older.error_code,
        }
    }
}

impl From<UpdateMetadataResponse5> for UpdateMetadataResponse6 {
    fn from(older: UpdateMetadataResponse5) -> Self {
        UpdateMetadataResponse6 {
            error_code: older.error_code,
        }
    }
}
