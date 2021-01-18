pub mod header;

pub mod api_versions;
pub mod controlled_shutdown;
pub mod describe_groups;
pub mod fetch;
pub mod find_coordinator;
pub mod heartbeat;
pub mod join_group;
pub mod leader_and_isr;
pub mod leave_group;
pub mod list_groups;
pub mod list_offsets;
pub mod metadata;
pub mod offset_commit;
pub mod offset_fetch;
pub mod produce;
pub mod sasl_handshake;
pub mod stop_replica;
pub mod sync_group;
pub mod update_metadata;

pub enum ApiNumbers {
    Produce = 0,
    Fetch = 1,
    ListOffsets = 2,
    Metadata = 3,
    LeaderAndIsr = 4,
    StopReplica = 5,
    UpdateMetadata = 6,
    ControlledShutdown = 7,
    OffsetCommit = 8,
    OffsetFetch = 9,
    FindCoordinator = 10,
    JoinGroup = 11,
    Heartbeat = 12,
    LeaveGroup = 13,
    SyncGroup = 14,
    DescribeGroups = 15,
    ListGroups = 16,
    SaslHandshake = 17,
    ApiVersions = 18,
    // CreateTopics = 19,
    // DeleteTopics = 20,
    // DeleteRecords = 21,
    // InitProducerId = 22,
    // OffsetForLeaderEpoch = 23,
    // AddPartitionsToTxn = 24,
    // AddOffsetsToTxn = 25,
    // EndTxn = 26,
    // WriteTxnMarkers = 27,
    // TxnOffsetCommit = 28,
    // DescribeAcls = 29,
    // CreateAcls = 30,
    // DeleteAcls = 31,
    // DescribeConfigs = 32,
    // AlterConfigs = 33,
    // AlterReplicaLogDirs = 34,
    // DescribeLogDirs = 35,
    // SaslAuthenticate = 36,
    // CreatePartitions = 37,
    // CreateDelegationToken = 38,
    // RenewDelegationToken = 39,
    // ExpireDelegationToken = 40,
    // DescribeDelegationToken = 41,
    // DeleteGroups = 42,
    // ElectLeaders = 43,
    // IncrementalAlterConfigs = 44,
    // AlterPartitionReassignments = 45,
    // ListPartitionReassignments = 46,
    // OffsetDelete = 47,
    // DescribeClientQuotas = 48,
    // AlterClientQuotas = 49,
    // DescribeUserScramCredentials = 50,
    // AlterUserScramCredentials = 51,
    // AlterIsr = 56,
    // UpdateFeatures = 57,
}

mod prelude {
    pub use super::super::error::Error;
    pub use super::super::from_bytes::FromBytes;
    pub use super::super::optional::Optional;
    pub use super::super::to_bytes::ToBytes;
    pub use bytes::BytesMut;
    pub use kafka_connector_derive::FromBytes;
    pub use kafka_connector_derive::ToBytes;
    pub use std::convert::TryFrom;
    pub use std::convert::TryInto;

    pub type Int8 = i8;
    pub type Int16 = i16;
    pub type Int32 = i32;
    pub type Int64 = i64;

    // TODO:
    pub type CompactString = String;
    pub type NullableString = String;
    pub type CompactNullableString = String;
    pub type Boolean = i64;
    pub type CompactRecords = i64;
    pub type Records = i64;
    pub type CompactBytes = i64;
    pub type Bytes = i64;
}
