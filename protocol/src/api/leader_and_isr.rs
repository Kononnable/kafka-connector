use super::prelude::*;

pub type LeaderAndIsrRequest = LeaderAndIsrRequest4;
pub type LeaderAndIsrResponse = LeaderAndIsrResponse4;
impl ApiCall for LeaderAndIsrRequest {
    type Response = LeaderAndIsrResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        4
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::LeaderAndIsr
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                LeaderAndIsrRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                LeaderAndIsrRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &LeaderAndIsrRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &LeaderAndIsrRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &LeaderAndIsrRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &LeaderAndIsrRequest3::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, LeaderAndIsrResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => LeaderAndIsrResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => LeaderAndIsrResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => LeaderAndIsrResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => LeaderAndIsrResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => LeaderAndIsrResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => LeaderAndIsrResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequest0 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<LeaderAndIsrRequestUngroupedPartitionStates0>,
    pub live_leaders: Vec<LeaderAndIsrRequestLiveLeaders0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestUngroupedPartitionStates0 {
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
pub struct LeaderAndIsrRequestLiveLeaders0 {
    pub broker_id: Int32,
    pub host_name: String,
    pub port: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequest1 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<LeaderAndIsrRequestUngroupedPartitionStates1>,
    pub live_leaders: Vec<LeaderAndIsrRequestLiveLeaders1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestUngroupedPartitionStates1 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
    pub is_new: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestLiveLeaders1 {
    pub broker_id: Int32,
    pub host_name: String,
    pub port: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequest2 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Int64,
    pub topic_states: Vec<LeaderAndIsrRequestTopicStates2>,
    pub live_leaders: Vec<LeaderAndIsrRequestLiveLeaders2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestTopicStates2 {
    pub topic_name: String,
    pub partition_states: Vec<LeaderAndIsrRequestTopicStatesPartitionStates2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestTopicStatesPartitionStates2 {
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
    pub is_new: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestLiveLeaders2 {
    pub broker_id: Int32,
    pub host_name: String,
    pub port: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequest3 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Int64,
    pub topic_states: Vec<LeaderAndIsrRequestTopicStates3>,
    pub live_leaders: Vec<LeaderAndIsrRequestLiveLeaders3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestTopicStates3 {
    pub topic_name: String,
    pub partition_states: Vec<LeaderAndIsrRequestTopicStatesPartitionStates3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestTopicStatesPartitionStates3 {
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
    pub adding_replicas: Vec<Int32>,
    pub removing_replicas: Vec<Int32>,
    pub is_new: Boolean,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestLiveLeaders3 {
    pub broker_id: Int32,
    pub host_name: String,
    pub port: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequest4 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Int64,
    pub topic_states: Vec<LeaderAndIsrRequestTopicStates4>,
    pub live_leaders: Vec<LeaderAndIsrRequestLiveLeaders4>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestTopicStates4 {
    pub topic_name: String,
    pub partition_states: Vec<LeaderAndIsrRequestTopicStatesPartitionStates4>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestTopicStatesPartitionStates4 {
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
    pub adding_replicas: Vec<Int32>,
    pub removing_replicas: Vec<Int32>,
    pub is_new: Boolean,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaderAndIsrRequestLiveLeaders4 {
    pub broker_id: Int32,
    pub host_name: String,
    pub port: Int32,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponse0 {
    pub error_code: Int16,
    pub partition_errors: Vec<LeaderAndIsrResponsePartitionErrors0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponsePartitionErrors0 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponse1 {
    pub error_code: Int16,
    pub partition_errors: Vec<LeaderAndIsrResponsePartitionErrors1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponsePartitionErrors1 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponse2 {
    pub error_code: Int16,
    pub partition_errors: Vec<LeaderAndIsrResponsePartitionErrors2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponsePartitionErrors2 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponse3 {
    pub error_code: Int16,
    pub partition_errors: Vec<LeaderAndIsrResponsePartitionErrors3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponsePartitionErrors3 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponse4 {
    pub error_code: Int16,
    pub partition_errors: Vec<LeaderAndIsrResponsePartitionErrors4>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaderAndIsrResponsePartitionErrors4 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<LeaderAndIsrRequest4> for LeaderAndIsrRequest0 {
    fn from(latest: LeaderAndIsrRequest4) -> LeaderAndIsrRequest0 {
        log::debug!("Using old api format - LeaderAndIsrRequest0, ignoring field broker_epoch");
        log::debug!("Using old api format - LeaderAndIsrRequest0, ignoring field topic_states");
        LeaderAndIsrRequest0 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_leaders: latest
                .live_leaders
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            ..LeaderAndIsrRequest0::default()
        }
    }
}

impl From<LeaderAndIsrRequestLiveLeaders4> for LeaderAndIsrRequestLiveLeaders0 {
    fn from(latest: LeaderAndIsrRequestLiveLeaders4) -> LeaderAndIsrRequestLiveLeaders0 {
        LeaderAndIsrRequestLiveLeaders0 {
            broker_id: latest.broker_id,
            host_name: latest.host_name,
            port: latest.port,
        }
    }
}

impl From<LeaderAndIsrRequest4> for LeaderAndIsrRequest1 {
    fn from(latest: LeaderAndIsrRequest4) -> LeaderAndIsrRequest1 {
        log::debug!("Using old api format - LeaderAndIsrRequest1, ignoring field broker_epoch");
        log::debug!("Using old api format - LeaderAndIsrRequest1, ignoring field topic_states");
        LeaderAndIsrRequest1 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_leaders: latest
                .live_leaders
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            ..LeaderAndIsrRequest1::default()
        }
    }
}

impl From<LeaderAndIsrRequestLiveLeaders4> for LeaderAndIsrRequestLiveLeaders1 {
    fn from(latest: LeaderAndIsrRequestLiveLeaders4) -> LeaderAndIsrRequestLiveLeaders1 {
        LeaderAndIsrRequestLiveLeaders1 {
            broker_id: latest.broker_id,
            host_name: latest.host_name,
            port: latest.port,
        }
    }
}

impl From<LeaderAndIsrRequest4> for LeaderAndIsrRequest2 {
    fn from(latest: LeaderAndIsrRequest4) -> LeaderAndIsrRequest2 {
        LeaderAndIsrRequest2 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            broker_epoch: latest.broker_epoch,
            topic_states: latest
                .topic_states
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            live_leaders: latest
                .live_leaders
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<LeaderAndIsrRequestTopicStates4> for LeaderAndIsrRequestTopicStates2 {
    fn from(latest: LeaderAndIsrRequestTopicStates4) -> LeaderAndIsrRequestTopicStates2 {
        LeaderAndIsrRequestTopicStates2 {
            topic_name: latest.topic_name,
            partition_states: latest
                .partition_states
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<LeaderAndIsrRequestTopicStatesPartitionStates4>
    for LeaderAndIsrRequestTopicStatesPartitionStates2
{
    fn from(
        latest: LeaderAndIsrRequestTopicStatesPartitionStates4,
    ) -> LeaderAndIsrRequestTopicStatesPartitionStates2 {
        log::debug!("Using old api format - LeaderAndIsrRequestTopicStatesPartitionStates2, ignoring field adding_replicas");
        log::debug!("Using old api format - LeaderAndIsrRequestTopicStatesPartitionStates2, ignoring field removing_replicas");
        LeaderAndIsrRequestTopicStatesPartitionStates2 {
            partition_index: latest.partition_index,
            controller_epoch: latest.controller_epoch,
            leader: latest.leader,
            leader_epoch: latest.leader_epoch,
            isr: latest.isr,
            zk_version: latest.zk_version,
            replicas: latest.replicas,
            is_new: latest.is_new,
        }
    }
}

impl From<LeaderAndIsrRequestLiveLeaders4> for LeaderAndIsrRequestLiveLeaders2 {
    fn from(latest: LeaderAndIsrRequestLiveLeaders4) -> LeaderAndIsrRequestLiveLeaders2 {
        LeaderAndIsrRequestLiveLeaders2 {
            broker_id: latest.broker_id,
            host_name: latest.host_name,
            port: latest.port,
        }
    }
}

impl From<LeaderAndIsrRequest4> for LeaderAndIsrRequest3 {
    fn from(latest: LeaderAndIsrRequest4) -> LeaderAndIsrRequest3 {
        LeaderAndIsrRequest3 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            broker_epoch: latest.broker_epoch,
            topic_states: latest
                .topic_states
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
            live_leaders: latest
                .live_leaders
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<LeaderAndIsrRequestTopicStates4> for LeaderAndIsrRequestTopicStates3 {
    fn from(latest: LeaderAndIsrRequestTopicStates4) -> LeaderAndIsrRequestTopicStates3 {
        LeaderAndIsrRequestTopicStates3 {
            topic_name: latest.topic_name,
            partition_states: latest
                .partition_states
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<LeaderAndIsrRequestTopicStatesPartitionStates4>
    for LeaderAndIsrRequestTopicStatesPartitionStates3
{
    fn from(
        latest: LeaderAndIsrRequestTopicStatesPartitionStates4,
    ) -> LeaderAndIsrRequestTopicStatesPartitionStates3 {
        LeaderAndIsrRequestTopicStatesPartitionStates3 {
            partition_index: latest.partition_index,
            controller_epoch: latest.controller_epoch,
            leader: latest.leader,
            leader_epoch: latest.leader_epoch,
            isr: latest.isr,
            zk_version: latest.zk_version,
            replicas: latest.replicas,
            adding_replicas: latest.adding_replicas,
            removing_replicas: latest.removing_replicas,
            is_new: latest.is_new,
        }
    }
}

impl From<LeaderAndIsrRequestLiveLeaders4> for LeaderAndIsrRequestLiveLeaders3 {
    fn from(latest: LeaderAndIsrRequestLiveLeaders4) -> LeaderAndIsrRequestLiveLeaders3 {
        LeaderAndIsrRequestLiveLeaders3 {
            broker_id: latest.broker_id,
            host_name: latest.host_name,
            port: latest.port,
        }
    }
}

impl From<LeaderAndIsrResponse0> for LeaderAndIsrResponse4 {
    fn from(older: LeaderAndIsrResponse0) -> Self {
        LeaderAndIsrResponse4 {
            error_code: older.error_code,
            partition_errors: older
                .partition_errors
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..LeaderAndIsrResponse4::default()
        }
    }
}

impl From<LeaderAndIsrResponsePartitionErrors0> for LeaderAndIsrResponsePartitionErrors4 {
    fn from(older: LeaderAndIsrResponsePartitionErrors0) -> Self {
        LeaderAndIsrResponsePartitionErrors4 {
            topic_name: older.topic_name,
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..LeaderAndIsrResponsePartitionErrors4::default()
        }
    }
}

impl From<LeaderAndIsrResponse1> for LeaderAndIsrResponse4 {
    fn from(older: LeaderAndIsrResponse1) -> Self {
        LeaderAndIsrResponse4 {
            error_code: older.error_code,
            partition_errors: older
                .partition_errors
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..LeaderAndIsrResponse4::default()
        }
    }
}

impl From<LeaderAndIsrResponsePartitionErrors1> for LeaderAndIsrResponsePartitionErrors4 {
    fn from(older: LeaderAndIsrResponsePartitionErrors1) -> Self {
        LeaderAndIsrResponsePartitionErrors4 {
            topic_name: older.topic_name,
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..LeaderAndIsrResponsePartitionErrors4::default()
        }
    }
}

impl From<LeaderAndIsrResponse2> for LeaderAndIsrResponse4 {
    fn from(older: LeaderAndIsrResponse2) -> Self {
        LeaderAndIsrResponse4 {
            error_code: older.error_code,
            partition_errors: older
                .partition_errors
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..LeaderAndIsrResponse4::default()
        }
    }
}

impl From<LeaderAndIsrResponsePartitionErrors2> for LeaderAndIsrResponsePartitionErrors4 {
    fn from(older: LeaderAndIsrResponsePartitionErrors2) -> Self {
        LeaderAndIsrResponsePartitionErrors4 {
            topic_name: older.topic_name,
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..LeaderAndIsrResponsePartitionErrors4::default()
        }
    }
}

impl From<LeaderAndIsrResponse3> for LeaderAndIsrResponse4 {
    fn from(older: LeaderAndIsrResponse3) -> Self {
        LeaderAndIsrResponse4 {
            error_code: older.error_code,
            partition_errors: older
                .partition_errors
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..LeaderAndIsrResponse4::default()
        }
    }
}

impl From<LeaderAndIsrResponsePartitionErrors3> for LeaderAndIsrResponsePartitionErrors4 {
    fn from(older: LeaderAndIsrResponsePartitionErrors3) -> Self {
        LeaderAndIsrResponsePartitionErrors4 {
            topic_name: older.topic_name,
            partition_index: older.partition_index,
            error_code: older.error_code,
            ..LeaderAndIsrResponsePartitionErrors4::default()
        }
    }
}
