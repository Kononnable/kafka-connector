use std::collections::HashMap;

use bytes::{Bytes, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use thiserror::Error as DeriveError;

use kafka_connector_protocol::{
    api::{
        api_versions::{
            deserialize_api_versions_response, serialize_api_versions_request, ApiVersionsRequest,
        },
        header::{HeaderRequest, HeaderResponse},
        ApiNumbers,
    },
    from_bytes::FromBytes,
    to_bytes::ToBytes,
};

#[derive(Debug)]
pub struct KafkaClient {
    pub clients: Vec<BrokerClient>,
    pub metadata: Metadata,
}
#[derive(Default, Debug)]
pub struct Metadata {}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {}
    }
}
#[derive(Debug)]
pub struct BrokerClient {
    pub connection: TcpStream,
    pub last_correlation: i32,
    pub supported_versions: HashMap<i16, (i16, i16)>,
}
#[derive(Debug, DeriveError)]
pub enum KafkaConnectionError {
    #[error("Error connecting to broker \"{0}\"")]
    ConnectionError(std::io::Error),
}

impl From<std::io::Error> for KafkaConnectionError {
    fn from(error: std::io::Error) -> Self {
        Self::ConnectionError(error)
    }
}

impl BrokerClient {
    // TODO: change type to  array of ToSocketAddrs
    pub async fn new(broker: &str) -> Result<BrokerClient, KafkaConnectionError> {
        let connection = TcpStream::connect(broker).await?;
        let mut client = BrokerClient {
            supported_versions: HashMap::new(),
            connection,
            last_correlation: 0,
        };
        client.get_supported_api_versions().await;
        Ok(client)
    }

    async fn get_supported_api_versions(&mut self) {
        // TODO: extract to function?
        let header = HeaderRequest::new(
            ApiNumbers::ApiVersions as i16,
            0,
            self.last_correlation + 1,
            "".to_owned(),
        );
        let mut buffer = BytesMut::with_capacity(4096);
        header.serialize(&mut buffer);
        let request = ApiVersionsRequest::default();
        serialize_api_versions_request(request, 0, &mut buffer).unwrap();
        let len = buffer.len() as i32;
        self.connection.write_all(&len.to_be_bytes()).await.unwrap();
        self.connection.write_all(&buffer).await.unwrap();
        let mut size: [u8; 4] = [0, 0, 0, 0];
        self.connection.read_exact(&mut size).await.unwrap();
        let cap = i32::from_be_bytes(size);
        let mut buf2 = vec![0; cap as usize];
        self.connection.read_exact(&mut buf2).await.unwrap();

        let mut buf2 = Bytes::from(buf2);
        let response_header = HeaderResponse::deserialize(&mut buf2);
        self.last_correlation = response_header.correlation;
        let response = deserialize_api_versions_response(0, &mut buf2);
        // TODO read last corelation
        if response.error_code != 0 {
            todo!("")
        }
        self.supported_versions.clear();
        for api_key in response.api_keys {
            self.supported_versions
                .insert(api_key.api_key, (api_key.min_version, api_key.max_version));
        }
    }
}

// let req = ApiVersionsRequest::new(1, "my-client".to_string());
// req.serialize(&mut buffer);
// let len = buffer.len() as i32;
// stream.write_all(&len.to_be_bytes()).await?;
// stream.write_all(&buffer).await?;
// let mut size: [u8; 4] = [0, 0, 0, 0];
// let read = stream.read_exact(&mut size).await?;
// println!("Read {} bytes: {:?}", read, size);
// let cap = i32::from_be_bytes(size);
// println!("Message size: {}", cap);
// let mut buf2 = vec![0; cap as usize];
// let _ = stream.read_exact(&mut buf2).await?;
// let mut x = buf2.iter().copied(

impl KafkaClient {
    pub async fn new(broker_addr: &str) -> Result<KafkaClient, KafkaConnectionError> {
        let clients = vec![BrokerClient::new(broker_addr).await?];
        Ok(KafkaClient {
            clients,
            metadata: Metadata::default(),
        })
    }
}
