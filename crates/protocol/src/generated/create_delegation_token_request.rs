use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CreateDelegationTokenRequest {
    /// A list of those who are allowed to renew this token before it expires.
    pub renewers: Vec<CreatableRenewers>,

    /// The maximum lifetime of the token in milliseconds, or -1 to use the server side default.
    pub max_lifetime_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CreatableRenewers {
    /// The type of the Kafka principal.
    pub principal_type: String,

    /// The name of the Kafka principal.
    pub principal_name: String,
}

impl ApiRequest for CreateDelegationTokenRequest {
    type Response = super::create_delegation_token_response::CreateDelegationTokenResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(38)
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
        self.renewers.serialize(version, _bytes);
        self.max_lifetime_ms.serialize(version, _bytes);
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let renewers = Vec::<CreatableRenewers>::deserialize(version, bytes);
        let max_lifetime_ms = i64::deserialize(version, bytes);
        CreateDelegationTokenRequest {
            renewers,
            max_lifetime_ms,
        }
    }
}

impl CreateDelegationTokenRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        for item in self.renewers.iter() {
            item.validate_fields(_version)?;
        }
        Ok(())
    }
}

impl ToBytes for CreatableRenewers {
    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
        self.principal_type.serialize(version, _bytes);
        self.principal_name.serialize(version, _bytes);
    }
}

impl CreatableRenewers {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for CreatableRenewers {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let principal_type = String::deserialize(version, bytes);
        let principal_name = String::deserialize(version, bytes);
        CreatableRenewers {
            principal_type,
            principal_name,
        }
    }
}
