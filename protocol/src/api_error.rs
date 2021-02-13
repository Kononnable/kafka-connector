use core::panic;

use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, DeriveError)]
pub enum ApiError {
    #[error("The server experienced an unexpected error when processing the request.")]
    UnknownServerError,
    #[error("The requested offset is not within the range of offsets maintained by the server.")]
    OffsetOutOfRange,
    #[error("This message has failed its CRC checksum, exceeds the valid size, has a null key for a compacted topic, or is otherwise corrupt.")]
    CorruptMessage,
    #[error("This server does not host this topic-partition.")]
    UnknownTopicOrPartition,
    #[error("The requested fetch size is invalid.")]
    InvalidFetchSize,
    #[error("There is no leader for this topic-partition as we are in the middle of a leadership election.")]
    LeaderNotAvailable,
    #[error("For requests intended only for the leader, this error indicates that the broker is not the current leader. For requests intended for any replica, this error indicates that the broker is not a replica of the topic partition.")]
    NotLeaderOrFollower,
    #[error("The request timed out.")]
    RequestTimedOut,
    #[error("The broker is not available.")]
    BrokerNotAvailable,
    #[error("The replica is not available for the requested topic-partition. Produce/Fetch requests and other requests intended only for the leader or follower return NOTLEADERORFOLLOWER if the broker is not a replica of the topic-partition.")]
    ReplicaNotAvailable,
    #[error(
        "The request included a message larger than the max message size the server will accept."
    )]
    MessageTooLarge,
    #[error("The controller moved to another broker.")]
    StaleControllerEpoch,
    #[error("The metadata field of the offset request was too large.")]
    OffsetMetadataTooLarge,
    #[error("The server disconnected before a response was received.")]
    NetworkException,
    #[error("The coordinator is loading and hence can't process requests.")]
    CoordinatorLoadInProgress,
    #[error("The coordinator is not available.")]
    CoordinatorNotAvailable,
    #[error("This is not the correct coordinator.")]
    NotCoordinator,
    #[error("The request attempted to perform an operation on an invalid topic.")]
    InvalidTopicException,
    #[error(
        "The request included message batch larger than the configured segment size on the server."
    )]
    RecordListTooLarge,
    #[error("Messages are rejected since there are fewer in-sync replicas than required.")]
    NotEnoughReplicas,
    #[error("Messages are written to the log, but to fewer in-sync replicas than required.")]
    NotEnoughReplicasAfterAppend,
    #[error("Produce request specified an invalid value for required acks.")]
    InvalidRequiredAcks,
    #[error("Specified group generation id is not valid.")]
    IllegalGeneration,
    #[error("The group member's supported protocols are incompatible with those of existing members or first group member tried to join with empty protocol type or empty protocol list.")]
    InconsistentGroupProtocol,
    #[error("The configured groupId is invalid.")]
    InvalidGroupId,
    #[error("The coordinator is not aware of this member.")]
    UnknownMemberId,
    #[error("The session timeout is not within the range allowed by the broker (as configured by group.min.session.timeout.ms and group.max.session.timeout.ms).")]
    InvalidSessionTimeout,
    #[error("The group is rebalancing, so a rejoin is needed.")]
    RebalanceInProgress,
    #[error("The committing offset data size is not valid.")]
    InvalidCommitOffsetSize,
    #[error("Topic authorization failed.")]
    TopicAuthorizationFailed,
    #[error("Group authorization failed.")]
    GroupAuthorizationFailed,
    #[error("Cluster authorization failed.")]
    ClusterAuthorizationFailed,
    #[error("The timestamp of the message is out of acceptable range.")]
    InvalidTimestamp,
    #[error("The broker does not support the requested SASL mechanism.")]
    UnsupportedSaslMechanism,
    #[error("Request is not valid given the current SASL state.")]
    IllegalSaslState,
    #[error("The version of API is not supported.")]
    UnsupportedVersion,
    #[error("Topic with this name already exists.")]
    TopicAlreadyExists,
    #[error("Number of partitions is below 1.")]
    InvalidPartitions,
    #[error("Replication factor is below 1 or larger than the number of available brokers.")]
    InvalidReplicationFactor,
    #[error("Replica assignment is invalid.")]
    InvalidReplicaAssignment,
    #[error("Configuration is invalid.")]
    InvalidConfig,
    #[error("This is not the correct controller for this cluster.")]
    NotController,
    #[error("This most likely occurs because of a request being malformed by the client library or the message was sent to an incompatible broker. See the broker logs for more details.")]
    InvalidRequest,
    #[error("The message format version on the broker does not support the request.")]
    UnsupportedForMessageFormat,
    #[error("Request parameters do not satisfy the configured policy.")]
    PolicyViolation,
    #[error("The broker received an out of order sequence number.")]
    OutOfOrderSequenceNumber,
    #[error("The broker received a duplicate sequence number.")]
    DuplicateSequenceNumber,
    #[error("Producer attempted to produce with an old epoch.")]
    InvalidProducerEpoch,
    #[error("The producer attempted a transactional operation in an invalid state.")]
    InvalidTxnState,
    #[error("The producer attempted to use a producer id which is not currently assigned to its transactional id.")]
    InvalidProducerIdMapping,
    #[error("The transaction timeout is larger than the maximum value allowed by the broker (as configured by transaction.max.timeout.ms).")]
    InvalidTransactionTimeout,
    #[error("The producer attempted to update a transaction while another concurrent operation on the same transaction was ongoing.")]
    ConcurrentTransactions,
    #[error("Indicates that the transaction coordinator sending a WriteTxnMarker is no longer the current coordinator for a given producer.")]
    TransactionCoordinatorFenced,
    #[error("Transactional Id authorization failed.")]
    TransactionalIdAuthorizationFailed,
    #[error("Security features are disabled.")]
    SecurityDisabled,
    #[error("The broker did not attempt to execute this operation. This may happen for batched RPCs where some operations in the batch failed, causing the broker to respond without trying the rest.")]
    OperationNotAttempted,
    #[error("Disk error when trying to access log file on the disk.")]
    KafkaStorageError,
    #[error("The user-specified log directory is not found in the broker config.")]
    LogDirNotFound,
    #[error("SASL Authentication failed.")]
    SaslAuthenticationFailed,
    #[error("This exception is raised by the broker if it could not locate the producer metadata associated with the producerId in question. This could happen if, for instance, the producer's records were deleted because their retention time had elapsed. Once the last records of the producerId are removed, the producer's metadata is removed from the broker, and future appends by the producer will return this exception.")]
    UnknownProducerId,
    #[error("A partition reassignment is in progress.")]
    ReassignmentInProgress,
    #[error("Delegation Token feature is not enabled.")]
    DelegationTokenAuthDisabled,
    #[error("Delegation Token is not found on server.")]
    DelegationTokenNotFound,
    #[error("Specified Principal is not valid Owner/Renewer.")]
    DelegationTokenOwnerMismatch,
    #[error("Delegation Token requests are not allowed on PLAINTEXT/1-way SSL channels and on delegation token authenticated channels.")]
    DelegationTokenRequestNotAllowed,
    #[error("Delegation Token authorization failed.")]
    DelegationTokenAuthorizationFailed,
    #[error("Delegation Token is expired.")]
    DelegationTokenExpired,
    #[error("Supplied principalType is not supported.")]
    InvalidPrincipalType,
    #[error("The group is not empty.")]
    NonEmptyGroup,
    #[error("The group id does not exist.")]
    GroupIdNotFound,
    #[error("The fetch session ID was not found.")]
    FetchSessionIdNotFound,
    #[error("The fetch session epoch is invalid.")]
    InvalidFetchSessionEpoch,
    #[error("There is no listener on the leader broker that matches the listener on which metadata request was processed.")]
    ListenerNotFound,
    #[error("Topic deletion is disabled.")]
    TopicDeletionDisabled,
    #[error("The leader epoch in the request is older than the epoch on the broker.")]
    FencedLeaderEpoch,
    #[error("The leader epoch in the request is newer than the epoch on the broker.")]
    UnknownLeaderEpoch,
    #[error("The requesting client does not support the compression type of given partition.")]
    UnsupportedCompressionType,
    #[error("Broker epoch has changed.")]
    StaleBrokerEpoch,
    #[error("The leader high watermark has not caught up from a recent leader election so the offsets cannot be guaranteed to be monotonically increasing.")]
    OffsetNotAvailable,
    #[error("The group member needs to have a valid member id before actually entering a consumer group.")]
    MemberIdRequired,
    #[error("The preferred leader was not available.")]
    PreferredLeaderNotAvailable,
    #[error("The consumer group has reached its max size.")]
    GroupMaxSizeReached,
    #[error("The broker rejected this static consumer since another consumer with the same group.instance.id has registered with a different member.id.")]
    FencedInstanceId,
    #[error("Eligible topic partition leaders are not available.")]
    EligibleLeadersNotAvailable,
    #[error("Leader election not needed for topic partition.")]
    ElectionNotNeeded,
    #[error("No partition reassignment is in progress.")]
    NoReassignmentInProgress,
    #[error("Deleting offsets of a topic is forbidden while the consumer group is actively subscribed to it.")]
    GroupSubscribedToTopic,
    #[error("This record has failed the validation on broker and hence will be rejected.")]
    InvalidRecord,
    #[error("There are unstable offsets that need to be cleared.")]
    UnstableOffsetCommit,
    #[error("The throttling quota has been exceeded.")]
    ThrottlingQuotaExceeded,
    #[error(
        "There is a newer producer with the same transactionalId which fences the current one."
    )]
    ProducerFenced,
    #[error("A request illegally referred to a resource that does not exist.")]
    ResourceNotFound,
    #[error("A request illegally referred to the same resource twice.")]
    DuplicateResource,
    #[error("Requested credential would not meet criteria for acceptability.")]
    UnacceptableCredential,
    #[error("Indicates that the either the sender or recipient of a voter-only request is not one of the expected voters")]
    InconsistentVoterSet,
    #[error("The given update version was invalid.")]
    InvalidUpdateVersion,
    #[error("Unable to update finalized features due to an unexpected server error.")]
    FeatureUpdateFailed,
    #[error("Unknown error code returned from kafka {0}.")]
    UnknownErrorCode(i32),
}
impl ApiError {
    pub fn from_i32(i: i32) -> ApiError {
        match i {
            -1 => ApiError::UnknownServerError,
            1 => ApiError::OffsetOutOfRange,
            2 => ApiError::CorruptMessage,
            3 => ApiError::UnknownTopicOrPartition,
            4 => ApiError::InvalidFetchSize,
            5 => ApiError::LeaderNotAvailable,
            6 => ApiError::NotLeaderOrFollower,
            7 => ApiError::RequestTimedOut,
            8 => ApiError::BrokerNotAvailable,
            9 => ApiError::ReplicaNotAvailable,
            10 => ApiError::MessageTooLarge,
            11 => ApiError::StaleControllerEpoch,
            12 => ApiError::OffsetMetadataTooLarge,
            13 => ApiError::NetworkException,
            14 => ApiError::CoordinatorLoadInProgress,
            15 => ApiError::CoordinatorNotAvailable,
            16 => ApiError::NotCoordinator,
            17 => ApiError::InvalidTopicException,
            18 => ApiError::RecordListTooLarge,
            19 => ApiError::NotEnoughReplicas,
            20 => ApiError::NotEnoughReplicasAfterAppend,
            21 => ApiError::InvalidRequiredAcks,
            22 => ApiError::IllegalGeneration,
            23 => ApiError::InconsistentGroupProtocol,
            24 => ApiError::InvalidGroupId,
            25 => ApiError::UnknownMemberId,
            26 => ApiError::InvalidSessionTimeout,
            27 => ApiError::RebalanceInProgress,
            28 => ApiError::InvalidCommitOffsetSize,
            29 => ApiError::TopicAuthorizationFailed,
            30 => ApiError::GroupAuthorizationFailed,
            31 => ApiError::ClusterAuthorizationFailed,
            32 => ApiError::InvalidTimestamp,
            33 => ApiError::UnsupportedSaslMechanism,
            34 => ApiError::IllegalSaslState,
            35 => ApiError::UnsupportedVersion,
            36 => ApiError::TopicAlreadyExists,
            37 => ApiError::InvalidPartitions,
            38 => ApiError::InvalidReplicationFactor,
            39 => ApiError::InvalidReplicaAssignment,
            40 => ApiError::InvalidConfig,
            41 => ApiError::NotController,
            42 => ApiError::InvalidRequest,
            43 => ApiError::UnsupportedForMessageFormat,
            44 => ApiError::PolicyViolation,
            45 => ApiError::OutOfOrderSequenceNumber,
            46 => ApiError::DuplicateSequenceNumber,
            47 => ApiError::InvalidProducerEpoch,
            48 => ApiError::InvalidTxnState,
            49 => ApiError::InvalidProducerIdMapping,
            50 => ApiError::InvalidTransactionTimeout,
            51 => ApiError::ConcurrentTransactions,
            52 => ApiError::TransactionCoordinatorFenced,
            53 => ApiError::TransactionalIdAuthorizationFailed,
            54 => ApiError::SecurityDisabled,
            55 => ApiError::OperationNotAttempted,
            56 => ApiError::KafkaStorageError,
            57 => ApiError::LogDirNotFound,
            58 => ApiError::SaslAuthenticationFailed,
            59 => ApiError::UnknownProducerId,
            60 => ApiError::ReassignmentInProgress,
            61 => ApiError::DelegationTokenAuthDisabled,
            62 => ApiError::DelegationTokenNotFound,
            63 => ApiError::DelegationTokenOwnerMismatch,
            64 => ApiError::DelegationTokenRequestNotAllowed,
            65 => ApiError::DelegationTokenAuthorizationFailed,
            66 => ApiError::DelegationTokenExpired,
            67 => ApiError::InvalidPrincipalType,
            68 => ApiError::NonEmptyGroup,
            69 => ApiError::GroupIdNotFound,
            70 => ApiError::FetchSessionIdNotFound,
            71 => ApiError::InvalidFetchSessionEpoch,
            72 => ApiError::ListenerNotFound,
            73 => ApiError::TopicDeletionDisabled,
            74 => ApiError::FencedLeaderEpoch,
            75 => ApiError::UnknownLeaderEpoch,
            76 => ApiError::UnsupportedCompressionType,
            77 => ApiError::StaleBrokerEpoch,
            78 => ApiError::OffsetNotAvailable,
            79 => ApiError::MemberIdRequired,
            80 => ApiError::PreferredLeaderNotAvailable,
            81 => ApiError::GroupMaxSizeReached,
            82 => ApiError::FencedInstanceId,
            83 => ApiError::EligibleLeadersNotAvailable,
            84 => ApiError::ElectionNotNeeded,
            85 => ApiError::NoReassignmentInProgress,
            86 => ApiError::GroupSubscribedToTopic,
            87 => ApiError::InvalidRecord,
            88 => ApiError::UnstableOffsetCommit,
            89 => ApiError::ThrottlingQuotaExceeded,
            90 => ApiError::ProducerFenced,
            91 => ApiError::ResourceNotFound,
            92 => ApiError::DuplicateResource,
            93 => ApiError::UnacceptableCredential,
            94 => ApiError::InconsistentVoterSet,
            95 => ApiError::InvalidUpdateVersion,
            96 => ApiError::FeatureUpdateFailed,
            0 => panic!("Kafka error code 0 is not an error"),
            x => ApiError::UnknownErrorCode(x),
        }
    }
    pub fn is_retriable(value: ApiError) -> bool {
        match value {
            ApiError::CorruptMessage
            | ApiError::UnknownTopicOrPartition
            | ApiError::LeaderNotAvailable
            | ApiError::NotLeaderOrFollower
            | ApiError::RequestTimedOut
            | ApiError::ReplicaNotAvailable
            | ApiError::NetworkException
            | ApiError::CoordinatorLoadInProgress
            | ApiError::CoordinatorNotAvailable
            | ApiError::NotCoordinator
            | ApiError::NotEnoughReplicas
            | ApiError::NotEnoughReplicasAfterAppend
            | ApiError::NotController
            | ApiError::KafkaStorageError
            | ApiError::FetchSessionIdNotFound
            | ApiError::InvalidFetchSessionEpoch
            | ApiError::ListenerNotFound
            | ApiError::TopicDeletionDisabled
            | ApiError::FencedLeaderEpoch
            | ApiError::UnknownLeaderEpoch
            | ApiError::OffsetNotAvailable
            | ApiError::PreferredLeaderNotAvailable
            | ApiError::EligibleLeadersNotAvailable
            | ApiError::ElectionNotNeeded
            | ApiError::UnstableOffsetCommit
            | ApiError::ThrottlingQuotaExceeded => true,
            ApiError::UnknownServerError
            | ApiError::OffsetOutOfRange
            | ApiError::InvalidFetchSize
            | ApiError::BrokerNotAvailable
            | ApiError::MessageTooLarge
            | ApiError::StaleControllerEpoch
            | ApiError::OffsetMetadataTooLarge
            | ApiError::InvalidTopicException
            | ApiError::RecordListTooLarge
            | ApiError::InvalidRequiredAcks
            | ApiError::IllegalGeneration
            | ApiError::InconsistentGroupProtocol
            | ApiError::InvalidGroupId
            | ApiError::UnknownMemberId
            | ApiError::InvalidSessionTimeout
            | ApiError::RebalanceInProgress
            | ApiError::InvalidCommitOffsetSize
            | ApiError::TopicAuthorizationFailed
            | ApiError::GroupAuthorizationFailed
            | ApiError::ClusterAuthorizationFailed
            | ApiError::InvalidTimestamp
            | ApiError::UnsupportedSaslMechanism
            | ApiError::IllegalSaslState
            | ApiError::UnsupportedVersion
            | ApiError::TopicAlreadyExists
            | ApiError::InvalidPartitions
            | ApiError::InvalidReplicationFactor
            | ApiError::InvalidReplicaAssignment
            | ApiError::InvalidConfig
            | ApiError::InvalidRequest
            | ApiError::UnsupportedForMessageFormat
            | ApiError::PolicyViolation
            | ApiError::OutOfOrderSequenceNumber
            | ApiError::DuplicateSequenceNumber
            | ApiError::InvalidProducerEpoch
            | ApiError::InvalidTxnState
            | ApiError::InvalidProducerIdMapping
            | ApiError::InvalidTransactionTimeout
            | ApiError::ConcurrentTransactions
            | ApiError::TransactionCoordinatorFenced
            | ApiError::TransactionalIdAuthorizationFailed
            | ApiError::SecurityDisabled
            | ApiError::OperationNotAttempted
            | ApiError::LogDirNotFound
            | ApiError::SaslAuthenticationFailed
            | ApiError::UnknownProducerId
            | ApiError::ReassignmentInProgress
            | ApiError::DelegationTokenAuthDisabled
            | ApiError::DelegationTokenNotFound
            | ApiError::DelegationTokenOwnerMismatch
            | ApiError::DelegationTokenRequestNotAllowed
            | ApiError::DelegationTokenAuthorizationFailed
            | ApiError::DelegationTokenExpired
            | ApiError::InvalidPrincipalType
            | ApiError::NonEmptyGroup
            | ApiError::GroupIdNotFound
            | ApiError::UnsupportedCompressionType
            | ApiError::StaleBrokerEpoch
            | ApiError::MemberIdRequired
            | ApiError::GroupMaxSizeReached
            | ApiError::FencedInstanceId
            | ApiError::NoReassignmentInProgress
            | ApiError::GroupSubscribedToTopic
            | ApiError::InvalidRecord
            | ApiError::ProducerFenced
            | ApiError::ResourceNotFound
            | ApiError::DuplicateResource
            | ApiError::UnacceptableCredential
            | ApiError::InconsistentVoterSet
            | ApiError::InvalidUpdateVersion
            | ApiError::FeatureUpdateFailed
            | ApiError::UnknownErrorCode(_) => false,
        }
    }
}
// NONE	0	False

// UNKNOWNSERVERERROR	-1	False	The server experienced an unexpected error when processing the request.
// OFFSETOUTOFRANGE	1	False	The requested offset is not within the range of offsets maintained by the server.
// CORRUPTMESSAGE	2	True	This message has failed its CRC checksum, exceeds the valid size, has a null key for a compacted topic, or is otherwise corrupt.
// UNKNOWNTOPICORPARTITION	3	True	This server does not host this topic-partition.
// INVALIDFETCHSIZE	4	False	The requested fetch size is invalid.
// LEADERNOTAVAILABLE	5	True	There is no leader for this topic-partition as we are in the middle of a leadership election.
// NOTLEADERORFOLLOWER	6	True	For requests intended only for the leader, this error indicates that the broker is not the current leader. For requests intended for any replica, this error indicates that the broker is not a replica of the topic partition.
// REQUESTTIMEDOUT	7	True	The request timed out.
// BROKERNOTAVAILABLE	8	False	The broker is not available.
// REPLICANOTAVAILABLE	9	True	The replica is not available for the requested topic-partition. Produce/Fetch requests and other requests intended only for the leader or follower return NOTLEADERORFOLLOWER if the broker is not a replica of the topic-partition.
// MESSAGETOOLARGE	10	False	The request included a message larger than the max message size the server will accept.
// STALECONTROLLEREPOCH	11	False	The controller moved to another broker.
// OFFSETMETADATATOOLARGE	12	False	The metadata field of the offset request was too large.
// NETWORKEXCEPTION	13	True	The server disconnected before a response was received.
// COORDINATORLOADINPROGRESS	14	True	The coordinator is loading and hence can't process requests.
// COORDINATORNOTAVAILABLE	15	True	The coordinator is not available.
// NOTCOORDINATOR	16	True	This is not the correct coordinator.
// INVALIDTOPICEXCEPTION	17	False	The request attempted to perform an operation on an invalid topic.
// RECORDLISTTOOLARGE	18	False	The request included message batch larger than the configured segment size on the server.
// NOTENOUGHREPLICAS	19	True	Messages are rejected since there are fewer in-sync replicas than required.
// NOTENOUGHREPLICASAFTERAPPEND	20	True	Messages are written to the log, but to fewer in-sync replicas than required.
// INVALIDREQUIREDACKS	21	False	Produce request specified an invalid value for required acks.
// ILLEGAL_GENERATION	22	False	Specified group generation id is not valid.
// INCONSISTENT_GROUPPROTOCOL	23	False	The group member's supported protocols are incompatible with those of existing members or first group member tried to join with empty protocol type or empty protocol list.
// INVALID_GROUPID	24	False	The configured groupId is invalid.
// UNKNOWNMEMBERID	25	False	The coordinator is not aware of this member.
// INVALIDSESSIONTIMEOUT	26	False	The session timeout is not within the range allowed by the broker (as configured by group.min.session.timeout.ms and group.max.session.timeout.ms).
// REBALANCEINPROGRESS	27	False	The group is rebalancing, so a rejoin is needed.
// INVALIDCOMMITOFFSETSIZE	28	False	The committing offset data size is not valid.
// TOPICAUTHORIZATIONFAILED	29	False	Topic authorization failed.
// GROUPAUTHORIZATIONFAILED	30	False	Group authorization failed.
// CLUSTERAUTHORIZATIONFAILED	31	False	Cluster authorization failed.
// INVALIDTIMESTAMP	32	False	The timestamp of the message is out of acceptable range.
// UNSUPPORTEDSASLMECHANISM	33	False	The broker does not support the requested SASL mechanism.
// ILLEGALSASLSTATE	34	False	Request is not valid given the current SASL state.
// UNSUPPORTEDVERSION	35	False	The version of API is not supported.
// TOPICALREADYEXISTS	36	False	Topic with this name already exists.
// INVALIDPARTITIONS	37	False	Number of partitions is below 1.
// INVALIDREPLICATIONFACTOR	38	False	Replication factor is below 1 or larger than the number of available brokers.
// INVALIDREPLICAASSIGNMENT	39	False	Replica assignment is invalid.
// INVALIDCONFIG	40	False	Configuration is invalid.
// NOTCONTROLLER	41	True	This is not the correct controller for this cluster.
// INVALIDREQUEST	42	False	This most likely occurs because of a request being malformed by the client library or the message was sent to an incompatible broker. See the broker logs for more details.
// UNSUPPORTEDFORMESSAGEFORMAT	43	False	The message format version on the broker does not support the request.
// POLICYVIOLATION	44	False	Request parameters do not satisfy the configured policy.
// OUTOFORDERSEQUENCENUMBER	45	False	The broker received an out of order sequence number.
// DUPLICATESEQUENCENUMBER	46	False	The broker received a duplicate sequence number.
// INVALIDPRODUCEREPOCH	47	False	Producer attempted to produce with an old epoch.
// INVALIDTXNSTATE	48	False	The producer attempted a transactional operation in an invalid state.
// INVALIDPRODUCERIDMAPPING	49	False	The producer attempted to use a producer id which is not currently assigned to its transactional id.
// INVALIDTRANSACTIONTIMEOUT	50	False	The transaction timeout is larger than the maximum value allowed by the broker (as configured by transaction.max.timeout.ms).
// CONCURRENTTRANSACTIONS	51	False	The producer attempted to update a transaction while another concurrent operation on the same transaction was ongoing.
// TRANSACTIONCOORDINATORFENCED	52	False	Indicates that the transaction coordinator sending a WriteTxnMarker is no longer the current coordinator for a given producer.
// TRANSACTIONALIDAUTHORIZATIONFAILED	53	False	Transactional Id authorization failed.
// SECURITY_DISABLED	54	False	Security features are disabled.
// OPERATIONNOTATTEMPTED	55	False	The broker did not attempt to execute this operation. This may happen for batched RPCs where some operations in the batch failed, causing the broker to respond without trying the rest.
// KAFKASTORAGEERROR	56	True	Disk error when trying to access log file on the disk.
// LOG_DIRNOTFOUND	57	False	The user-specified log directory is not found in the broker config.
// SASLAUTHENTICATIONFAILED	58	False	SASL Authentication failed.
// UNKNOWNPRODUCERID	59	False	This exception is raised by the broker if it could not locate the producer metadata associated with the producerId in question. This could happen if, for instance, the producer's records were deleted because their retention time had elapsed. Once the last records of the producerId are removed, the producer's metadata is removed from the broker, and future appends by the producer will return this exception.
// REASSIGNMENTINPROGRESS	60	False	A partition reassignment is in progress.
// DELEGATIONTOKENAUTH_DISABLED	61	False	Delegation Token feature is not enabled.
// DELEGATIONTOKENNOTFOUND	62	False	Delegation Token is not found on server.
// DELEGATIONTOKENOWNERMISMATCH	63	False	Specified Principal is not valid Owner/Renewer.
// DELEGATIONTOKENREQUESTNOTALLOWED	64	False	Delegation Token requests are not allowed on PLAINTEXT/1-way SSL channels and on delegation token authenticated channels.
// DELEGATIONTOKENAUTHORIZATIONFAILED	65	False	Delegation Token authorization failed.
// DELEGATIONTOKENEXPIRED	66	False	Delegation Token is expired.
// INVALIDPRINCIPALTYPE	67	False	Supplied principalType is not supported.
// NONEMPTY_GROUP	68	False	The group is not empty.
// GROUPIDNOTFOUND	69	False	The group id does not exist.
// FETCHSESSIONIDNOTFOUND	70	True	The fetch session ID was not found.
// INVALIDFETCHSESSIONEPOCH	71	True	The fetch session epoch is invalid.
// LISTENERNOTFOUND	72	True	There is no listener on the leader broker that matches the listener on which metadata request was processed.
// TOPIC_DELETION_DISABLED	73	False	Topic deletion is disabled.
// FENCEDLEADEREPOCH	74	True	The leader epoch in the request is older than the epoch on the broker.
// UNKNOWNLEADEREPOCH	75	True	The leader epoch in the request is newer than the epoch on the broker.
// UNSUPPORTEDCOMPRESSIONTYPE	76	False	The requesting client does not support the compression type of given partition.
// STALEBROKEREPOCH	77	False	Broker epoch has changed.
// OFFSETNOTAVAILABLE	78	True	The leader high watermark has not caught up from a recent leader election so the offsets cannot be guaranteed to be monotonically increasing.
// MEMBERIDREQUIRED	79	False	The group member needs to have a valid member id before actually entering a consumer group.
// PREFERREDLEADERNOTAVAILABLE	80	True	The preferred leader was not available.
// GROUPMAXSIZEREACHED	81	False	The consumer group has reached its max size.
// FENCEDINSTANCEID	82	False	The broker rejected this static consumer since another consumer with the same group.instance.id has registered with a different member.id.
// ELIGIBLELEADERSNOTAVAILABLE	83	True	Eligible topic partition leaders are not available.
// ELECTIONNOTNEEDED	84	True	Leader election not needed for topic partition.
// NOREASSIGNMENTINPROGRESS	85	False	No partition reassignment is in progress.
// GROUPSUBSCRIBEDTOTOPIC	86	False	Deleting offsets of a topic is forbidden while the consumer group is actively subscribed to it.
// INVALIDRECORD	87	False	This record has failed the validation on broker and hence will be rejected.
// UNSTABLEOFFSETCOMMIT	88	True	There are unstable offsets that need to be cleared.
// THROTTLINGQUOTAEXCEEDED	89	True	The throttling quota has been exceeded.
// PRODUCERFENCED	90	False	There is a newer producer with the same transactionalId which fences the current one.
// RESOURCENOTFOUND	91	False	A request illegally referred to a resource that does not exist.
// DUPLICATERESOURCE	92	False	A request illegally referred to the same resource twice.
// UNACCEPTABLECREDENTIAL	93	False	Requested credential would not meet criteria for acceptability.
// INCONSISTENTVOTERSET	94	False	Indicates that the either the sender or recipient of a voter-only request is not one of the expected voters
// INVALIDUPDATEVERSION	95	False	The given update version was invalid.
// FEATUREUPDATEFAILED	96	False	Unable to update finalized features due to an unexpected server error.
