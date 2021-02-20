use super::prelude::*;
pub type DescribeGroupsRequest = DescribeGroupsRequest0;
impl ApiCall for DescribeGroupsRequest0 {
    type Response = DescribeGroupsResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        5
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::DescribeGroups
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(version: u16) -> bool {
        version >= 5
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
pub struct DescribeGroupsRequest0 {
    #[min_version = 0]
    pub groups: Vec<String>,
    #[min_version = 3]
    pub include_authorized_operations: Boolean,
    #[min_version = 5]
    pub tag_buffer: TagBuffer,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponse0 {
    #[min_version = 1]
    pub throttle_time_ms: Option<Int32>,
    #[min_version = 0]
    pub groups: Vec<DescribeGroupsResponseGroups0>,
    #[min_version = 5]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroups0 {
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub group_id: String,
    #[min_version = 0]
    pub group_state: String,
    #[min_version = 0]
    pub protocol_type: String,
    #[min_version = 0]
    pub protocol_data: String,
    #[min_version = 0]
    pub members: Vec<DescribeGroupsResponseGroupsMembers0>,
    #[min_version = 3]
    pub authorized_operations: Option<Int32>,
    #[min_version = 5]
    pub tag_buffer: Option<TagBuffer>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct DescribeGroupsResponseGroupsMembers0 {
    #[min_version = 0]
    pub member_id: String,
    #[min_version = 4]
    pub group_instance_id: Option<NullableString>,
    #[min_version = 0]
    pub client_id: String,
    #[min_version = 0]
    pub client_host: String,
    #[min_version = 0]
    pub member_metadata: KafkaBytes,
    #[min_version = 0]
    pub member_assignment: KafkaBytes,
    #[min_version = 5]
    pub tag_buffer: Option<TagBuffer>,
}

impl DescribeGroupsResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.groups.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DescribeGroupsResponseGroups0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.members.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl DescribeGroupsResponseGroupsMembers0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
