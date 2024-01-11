use super::super::prelude::*;

#[derive(Clone, Debug)]
pub struct DescribeDelegationTokenRequest {
    /// Each owner that we want to describe delegation tokens for, or null to describe all tokens.
    pub owners: Vec<DescribeDelegationTokenOwner>,
}

#[derive(Debug, Clone)]
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

    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) {
        debug_assert!(header.request_api_key == Self::get_api_key());
        debug_assert!(header.request_api_version == version);
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        header.serialize(0, bytes);
        if version >= 0 {
            self.owners.serialize(version, bytes);
        }
    }
}

impl Default for DescribeDelegationTokenRequest {
    fn default() -> Self {
        Self {
            owners: Default::default(),
        }
    }
}

impl ToBytes for DescribeDelegationTokenOwner {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) {
        if version >= 0 {
            self.principal_type.serialize(version, bytes);
        }
        if version >= 0 {
            self.principal_name.serialize(version, bytes);
        }
    }
}

impl Default for DescribeDelegationTokenOwner {
    fn default() -> Self {
        Self {
            principal_type: Default::default(),
            principal_name: Default::default(),
        }
    }
}
