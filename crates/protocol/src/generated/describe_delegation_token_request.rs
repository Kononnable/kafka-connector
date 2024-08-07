use super::super::prelude::*;

/// Version 1 is the same as version 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DescribeDelegationTokenRequest {
    /// Each owner that we want to describe delegation tokens for, or null to describe all tokens.
    pub owners: Option<Vec<DescribeDelegationTokenOwner>>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DescribeDelegationTokenOwner {
    /// The owner principal type.
    pub principal_type: String,

    /// The owner principal name.
    pub principal_name: String,
}

impl ApiRequest for DescribeDelegationTokenRequest {
    type Response = super::describe_delegation_token_response::DescribeDelegationTokenResponse;

    fn get_api_key() -> i16 {
        41
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        1
    }

    fn serialize(
        &self,
        version: i16,
        bytes: &mut BytesMut,
        header: &RequestHeader,
    ) -> Result<(), SerializationError> {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        header.serialize(0, bytes)?;
        self.owners.serialize(version, bytes)?;
        Ok(())
    }

    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let owners = Option::<Vec<DescribeDelegationTokenOwner>>::deserialize(version, bytes);
        DescribeDelegationTokenRequest { owners }
    }
}

impl DescribeDelegationTokenRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.owners.is_none() {
            return Err(SerializationError::NullValue(
                "owners",
                _version,
                "DescribeDelegationTokenRequest",
            ));
        }
        Ok(())
    }
}

impl ToBytes for DescribeDelegationTokenOwner {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.principal_type.serialize(version, bytes)?;
        self.principal_name.serialize(version, bytes)?;
        Ok(())
    }
}

impl DescribeDelegationTokenOwner {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for DescribeDelegationTokenOwner {
    fn deserialize(version: i16, bytes: &mut BytesMut) -> Self {
        let principal_type = String::deserialize(version, bytes);
        let principal_name = String::deserialize(version, bytes);
        DescribeDelegationTokenOwner {
            principal_type,
            principal_name,
        }
    }
}
