use super::prelude::*;

pub type UpdateFeaturesRequest = UpdateFeaturesRequest0;
pub type UpdateFeaturesResponse = UpdateFeaturesResponse0;
impl ApiCall for UpdateFeaturesRequest {
    type Response = UpdateFeaturesResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        0
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::UpdateFeatures
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => true,
            _ => true,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                UpdateFeaturesRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                UpdateFeaturesRequest::get_api_key(),
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
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, UpdateFeaturesResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => UpdateFeaturesResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => UpdateFeaturesResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateFeaturesRequest0 {
    pub timeout_ms: Int32,
    pub feature_updates: Vec<UpdateFeaturesRequestFeatureUpdates0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateFeaturesRequestFeatureUpdates0 {
    pub feature: String,
    pub max_version_level: Int16,
    pub allow_downgrade: Boolean,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct UpdateFeaturesResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub results: Vec<UpdateFeaturesResponseResults0>,
    pub tag_buffer: TagBuffer,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct UpdateFeaturesResponseResults0 {
    pub feature: String,
    pub error_code: Int16,
    pub error_message: NullableString,
    pub tag_buffer: TagBuffer,
}
