use crate::protocol_consts::ListOffsetsTimestampType::Specific;

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
