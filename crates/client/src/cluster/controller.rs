use crate::cluster::error::ClusterControllerCreationError;
use crate::cluster::options::ClusterControllerOptions;
use bytes::BytesMut;
use kafka_connector_protocol::api_versions_request::ApiVersionsRequest;
use kafka_connector_protocol::api_versions_response::{
    ApiVersionsResponseKeyKey,
};
use kafka_connector_protocol::metadata_request::MetadataRequest;
use kafka_connector_protocol::metadata_response::{MetadataResponse};
use kafka_connector_protocol::request_header::RequestHeader;
use kafka_connector_protocol::{ApiRequest, ApiResponse};
use std::net::ToSocketAddrs;
use indexmap::IndexMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::broker::controller::{BrokerController, BrokerStatus};
// TODO: Add tracing
// TODO: protocol level tracing needs to be explicitly enabled(controller options)
// TODO: logging support for tracing(compatibility layer)

/// Main entrypoint for communication with Kafka cluster.
pub struct ClusterController {
    broker_list: IndexMap<i32, BrokerController>,
}


impl ClusterController {
    /// Initializes communication with Kafka cluster.
    /// Will wait for successful connection with first available broker from `bootstrap_servers`.
    pub async fn new(
        bootstrap_servers: Vec<impl ToSocketAddrs>,
        options: ClusterControllerOptions,
    ) -> Result<ClusterController, ClusterControllerCreationError> {
        let metadata = Self::initial_broker_list_fetch(bootstrap_servers, &options).await;

        let broker_list = metadata.brokers.iter().map(|(k, v)| {
            (k.node_id, BrokerController::new(&v.host, v.port)) // TODO: setup broker controllers
        }).collect();
        Ok(Self {
            broker_list,
        })
    }

    /// Returns known broker list with their statuses
    // TODO: Check if needed for 'production', or if rename is needed
    pub fn get_broker_list(&self) -> IndexMap<i32, (String, BrokerStatus)> {
        self.broker_list.iter().map(|(k, v)| (*k, (v.get_address().to_owned(), v.get_status()))).collect()
    }

    /// Connect to first available bootstrap server and fetch kafka cluster broker list
    async fn initial_broker_list_fetch(bootstrap_servers: Vec<impl ToSocketAddrs>, _options: &ClusterControllerOptions) -> MetadataResponse {
        for addr in bootstrap_servers {
            for addresses in addr.to_socket_addrs().expect("TODO") {
                // TODO: Handle timeout
                let mut connection = match TcpStream::connect(addresses).await {
                    Ok(connection) => connection,
                    Err(_err) => {
                        // TODO: Handle error
                        continue;
                    }
                };


                let mut buffer = BytesMut::with_capacity(1_000); // TODO:

                let api_versions_request = ApiVersionsRequest {};

                let api_versions_response =
                    send_api_request(&api_versions_request, &mut buffer, 0, 0, &mut connection)
                        .await;

                let mut metadata_request_version = api_versions_response
                    .api_keys
                    .get(&ApiVersionsResponseKeyKey {
                        index: MetadataRequest::get_api_key(),
                    })
                    .expect("TODO")
                    .max_version;
                if metadata_request_version > MetadataRequest::get_max_supported_version() {
                    metadata_request_version = MetadataRequest::get_max_supported_version();
                }
                let metadata_request = MetadataRequest {
                    topics: Some(vec![]),
                    allow_auto_topic_creation: false,
                };
                return send_api_request(
                    &metadata_request,
                    &mut buffer,
                    metadata_request_version,
                    1,
                    &mut connection,
                )
                    .await;
            }
        }
        todo!();

        /// Outside of initial cluster metadata fetch `BrokerController` is solely responsible for communication with kafka brokers
        async fn send_api_request<R: ApiRequest>(
            request: &R,
            buffer: &mut BytesMut,
            api_version: i16,
            correlation_id: i32,
            connection: &mut TcpStream,
        ) -> R::Response {
            request
                .serialize(
                    api_version,
                    buffer,
                    &RequestHeader {
                        request_api_key: R::get_api_key(),
                        request_api_version: api_version,
                        correlation_id,
                        client_id: "TODO".to_string(),
                    },
                )
                .expect("TODO");
            let len = buffer.len() as i32;
            connection.write_all(&len.to_be_bytes()).await.unwrap();
            connection.write_all(buffer).await.unwrap();
            buffer.clear();

            let mut buffer_size = [0; 4];
            connection.read_exact(&mut buffer_size).await.expect("TODO");
            let message_size = i32::from_be_bytes(buffer_size);

            buffer.reserve(message_size as usize);
            unsafe {
                buffer.set_len(message_size as usize);
            }
            connection.read_exact(buffer).await.expect("TODO");

            R::Response::deserialize(api_version, buffer).1
        }
    }
}
