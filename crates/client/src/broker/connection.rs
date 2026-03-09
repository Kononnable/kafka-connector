use crate::cluster::{error::ApiCallError, options::ClusterControllerOptions};
use bytes::BytesMut;
use kafka_connector_protocol::{
    ApiKey, ApiVersion, request_header::RequestHeader, response_header::ResponseHeader,
};
use std::fmt::Debug;
use std::io::ErrorKind;
use thiserror::Error as DeriveError;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub(crate) enum BrokerConnectionInitializationError {
    #[error("Failed to connect with the broker. {0}")]
    ConnectionError(std::io::Error),
    #[error("Failed to initialize broker connection within specified time")]
    ConnectionTimeoutReached,
    #[error("Failed to communicate with the broker. {0}")]
    ProtocolError(#[from] ApiCallError),
}

// TODO: Rename?
pub(crate) struct BrokerConnection {
    buffer: BytesMut,
    stream: TcpStream,
    header: RequestHeader,
    buffer_grow_size: usize,
}

impl BrokerConnection {
    pub fn new(stream: TcpStream, options: &ClusterControllerOptions) -> BrokerConnection {
        let buffer = BytesMut::with_capacity(options.advanced.buffer_size);
        let header = RequestHeader {
            client_id: options.client_name.to_owned(),
            ..Default::default()
        };
        let buffer_grow_size = options.advanced.buffer_grow_size;
        BrokerConnection {
            buffer,
            stream,
            header,
            buffer_grow_size,
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
        if self.buffer.capacity() < self.buffer_grow_size / 2 {
            self.buffer.reserve(self.buffer_grow_size);
        }

        loop {
            let read_bytes_len = self.stream.read_buf(&mut self.buffer).await;
            match read_bytes_len {
                Ok(read_bytes_len) => {
                    if read_bytes_len == 0 {
                        return Some(Err(ApiCallError::BrokerConnectionClosed));
                    }
                    break;
                }
                Err(err) => {
                    if err.kind() == ErrorKind::Interrupted {
                        continue;
                    }
                    return Some(Err(ApiCallError::IoError(err)));
                }
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
                    self.buffer
                        .reserve(usize::max(bytes_missing, self.buffer_grow_size));
                }
            }
        }
        None
    }

    // docs - mention not cancellation safety
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    mod try_recv {
        use crate::broker::connection::BrokerConnection;
        use crate::cluster::options::ClusterControllerOptions;
        use bytes::BytesMut;
        use kafka_connector_protocol::api_versions_request::ApiVersionsRequest;
        use kafka_connector_protocol::request_header::RequestHeader;
        use kafka_connector_protocol::{ApiRequest, ApiVersion};
        use tokio::io::AsyncWriteExt;
        use tokio::net::{TcpListener, TcpStream};
        use tokio::sync::mpsc::UnboundedSender;

        async fn setup_connection() -> (BrokerConnection, UnboundedSender<Vec<u8>>) {
            let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let addr = server.local_addr().unwrap();

            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();

            tokio::spawn(async move {
                let (mut connection, _) = server.accept().await.unwrap();
                while let Some(bytes) = rx.recv().await {
                    connection.write_all(&bytes[..]).await.unwrap();
                    connection.flush().await.unwrap();
                }
            });
            let stream = TcpStream::connect(addr).await.unwrap();
            let options = ClusterControllerOptions::default();
            let connection = BrokerConnection::new(stream, &options);
            (connection, tx)
        }

        #[test_log::test(tokio::test)]
        async fn handles_partial_messages_correctly() {
            let (mut connection, server_response_tx) = setup_connection().await;

            let mut messages_to_be_sent = BytesMut::with_capacity(1024);

            // empty buffer, buffer with < 4 bytes (len), buffer with less bytes then in resp.len(), buffer with more than one message

            let mut tmp_buf = BytesMut::with_capacity(1024);
            let header = RequestHeader {
                request_api_key: ApiVersionsRequest::get_api_key().0,
                request_api_version: 1,
                correlation_id: 1,
                client_id: "Some text to make message longer".to_owned(),
            };
            let request = ApiVersionsRequest {};
            header.serialize(ApiVersion(0), &mut tmp_buf).unwrap();
            request.serialize(ApiVersion(0), &mut tmp_buf).unwrap();
            let request_len = tmp_buf.len();
            const MSG_SIZE_LEN: usize = 4;

            messages_to_be_sent.extend_from_slice(&(request_len as i32).to_be_bytes());
            messages_to_be_sent.extend_from_slice(&tmp_buf);

            messages_to_be_sent.extend_from_slice(&(request_len as i32).to_be_bytes());
            messages_to_be_sent.extend_from_slice(&tmp_buf);

            messages_to_be_sent.extend_from_slice(&(request_len as i32).to_be_bytes());
            messages_to_be_sent.extend_from_slice(&tmp_buf);

            // Not enough to see message length
            server_response_tx
                .send(messages_to_be_sent.split_to(1).to_vec())
                .unwrap();
            assert!(connection.try_recv().await.is_none());

            // Enough only to see message length
            server_response_tx
                .send(messages_to_be_sent.split_to(MSG_SIZE_LEN - 1).to_vec())
                .unwrap();
            assert!(connection.try_recv().await.is_none());

            // Not enough to decode first message
            server_response_tx
                .send(messages_to_be_sent.split_to(request_len - 1).to_vec())
                .unwrap();
            assert!(connection.try_recv().await.is_none());

            // Exactly one message
            server_response_tx
                .send(messages_to_be_sent.split_to(1).to_vec())
                .unwrap();
            assert!(connection.try_recv().await.is_some());

            // Not enough to see message length
            server_response_tx
                .send(messages_to_be_sent.split_to(1).to_vec())
                .unwrap();
            assert!(connection.try_recv().await.is_none());

            // Two messages at once
            server_response_tx
                .send(
                    messages_to_be_sent
                        .split_to(2 * MSG_SIZE_LEN + 2 * request_len - 1)
                        .to_vec(),
                )
                .unwrap();
            assert!(connection.try_recv().await.is_some());
            assert!(connection.try_recv().await.is_some());
        }
    }

    mod send {
        use super::*;
        use kafka_connector_protocol::ApiRequest;
        use kafka_connector_protocol::api_versions_request::ApiVersionsRequest;

        async fn read_message_as_server<T: ApiRequest>(
            connection: &mut TcpStream,
            api_version: ApiVersion,
        ) -> (RequestHeader, T) {
            let mut buffer = BytesMut::with_capacity(1_024);
            let mut buffer_size = [0; 4];

            connection.read_exact(&mut buffer_size).await.unwrap();
            let message_size = i32::from_be_bytes(buffer_size) as usize;
            buffer.reserve(message_size);
            unsafe {
                buffer.set_len(message_size);
            }
            connection.read_exact(&mut buffer).await.unwrap();
            let header = RequestHeader::deserialize(ApiVersion(0), &mut buffer);
            let payload = T::deserialize(api_version, &mut buffer);
            (header, payload)
        }
        #[test_log::test(tokio::test)]
        async fn sends_api_requests_correctly() {
            let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let addr = server.local_addr().unwrap();

            tokio::spawn(async move {
                let (mut connection, _) = server.accept().await.unwrap();

                let (header, _) =
                    read_message_as_server::<ApiVersionsRequest>(&mut connection, ApiVersion(0))
                        .await;
                assert_eq!(
                    ApiKey(header.request_api_key),
                    ApiVersionsRequest::get_api_key()
                );
                assert_eq!(header.request_api_version, 1);
                assert_eq!(header.request_api_key, ApiVersionsRequest::get_api_key().0);

                let (header, _) =
                    read_message_as_server::<ApiVersionsRequest>(&mut connection, ApiVersion(0))
                        .await;
                assert_eq!(
                    ApiKey(header.request_api_key),
                    ApiVersionsRequest::get_api_key()
                );
                assert_eq!(header.request_api_version, 2);
                assert_eq!(header.request_api_key, ApiVersionsRequest::get_api_key().0);
            });

            let stream = TcpStream::connect(addr).await.unwrap();
            let options = ClusterControllerOptions::default();
            let mut connection = BrokerConnection::new(stream, &options);

            let req1 = connection
                .send(
                    ApiVersionsRequest::get_api_key(),
                    ApiVersion(1),
                    BytesMut::with_capacity(0),
                )
                .await
                .unwrap();
            assert_eq!(req1, 1);
            let req2 = connection
                .send(
                    ApiVersionsRequest::get_api_key(),
                    ApiVersion(2),
                    BytesMut::with_capacity(0),
                )
                .await
                .unwrap();
            assert_eq!(req2, 2);
            assert_eq!(connection.header.correlation_id, 2);
        }
    }
}
