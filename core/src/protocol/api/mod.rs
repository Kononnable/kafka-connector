pub mod header;

pub mod fetch;
pub mod produce;

pub enum ApiNumbers {
    Produce = 0,
    Fetch = 1,
    // Offsets = 2,
    // Metadata = 3,
    // LeaderAndIsr = 4,
    // StopReplica = 5,
    // UpdateMetadata = 6,
    // ControlledShutdown = 7,
    // OffsetCommit = 8,
    // OffsetFetch = 9,
    // GroupCoordinator = 10,
    // JoinGroup = 11,
    // Heartbeat = 12,
    // LeaveGroup = 13,
    // SyncGroup = 14,
    // DescribeGroups = 15,
    // ListGroups = 16,
    // SaslHandshake = 17,
    // ApiVersions = 18,
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
    pub use super::super::from_bytes::FromBytes;
    pub use super::super::to_bytes::ToBytes;
    pub use kafka_connector_derive::FromBytes;
    pub use kafka_connector_derive::ToBytes;
    pub use std::convert::TryFrom;
    pub use std::convert::TryInto;
    pub use bytes::BytesMut;
    use thiserror::Error as DeriveError;

    pub type Int8 = i8;
    pub type Int16 = i16;
    pub type Int32 = i32;
    pub type Int64 = i64;
    pub type NullableString = Option<String>;

    // TODO:
    pub type CompactString = String;
    pub type CompactRecords = i64;
    pub type Records = i64;


    ///Fields not supported by some old kafka version
    pub enum Optional<T>
    where T: Default
     {
        Some(T),
        None
    }
    
    impl<T> Optional<T>
    where T: Default{
        pub fn is_some(&self)->bool{
            match self{
                Self::Some(_)=>true,
                Self::None=>false
            }
        }
        pub fn try_into<R>(self)-> Result<Optional<R>, Error>
        where T: Default + TryInto<R,Error=Error>, R: Default{
            match self{
                Optional::Some(x) => {
                    let v = x.try_into()?;
                    Ok(Optional::Some(v))
                }
                Optional::None => {Ok(Optional::None)}
            }
        }
        pub fn into<R>(self)-> Optional<R>
        where T: Default + Into<R>, R: Default{
            match self{
                Optional::Some(x) => {
                    let v = x.into();
                    Optional::Some(v)
                }
                Optional::None => Optional::None
            }
        }
    }
    impl<T> Default for Optional<T>
    where T: Default{
        fn default() -> Self { 
            Optional::Some(T::default())
        }
    }
    impl<T> FromBytes for Optional<T> 
    where T: FromBytes + Default{
        fn deserialize<R>(buf: &mut R) -> Self
        where
            R: Iterator<Item = u8>,
        {
          Optional::Some(T::deserialize(buf))
        }
    }
    impl<T> ToBytes for Optional<T> 
    where T: ToBytes + Default{
        fn serialize(&self, buf: &mut BytesMut) {
            match self{
                Self::Some(value)=>T::serialize(value, buf),
                Self::None=>T::serialize(&T::default(), buf)
            }
            
        }
    }


    #[derive(Debug, DeriveError)]
    pub enum Error {
        #[error("Connected kafka broker doesn't support requested API version. API name: {0} API version: {1} Api field: {2}")]
        OldKafkaVersion(&'static str,i32,&'static str),
    }
    // impl From<Infallible> for Error {
    //     fn from(error: Infallible) -> Self {
    //         panic!("Parsing cannot fail {:?}",error);
    //     }
    // }
    // impl From<Error> for Infallible{
    //     fn from(error: Infallible) -> Self {
    //         panic!("Parsing cannot fail {:?}",error);
    //     }
    // }
}
