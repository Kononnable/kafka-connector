use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SaslAuthenticateRequest {
    /// The SASL authentication bytes from the client, as defined by the SASL mechanism.
    pub auth_bytes: Vec<u8>,
}

impl ApiRequest for SaslAuthenticateRequest {
    type Response = super::sasl_authenticate_response::SaslAuthenticateResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(36)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(1)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.auth_bytes.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let auth_bytes = Vec::<u8>::deserialize(version, bytes);
        SaslAuthenticateRequest { auth_bytes }
    }
}

impl SaslAuthenticateRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
