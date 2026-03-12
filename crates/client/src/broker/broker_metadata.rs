// TODO: split to two structs? (one with id for internal usage - Vec<BrokerMetadata>, one without id for public api - HashMap<id, BrokerMetadata>

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BrokerMetadata {
    /// Broker id
    pub(crate) broker_id: i32,

    /// The broker hostname.
    pub host: String,

    /// The broker port.
    pub port: i32,

    /// The rack of the broker, or null if it has not been assigned to a rack.
    pub rack: Option<String>,
}
