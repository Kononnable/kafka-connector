use std::{
    cmp::{max, min},
    collections::HashMap,
    time::Duration,
};

use bytes::{Bytes, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use kafka_connector_protocol::{
    api::{api_versions::ApiVersionsRequest, ApiNumbers},
    api_error::ApiError,
    ApiCall,
};

use super::error::KafkaApiCallError;

#[derive(Debug)]
pub struct BrokerClient {
    connection: TcpStream,
    client_id: String,
    last_correlation: i32,
    pub supported_versions: HashMap<u16, (u16, u16)>, // TODO: Change later(?)
    send_buffer: BytesMut,
}

impl BrokerClient {
    // TODO: change type to  array of ToSocketAddrs
    pub async fn new(broker: &str, client_id: String) -> Result<BrokerClient, KafkaApiCallError> {
        let connection = TcpStream::connect(broker).await?;
        let mut client = BrokerClient {
            supported_versions: HashMap::new(),
            connection,
            last_correlation: 0,
            client_id,
            send_buffer: BytesMut::with_capacity(4096), // TODO: Change size(?)
        };
        client.get_supported_api_versions().await?;
        Ok(client)
    }

    pub async fn run_api_call<T>(
        &mut self,
        request: &T,
        api_version: Option<u16>,
    ) -> Result<T::Response, KafkaApiCallError>
    where
        T: ApiCall,
    {
        let api_version = if T::get_api_key() == ApiNumbers::ApiVersions {
            api_version.unwrap_or_default()
        } else {
            let supported_versions = self
                .supported_versions
                .get(&(T::get_api_key() as u16))
                .ok_or_else(|| KafkaApiCallError::UnsupportedKafkaApiVersion {
                    api: T::get_api_key(),
                    version: 0,
                })?;
            match api_version {
                Some(v) => {
                    if v >= supported_versions.0
                        && v <= supported_versions.1
                        && v >= T::get_min_supported_version()
                        && v <= T::get_max_supported_version()
                    {
                        v
                    } else {
                        return Err(KafkaApiCallError::UnsupportedKafkaApiVersion {
                            api: T::get_api_key(),
                            version: v,
                        });
                    }
                }
                None => {
                    let min_supported = max(supported_versions.0, T::get_min_supported_version());
                    let max_supported = min(supported_versions.1, T::get_max_supported_version());
                    if max_supported < min_supported {
                        return Err(KafkaApiCallError::UnsupportedKafkaApiVersion {
                            api: T::get_api_key(),
                            version: T::get_min_supported_version(),
                        });
                    }
                    max_supported
                }
            }
        };

        request.serialize(
            api_version,
            &mut self.send_buffer,
            self.last_correlation + 1,
            &self.client_id,
        );
        let len = self.send_buffer.len() as i32;
        self.connection.write_all(&len.to_be_bytes()).await.unwrap();
        self.connection.write_all(&self.send_buffer).await.unwrap();
        self.send_buffer.clear();
        let mut size: [u8; 4] = [0, 0, 0, 0];
        self.connection.read_exact(&mut size).await.unwrap();
        let cap = i32::from_be_bytes(size);
        let mut buf2 = vec![0; cap as usize];
        self.connection.read_exact(&mut buf2).await.unwrap();
        let mut buf2 = Bytes::from(buf2);
        log::trace!("Received bytes: {:?}", buf2);

        let (correlation, response) = T::deserialize_response(api_version, &mut buf2);
        self.last_correlation = correlation;
        if let Some(err) = T::get_first_error(&response) {
            return Err(KafkaApiCallError::KafkaApiError(err));
        }
        Ok(response)
    }

    async fn get_supported_api_versions(&mut self) -> Result<(), KafkaApiCallError> {
        let response = self
            .run_api_call(&ApiVersionsRequest::default(), Some(0))
            .await?;
        if response.error_code != 0 {
            return Err(KafkaApiCallError::KafkaApiError(ApiError::from(
                response.error_code,
            )));
        }
        self.supported_versions.clear();
        for api_key in response.api_keys {
            self.supported_versions.insert(
                api_key.api_key as u16,
                (api_key.min_version as u16, api_key.max_version as u16),
            );
        }
        Ok(())
    }
    pub async fn run_api_call_with_retry<T, C>(
        &mut self,
        request: T,
        api_version: Option<u16>,
    ) -> Result<T::Response, KafkaApiCallError>
    where
        T: ApiCall + Clone,
    {
        let mut response = self.run_api_call(&request, api_version).await;
        for _i in 0..=3 {
            // TODO: Extract to config value
            if let Err(KafkaApiCallError::KafkaApiError(_)) = response {
                tokio::time::sleep(Duration::from_millis(100)).await; // TODO: Extract to config value, gradually increase wait duration
                response = self.run_api_call(&request, api_version).await;
            } else {
                break;
            }
        }
        return response;
    }
}
