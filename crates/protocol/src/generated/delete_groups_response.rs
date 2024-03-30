use super::super::prelude::*;

/// Starting in version 1, on quota violation, brokers send out responses before throttling.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DeleteGroupsResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// The deletion results
    pub results: IndexMap<DeletableGroupResultKey, DeletableGroupResult>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct DeletableGroupResultKey {
    /// The group id
    pub group_id: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DeletableGroupResult {
    /// The deletion error, or 0 if the deletion succeeded.
    pub error_code: i16,
}

impl ApiResponse for DeleteGroupsResponse {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = i32::deserialize(version, bytes);
        let results =
            IndexMap::<DeletableGroupResultKey, DeletableGroupResult>::deserialize(version, bytes);
        (
            header,
            DeleteGroupsResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl FromBytes for DeletableGroupResultKey {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let group_id = String::deserialize(version, bytes);
        DeletableGroupResultKey { group_id }
    }
}

impl FromBytes for DeletableGroupResult {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let error_code = i16::deserialize(version, bytes);
        DeletableGroupResult { error_code }
    }
}
