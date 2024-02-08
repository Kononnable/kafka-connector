use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DeleteGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The deletion results
    pub results: Vec<DeletableGroupResult>,
}

#[derive(Clone, Debug, Default)]
pub struct DeletableGroupResult {
    /// The group id
    pub group_id: String,

    /// The deletion error, or 0 if the deletion succeeded.
    pub error_code: i16,
}

impl ApiResponse for DeleteGroupsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results = Vec::<DeletableGroupResult>::deserialize(version, bytes);
        (
            header,
            DeleteGroupsResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl FromBytes for DeletableGroupResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let group_id = String::deserialize(version, bytes);
        let error_code = i16::deserialize(version, bytes);
        DeletableGroupResult {
            group_id,
            error_code,
        }
    }
}
