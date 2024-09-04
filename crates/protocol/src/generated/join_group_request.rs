use super::super::prelude::*;

/// Version 1 adds RebalanceTimeoutMs.
/// Version 2 and 3 are the same as version 1.
/// Starting from version 4, the client needs to issue a second request to join group
/// with assigned id.
/// Note: if RebalanceTimeoutMs is not present, SessionTimeoutMs should be
/// used instead.  The default of -1 here is just intended as a placeholder.
#[derive(Clone, Debug, PartialEq)]
pub struct JoinGroupRequest {
    /// The group identifier.
    pub group_id: String,

    /// The coordinator considers the consumer dead if it receives no heartbeat after this timeout in milliseconds.
    pub session_timeout_ms: i32,

    /// The maximum time in milliseconds that the coordinator will wait for each member to rejoin when rebalancing the group.
    pub rebalance_timeout_ms: i32,

    /// The member id assigned by the group coordinator.
    pub member_id: String,

    /// The unique name the for class of protocols implemented by the group we want to join.
    pub protocol_type: String,

    /// The list of protocols that the member supports.
    pub protocols: IndexMap<JoinGroupRequestProtocolKey, JoinGroupRequestProtocol>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Hash)]
pub struct JoinGroupRequestProtocolKey {
    /// The protocol name.
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct JoinGroupRequestProtocol {
    /// The protocol metadata.
    pub metadata: Vec<u8>,
}

impl ApiRequest for JoinGroupRequest {
    type Response = super::join_group_response::JoinGroupResponse;

    fn get_api_key() -> ApiKey {
        ApiKey(11)
    }

    fn get_min_supported_version() -> ApiVersion {
        ApiVersion(0)
    }

    fn get_max_supported_version() -> ApiVersion {
        ApiVersion(4)
    }

    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        debug_assert!(version >= Self::get_min_supported_version());
        debug_assert!(version <= Self::get_max_supported_version());
        self.validate_fields(version)?;
        self.group_id.serialize(version, _bytes)?;
        self.session_timeout_ms.serialize(version, _bytes)?;
        if version >= ApiVersion(1) {
            self.rebalance_timeout_ms.serialize(version, _bytes)?;
        }
        self.member_id.serialize(version, _bytes)?;
        self.protocol_type.serialize(version, _bytes)?;
        self.protocols.serialize(version, _bytes)?;
        Ok(())
    }

    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let group_id = String::deserialize(version, bytes);
        let session_timeout_ms = i32::deserialize(version, bytes);
        let rebalance_timeout_ms = if version >= ApiVersion(1) {
            i32::deserialize(version, bytes)
        } else {
            Default::default()
        };
        let member_id = String::deserialize(version, bytes);
        let protocol_type = String::deserialize(version, bytes);
        let protocols =
            IndexMap::<JoinGroupRequestProtocolKey, JoinGroupRequestProtocol>::deserialize(
                version, bytes,
            );
        JoinGroupRequest {
            group_id,
            session_timeout_ms,
            rebalance_timeout_ms,
            member_id,
            protocol_type,
            protocols,
        }
    }
}

impl JoinGroupRequest {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl Default for JoinGroupRequest {
    fn default() -> Self {
        Self {
            group_id: Default::default(),
            session_timeout_ms: Default::default(),
            rebalance_timeout_ms: -1,
            member_id: Default::default(),
            protocol_type: Default::default(),
            protocols: Default::default(),
        }
    }
}

impl ToBytes for JoinGroupRequestProtocolKey {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, _bytes)?;
        Ok(())
    }
}

impl JoinGroupRequestProtocolKey {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for JoinGroupRequestProtocolKey {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let name = String::deserialize(version, bytes);
        JoinGroupRequestProtocolKey { name }
    }
}

impl ToBytes for JoinGroupRequestProtocol {
    fn serialize(
        &self,
        version: ApiVersion,
        _bytes: &mut BytesMut,
    ) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.metadata.serialize(version, _bytes)?;
        Ok(())
    }
}

impl JoinGroupRequestProtocol {
    fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
        Ok(())
    }
}

impl FromBytes for JoinGroupRequestProtocol {
    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
        let metadata = Vec::<u8>::deserialize(version, bytes);
        JoinGroupRequestProtocol { metadata }
    }
}
