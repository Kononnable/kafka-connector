pub mod cluster_controller {
    use bytes::BytesMut;
    use kafka_connector_protocol::api_versions_response::{
        ApiVersionsResponse, ApiVersionsResponseKey, ApiVersionsResponseKeyKey,
    };
    use kafka_connector_protocol::metadata_request::MetadataRequest;
    use kafka_connector_protocol::metadata_response::{
        MetadataResponse, MetadataResponseBroker, MetadataResponseBrokerKey,
    };
    use kafka_connector_protocol::request_header::RequestHeader;
    use kafka_connector_protocol::response_header::ResponseHeader;
    use kafka_connector_protocol::{ApiKey, ApiRequest, ApiResponse, ApiVersion};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::{TcpListener, TcpStream};

    pub async fn send_server_response<R: ApiResponse>(connection: &mut TcpStream, response: R) {
        let mut buffer = BytesMut::with_capacity(1_024);

        let mut buffer_size = [0; 4];
        connection.read_exact(&mut buffer_size).await.unwrap();
        let message_size = i32::from_be_bytes(buffer_size);
        buffer.reserve(message_size as usize);
        unsafe {
            buffer.set_len(message_size as usize);
        }
        connection.read_exact(&mut buffer).await.unwrap();

        let request_header = RequestHeader::deserialize(ApiVersion(0), &mut buffer);
        assert_eq!(
            ApiKey(request_header.request_api_key),
            R::Request::get_api_key()
        );
        let _ =
            R::Request::deserialize(ApiVersion(request_header.request_api_version), &mut buffer);

        let response_header = ResponseHeader {
            correlation_id: request_header.correlation_id,
        };
        response_header
            .serialize(ApiVersion(0), &mut buffer)
            .expect("Failed to serialize initial Api Version Response Header");
        response
            .serialize(ApiVersion(request_header.request_api_version), &mut buffer)
            .expect("Failed to serialize initial Api Version Response");
        let len = buffer.len() as i32;
        connection.write_all(&len.to_be_bytes()).await.unwrap();
        connection.write_all(&buffer).await.unwrap();
        buffer.clear();
    }

    pub async fn initialize_as_single_broker_cluster(server: &TcpListener) {
        let (mut connection, _) = server.accept().await.unwrap();

        let mut api_versions_response = ApiVersionsResponse::default();
        api_versions_response.api_keys.insert(
            ApiVersionsResponseKeyKey {
                index: MetadataRequest::get_api_key().0,
            },
            ApiVersionsResponseKey {
                min_version: ApiVersionsResponse::get_min_supported_version().0,
                max_version: ApiVersionsResponse::get_max_supported_version().0,
            },
        );
        send_server_response(&mut connection, api_versions_response).await;

        let mut metadata_response = MetadataResponse::default();
        metadata_response.brokers.insert(
            MetadataResponseBrokerKey { node_id: 0 },
            MetadataResponseBroker {
                port: server.local_addr().unwrap().port() as i32,
                host: server.local_addr().unwrap().ip().to_string(),
                rack: None,
            },
        );
        send_server_response(&mut connection, metadata_response).await;
    }
}
