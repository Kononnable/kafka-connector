use super::super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct AlterConfigsResponse {
    pub throttle_time_ms: i32,
    pub resources: Vec<AlterConfigsResourceResponse>,
}

#[derive(Debug, Default, Clone)]
pub struct AlterConfigsResourceResponse {
    pub error_code: i16,
    pub error_message: String,
    pub resource_type: i8,
    pub resource_name: String,
}

impl ApiResponse for AlterConfigsResponse {
    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {
        let header = ResponseHeader::deserialize(0, bytes);
        let throttle_time_ms = if version >= 0 {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let resources = if version >= 0 {
            Vec::<AlterConfigsResourceResponse>::deserialize(version, bytes)
        } else {
            Default::default()
        };
        (
            header,
            AlterConfigsResponse {
                throttle_time_ms,
                resources,
            },
        )
    }
}

impl FromBytes for AlterConfigsResourceResponse {
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
        let resource_type = if version >= 0 {
            i8::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let resource_name = if version >= 0 {
            String::deserialize(version, bytes)
        } else {
            Default::default()
        };
        AlterConfigsResourceResponse {
            error_code,
            error_message,
            resource_type,
            resource_name,
        }
    }
}
