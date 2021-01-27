use super::prelude::*;

pub type LeaderAndIsrRequest = LeaderAndIsrRequest4;
pub type LeaderAndIsrResponse = LeaderAndIsrResponse4;
pub fn serialize_leader_and_isr_request(
    data: LeaderAndIsrRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&LeaderAndIsrRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&LeaderAndIsrRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&LeaderAndIsrRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&LeaderAndIsrRequest3::try_from(data)?, buf),
        4 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_leader_and_isr_response(version: i32, buf: &mut Bytes) -> LeaderAndIsrResponse {
    match version {
        0 => LeaderAndIsrResponse0::deserialize(buf).into(),
        1 => LeaderAndIsrResponse1::deserialize(buf).into(),
        2 => LeaderAndIsrResponse2::deserialize(buf).into(),
        3 => LeaderAndIsrResponse3::deserialize(buf).into(),
        4 => LeaderAndIsrResponse::deserialize(buf),
        _ => LeaderAndIsrResponse::deserialize(buf),
    }
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequest0 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<LeaderAndIsrRequestUngroupedPartitionStates0>,
    pub live_leaders: Vec<LeaderAndIsrRequestLiveLeaders0>,
}

#[derive(Default, Debug, ToBytes)]
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

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequestLiveLeaders0 {
    pub broker_id: Int32,
    pub host_name: String,
    pub port: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequest1 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub ungrouped_partition_states: Vec<LeaderAndIsrRequestUngroupedPartitionStates1>,
    pub live_leaders: Vec<LeaderAndIsrRequestLiveLeaders1>,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequestUngroupedPartitionStates1 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
    pub is_new: Optional<Boolean>,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequestLiveLeaders1 {
    pub broker_id: Int32,
    pub host_name: String,
    pub port: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequest2 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Optional<Int64>,
    pub topic_states: Optional<Vec<LeaderAndIsrRequestTopicStates2>>,
    pub live_leaders: Vec<LeaderAndIsrRequestLiveLeaders2>,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequestTopicStates2 {
    pub topic_name: String,
    pub partition_states: Vec<LeaderAndIsrRequestTopicStatesPartitionStates2>,
}

#[derive(Default, Debug, ToBytes)]
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

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequestLiveLeaders2 {
    pub broker_id: Int32,
    pub host_name: String,
    pub port: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequest3 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Optional<Int64>,
    pub topic_states: Optional<Vec<LeaderAndIsrRequestTopicStates3>>,
    pub live_leaders: Vec<LeaderAndIsrRequestLiveLeaders3>,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequestTopicStates3 {
    pub topic_name: String,
    pub partition_states: Vec<LeaderAndIsrRequestTopicStatesPartitionStates3>,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequestTopicStatesPartitionStates3 {
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
    pub adding_replicas: Optional<Vec<Int32>>,
    pub removing_replicas: Optional<Vec<Int32>>,
    pub is_new: Boolean,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequestLiveLeaders3 {
    pub broker_id: Int32,
    pub host_name: String,
    pub port: Int32,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequest4 {
    pub controller_id: Int32,
    pub controller_epoch: Int32,
    pub broker_epoch: Optional<Int64>,
    pub topic_states: Optional<Vec<LeaderAndIsrRequestTopicStates4>>,
    pub live_leaders: Vec<LeaderAndIsrRequestLiveLeaders4>,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequestTopicStates4 {
    pub topic_name: CompactString,
    pub partition_states: Vec<LeaderAndIsrRequestTopicStatesPartitionStates4>,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequestTopicStatesPartitionStates4 {
    pub partition_index: Int32,
    pub controller_epoch: Int32,
    pub leader: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub zk_version: Int32,
    pub replicas: Vec<Int32>,
    pub adding_replicas: Optional<Vec<Int32>>,
    pub removing_replicas: Optional<Vec<Int32>>,
    pub is_new: Boolean,
}

#[derive(Default, Debug, ToBytes)]
pub struct LeaderAndIsrRequestLiveLeaders4 {
    pub broker_id: Int32,
    pub host_name: CompactString,
    pub port: Int32,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaderAndIsrResponse0 {
    pub error_code: Int16,
    pub partition_errors: Vec<LeaderAndIsrResponsePartitionErrors0>,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaderAndIsrResponsePartitionErrors0 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaderAndIsrResponse1 {
    pub error_code: Int16,
    pub partition_errors: Vec<LeaderAndIsrResponsePartitionErrors1>,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaderAndIsrResponsePartitionErrors1 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaderAndIsrResponse2 {
    pub error_code: Int16,
    pub partition_errors: Vec<LeaderAndIsrResponsePartitionErrors2>,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaderAndIsrResponsePartitionErrors2 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaderAndIsrResponse3 {
    pub error_code: Int16,
    pub partition_errors: Vec<LeaderAndIsrResponsePartitionErrors3>,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaderAndIsrResponsePartitionErrors3 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub error_code: Int16,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaderAndIsrResponse4 {
    pub error_code: Int16,
    pub partition_errors: Vec<LeaderAndIsrResponsePartitionErrors4>,
}

#[derive(Default, Debug, FromBytes)]
pub struct LeaderAndIsrResponsePartitionErrors4 {
    pub topic_name: CompactString,
    pub partition_index: Int32,
    pub error_code: Int16,
}

impl TryFrom<LeaderAndIsrRequest4> for LeaderAndIsrRequest0 {
    type Error = Error;
    fn try_from(latest: LeaderAndIsrRequest4) -> Result<Self, Self::Error> {
        if latest.broker_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "LeaderAndIsrRequest",
                0,
                "broker_epoch",
            ));
        }
        if latest.topic_states.is_some() {
            return Err(Error::OldKafkaVersion(
                "LeaderAndIsrRequest",
                0,
                "topic_states",
            ));
        }
        Ok(LeaderAndIsrRequest0 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_leaders: latest
                .live_leaders
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            ..LeaderAndIsrRequest0::default()
        })
    }
}

impl TryFrom<LeaderAndIsrRequestLiveLeaders4> for LeaderAndIsrRequestLiveLeaders0 {
    type Error = Error;
    fn try_from(latest: LeaderAndIsrRequestLiveLeaders4) -> Result<Self, Self::Error> {
        Ok(LeaderAndIsrRequestLiveLeaders0 {
            broker_id: latest.broker_id,
            host_name: latest.host_name.into(),
            port: latest.port,
        })
    }
}

impl TryFrom<LeaderAndIsrRequest4> for LeaderAndIsrRequest1 {
    type Error = Error;
    fn try_from(latest: LeaderAndIsrRequest4) -> Result<Self, Self::Error> {
        if latest.broker_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "LeaderAndIsrRequest",
                1,
                "broker_epoch",
            ));
        }
        if latest.topic_states.is_some() {
            return Err(Error::OldKafkaVersion(
                "LeaderAndIsrRequest",
                1,
                "topic_states",
            ));
        }
        Ok(LeaderAndIsrRequest1 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            live_leaders: latest
                .live_leaders
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
            ..LeaderAndIsrRequest1::default()
        })
    }
}

impl TryFrom<LeaderAndIsrRequestLiveLeaders4> for LeaderAndIsrRequestLiveLeaders1 {
    type Error = Error;
    fn try_from(latest: LeaderAndIsrRequestLiveLeaders4) -> Result<Self, Self::Error> {
        Ok(LeaderAndIsrRequestLiveLeaders1 {
            broker_id: latest.broker_id,
            host_name: latest.host_name.into(),
            port: latest.port,
        })
    }
}

impl TryFrom<LeaderAndIsrRequest4> for LeaderAndIsrRequest2 {
    type Error = Error;
    fn try_from(latest: LeaderAndIsrRequest4) -> Result<Self, Self::Error> {
        Ok(LeaderAndIsrRequest2 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            broker_epoch: latest.broker_epoch,
            topic_states: latest
                .topic_states
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
            live_leaders: latest
                .live_leaders
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<LeaderAndIsrRequestTopicStates4> for LeaderAndIsrRequestTopicStates2 {
    type Error = Error;
    fn try_from(latest: LeaderAndIsrRequestTopicStates4) -> Result<Self, Self::Error> {
        Ok(LeaderAndIsrRequestTopicStates2 {
            topic_name: latest.topic_name.into(),
            partition_states: latest
                .partition_states
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<LeaderAndIsrRequestTopicStatesPartitionStates4>
    for LeaderAndIsrRequestTopicStatesPartitionStates2
{
    type Error = Error;
    fn try_from(
        latest: LeaderAndIsrRequestTopicStatesPartitionStates4,
    ) -> Result<Self, Self::Error> {
        if latest.adding_replicas.is_some() {
            return Err(Error::OldKafkaVersion(
                "LeaderAndIsrRequestTopicStatesPartitionStates",
                2,
                "adding_replicas",
            ));
        }
        if latest.removing_replicas.is_some() {
            return Err(Error::OldKafkaVersion(
                "LeaderAndIsrRequestTopicStatesPartitionStates",
                2,
                "removing_replicas",
            ));
        }
        Ok(LeaderAndIsrRequestTopicStatesPartitionStates2 {
            partition_index: latest.partition_index,
            controller_epoch: latest.controller_epoch,
            leader: latest.leader,
            leader_epoch: latest.leader_epoch,
            isr: latest.isr,
            zk_version: latest.zk_version,
            replicas: latest.replicas,
            is_new: latest.is_new,
        })
    }
}

impl TryFrom<LeaderAndIsrRequestLiveLeaders4> for LeaderAndIsrRequestLiveLeaders2 {
    type Error = Error;
    fn try_from(latest: LeaderAndIsrRequestLiveLeaders4) -> Result<Self, Self::Error> {
        Ok(LeaderAndIsrRequestLiveLeaders2 {
            broker_id: latest.broker_id,
            host_name: latest.host_name.into(),
            port: latest.port,
        })
    }
}

impl TryFrom<LeaderAndIsrRequest4> for LeaderAndIsrRequest3 {
    type Error = Error;
    fn try_from(latest: LeaderAndIsrRequest4) -> Result<Self, Self::Error> {
        Ok(LeaderAndIsrRequest3 {
            controller_id: latest.controller_id,
            controller_epoch: latest.controller_epoch,
            broker_epoch: latest.broker_epoch,
            topic_states: latest
                .topic_states
                .map(|val| {
                    val.into_iter()
                        .map(|el| el.try_into())
                        .collect::<Result<_, Error>>()
                })
                .wrap_result()?,
            live_leaders: latest
                .live_leaders
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<LeaderAndIsrRequestTopicStates4> for LeaderAndIsrRequestTopicStates3 {
    type Error = Error;
    fn try_from(latest: LeaderAndIsrRequestTopicStates4) -> Result<Self, Self::Error> {
        Ok(LeaderAndIsrRequestTopicStates3 {
            topic_name: latest.topic_name.into(),
            partition_states: latest
                .partition_states
                .into_iter()
                .map(|ele| ele.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<LeaderAndIsrRequestTopicStatesPartitionStates4>
    for LeaderAndIsrRequestTopicStatesPartitionStates3
{
    type Error = Error;
    fn try_from(
        latest: LeaderAndIsrRequestTopicStatesPartitionStates4,
    ) -> Result<Self, Self::Error> {
        Ok(LeaderAndIsrRequestTopicStatesPartitionStates3 {
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
        })
    }
}

impl TryFrom<LeaderAndIsrRequestLiveLeaders4> for LeaderAndIsrRequestLiveLeaders3 {
    type Error = Error;
    fn try_from(latest: LeaderAndIsrRequestLiveLeaders4) -> Result<Self, Self::Error> {
        Ok(LeaderAndIsrRequestLiveLeaders3 {
            broker_id: latest.broker_id,
            host_name: latest.host_name.into(),
            port: latest.port,
        })
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
        }
    }
}

impl From<LeaderAndIsrResponsePartitionErrors0> for LeaderAndIsrResponsePartitionErrors4 {
    fn from(older: LeaderAndIsrResponsePartitionErrors0) -> Self {
        LeaderAndIsrResponsePartitionErrors4 {
            topic_name: older.topic_name.into(),
            partition_index: older.partition_index,
            error_code: older.error_code,
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
        }
    }
}

impl From<LeaderAndIsrResponsePartitionErrors1> for LeaderAndIsrResponsePartitionErrors4 {
    fn from(older: LeaderAndIsrResponsePartitionErrors1) -> Self {
        LeaderAndIsrResponsePartitionErrors4 {
            topic_name: older.topic_name.into(),
            partition_index: older.partition_index,
            error_code: older.error_code,
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
        }
    }
}

impl From<LeaderAndIsrResponsePartitionErrors2> for LeaderAndIsrResponsePartitionErrors4 {
    fn from(older: LeaderAndIsrResponsePartitionErrors2) -> Self {
        LeaderAndIsrResponsePartitionErrors4 {
            topic_name: older.topic_name.into(),
            partition_index: older.partition_index,
            error_code: older.error_code,
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
        }
    }
}

impl From<LeaderAndIsrResponsePartitionErrors3> for LeaderAndIsrResponsePartitionErrors4 {
    fn from(older: LeaderAndIsrResponsePartitionErrors3) -> Self {
        LeaderAndIsrResponsePartitionErrors4 {
            topic_name: older.topic_name.into(),
            partition_index: older.partition_index,
            error_code: older.error_code,
        }
    }
}
