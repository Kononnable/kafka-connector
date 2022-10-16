#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    DescribeCluster = 60,
    DescribeProducers = 61,
    DescribeTransactions = 65,
    ListTransactions = 66,
    AllocateProducerIds = 67,
}
impl ApiNumbers {
    pub fn from_i16(input: i16) -> ApiNumbers {
        match input {
            0 => ApiNumbers::Produce,
            1 => ApiNumbers::Fetch,
            2 => ApiNumbers::ListOffsets,
            3 => ApiNumbers::Metadata,
            4 => ApiNumbers::LeaderAndIsr,
            5 => ApiNumbers::StopReplica,
            6 => ApiNumbers::UpdateMetadata,
            7 => ApiNumbers::ControlledShutdown,
            8 => ApiNumbers::OffsetCommit,
            9 => ApiNumbers::OffsetFetch,
            10 => ApiNumbers::FindCoordinator,
            11 => ApiNumbers::JoinGroup,
            12 => ApiNumbers::Heartbeat,
            13 => ApiNumbers::LeaveGroup,
            14 => ApiNumbers::SyncGroup,
            15 => ApiNumbers::DescribeGroups,
            16 => ApiNumbers::ListGroups,
            17 => ApiNumbers::SaslHandshake,
            18 => ApiNumbers::ApiVersions,
            19 => ApiNumbers::CreateTopics,
            20 => ApiNumbers::DeleteTopics,
            21 => ApiNumbers::DeleteRecords,
            22 => ApiNumbers::InitProducerId,
            23 => ApiNumbers::OffsetForLeaderEpoch,
            24 => ApiNumbers::AddPartitionsToTxn,
            25 => ApiNumbers::AddOffsetsToTxn,
            26 => ApiNumbers::EndTxn,
            27 => ApiNumbers::WriteTxnMarkers,
            28 => ApiNumbers::TxnOffsetCommit,
            29 => ApiNumbers::DescribeAcls,
            30 => ApiNumbers::CreateAcls,
            31 => ApiNumbers::DeleteAcls,
            32 => ApiNumbers::DescribeConfigs,
            33 => ApiNumbers::AlterConfigs,
            34 => ApiNumbers::AlterReplicaLogDirs,
            35 => ApiNumbers::DescribeLogDirs,
            36 => ApiNumbers::SaslAuthenticate,
            37 => ApiNumbers::CreatePartitions,
            38 => ApiNumbers::CreateDelegationToken,
            39 => ApiNumbers::RenewDelegationToken,
            40 => ApiNumbers::ExpireDelegationToken,
            41 => ApiNumbers::DescribeDelegationToken,
            42 => ApiNumbers::DeleteGroups,
            43 => ApiNumbers::ElectLeaders,
            44 => ApiNumbers::IncrementalAlterConfigs,
            45 => ApiNumbers::AlterPartitionReassignments,
            46 => ApiNumbers::ListPartitionReassignments,
            47 => ApiNumbers::OffsetDelete,
            48 => ApiNumbers::DescribeClientQuotas,
            49 => ApiNumbers::AlterClientQuotas,
            50 => ApiNumbers::DescribeUserScramCredentials,
            51 => ApiNumbers::AlterUserScramCredentials,
            56 => ApiNumbers::AlterIsr,
            57 => ApiNumbers::UpdateFeatures,
            60 => ApiNumbers::DescribeCluster,
            61 => ApiNumbers::DescribeProducers,
            65 => ApiNumbers::DescribeTransactions,
            66 => ApiNumbers::ListTransactions,
            67 => ApiNumbers::AllocateProducerIds,
            _ => unreachable!("Unknown api number"),
        }
    }
}