use crate::cluster::{error::ApiCallError, options::ClusterControllerOptions};
use bytes::BytesMut;
use indexmap::IndexMap;
use kafka_connector_protocol::api_versions_response::ApiVersionsResponseKey;
use kafka_connector_protocol::{
    ApiKey, ApiRequest, ApiResponse, ApiVersion, api_versions_request::ApiVersionsRequest,
    api_versions_response::ApiVersionsResponseKeyKey, metadata_request::MetadataRequest,
    metadata_response::MetadataResponse, request_header::RequestHeader,
    response_header::ResponseHeader,
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
    #[error("Failed to connect with the broker. {0}")]
    ConnectionError(std::io::Error),
    #[error("Failed to initialize broker connection within specified time")]
    ConnectionTimeoutReached,
}

// TODO: Proper documentation (whole file)

// TODO: Rename?
pub(crate) struct BrokerConnection {
    buffer: BytesMut,
    stream: TcpStream,
    header: RequestHeader,
}

impl BrokerConnection {
    pub fn new(buffer: BytesMut, stream: TcpStream, header: RequestHeader) -> BrokerConnection {
        BrokerConnection {
            buffer,
            stream,
            header,
        }
    }

    /// Reads broker response from TCP stream.
    /// Returns `None` if there is not enough data to read whole response.
    ///
    /// This method is cancel safe
    pub async fn try_recv(&mut self) -> Option<Result<(ResponseHeader, BytesMut), ApiCallError>> {
        if let Some(value) = self.try_read_next_response() {
            return Some(value);
        }
        if self.buffer.capacity() < 4 {
            self.buffer.reserve(4);
        }

        let read_bytes_len = self.stream.read_buf(&mut self.buffer).await;
        match read_bytes_len {
            Ok(read_bytes_len) => {
                if read_bytes_len == 0 {
                    return Some(Err(ApiCallError::BrokerConnectionClosed));
                }
            }
            Err(err) => {
                // TODO: if ErrorKind::Interrupted - retry (or recursive call?)
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
            } else {
                let bytes_missing = resp_len + 4 - self.buffer.len();
                if self.buffer.capacity() < bytes_missing {
                    self.buffer.reserve(bytes_missing);
                }
            }
        }
        None
    }

    // TODO: docs + mention not cancellation safety
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
) -> Result<
    (
        BrokerConnection,
        IndexMap<i16, ApiVersionsResponseKey>,
        MetadataResponse,
    ),
    BrokerConnectionInitializationError,
> {
    timeout(options.connection_timeout, async {
        let stream = TcpStream::connect(address)
            .await
            .map_err(BrokerConnectionInitializationError::ConnectionError)?;

        let buffer = BytesMut::with_capacity(options.buffer_size);
        let header = RequestHeader {
            client_id: options.client_name.to_owned(),
            ..Default::default()
        };

        let mut connection = BrokerConnection::new(buffer, stream, header);

        let api_versions_response = call_api_inline(
            &mut connection,
            ApiVersionsRequest::default(),
            ApiVersionsRequest::get_min_supported_version(),
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

        let supported_api_versions: IndexMap<i16, ApiVersionsResponseKey> = api_versions_response
            .api_keys
            .into_iter()
            .map(|(k, v)| (k.index, v))
            .collect();

        if metadata_request_version > MetadataRequest::get_max_supported_version() {
            metadata_request_version = MetadataRequest::get_max_supported_version();
        }
        let metadata_request = MetadataRequest {
            topics: Some(vec![]),
            ..Default::default()
        };
        let metadata =
            call_api_inline(&mut connection, metadata_request, metadata_request_version).await?;
        Ok((connection, supported_api_versions, metadata))
    })
    .await
    .map_err(|_| BrokerConnectionInitializationError::ConnectionTimeoutReached)?
}

/// Used only for initial cluster metadata fetch, outside of that `BrokerController` is solely responsible for direct communication with kafka brokers
/// If it fails(e.g. timeout) whole connection needs to be reseted
pub(super) async fn call_api_inline<R: ApiRequest>(
    connection: &mut BrokerConnection,
    request: R,
    version: ApiVersion,
) -> Result<R::Response, BrokerConnectionInitializationError> {
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
}
fn map_error_inline(value: ApiCallError) -> BrokerConnectionInitializationError {
    match value {
        ApiCallError::BrokerConnectionClosed => {
            BrokerConnectionInitializationError::NetworkError(std::io::Error::new(
                std::io::ErrorKind::ConnectionAborted,
                "Connection closed during initialization",
            ))
        }
        ApiCallError::IoError(e) => BrokerConnectionInitializationError::NetworkError(e),
        // TODO: check if needed
        ApiCallError::SerializationError(e) => {
            panic!("Serialization failure during broker connection. {:?}", e)
        }
        // TODO: check if needed
        ApiCallError::TimeoutReached => {
            panic!("TODO")
        }
        // TODO: check if needed
        ApiCallError::BrokerNotFound(_) => {
            unreachable!();
        }
    }
}
