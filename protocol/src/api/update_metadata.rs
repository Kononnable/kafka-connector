use super::prelude::*;

pub type UpdateMetadataRequest = UpdateMetadataRequest6;
pub type UpdateMetadataResponse = UpdateMetadataResponse6;
impl ApiCall for UpdateMetadataRequest {
    type Response = UpdateMetadataResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        6
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::UpdateMetadata
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => false,
            5 => false,
            6 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                UpdateMetadataRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                UpdateMetadataRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &UpdateMetadataRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &UpdateMetadataRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &UpdateMetadataRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &UpdateMetadataRequest3::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(
                &UpdateMetadataRequest4::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            5 => ToBytes::serialize(
                &UpdateMetadataRequest5::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            6 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, UpdateMetadataResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response =
            match version {
                0 => UpdateMetadataResponse0::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                1 => UpdateMetadataResponse1::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                2 => UpdateMetadataResponse2::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                3 => UpdateMetadataResponse3::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                4 => UpdateMetadataResponse4::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                5 => UpdateMetadataResponse5::deserialize(buf, Self::is_flexible_version(version))
                    .into(),
                6 => UpdateMetadataResponse::deserialize(buf, Self::is_flexible_version(version)),
                _ => UpdateMetadataResponse::deserialize(buf, Self::is_flexible_version(version)),
            };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequest0 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<UpdateMetadataRequestUngroupedPartitionStates0>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
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

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers0 {
    pub id: Int32,
    pub v0_host: String,
    pub v0_port: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequest1 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<UpdateMetadataRequestUngroupedPartitionStates1>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
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

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers1 {
    pub id: Int32,
    pub endpoints: Vec<UpdateMetadataRequestLiveBrokersEndpoints1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints1 {
    pub port: Int32,
    pub host: String,
    pub security_protocol: Int16,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequest2 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<UpdateMetadataRequestUngroupedPartitionStates2>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
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

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers2 {
    pub id: Int32,
    pub endpoints: Vec<UpdateMetadataRequestLiveBrokersEndpoints2>,
    pub rack: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints2 {
    pub port: Int32,
    pub host: String,
    pub security_protocol: Int16,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequest3 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<UpdateMetadataRequestUngroupedPartitionStates3>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
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

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers3 {
    pub id: Int32,
    pub endpoints: Vec<UpdateMetadataRequestLiveBrokersEndpoints3>,
    pub rack: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints3 {
    pub port: Int32,
    pub host: String,
    pub listener: String,
    pub security_protocol: Int16,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequest4 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<UpdateMetadataRequestUngroupedPartitionStates4>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers4>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestUngroupedPartitionStates4 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
    pub offline_replicas: Vec<Int32>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers4 {
    pub id: Int32,
    pub endpoints: Vec<UpdateMetadataRequestLiveBrokersEndpoints4>,
    pub rack: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints4 {
    pub port: Int32,
    pub host: String,
    pub listener: String,
    pub security_protocol: Int16,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequest5 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Int64,
    pub topic_states: Vec<UpdateMetadataRequestTopicStates5>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers5>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestTopicStates5 {
    pub topic_name: String,
    pub partition_states: Vec<UpdateMetadataRequestTopicStatesPartitionStates5>,
}

#[derive(Default, Debug, Clone, ToBytes)]
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

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers5 {
    pub id: Int32,
    pub endpoints: Vec<UpdateMetadataRequestLiveBrokersEndpoints5>,
    pub rack: NullableString,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints5 {
    pub port: Int32,
    pub host: String,
    pub listener: String,
    pub security_protocol: Int16,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequest6 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Int64,
    pub topic_states: Vec<UpdateMetadataRequestTopicStates6>,
    pub live_brokers: Vec<UpdateMetadataRequestLiveBrokers6>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestTopicStates6 {
    pub topic_name: String,
    pub partition_states: Vec<UpdateMetadataRequestTopicStatesPartitionStates6>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestTopicStatesPartitionStates6 {
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
    pub offline_replicas: Vec<Int32>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokers6 {
    pub id: Int32,
    pub endpoints: Vec<UpdateMetadataRequestLiveBrokersEndpoints6>,
    pub rack: NullableString,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateMetadataRequestLiveBrokersEndpoints6 {
    pub port: Int32,
    pub host: String,
    pub listener: String,
    pub security_protocol: Int16,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct UpdateMetadataResponse0 {
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct UpdateMetadataResponse1 {
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct UpdateMetadataResponse2 {
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct UpdateMetadataResponse3 {
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct UpdateMetadataResponse4 {
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct UpdateMetadataResponse5 {
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct UpdateMetadataResponse6 {
    pub error_code: Int16,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<UpdateMetadataRequest6> for UpdateMetadataRequest0 {
    fn from(latest: UpdateMetadataRequest6) -> UpdateMetadataRequest0 {
        log::debug!("Using old api format - UpdateMetadataRequest0, ignoring field broker_epoch");
        log::debug!("Using old api format - UpdateMetadataRequest0, ignoring field topic_states");
        UpdateMetadataRequest0 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_brokers: latest
                .live_brokers
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            ..UpdateMetadataRequest0::default()
        }
    }
}

impl From<UpdateMetadataRequestLiveBrokers6> for UpdateMetadataRequestLiveBrokers0 {
    fn from(latest: UpdateMetadataRequestLiveBrokers6) -> UpdateMetadataRequestLiveBrokers0 {
        log::debug!(
            "Using old api format - UpdateMetadataRequestLiveBrokers0, ignoring field endpoints"
        );
        UpdateMetadataRequestLiveBrokers0 {
            id: latest.id,
            ..UpdateMetadataRequestLiveBrokers0::default()
        }
    }
}

impl From<UpdateMetadataRequest6> for UpdateMetadataRequest1 {
    fn from(latest: UpdateMetadataRequest6) -> UpdateMetadataRequest1 {
        log::debug!("Using old api format - UpdateMetadataRequest1, ignoring field broker_epoch");
        log::debug!("Using old api format - UpdateMetadataRequest1, ignoring field topic_states");
        UpdateMetadataRequest1 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_brokers: latest
                .live_brokers
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            ..UpdateMetadataRequest1::default()
        }
    }
}

impl From<UpdateMetadataRequestLiveBrokers6> for UpdateMetadataRequestLiveBrokers1 {
    fn from(latest: UpdateMetadataRequestLiveBrokers6) -> UpdateMetadataRequestLiveBrokers1 {
        UpdateMetadataRequestLiveBrokers1 {
            id: latest.id,
            endpoints: latest.endpoints.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<UpdateMetadataRequestLiveBrokersEndpoints6>
    for UpdateMetadataRequestLiveBrokersEndpoints1
{
    fn from(
        latest: UpdateMetadataRequestLiveBrokersEndpoints6,
    ) -> UpdateMetadataRequestLiveBrokersEndpoints1 {
        UpdateMetadataRequestLiveBrokersEndpoints1 {
            port: latest.port,
            host: latest.host,
            security_protocol: latest.security_protocol,
        }
    }
}

impl From<UpdateMetadataRequest6> for UpdateMetadataRequest2 {
    fn from(latest: UpdateMetadataRequest6) -> UpdateMetadataRequest2 {
        log::debug!("Using old api format - UpdateMetadataRequest2, ignoring field broker_epoch");
        log::debug!("Using old api format - UpdateMetadataRequest2, ignoring field topic_states");
        UpdateMetadataRequest2 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_brokers: latest
                .live_brokers
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            ..UpdateMetadataRequest2::default()
        }
    }
}

impl From<UpdateMetadataRequestLiveBrokers6> for UpdateMetadataRequestLiveBrokers2 {
    fn from(latest: UpdateMetadataRequestLiveBrokers6) -> UpdateMetadataRequestLiveBrokers2 {
        UpdateMetadataRequestLiveBrokers2 {
            id: latest.id,
            endpoints: latest.endpoints.into_iter().map(|ele| ele.into()).collect(),
            rack: latest.rack,
        }
    }
}

impl From<UpdateMetadataRequestLiveBrokersEndpoints6>
    for UpdateMetadataRequestLiveBrokersEndpoints2
{
    fn from(
        latest: UpdateMetadataRequestLiveBrokersEndpoints6,
    ) -> UpdateMetadataRequestLiveBrokersEndpoints2 {
        UpdateMetadataRequestLiveBrokersEndpoints2 {
            port: latest.port,
            host: latest.host,
            security_protocol: latest.security_protocol,
        }
    }
}

impl From<UpdateMetadataRequest6> for UpdateMetadataRequest3 {
    fn from(latest: UpdateMetadataRequest6) -> UpdateMetadataRequest3 {
        log::debug!("Using old api format - UpdateMetadataRequest3, ignoring field broker_epoch");
        log::debug!("Using old api format - UpdateMetadataRequest3, ignoring field topic_states");
        UpdateMetadataRequest3 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_brokers: latest
                .live_brokers
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            ..UpdateMetadataRequest3::default()
        }
    }
}

impl From<UpdateMetadataRequestLiveBrokers6> for UpdateMetadataRequestLiveBrokers3 {
    fn from(latest: UpdateMetadataRequestLiveBrokers6) -> UpdateMetadataRequestLiveBrokers3 {
        UpdateMetadataRequestLiveBrokers3 {
            id: latest.id,
            endpoints: latest.endpoints.into_iter().map(|ele| ele.into()).collect(),
            rack: latest.rack,
        }
    }
}

impl From<UpdateMetadataRequestLiveBrokersEndpoints6>
    for UpdateMetadataRequestLiveBrokersEndpoints3
{
    fn from(
        latest: UpdateMetadataRequestLiveBrokersEndpoints6,
    ) -> UpdateMetadataRequestLiveBrokersEndpoints3 {
        UpdateMetadataRequestLiveBrokersEndpoints3 {
            port: latest.port,
            host: latest.host,
            listener: latest.listener,
            security_protocol: latest.security_protocol,
        }
    }
}

impl From<UpdateMetadataRequest6> for UpdateMetadataRequest4 {
    fn from(latest: UpdateMetadataRequest6) -> UpdateMetadataRequest4 {
        log::debug!("Using old api format - UpdateMetadataRequest4, ignoring field broker_epoch");
        log::debug!("Using old api format - UpdateMetadataRequest4, ignoring field topic_states");
        UpdateMetadataRequest4 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_brokers: latest
                .live_brokers
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            ..UpdateMetadataRequest4::default()
        }
    }
}

impl From<UpdateMetadataRequestLiveBrokers6> for UpdateMetadataRequestLiveBrokers4 {
    fn from(latest: UpdateMetadataRequestLiveBrokers6) -> UpdateMetadataRequestLiveBrokers4 {
        UpdateMetadataRequestLiveBrokers4 {
            id: latest.id,
            endpoints: latest.endpoints.into_iter().map(|ele| ele.into()).collect(),
            rack: latest.rack,
        }
    }
}

impl From<UpdateMetadataRequestLiveBrokersEndpoints6>
    for UpdateMetadataRequestLiveBrokersEndpoints4
{
    fn from(
        latest: UpdateMetadataRequestLiveBrokersEndpoints6,
    ) -> UpdateMetadataRequestLiveBrokersEndpoints4 {
        UpdateMetadataRequestLiveBrokersEndpoints4 {
            port: latest.port,
            host: latest.host,
            listener: latest.listener,
            security_protocol: latest.security_protocol,
        }
    }
}

impl From<UpdateMetadataRequest6> for UpdateMetadataRequest5 {
    fn from(latest: UpdateMetadataRequest6) -> UpdateMetadataRequest5 {
        UpdateMetadataRequest5 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            broker_epoch: latest.broker_epoch,
            topic_states: latest
                .topic_states
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            live_brokers: latest
                .live_brokers
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<UpdateMetadataRequestTopicStates6> for UpdateMetadataRequestTopicStates5 {
    fn from(latest: UpdateMetadataRequestTopicStates6) -> UpdateMetadataRequestTopicStates5 {
        UpdateMetadataRequestTopicStates5 {
            topic_name: latest.topic_name,
            partition_states: latest
                .partition_states
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<UpdateMetadataRequestTopicStatesPartitionStates6>
    for UpdateMetadataRequestTopicStatesPartitionStates5
{
    fn from(
        latest: UpdateMetadataRequestTopicStatesPartitionStates6,
    ) -> UpdateMetadataRequestTopicStatesPartitionStates5 {
        UpdateMetadataRequestTopicStatesPartitionStates5 {
            partition_index: latest.partition_index,
            controller_epoch: latest.controller_epoch,
            leader: latest.leader,
            leader_epoch: latest.leader_epoch,
            isr: latest.isr,
            zk_version: latest.zk_version,
            replicas: latest.replicas,
            offline_replicas: latest.offline_replicas,
        }
    }
}

impl From<UpdateMetadataRequestLiveBrokers6> for UpdateMetadataRequestLiveBrokers5 {
    fn from(latest: UpdateMetadataRequestLiveBrokers6) -> UpdateMetadataRequestLiveBrokers5 {
        UpdateMetadataRequestLiveBrokers5 {
            id: latest.id,
            endpoints: latest.endpoints.into_iter().map(|ele| ele.into()).collect(),
            rack: latest.rack,
        }
    }
}

impl From<UpdateMetadataRequestLiveBrokersEndpoints6>
    for UpdateMetadataRequestLiveBrokersEndpoints5
{
    fn from(
        latest: UpdateMetadataRequestLiveBrokersEndpoints6,
    ) -> UpdateMetadataRequestLiveBrokersEndpoints5 {
        UpdateMetadataRequestLiveBrokersEndpoints5 {
            port: latest.port,
            host: latest.host,
            listener: latest.listener,
            security_protocol: latest.security_protocol,
        }
    }
}

impl From<UpdateMetadataResponse0> for UpdateMetadataResponse6 {
    fn from(older: UpdateMetadataResponse0) -> Self {
        UpdateMetadataResponse6 {
            error_code: older.error_code,
            ..UpdateMetadataResponse6::default()
        }
    }
}

impl From<UpdateMetadataResponse1> for UpdateMetadataResponse6 {
    fn from(older: UpdateMetadataResponse1) -> Self {
        UpdateMetadataResponse6 {
            error_code: older.error_code,
            ..UpdateMetadataResponse6::default()
        }
    }
}

impl From<UpdateMetadataResponse2> for UpdateMetadataResponse6 {
    fn from(older: UpdateMetadataResponse2) -> Self {
        UpdateMetadataResponse6 {
            error_code: older.error_code,
            ..UpdateMetadataResponse6::default()
        }
    }
}

impl From<UpdateMetadataResponse3> for UpdateMetadataResponse6 {
    fn from(older: UpdateMetadataResponse3) -> Self {
        UpdateMetadataResponse6 {
            error_code: older.error_code,
            ..UpdateMetadataResponse6::default()
        }
    }
}

impl From<UpdateMetadataResponse4> for UpdateMetadataResponse6 {
    fn from(older: UpdateMetadataResponse4) -> Self {
        UpdateMetadataResponse6 {
            error_code: older.error_code,
            ..UpdateMetadataResponse6::default()
        }
    }
}

impl From<UpdateMetadataResponse5> for UpdateMetadataResponse6 {
    fn from(older: UpdateMetadataResponse5) -> Self {
        UpdateMetadataResponse6 {
            error_code: older.error_code,
            ..UpdateMetadataResponse6::default()
        }
    }
}
