use crate::{broker::controller::ApiCallError, cluster::options::ClusterControllerOptions};
use bytes::BytesMut;
use kafka_connector_protocol::{
    api_versions_request::ApiVersionsRequest, api_versions_response::ApiVersionsResponseKeyKey,
    metadata_request::MetadataRequest, metadata_response::MetadataResponse,
    request_header::RequestHeader, response_header::ResponseHeader, ApiKey, ApiRequest,
    ApiResponse, ApiVersion,
};
use std::fmt::Debug;
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

// TODO: Proper documentation (whole file)

// TODO: Rename?
// TODO: remove pub from fields?
pub(crate) struct BrokerConnection {
    pub buffer: BytesMut,
    pub stream: TcpStream,
    pub header: RequestHeader,
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
                let mut resp = self.buffer.split_to(resp_len);
                let header = ResponseHeader::deserialize(ApiVersion(0), &mut resp);
                return Some(Ok((header, resp)));
            }
        }
        None
    }

    pub async fn send(
        &mut self,
        api_key: ApiKey,
        api_version: ApiVersion,
        buf: BytesMut,
    ) -> Result<i32, ApiCallError> {
        self.header.request_api_key = *api_key;
        self.header.request_api_version = *api_version;
        self.header.correlation_id += 1;

        self.header.serialize(ApiVersion(0), &mut self.buffer)?;
        let request_len = (self.buffer.len() + buf.len()) as i32;
        self.stream.write_all(&request_len.to_be_bytes()).await?;
        self.stream.write_all(&self.buffer).await?;
        self.stream.write_all(&buf).await?;
        self.buffer.clear();

        Ok(self.header.correlation_id)
    }
}

// TODO: Rename
#[instrument(level = "debug", skip_all)]
pub(crate) async fn fetch_initial_broker_list_from_broker(
    options: &ClusterControllerOptions,
    address: impl ToSocketAddrs + Debug,
) -> Result<(BrokerConnection, MetadataResponse), BrokerConnectionInitializationError> {
    let stream = timeout(options.connection_timeout, TcpStream::connect(address))
        .await
        .map_err(|_| BrokerConnectionInitializationError::ConnectionTimeoutReached)?
        .map_err(BrokerConnectionInitializationError::ConnectionError)?;

    let buffer = BytesMut::with_capacity(1_024); // TODO: Size
    let header = RequestHeader {
        client_id: options.client_name.to_owned(),
        ..Default::default()
    };

    let mut connection = BrokerConnection {
        buffer,
        stream,
        header,
    };

    let api_versions_response = call_api_inline(
        &mut connection,
        ApiVersionsRequest::default(),
        ApiVersionsRequest::get_min_supported_version(),
        options,
    )
    .await?;

    let mut metadata_request_version = ApiVersion(
        api_versions_response
            .api_keys
            .get(&ApiVersionsResponseKeyKey {
                index: *MetadataRequest::get_api_key(),
            })
            .expect("Server should support metadata requests")
            .max_version,
    );
    if metadata_request_version > MetadataRequest::get_max_supported_version() {
        metadata_request_version = MetadataRequest::get_max_supported_version();
    }
    let metadata_request = MetadataRequest {
        topics: Some(vec![]),
        allow_auto_topic_creation: false,
    };
    let metadata = call_api_inline(
        &mut connection,
        metadata_request,
        metadata_request_version,
        options,
    )
    .await?;
    Ok((connection, metadata))
}

/// Used only for initial cluster metadata fetch, outside of that `BrokerController` is solely responsible for direct communication with kafka brokers
/// If it fails(e.g. timeout) whole connection needs to be reseted
async fn call_api_inline<R: ApiRequest>(
    mut connection: &mut BrokerConnection,
    request: R,
    version: ApiVersion,
    options: &ClusterControllerOptions,
) -> Result<R::Response, BrokerConnectionInitializationError> {
    timeout(options.request_timeout, async {
        request
            .serialize(version, &mut connection.buffer)
            .expect("Serialization failure during establishing broker connection");

        let request = connection.buffer.split();
        let correlation_id = connection
            .send(R::get_api_key(), version, request)
            .await
            .map_err(map_error_inline)?;
        loop {
            if let Some(result) = connection.try_recv().await {
                let (header, mut response) = result.map_err(map_error_inline)?;
                if header.correlation_id == correlation_id {
                    break Ok(R::Response::deserialize(version, &mut response));
                }
            }
        }
    })
    .await
    .map_err(|_| BrokerConnectionInitializationError::ApiCallTimeoutReached)?
}
fn map_error_inline(value: ApiCallError) -> BrokerConnectionInitializationError {
    match value {
        ApiCallError::BrokerConnectionClosing => {
            BrokerConnectionInitializationError::NetworkError(std::io::Error::new(
                std::io::ErrorKind::ConnectionAborted,
                "Connection closed during initialization",
            ))
        }
        ApiCallError::IoError(e) => BrokerConnectionInitializationError::NetworkError(e),
        ApiCallError::SerializationError(e) => {
            panic!("Serialization failure during broker connection. {:?}", e)
        }
    }
}

// TODO: Tests
