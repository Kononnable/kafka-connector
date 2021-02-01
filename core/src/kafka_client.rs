use std::collections::HashMap;

use bytes::{Bytes, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use thiserror::Error as DeriveError;

use kafka_connector_protocol::{api::api_versions::ApiVersionsRequest, ApiCall};

#[derive(Debug)]
pub struct KafkaClient {
    pub client_id: String,
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
    pub client_id: String,
    pub last_correlation: i32,
    pub supported_versions: HashMap<i16, (i16, i16)>,
}
#[non_exhaustive]
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

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum KafkaApiCallError {
    #[error("TODO error: {0}")]
    TODOError(String),
}

impl BrokerClient {
    // TODO: change type to  array of ToSocketAddrs
    pub async fn new(
        broker: &str,
        client_id: String,
    ) -> Result<BrokerClient, KafkaConnectionError> {
        let connection = TcpStream::connect(broker).await?;
        let mut client = BrokerClient {
            supported_versions: HashMap::new(),
            connection,
            last_correlation: 0,
            client_id,
        };
        client.get_supported_api_versions().await;
        Ok(client)
    }

    pub async fn run_api_call<T>(
        &mut self,
        request: T,
        api_version: Option<i16>,
    ) -> Result<T::Response, KafkaApiCallError>
    where
        T: ApiCall,
    {
        let api_version = match api_version {
            Some(v) => v,
            None => {
                let supported = self
                    .supported_versions
                    .get(&(T::get_api_key() as i16))
                    .ok_or_else(|| {
                        KafkaApiCallError::TODOError("supported api version not found".to_owned())
                    })?;
                supported.1
            }
        };

        // let header = HeaderRequest1::new(
        //     T::get_api_key() as i16,
        //     api_version,
        //     self.last_correlation + 1,
        //     self.client_id.to_owned(),
        // );
        let mut buffer = BytesMut::with_capacity(4096); // TODO: Change size(?)
                                                        // header.serialize(&mut buffer);
        request
            .serialize(
                api_version,
                &mut buffer,
                self.last_correlation + 1,
                &self.client_id,
            )
            .unwrap();
        let len = buffer.len() as i32;
        self.connection.write_all(&len.to_be_bytes()).await.unwrap();
        self.connection.write_all(&buffer).await.unwrap();
        let mut size: [u8; 4] = [0, 0, 0, 0];
        self.connection.read_exact(&mut size).await.unwrap();
        let cap = i32::from_be_bytes(size);
        let mut buf2 = vec![0; cap as usize];
        self.connection.read_exact(&mut buf2).await.unwrap();
        let mut buf2 = Bytes::from(buf2);
        log::trace!("Received bytes: {:?}", buf2);

        // let response_header = HeaderResponse::deserialize(&mut buf2);
        let (correlation, response) = T::deserialize_response(api_version, &mut buf2);
        self.last_correlation = correlation;
        Ok(response)
    }

    async fn get_supported_api_versions(&mut self) {
        let response = self
            .run_api_call(ApiVersionsRequest::default(), Some(0))
            .await
            .unwrap();
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

impl KafkaClient {
    pub async fn new(
        broker_addr: &str,
        client_id: &str,
    ) -> Result<KafkaClient, KafkaConnectionError> {
        let clients = vec![BrokerClient::new(broker_addr, client_id.to_owned()).await?];
        Ok(KafkaClient {
            clients,
            metadata: Metadata::default(),
            client_id: client_id.to_owned(),
        })
    }
}
