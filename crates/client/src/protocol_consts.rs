use crate::protocol_consts::ListOffsetsTimestampType::Specific;

// TODO: Split, move to better location (?)

pub enum IsolationLevel {
    ReadUncommited = 0,
    ReadCommited = 1,
}
impl From<i8> for IsolationLevel {
    fn from(value: i8) -> Self {
        match value {
            0 => IsolationLevel::ReadUncommited,
            1 => IsolationLevel::ReadCommited,
            v => panic!("Unknown Isolation Level value: {}", v),
        }
    }
}
impl From<IsolationLevel> for i8 {
    fn from(value: IsolationLevel) -> Self {
        value as i8
    }
}

pub enum ListOffsetsTimestampType {
    Earliest,
    Latest,
    MaxTimestamp,
    Specific(i64),
}
impl From<i64> for ListOffsetsTimestampType {
    fn from(value: i64) -> Self {
        match value {
            -3 => ListOffsetsTimestampType::MaxTimestamp,
            -2 => ListOffsetsTimestampType::Earliest,
            -1 => ListOffsetsTimestampType::Latest,
            o => {
                assert!(o >= 0, "Unknown ListOffsetsTimestampType value");
                Specific(o)
            }
        }
    }
}
impl From<ListOffsetsTimestampType> for i64 {
    fn from(value: ListOffsetsTimestampType) -> Self {
        match value {
            ListOffsetsTimestampType::MaxTimestamp => -3,
            ListOffsetsTimestampType::Earliest => -2,
            ListOffsetsTimestampType::Latest => -1,
            Specific(o) => o,
        }
    }
}

pub enum FindCoordinatorKeyType {
    Group,
    Transaction,
}
impl From<i8> for FindCoordinatorKeyType {
    fn from(value: i8) -> Self {
        match value {
            0 => FindCoordinatorKeyType::Group,
            1 => FindCoordinatorKeyType::Transaction,
            _ => {
                panic!("Unrecognized FindCoordinatorKeyType value: {value}");
            }
        }
    }
}
impl From<FindCoordinatorKeyType> for i8 {
    fn from(value: FindCoordinatorKeyType) -> Self {
        match value {
            FindCoordinatorKeyType::Group => 0,
            FindCoordinatorKeyType::Transaction => 1,
        }
    }
}

// TODO: Replace with ConsumerProtocolAssignement.json, ConsumerProtocolSubscription.json generated structs
// after protocol crate support newer protocol version
pub mod consumer_protocol_assignment {
    use bytes::BytesMut;
    use kafka_connector_protocol::{ApiVersion, FromBytes, SerializationError, ToBytes};

    /// Assignment part of the Consumer Protocol.
    ///
    /// The current implementation assumes that future versions will not break compatibility. When
    /// it encounters a newer version, it parses it using the current format. This basically means
    /// that new versions cannot remove or reorder any of the existing fields.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ConsumerProtocolAssignment {
        pub assigned_partitions: Vec<TopicPartition>,

        pub user_data: Option<Vec<u8>>,
    }

    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct TopicPartition {
        pub topic: String,

        pub partitions: Vec<i32>,
    }

    impl ConsumerProtocolAssignment {
        pub fn get_min_supported_version() -> ApiVersion {
            ApiVersion(0)
        }

        pub fn get_max_supported_version() -> ApiVersion {
            ApiVersion(1)
        }

        pub fn serialize(
            &self,
            version: ApiVersion,
            _bytes: &mut BytesMut,
        ) -> Result<(), SerializationError> {
            debug_assert!(version >= Self::get_min_supported_version());
            debug_assert!(version <= Self::get_max_supported_version());
            self.validate_fields(version)?;
            self.assigned_partitions.serialize(version, _bytes);
            self.user_data.serialize(version, _bytes);
            Ok(())
        }

        pub fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
            let assigned_partitions = Vec::<TopicPartition>::deserialize(version, bytes);
            let user_data = Option::<Vec<u8>>::deserialize(version, bytes);
            ConsumerProtocolAssignment {
                assigned_partitions,
                user_data,
            }
        }
    }

    impl ConsumerProtocolAssignment {
        fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
            for item in self.assigned_partitions.iter() {
                item.validate_fields(_version)?;
            }
            Ok(())
        }
    }

    impl ToBytes for TopicPartition {
        fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
            self.topic.serialize(version, _bytes);
            self.partitions.serialize(version, _bytes);
        }
    }

    impl TopicPartition {
        fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
            Ok(())
        }
    }

    impl FromBytes for TopicPartition {
        fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
            let topic = String::deserialize(version, bytes);
            let partitions = Vec::<i32>::deserialize(version, bytes);
            TopicPartition { topic, partitions }
        }
    }
}
pub mod consumer_protocol_subscription {
    use bytes::BytesMut;
    use kafka_connector_protocol::{ApiVersion, FromBytes, SerializationError, ToBytes};

    /// Subscription part of the Consumer Protocol.
    ///
    /// The current implementation assumes that future versions will not break compatibility. When
    /// it encounters a newer version, it parses it using the current format. This basically means
    /// that new versions cannot remove or reorder any of the existing fields.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ConsumerProtocolSubscription {
        pub topics: Vec<String>,

        pub user_data: Option<Vec<u8>>,

        pub owned_partitions: Vec<TopicPartition>,
    }

    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct TopicPartition {
        pub topic: String,

        pub partitions: Vec<i32>,
    }

    impl ConsumerProtocolSubscription {
        pub fn get_min_supported_version() -> ApiVersion {
            ApiVersion(0)
        }

        pub fn get_max_supported_version() -> ApiVersion {
            ApiVersion(1)
        }

        pub fn serialize(
            &self,
            version: ApiVersion,
            _bytes: &mut BytesMut,
        ) -> Result<(), SerializationError> {
            debug_assert!(version >= Self::get_min_supported_version());
            debug_assert!(version <= Self::get_max_supported_version());
            self.validate_fields(version)?;
            self.topics.serialize(version, _bytes);
            self.user_data.serialize(version, _bytes);
            if version >= ApiVersion(1) {
                self.owned_partitions.serialize(version, _bytes);
            }
            Ok(())
        }

        pub fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
            let topics = Vec::<String>::deserialize(version, bytes);
            let user_data = Option::<Vec<u8>>::deserialize(version, bytes);
            let owned_partitions = if version >= ApiVersion(1) {
                Vec::<TopicPartition>::deserialize(version, bytes)
            } else {
                Default::default()
            };
            ConsumerProtocolSubscription {
                topics,
                user_data,
                owned_partitions,
            }
        }
    }

    impl ConsumerProtocolSubscription {
        fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
            for item in self.owned_partitions.iter() {
                item.validate_fields(_version)?;
            }
            Ok(())
        }
    }

    impl ToBytes for TopicPartition {
        fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {
            if version >= ApiVersion(1) {
                self.topic.serialize(version, _bytes);
            }
            if version >= ApiVersion(1) {
                self.partitions.serialize(version, _bytes);
            }
        }
    }

    impl TopicPartition {
        fn validate_fields(&self, _version: ApiVersion) -> Result<(), SerializationError> {
            if self.topic != String::default() && _version.0 < 1 {
                return Err(SerializationError::NonIgnorableFieldSet(
                    "topic",
                    *_version,
                    "TopicPartition",
                ));
            }
            if self.partitions != Vec::<i32>::default() && _version.0 < 1 {
                return Err(SerializationError::NonIgnorableFieldSet(
                    "partitions",
                    *_version,
                    "TopicPartition",
                ));
            }
            Ok(())
        }
    }

    impl FromBytes for TopicPartition {
        fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {
            let topic = if version >= ApiVersion(1) {
                String::deserialize(version, bytes)
            } else {
                Default::default()
            };
            let partitions = if version >= ApiVersion(1) {
                Vec::<i32>::deserialize(version, bytes)
            } else {
                Default::default()
            };
            TopicPartition { topic, partitions }
        }
    }
}
