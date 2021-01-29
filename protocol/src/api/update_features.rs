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
    fn serialize(self, version: i16, buf: &mut BytesMut) -> Result<(), Error> {
        match version {
            0 => ToBytes::serialize(&self, buf),
            _ => ToBytes::serialize(&self, buf),
        }
        Ok(())
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> UpdateFeaturesResponse {
        match version {
            0 => UpdateFeaturesResponse::deserialize(buf),
            _ => UpdateFeaturesResponse::deserialize(buf),
        }
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateFeaturesRequest0 {
    pub timeout_ms: Int32,
    pub feature_updates: Vec<UpdateFeaturesRequestFeatureUpdates0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct UpdateFeaturesRequestFeatureUpdates0 {
    pub feature: CompactString,
    pub max_version_level: Int16,
    pub allow_downgrade: Boolean,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct UpdateFeaturesResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub results: Vec<UpdateFeaturesResponseResults0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct UpdateFeaturesResponseResults0 {
    pub feature: CompactString,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
}
