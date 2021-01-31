use super::prelude::*;

pub type AlterIsrRequest = AlterIsrRequest0;
pub type AlterIsrResponse = AlterIsrResponse0;
impl ApiCall for AlterIsrRequest {
    type Response = AlterIsrResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::AlterIsr
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => true,
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
                AlterIsrRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                AlterIsrRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, AlterIsrResponse) {
        let header = HeaderResponse::deserialize(buf, false);
        let response = match version {
            0 => AlterIsrResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => AlterIsrResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (header.correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterIsrRequest0 {
    pub broker_id: Int32,
    pub broker_epoch: Int64,
    pub topics: Vec<AlterIsrRequestTopics0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterIsrRequestTopics0 {
    pub name: String,
    pub partitions: Vec<AlterIsrRequestTopicsPartitions0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct AlterIsrRequestTopicsPartitions0 {
    pub partition_index: Int32,
    pub leader_epoch: Int32,
    pub new_isr: Vec<Int32>,
    pub current_isr_version: Int32,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterIsrResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub topics: Vec<AlterIsrResponseTopics0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterIsrResponseTopics0 {
    pub name: String,
    pub partitions: Vec<AlterIsrResponseTopicsPartitions0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct AlterIsrResponseTopicsPartitions0 {
    pub partition_index: Int32,
    pub error_code: Int16,
    pub leader_id: Int32,
    pub leader_epoch: Int32,
    pub isr: Vec<Int32>,
    pub current_isr_version: Int32,
    pub tag_buffer: TagBuffer,
}
