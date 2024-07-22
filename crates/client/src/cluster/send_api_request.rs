use bytes::BytesMut;
use kafka_connector_protocol::request_header::RequestHeader;
use kafka_connector_protocol::{ApiRequest, ApiResponse};
use std::time::Duration;
use thiserror::Error as DeriveError;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum SendApiRequestError {
    #[error("Error encountered during network communication. {0}")]
    NetworkError(std::io::Error),
    #[error("Broker did not respond within specified time")]
    ApiCallTimeoutReached,
}

/// Used only for initial cluster metadata fetch, outside of that `BrokerController` is solely responsible for direct communication with kafka brokers
pub async fn send_api_request<R: ApiRequest>(
    request: &R,
    buffer: &mut BytesMut,
    api_version: i16,
    request_header: &mut RequestHeader,
    connection: &mut TcpStream,
    request_timeout: Duration,
) -> Result<R::Response, SendApiRequestError> {
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
            .map_err(SendApiRequestError::NetworkError)?;
        connection
            .write_all(buffer)
            .await
            .map_err(SendApiRequestError::NetworkError)?;
        buffer.clear();

        let mut buffer_size = [0; 4];
        connection
            .read_exact(&mut buffer_size)
            .await
            .map_err(SendApiRequestError::NetworkError)?;
        let message_size = i32::from_be_bytes(buffer_size);

        buffer.reserve(message_size as usize);
        unsafe {
            buffer.set_len(message_size as usize);
        }
        connection
            .read_exact(buffer)
            .await
            .map_err(SendApiRequestError::NetworkError)?;
        Ok(R::Response::deserialize(api_version, buffer).1)
    })
    .await
    .map_err(|_| SendApiRequestError::ApiCallTimeoutReached)?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sends_request_and_receives_response() {
        unimplemented!()
    }

    #[test]
    fn errors_when_no_response_within_defined_time() {
        unimplemented!()
    }

    #[test]
    fn errors_if_connection_closed_during_communication() {
        unimplemented!()
    }
}
