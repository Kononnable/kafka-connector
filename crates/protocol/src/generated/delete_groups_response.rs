use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct DeleteGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The deletion results
    pub results: Vec<DeletableGroupResult>,
}

#[derive(Debug, Clone)]
pub struct DeletableGroupResult {
    /// The group id
    pub group_id: String,

    /// The deletion error, or 0 if the deletion succeeded.
    pub error_code: i16,
}

impl ApiResponse for DeleteGroupsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let results = if version >= 0 {
            Vec::<DeletableGroupResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            DeleteGroupsResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl Default for DeleteGroupsResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: Default::default(),
            results: Default::default(),
        }
    }
}

impl FromBytes for DeletableGroupResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let group_id = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        DeletableGroupResult {
            group_id,
            error_code,
        }
    }
}

impl Default for DeletableGroupResult {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            error_code: Default::default(),
        }
    }
}