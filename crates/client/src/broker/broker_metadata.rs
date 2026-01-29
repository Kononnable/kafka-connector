use kafka_connector_protocol::metadata_response::MetadataResponseBroker;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BrokerMetadata {
    /// The broker hostname.
    pub host: String,

    /// The broker port.
    pub port: i32,

    /// The rack of the broker, or null if it has not been assigned to a rack.
    pub rack: Option<String>,
}

impl From<MetadataResponseBroker> for BrokerMetadata {
    fn from(metadata: MetadataResponseBroker) -> Self {
        let MetadataResponseBroker { host, port, rack } = metadata;
        BrokerMetadata { host, port, rack }
    }
}
