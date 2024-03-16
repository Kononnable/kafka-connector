use super::super::prelude::*;

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
    pub protocols: BTreeMap<JoinGroupRequestProtocolKey, JoinGroupRequestProtocol>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq, Ord, PartialOrd)]
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

    fn get_api_key() -> i16 {
        11
    }

    fn get_min_supported_version() -> i16 {
        0
    }

    fn get_max_supported_version() -> i16 {
        4
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
        self.group_id.serialize(version, bytes)?;
        self.session_timeout_ms.serialize(version, bytes)?;
        if version >= 1 {
            self.rebalance_timeout_ms.serialize(version, bytes)?;
        }
        self.member_id.serialize(version, bytes)?;
        self.protocol_type.serialize(version, bytes)?;
        self.protocols.serialize(version, bytes)?;
        Ok(())
    }
}

impl JoinGroupRequest {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.group_id != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "group_id",
                _version,
                "JoinGroupRequest",
            ));
        }
        if self.session_timeout_ms != i32::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "session_timeout_ms",
                _version,
                "JoinGroupRequest",
            ));
        }
        if self.member_id != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "member_id",
                _version,
                "JoinGroupRequest",
            ));
        }
        if self.protocol_type != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "protocol_type",
                _version,
                "JoinGroupRequest",
            ));
        }
        if self.protocols
            != BTreeMap::<JoinGroupRequestProtocolKey, JoinGroupRequestProtocol>::default()
        {
            return Err(SerializationError::NonIgnorableFieldSet(
                "protocols",
                _version,
                "JoinGroupRequest",
            ));
        }
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
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.name.serialize(version, bytes)?;
        Ok(())
    }
}

impl JoinGroupRequestProtocolKey {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.name != String::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "name",
                _version,
                "JoinGroupRequestProtocolKey",
            ));
        }
        Ok(())
    }
}

impl ToBytes for JoinGroupRequestProtocol {
    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(), SerializationError> {
        self.validate_fields(version)?;
        self.metadata.serialize(version, bytes)?;
        Ok(())
    }
}

impl JoinGroupRequestProtocol {
    fn validate_fields(&self, _version: i16) -> Result<(), SerializationError> {
        if self.metadata != Vec::<u8>::default() {
            return Err(SerializationError::NonIgnorableFieldSet(
                "metadata",
                _version,
                "JoinGroupRequestProtocol",
            ));
        }
        Ok(())
    }
}
