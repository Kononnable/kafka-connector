use crate::{broker::controller::ApiCallError, cluster::options::ClusterControllerOptions};
use bytes::BytesMut;
use kafka_connector_protocol::{
    api_versions_request::ApiVersionsRequest, api_versions_response::ApiVersionsResponseKeyKey,
    metadata_request::MetadataRequest, metadata_response::MetadataResponse,
    request_header::RequestHeader, response_header::ResponseHeader, ApiRequest, ApiResponse,
};
use std::{fmt::Debug, time::Duration};
use thiserror::Error as DeriveError;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
    time::timeout,
};
use tracing::instrument;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub(crate) enum BrokerConnectionInitializationError {
    #[error("Error encountered during network communication. {0}")]
    NetworkError(std::io::Error),
    #[error("Broker did not respond within specified time")]
    ApiCallTimeoutReached,
    #[error("Failed to connect with the broker. {0}")]
    ConnectionError(std::io::Error),
    #[error("Connection not established within specified time")]
    ConnectionTimeoutReached,
}

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub(crate) enum SendRequestError {
    #[error("Error encountered during network communication. {0}")]
    NetworkError(#[from] std::io::Error),
    #[error("Serialization error {0}")]
    SerializationError(#[from] kafka_connector_protocol::SerializationError)
}

// TODO: Proper documentation (whole file)

// TODO: Rename?
// TODO: remove pub from fields? 
pub(crate) struct BrokerConnection {
    pub buffer: BytesMut,
    pub stream: TcpStream,
    pub header: RequestHeader,
    pub metadata: MetadataResponse,
}

impl BrokerConnection {
    // TODO: Docs - cancellation safe
    pub async fn try_recv(&mut self) -> Option<Result<(ResponseHeader, BytesMut), ApiCallError>> {
        if let Some(value) = self.try_read_next_response() {
            return Some(value);
        }
        // TODO: Value from config
        if self.buffer.capacity() < 100 {
            // TODO: Value from config, one third of the buffer size?
            self.buffer.reserve(1024);
        }
        // TODO: More advanced logic on reserve(?) so same buffer is reused more often

        let read_bytes_len = self.stream.read_buf(&mut self.buffer).await;
        match read_bytes_len {
            Ok(read_bytes_len) => {
                if read_bytes_len == 0 {
                    return Some(Err(ApiCallError::BrokerConnectionClosing));
                }
            }
            Err(err) => {
                return Some(Err(ApiCallError::IoError(err)));
            }
        }

        self.try_read_next_response()
    }

    fn try_read_next_response(
        &mut self,
    ) -> Option<Result<(ResponseHeader, BytesMut), ApiCallError>> {
        if self.buffer.len() >= 4 {
            let resp_len = i32::from_be_bytes((&self.buffer[0..4]).try_into().unwrap()) as usize;
            if self.buffer.len() >= resp_len + 4 {
                let _ = self.buffer.split_to(4);
                let header = ResponseHeader::deserialize(0, &mut self.buffer);
                return Some(Ok((header, self.buffer.split_to(resp_len))));
            }
        }
        None
    }

    pub async fn send(
        &mut self,
        api_key: i16,
        api_version: i16,
        buf: BytesMut,
    ) -> Result<i32, SendRequestError> {
        self.header.request_api_key = api_key;
        self.header.request_api_version = api_version;
        self.header.correlation_id += 1;

        self.header.serialize(0, &mut self.buffer)?;
        let request_len = (self.buffer.len() + buf.len()) as i32;
        self.stream
            .write_all(&request_len.to_be_bytes())
            .await?;
        self.stream.write_all(&self.buffer).await?;
        self.stream.write_all(&buf).await?;
        self.buffer.clear();

        Ok(self.header.correlation_id)
    }
}

// TODO: Rename
// TODO: Reuse some code from broker loop(?)
#[instrument(level = "debug", skip_all)]
pub(crate) async fn fetch_initial_broker_list_from_broker(
    options: &ClusterControllerOptions,
    address: impl ToSocketAddrs + Debug,
) -> Result<BrokerConnection, BrokerConnectionInitializationError> {
    let mut connection = timeout(options.connection_timeout, TcpStream::connect(address))
        .await
        .map_err(|_| BrokerConnectionInitializationError::ConnectionTimeoutReached)?
        .map_err(BrokerConnectionInitializationError::ConnectionError)?;

    let mut buffer = BytesMut::with_capacity(1_024); // TODO: Size

    let api_versions_request = ApiVersionsRequest::default();
    let mut header = RequestHeader {
        client_id: options.client_name.to_owned(),
        ..Default::default()
    };

    let api_versions_response = send_api_request(
        &api_versions_request,
        &mut buffer,
        0,
        &mut header,
        &mut connection,
        options.request_timeout,
    )
    .await?;

    let mut metadata_request_version = api_versions_response
        .api_keys
        .get(&ApiVersionsResponseKeyKey {
            index: MetadataRequest::get_api_key(),
        })
        .expect("Server should support metadata requests")
        .max_version;
    if metadata_request_version > MetadataRequest::get_max_supported_version() {
        metadata_request_version = MetadataRequest::get_max_supported_version();
    }
    let metadata_request = MetadataRequest {
        topics: Some(vec![]),
        allow_auto_topic_creation: false,
    };
    let metadata = send_api_request(
        &metadata_request,
        &mut buffer,
        metadata_request_version,
        &mut header,
        &mut connection,
        options.request_timeout,
    )
    .await?;
    Ok(BrokerConnection {
        buffer,
        stream: connection,
        header,
        metadata,
    })
}

/// Used only for initial cluster metadata fetch, outside of that `BrokerController` is solely responsible for direct communication with kafka brokers
// TODO: migrate to BrokerConnection::send
async fn send_api_request<R: ApiRequest>(
    request: &R,
    buffer: &mut BytesMut,
    api_version: i16,
    request_header: &mut RequestHeader,
    connection: &mut TcpStream,
    request_timeout: Duration,
) -> Result<R::Response, BrokerConnectionInitializationError> {
    // TODO: What if timeout happen when only part of the api request is written?
    //       e.g. execution waits on write_all await point when timeout is handled
    timeout(request_timeout, async {
        request_header.request_api_key = R::get_api_key();
        request_header.request_api_version = api_version;
        request_header.correlation_id += 1;

        request
            .serialize(api_version, buffer, request_header)
            .expect("Failed to serialize initial Api Request");
        let len = buffer.len() as i32;
        connection
            .write_all(&len.to_be_bytes())
            .await
            .map_err(BrokerConnectionInitializationError::NetworkError)?;
        connection
            .write_all(buffer)
            .await
            .map_err(BrokerConnectionInitializationError::NetworkError)?;
        buffer.clear();

        let mut buffer_size = [0; 4];
        connection
            .read_exact(&mut buffer_size)
            .await
            .map_err(BrokerConnectionInitializationError::NetworkError)?;
        let message_size = i32::from_be_bytes(buffer_size);

        buffer.reserve(message_size as usize);
        unsafe {
            buffer.set_len(message_size as usize);
        }
        connection
            .read_exact(buffer)
            .await
            .map_err(BrokerConnectionInitializationError::NetworkError)?;
        Ok(R::Response::deserialize(api_version, buffer).1)
    })
    .await
    .map_err(|_| BrokerConnectionInitializationError::ApiCallTimeoutReached)?
}

// TODO: Tests
