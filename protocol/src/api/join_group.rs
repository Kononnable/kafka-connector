use super::prelude::*;
pub type JoinGroupRequest = JoinGroupRequest0;
impl ApiCall for JoinGroupRequest0 {
    type Response = JoinGroupResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        7
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::JoinGroup
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 6
    }
    fn serialize(&self, version: u16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 2),
            false => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 1),
        }
        ToBytes::serialize(self, buf, Self::is_flexible_version(version), version);
    }
    fn deserialize_response(version: u16, buf: &mut Bytes) -> (i32, Self::Response) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse::deserialize(buf, false, 2).correlation,
            false => HeaderResponse::deserialize(buf, false, 1).correlation,
        };
        let response =
            Self::Response::deserialize(buf, Self::is_flexible_version(version), version);
        (correlation, response)
    }
    fn deserialize_request(version: u16, buf: &mut Bytes) -> (OwnedHeaderRequest, Self) {
        let header = match Self::is_flexible_version(version) {
            true => OwnedHeaderRequest::deserialize(buf, false, 2),
            false => OwnedHeaderRequest::deserialize(buf, false, 1),
        };
        let request = Self::deserialize(buf, Self::is_flexible_version(version), version);
        (header, request)
    }
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct JoinGroupRequest0 {
    #[min_version = 0]
    pub group_id: String,
    #[min_version = 0]
    pub session_timeout_ms: Int32,
    #[min_version = 1]
    pub rebalance_timeout_ms: Int32,
    #[min_version = 0]
    pub member_id: String,
    #[min_version = 5]
    pub group_instance_id: NullableString,
    #[min_version = 0]
    pub protocol_type: String,
    #[min_version = 0]
    pub protocols: Vec<JoinGroupRequestProtocols0>,
    #[min_version = 6]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes, ToBytes)]
pub struct JoinGroupRequestProtocols0 {
    #[min_version = 0]
    pub name: String,
    #[min_version = 0]
    pub metadata: KafkaBytes,
    #[min_version = 6]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponse0 {
    #[min_version = 2]
    pub throttle_time_ms: Option<Int32>,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub generation_id: Int32,
    #[min_version = 7]
    pub protocol_type: Option<NullableString>,
    #[min_version = 0]
    pub protocol_name: NullableString,
    #[min_version = 0]
    pub leader: String,
    #[min_version = 0]
    pub member_id: String,
    #[min_version = 0]
    pub members: Vec<JoinGroupResponseMembers0>,
    #[min_version = 6]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct JoinGroupResponseMembers0 {
    #[min_version = 0]
    pub member_id: String,
    #[min_version = 5]
    pub group_instance_id: Option<NullableString>,
    #[min_version = 0]
    pub metadata: KafkaBytes,
    #[min_version = 6]
    pub tag_buffer: Option<TagBuffer>,
}

impl JoinGroupResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        if self.error_code != 0 {
            return Some(self.error_code.into());
        }
        None
    }
}
