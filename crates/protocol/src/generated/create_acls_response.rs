use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct CreateAclsResponse {
    pub throttle_time_ms: i32,
    pub results: Vec<CreatableAclResult>,
}

#[derive(Debug, Default, Clone)]
pub struct CreatableAclResult {
    pub error_code: i16,
    pub error_message: String,
}

impl ApiResponse for CreateAclsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let results = if version >= 0 {
            Vec::<CreatableAclResult>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            CreateAclsResponse {
                throttle_time_ms,
                results,
            },
        )
    }
}

impl FromBytes for CreatableAclResult {
    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {
        let error_code = if version >= 0 {
            i16::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let error_message = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        CreatableAclResult {
            error_code,
            error_message,
        }
    }
}
