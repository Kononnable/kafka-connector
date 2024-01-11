use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct DeleteGroupsResponse {
    pub throttle_time_ms: i32,
    pub results: Vec<DeletableGroupResult>,
}

#[derive(Debug, Default, Clone)]
pub struct DeletableGroupResult {
    pub group_id: String,
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
