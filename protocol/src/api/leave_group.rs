use super::prelude::*;
pub type LeaveGroupRequest = LeaveGroupRequest0;
impl ApiCall for LeaveGroupRequest0 {
    type Response = LeaveGroupResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        4
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::LeaveGroup
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 4
    }
    fn serialize(self, version: u16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 2),
            false => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 1),
        }
        ToBytes::serialize(&self, buf, Self::is_flexible_version(version), version);
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
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaveGroupRequest0 {
    #[min_version = 0]
    pub group_id: String,
    #[min_version = 3]
    pub members: Vec<LeaveGroupRequestMembers0>,
    #[min_version = 4]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct LeaveGroupRequestMembers0 {
    #[min_version = 3]
    pub member_id: String,
    #[min_version = 3]
    pub group_instance_id: NullableString,
    #[min_version = 4]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponse0 {
    #[min_version = 1]
    pub throttle_time_ms: Option<Int32>,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 3]
    pub members: Option<Vec<LeaveGroupResponseMembers0>>,
    #[min_version = 4]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct LeaveGroupResponseMembers0 {
    #[min_version = 3]
    pub member_id: Option<String>,
    #[min_version = 3]
    pub group_instance_id: Option<NullableString>,
    #[min_version = 3]
    pub error_code: Option<Int16>,
    #[min_version = 4]
    pub tag_buffer: Option<TagBuffer>,
}

impl LeaveGroupResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        if let Some(vec) = self.members.as_ref() {
            for item in vec {
                if let Some(x) = item.get_first_error() {
                    return Some(x);
                };
            }
        }
        None
    }
}
impl LeaveGroupResponseMembers0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
