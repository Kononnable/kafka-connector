use super::prelude::*;

pub type UpdateFeaturesRequest = UpdateFeaturesRequest0;
pub type UpdateFeaturesResponse = UpdateFeaturesResponse0;
pub fn serialize_update_features_request(
    data: UpdateFeaturesRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_update_features_response(
    version: i32,
    buf: &mut Bytes,
) -> UpdateFeaturesResponse {
    match version {
        0 => UpdateFeaturesResponse::deserialize(buf),
        _ => UpdateFeaturesResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct UpdateFeaturesRequest0 {
    pub timeout_ms: Int32,
    pub feature_updates: Vec<UpdateFeaturesRequestFeatureUpdates0>,
}

#[derive(Default, ToBytes)]
pub struct UpdateFeaturesRequestFeatureUpdates0 {
    pub feature: CompactString,
    pub max_version_level: Int16,
    pub allow_downgrade: Boolean,
}

#[derive(Default, FromBytes)]
pub struct UpdateFeaturesResponse0 {
    pub throttle_time_ms: Int32,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
    pub results: Vec<UpdateFeaturesResponseResults0>,
}

#[derive(Default, FromBytes)]
pub struct UpdateFeaturesResponseResults0 {
    pub feature: CompactString,
    pub error_code: Int16,
    pub error_message: CompactNullableString,
}
