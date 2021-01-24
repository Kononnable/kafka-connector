use super::prelude::*;

pub type ControlledShutdownRequest = ControlledShutdownRequest3;
pub type ControlledShutdownResponse = ControlledShutdownResponse3;
pub fn serialize_controlled_shutdown_request(
    data: ControlledShutdownRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&ControlledShutdownRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&ControlledShutdownRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&ControlledShutdownRequest2::try_from(data)?, buf),
        4 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_controlled_shutdown_response<T>(
    version: i32,
    buf: &mut T,
) -> ControlledShutdownResponse
where
    T: Iterator<Item = u8>,
{
    match version {
        0 => ControlledShutdownResponse0::deserialize(buf).into(),
        1 => ControlledShutdownResponse1::deserialize(buf).into(),
        2 => ControlledShutdownResponse2::deserialize(buf).into(),
        4 => ControlledShutdownResponse::deserialize(buf),
        _ => ControlledShutdownResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct ControlledShutdownRequest0 {
    pub broker_id: Int32,
}

#[derive(Default, ToBytes)]
pub struct ControlledShutdownRequest1 {
    pub broker_id: Int32,
}

#[derive(Default, ToBytes)]
pub struct ControlledShutdownRequest2 {
    pub broker_id: Int32,
    pub broker_epoch: Optional<Int64>,
}

#[derive(Default, ToBytes)]
pub struct ControlledShutdownRequest3 {
    pub broker_id: Int32,
    pub broker_epoch: Optional<Int64>,
}

#[derive(Default, FromBytes)]
pub struct ControlledShutdownResponse0 {
    pub error_code: Int16,
    pub remaining_partitions: Vec<ControlledShutdownResponseRemainingPartitions0>,
}

#[derive(Default, FromBytes)]
pub struct ControlledShutdownResponseRemainingPartitions0 {
    pub topic_name: String,
    pub partition_index: Int32,
}

#[derive(Default, FromBytes)]
pub struct ControlledShutdownResponse1 {
    pub error_code: Int16,
    pub remaining_partitions: Vec<ControlledShutdownResponseRemainingPartitions1>,
}

#[derive(Default, FromBytes)]
pub struct ControlledShutdownResponseRemainingPartitions1 {
    pub topic_name: String,
    pub partition_index: Int32,
}

#[derive(Default, FromBytes)]
pub struct ControlledShutdownResponse2 {
    pub error_code: Int16,
    pub remaining_partitions: Vec<ControlledShutdownResponseRemainingPartitions2>,
}

#[derive(Default, FromBytes)]
pub struct ControlledShutdownResponseRemainingPartitions2 {
    pub topic_name: String,
    pub partition_index: Int32,
}

#[derive(Default, FromBytes)]
pub struct ControlledShutdownResponse3 {
    pub error_code: Int16,
    pub remaining_partitions: Vec<ControlledShutdownResponseRemainingPartitions3>,
}

#[derive(Default, FromBytes)]
pub struct ControlledShutdownResponseRemainingPartitions3 {
    pub topic_name: CompactString,
    pub partition_index: Int32,
}

impl TryFrom<ControlledShutdownRequest3> for ControlledShutdownRequest0 {
    type Error = Error;
    fn try_from(latest: ControlledShutdownRequest3) -> Result<Self, Self::Error> {
        if latest.broker_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "ControlledShutdownRequest",
                0,
                "broker_epoch",
            ));
        }
        Ok(ControlledShutdownRequest0 {
            broker_id: latest.broker_id,
        })
    }
}

impl TryFrom<ControlledShutdownRequest3> for ControlledShutdownRequest1 {
    type Error = Error;
    fn try_from(latest: ControlledShutdownRequest3) -> Result<Self, Self::Error> {
        if latest.broker_epoch.is_some() {
            return Err(Error::OldKafkaVersion(
                "ControlledShutdownRequest",
                1,
                "broker_epoch",
            ));
        }
        Ok(ControlledShutdownRequest1 {
            broker_id: latest.broker_id,
        })
    }
}

impl TryFrom<ControlledShutdownRequest3> for ControlledShutdownRequest2 {
    type Error = Error;
    fn try_from(latest: ControlledShutdownRequest3) -> Result<Self, Self::Error> {
        Ok(ControlledShutdownRequest2 {
            broker_id: latest.broker_id,
            broker_epoch: latest.broker_epoch.map(|val| val),
        })
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
        }
    }
}
