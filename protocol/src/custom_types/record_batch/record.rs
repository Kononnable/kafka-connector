use crate::custom_types::zig_zag_vec::ZigZagVec;

use super::header::Header;

// TODO: Change to something more user-friendly
#[derive(Debug, Default, Clone)]
pub struct Record {
    pub attributes: i8,
    pub timestamp: i64,
    pub offset: i64,
    pub key: ZigZagVec<u8>,
    pub value: ZigZagVec<u8>,
    pub headers: ZigZagVec<Header>,
}
