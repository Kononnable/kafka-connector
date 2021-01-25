pub mod header;

pub mod add_offsets_to_txn;
pub mod add_partitions_to_txn;
pub mod alter_client_quotas;
pub mod alter_configs;
pub mod alter_isr;
pub mod alter_partition_reassignments;
pub mod alter_replica_log_dirs;
pub mod alter_user_scram_credentials;
pub mod api_versions;
pub mod controlled_shutdown;
pub mod create_acls;
pub mod create_delegation_token;
pub mod create_partitions;
pub mod create_topics;
pub mod delete_acls;
pub mod delete_groups;
pub mod delete_records;
pub mod delete_topics;
pub mod describe_acls;
pub mod describe_client_quotas;
pub mod describe_configs;
pub mod describe_delegation_token;
pub mod describe_groups;
pub mod describe_log_dirs;
pub mod describe_user_scram_credentials;
pub mod elect_leaders;
pub mod end_txn;
pub mod expire_delegation_token;
pub mod fetch;
pub mod find_coordinator;
pub mod heartbeat;
pub mod incremental_alter_configs;
pub mod init_producer_id;
pub mod join_group;
pub mod leader_and_isr;
pub mod leave_group;
pub mod list_groups;
pub mod list_offsets;
pub mod list_partition_reassignments;
pub mod metadata;
pub mod offset_commit;
pub mod offset_delete;
pub mod offset_fetch;
pub mod offset_for_leader_epoch;
pub mod produce;
pub mod renew_delegation_token;
pub mod sasl_authenticate;
pub mod sasl_handshake;
pub mod stop_replica;
pub mod sync_group;
pub mod txn_offset_commit;
pub mod update_features;
pub mod update_metadata;
pub mod write_txn_markers;

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
    CreateTopics = 19,
    DeleteTopics = 20,
    DeleteRecords = 21,
    InitProducerId = 22,
    OffsetForLeaderEpoch = 23,
    AddPartitionsToTxn = 24,
    AddOffsetsToTxn = 25,
    EndTxn = 26,
    WriteTxnMarkers = 27,
    TxnOffsetCommit = 28,
    DescribeAcls = 29,
    CreateAcls = 30,
    DeleteAcls = 31,
    DescribeConfigs = 32,
    AlterConfigs = 33,
    AlterReplicaLogDirs = 34,
    DescribeLogDirs = 35,
    SaslAuthenticate = 36,
    CreatePartitions = 37,
    CreateDelegationToken = 38,
    RenewDelegationToken = 39,
    ExpireDelegationToken = 40,
    DescribeDelegationToken = 41,
    DeleteGroups = 42,
    ElectLeaders = 43,
    IncrementalAlterConfigs = 44,
    AlterPartitionReassignments = 45,
    ListPartitionReassignments = 46,
    OffsetDelete = 47,
    DescribeClientQuotas = 48,
    AlterClientQuotas = 49,
    DescribeUserScramCredentials = 50,
    AlterUserScramCredentials = 51,
    AlterIsr = 56,
    UpdateFeatures = 57,
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

    pub type Boolean = bool;
    pub type Bytes = Vec<u8>;
    pub type Int8 = i8;
    pub type Int16 = i16;
    pub type Int32 = i32;
    pub type Int64 = i64;
    pub type Float64 = f64;

    // TODO:
    pub type CompactString = String;
    pub type NullableString = String;
    pub type CompactNullableString = String;
    pub type CompactRecords = i64;
    pub type Records = i64;
    pub type CompactBytes = Vec<u8>;
}
