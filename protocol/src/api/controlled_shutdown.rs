use super::prelude::*;

pub type ControlledShutdownRequest = ControlledShutdownRequest3;
pub type ControlledShutdownResponse = ControlledShutdownResponse3;
impl ApiCall for ControlledShutdownRequest {
    type Response = ControlledShutdownResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        3
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::ControlledShutdown
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                ControlledShutdownRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                ControlledShutdownRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &ControlledShutdownRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &ControlledShutdownRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &ControlledShutdownRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, ControlledShutdownResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => ControlledShutdownResponse0::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            1 => ControlledShutdownResponse1::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            2 => ControlledShutdownResponse2::deserialize(buf, Self::is_flexible_version(version))
                .into(),
            3 => ControlledShutdownResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => ControlledShutdownResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct ControlledShutdownRequest0 {
    pub broker_id: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ControlledShutdownRequest1 {
    pub broker_id: Int32,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ControlledShutdownRequest2 {
    pub broker_id: Int32,
    pub broker_epoch: Int64,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ControlledShutdownRequest3 {
    pub broker_id: Int32,
    pub broker_epoch: Int64,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ControlledShutdownResponse0 {
    pub error_code: Int16,
    pub remaining_partitions: Vec<ControlledShutdownResponseRemainingPartitions0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ControlledShutdownResponseRemainingPartitions0 {
    pub topic_name: String,
    pub partition_index: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ControlledShutdownResponse1 {
    pub error_code: Int16,
    pub remaining_partitions: Vec<ControlledShutdownResponseRemainingPartitions1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ControlledShutdownResponseRemainingPartitions1 {
    pub topic_name: String,
    pub partition_index: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ControlledShutdownResponse2 {
    pub error_code: Int16,
    pub remaining_partitions: Vec<ControlledShutdownResponseRemainingPartitions2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ControlledShutdownResponseRemainingPartitions2 {
    pub topic_name: String,
    pub partition_index: Int32,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ControlledShutdownResponse3 {
    pub error_code: Int16,
    pub remaining_partitions: Vec<ControlledShutdownResponseRemainingPartitions3>,
    pub tag_buffer: Option<TagBuffer>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ControlledShutdownResponseRemainingPartitions3 {
    pub topic_name: String,
    pub partition_index: Int32,
    pub tag_buffer: Option<TagBuffer>,
}

impl From<ControlledShutdownRequest3> for ControlledShutdownRequest0 {
    fn from(latest: ControlledShutdownRequest3) -> ControlledShutdownRequest0 {
        log::debug!(
            "Using old api format - ControlledShutdownRequest0, ignoring field broker_epoch"
        );
        ControlledShutdownRequest0 {
            broker_id: latest.broker_id,
        }
    }
}

impl From<ControlledShutdownRequest3> for ControlledShutdownRequest1 {
    fn from(latest: ControlledShutdownRequest3) -> ControlledShutdownRequest1 {
        log::debug!(
            "Using old api format - ControlledShutdownRequest1, ignoring field broker_epoch"
        );
        ControlledShutdownRequest1 {
            broker_id: latest.broker_id,
        }
    }
}

impl From<ControlledShutdownRequest3> for ControlledShutdownRequest2 {
    fn from(latest: ControlledShutdownRequest3) -> ControlledShutdownRequest2 {
        ControlledShutdownRequest2 {
            broker_id: latest.broker_id,
            broker_epoch: latest.broker_epoch,
        }
    }
}

impl From<ControlledShutdownResponse0> for ControlledShutdownResponse3 {
    fn from(older: ControlledShutdownResponse0) -> Self {
        ControlledShutdownResponse3 {
            error_code: older.error_code,
            remaining_partitions: older
                .remaining_partitions
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..ControlledShutdownResponse3::default()
        }
    }
}

impl From<ControlledShutdownResponseRemainingPartitions0>
    for ControlledShutdownResponseRemainingPartitions3
{
    fn from(older: ControlledShutdownResponseRemainingPartitions0) -> Self {
        ControlledShutdownResponseRemainingPartitions3 {
            topic_name: older.topic_name,
            partition_index: older.partition_index,
            ..ControlledShutdownResponseRemainingPartitions3::default()
        }
    }
}

impl From<ControlledShutdownResponse1> for ControlledShutdownResponse3 {
    fn from(older: ControlledShutdownResponse1) -> Self {
        ControlledShutdownResponse3 {
            error_code: older.error_code,
            remaining_partitions: older
                .remaining_partitions
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..ControlledShutdownResponse3::default()
        }
    }
}

impl From<ControlledShutdownResponseRemainingPartitions1>
    for ControlledShutdownResponseRemainingPartitions3
{
    fn from(older: ControlledShutdownResponseRemainingPartitions1) -> Self {
        ControlledShutdownResponseRemainingPartitions3 {
            topic_name: older.topic_name,
            partition_index: older.partition_index,
            ..ControlledShutdownResponseRemainingPartitions3::default()
        }
    }
}

impl From<ControlledShutdownResponse2> for ControlledShutdownResponse3 {
    fn from(older: ControlledShutdownResponse2) -> Self {
        ControlledShutdownResponse3 {
            error_code: older.error_code,
            remaining_partitions: older
                .remaining_partitions
                .into_iter()
                .map(|el| el.into())
                .collect(),
            ..ControlledShutdownResponse3::default()
        }
    }
}

impl From<ControlledShutdownResponseRemainingPartitions2>
    for ControlledShutdownResponseRemainingPartitions3
{
    fn from(older: ControlledShutdownResponseRemainingPartitions2) -> Self {
        ControlledShutdownResponseRemainingPartitions3 {
            topic_name: older.topic_name,
            partition_index: older.partition_index,
            ..ControlledShutdownResponseRemainingPartitions3::default()
        }
    }
}
