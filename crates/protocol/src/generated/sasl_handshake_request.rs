use super::super::prelude::*;

/// Version 1 supports SASL_AUTHENTICATE.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SaslHandshakeRequest {
    /// The SASL mechanism chosen by the client.
    pub mechanism: String,
}

impl ApiRequest for SaslHandshakeRequest {
    type Response = super::sasl_handshake_response::SaslHandshakeResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(17)
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
        self.mechanism.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let mechanism = String::deserialize(version, bytes);
        SaslHandshakeRequest { mechanism }
    }
}

impl SaslHandshakeRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}
