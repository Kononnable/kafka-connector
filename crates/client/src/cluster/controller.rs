use crate::{
    broker::controller::{BrokerController, BrokerStatus},
    cluster::{
        error::{ClusterControllerCreationError, ClusterControllerInitializationError},
        options::ClusterControllerOptions,
    },
};
use bytes::BytesMut;
use indexmap::IndexMap;
use kafka_connector_protocol::{
    api_versions_request::ApiVersionsRequest, api_versions_response::ApiVersionsResponseKeyKey,
    metadata_request::MetadataRequest, metadata_response::MetadataResponse,
    request_header::RequestHeader, ApiRequest, ApiResponse,
};
use std::{
    fmt::Debug,
    net::{SocketAddr, ToSocketAddrs},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::timeout,
};
use tracing::{debug, instrument};

/// Main entrypoint for communication with Kafka cluster.
pub struct ClusterController {
    broker_list: IndexMap<i32, BrokerController>,
    options: ClusterControllerOptions,
}

impl ClusterController {
    /// Initializes communication with Kafka cluster.
    /// Will wait for successful connection with first available broker from `bootstrap_servers`.
    #[instrument(level = "debug")]
    pub async fn new(
        bootstrap_servers: Vec<impl ToSocketAddrs + Debug>,
        options: ClusterControllerOptions,
    ) -> Result<ClusterController, ClusterControllerCreationError> {
        let metadata = Self::fetch_initial_broker_list(bootstrap_servers, &options).await?;

        let broker_list = metadata
            .brokers
            .iter()
            .map(|(k, v)| {
                (k.node_id, BrokerController::new(&v.host, v.port)) // TODO: setup broker controllers
            })
            .collect();
        Ok(Self {
            broker_list,
            options,
        })
    }

    /// Connect to first available bootstrap server and fetch kafka cluster broker list
    #[instrument(level = "debug", skip_all)]
    async fn fetch_initial_broker_list(
        bootstrap_servers: Vec<impl ToSocketAddrs + Debug>,
        options: &ClusterControllerOptions,
    ) -> Result<MetadataResponse, ClusterControllerCreationError> {
        if bootstrap_servers.is_empty() {
            return Err(ClusterControllerCreationError::NoClusterAddressFound);
        }
        for i in 0..=options.connection_retires {
            if i > 0 {
                debug!(
                    "No connection established, retry in {} ms",
                    options.connection_retry_delay.as_millis()
                );
                tokio::time::sleep(options.connection_retry_delay).await;
            }
            debug!(
                "Connecting to kafka cluster attempt {} of {}",
                i + 1,
                options.connection_retires + 1
            );
            for server in &bootstrap_servers {
                let addresses = {
                    match server.to_socket_addrs() {
                        Ok(addr) => addr,
                        Err(err) => {
                            debug!(?server, ?err, "Failed to resolve bootstrap address");
                            continue;
                        }
                    }
                };
                for address in addresses {
                    debug!(?address, "Connecting to kafka broker");
                    match Self::fetch_initial_broker_list_from_broker(options, address).await {
                        Ok(resp) => {
                            return Ok(resp);
                        }
                        Err(err) => {
                            debug!(?address, ?err, "Failed to connect to broker");
                            continue;
                        }
                    }
                }
            }
        }
        Err(ClusterControllerCreationError::OutOfConnectionAttempts(
            options.connection_retires as u16 + 1,
        ))
    }

    async fn fetch_initial_broker_list_from_broker(
        options: &ClusterControllerOptions,
        address: SocketAddr,
    ) -> Result<MetadataResponse, ClusterControllerInitializationError> {
        let mut connection = timeout(options.connection_timeout, TcpStream::connect(address))
            .await
            .map_err(|_| ClusterControllerInitializationError::ConnectionTimeoutReached)?
            .map_err(ClusterControllerInitializationError::ConnectionError)?;

        let mut buffer = BytesMut::with_capacity(1_024);

        let api_versions_request = ApiVersionsRequest::default();
        let mut request_header = RequestHeader {
            client_id: options.client_name.to_owned(),
            ..Default::default()
        };

        let api_versions_response = Self::send_api_request(
            &api_versions_request,
            &mut buffer,
            0,
            &mut request_header,
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
        Self::send_api_request(
            &metadata_request,
            &mut buffer,
            metadata_request_version,
            &mut request_header,
            &mut connection,
            options.request_timeout,
        )
        .await
    }

    /// Used only for initial cluster metadata fetch, outside of that `BrokerController` is solely responsible for direct communication with kafka brokers
    async fn send_api_request<R: ApiRequest>(
        request: &R,
        buffer: &mut BytesMut,
        api_version: i16,
        request_header: &mut RequestHeader,
        connection: &mut TcpStream,
        request_timeout: Duration,
    ) -> Result<R::Response, ClusterControllerInitializationError> {
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
                .map_err(ClusterControllerInitializationError::NetworkError)?;
            connection
                .write_all(buffer)
                .await
                .map_err(ClusterControllerInitializationError::NetworkError)?;
            buffer.clear();

            let mut buffer_size = [0; 4];
            connection
                .read_exact(&mut buffer_size)
                .await
                .map_err(ClusterControllerInitializationError::NetworkError)?;
            let message_size = i32::from_be_bytes(buffer_size);

            buffer.reserve(message_size as usize);
            unsafe {
                buffer.set_len(message_size as usize);
            }
            connection
                .read_exact(buffer)
                .await
                .map_err(ClusterControllerInitializationError::NetworkError)?;
            Ok(R::Response::deserialize(api_version, buffer).1)
        })
        .await
        .map_err(|_| ClusterControllerInitializationError::ApiCallTimeoutReached)?
    }

    /// Returns known broker list with their statuses
    // TODO: Check if needed for 'production', or if rename is needed
    // TODO: should it be tested?
    pub fn get_broker_list(&self) -> IndexMap<i32, (String, BrokerStatus)> {
        self.broker_list
            .iter()
            .map(|(k, v)| (*k, (v.get_address().to_owned(), v.get_status())))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod creating_and_initializing {
        use super::*;
        use kafka_connector_protocol::{
            api_versions_response::{ApiVersionsResponse, ApiVersionsResponseKey},
            metadata_response::{MetadataResponseBroker, MetadataResponseBrokerKey},
            response_header::ResponseHeader,
            ApiResponse,
        };
        use std::{ops::Sub, time::Duration};
        use tokio::{
            io::{AsyncReadExt, AsyncWriteExt},
            net::TcpListener,
            time::Instant,
        };
        use tracing::Level;
        use tracing_subscriber::FmtSubscriber;

        // TODO: Remove from start of every test(before all)
        fn setup_tracing() {
            let my_subscriber = FmtSubscriber::builder()
                .with_max_level(Level::DEBUG)
                .with_test_writer()
                .compact()
                .finish();
            let _ = tracing::subscriber::set_global_default(my_subscriber);
            // .expect("setting tracing default failed");
        }
        async fn send_server_response<R: ApiResponse>(connection: &mut TcpStream, response: R) {
            let mut buffer = BytesMut::with_capacity(1_024);

            let mut buffer_size = [0; 4];
            connection.read_exact(&mut buffer_size).await.unwrap();
            let message_size = i32::from_be_bytes(buffer_size);
            buffer.reserve(message_size as usize);
            unsafe {
                buffer.set_len(message_size as usize);
            }
            connection.read_exact(&mut buffer).await.unwrap();

            let request_header = RequestHeader::deserialize(0, &mut buffer);
            assert_eq!(request_header.request_api_key, R::Request::get_api_key());
            let _ = R::Request::deserialize(request_header.request_api_version, &mut buffer);

            let response_header = ResponseHeader {
                correlation_id: request_header.correlation_id,
            };
            response
                .serialize(
                    request_header.request_api_version,
                    &mut buffer,
                    &response_header,
                )
                .expect("Failed to serialize initial Api Request");
            let len = buffer.len() as i32;
            connection.write_all(&len.to_be_bytes()).await.unwrap();
            connection.write_all(&buffer).await.unwrap();
            buffer.clear();
        }
        async fn initialize_as_single_broker_cluster(server: &TcpListener) {
            let (mut connection, _) = server.accept().await.unwrap();

            let mut api_versions_response = ApiVersionsResponse::default();
            api_versions_response.api_keys.insert(
                ApiVersionsResponseKeyKey {
                    index: MetadataRequest::get_api_key(),
                },
                ApiVersionsResponseKey {
                    min_version: ApiVersionsResponse::get_min_supported_version(),
                    max_version: ApiVersionsResponse::get_max_supported_version(),
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

        #[tokio::test]
        async fn errors_if_bootstrap_servers_is_empty() {
            setup_tracing();

            let timeout_start = Instant::now();
            let result =
                ClusterController::new(Vec::<String>::new(), ClusterControllerOptions::default())
                    .await;
            let connection_delay = Instant::now().sub(timeout_start);
            debug!(?connection_delay);
            assert!(connection_delay.as_millis() < 5);

            assert!(matches!(
                result,
                Err(ClusterControllerCreationError::NoClusterAddressFound)
            ));
        }
        #[tokio::test]
        async fn connects_and_initializes_broker_clients_successfully() {
            setup_tracing();
            let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let bootstrap_servers = vec![server.local_addr().unwrap()];

            tokio::spawn(async move {
                initialize_as_single_broker_cluster(&server).await;
            });

            let result =
                ClusterController::new(bootstrap_servers, ClusterControllerOptions::default())
                    .await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap().get_broker_list().len(), 1);
        }

        #[tokio::test]
        async fn connects_first_available_broker() {
            setup_tracing();
            let first_server = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let second_server = TcpListener::bind("127.0.0.2:0").await.unwrap();
            let bootstrap_servers = vec![
                first_server.local_addr().unwrap(),
                second_server.local_addr().unwrap(),
            ];

            tokio::spawn(async move {
                // Accept connection and reset it right away
                first_server.accept().await.unwrap();

                initialize_as_single_broker_cluster(&second_server).await;
            });

            let result = ClusterController::new(
                bootstrap_servers.clone(),
                ClusterControllerOptions {
                    connection_timeout: Duration::from_millis(10),
                    connection_retires: 0,
                    ..Default::default()
                },
            )
            .await;
            assert!(result.is_ok());
            assert_eq!(
                result.unwrap().get_broker_list()[0].0,
                bootstrap_servers[1].to_string()
            );
        }

        #[tokio::test(start_paused = true)]
        async fn retries_with_specified_amount_of_times_and_delay() {
            setup_tracing();

            let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let bootstrap_servers = vec![server.local_addr().unwrap()];

            tokio::spawn(async move {
                let attempt_1 = server.accept().await;
                drop(attempt_1);

                let timeout_start = Instant::now();
                let attempt_2 = server.accept().await;
                let connection_delay = Instant::now().sub(timeout_start);
                assert!(9_999 < connection_delay.as_millis());
                assert!(connection_delay.as_millis() < 10_005);
                drop(attempt_2);

                let timeout_start = Instant::now();
                let attempt_3 = server.accept().await;
                let connection_delay = Instant::now().sub(timeout_start);
                assert!(9_999 < connection_delay.as_millis());
                assert!(connection_delay.as_millis() < 10_005);
                drop(attempt_3);
            });

            let timeout_start = Instant::now();
            let result = ClusterController::new(
                bootstrap_servers,
                ClusterControllerOptions {
                    connection_retry_delay: Duration::from_millis(10_000),
                    connection_retires: 2,
                    connection_timeout: Duration::from_millis(20),
                    ..Default::default()
                },
            )
            .await;

            let connection_delay = Instant::now().sub(timeout_start);
            debug!(?connection_delay);
            assert!(19_999 < connection_delay.as_millis());
            assert!(connection_delay.as_millis() < 20_005);

            assert!(result.is_err());
            assert!(matches!(
                result,
                Err(ClusterControllerCreationError::OutOfConnectionAttempts(3))
            ));
        }

        #[tokio::test(start_paused = true)]
        async fn handles_timeout_on_connect_operation() {
            setup_tracing();

            let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let mut bootstrap_servers = vec![server.local_addr().unwrap()];
            //255.255.255.0 is a class E address, reserved for research, so it can act as a black hole
            bootstrap_servers[0].set_ip("255.255.255.0".parse().unwrap());

            let timeout_start = Instant::now();
            let result = ClusterController::new(
                bootstrap_servers,
                ClusterControllerOptions {
                    connection_retires: 0,
                    connection_timeout: Duration::from_millis(29_000),
                    ..Default::default()
                },
            )
            .await;

            let connection_delay = Instant::now().sub(timeout_start);
            debug!(?connection_delay);
            assert!(28_999 < connection_delay.as_millis());
            assert!(connection_delay.as_millis() < 29_005);

            assert!(result.is_err());
            assert!(matches!(
                result,
                Err(ClusterControllerCreationError::OutOfConnectionAttempts(1))
            ));
        }
        #[tokio::test]
        async fn handles_rejection_of_tcp_connection() {
            setup_tracing();

            let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let mut bootstrap_servers = vec![server.local_addr().unwrap()];
            // 127.0.0.2 is a loopback address
            // we're only listening on different loopback address, so connection will be refused
            bootstrap_servers[0].set_ip("127.0.0.2".parse().unwrap());

            let timeout_start = Instant::now();
            let result = ClusterController::new(
                bootstrap_servers,
                ClusterControllerOptions {
                    connection_retires: 0,
                    ..Default::default()
                },
            )
            .await;

            let connection_delay = Instant::now().sub(timeout_start);
            debug!(?connection_delay);
            assert!(connection_delay.as_millis() < 5);

            assert!(result.is_err());
            assert!(matches!(
                result,
                Err(ClusterControllerCreationError::OutOfConnectionAttempts(1))
            ));
        }

        #[tokio::test(start_paused = true)]
        async fn handles_timeout_on_api_call() {
            setup_tracing();

            let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let bootstrap_servers = vec![server.local_addr().unwrap()];

            let timeout_start = Instant::now();
            let result = ClusterController::new(
                bootstrap_servers,
                ClusterControllerOptions {
                    connection_retires: 0,
                    request_timeout: Duration::from_millis(29_000),
                    ..Default::default()
                },
            )
            .await;

            let connection_delay = Instant::now().sub(timeout_start);
            debug!(?connection_delay);
            assert!(28_999 < connection_delay.as_millis());
            assert!(connection_delay.as_millis() < 29_005);

            assert!(result.is_err());
            assert!(matches!(
                result,
                Err(ClusterControllerCreationError::OutOfConnectionAttempts(1))
            ));
        }

        #[tokio::test]
        async fn errors_if_connection_closed_during_communication() {
            setup_tracing();

            let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let bootstrap_servers = vec![server.local_addr().unwrap()];

            tokio::spawn(async move {
                let (mut connection, _) = server.accept().await.unwrap();

                let mut buffer = BytesMut::with_capacity(1_024);

                let mut buffer_size = [0; 4];
                connection.read_exact(&mut buffer_size).await.unwrap();
                let message_size = i32::from_be_bytes(buffer_size);
                buffer.reserve(message_size as usize);
                unsafe {
                    buffer.set_len(message_size as usize);
                }
                connection.read_exact(&mut buffer).await.unwrap();

                let len = 4i32; // Indicate more data is coming
                connection.write_all(&len.to_be_bytes()).await.unwrap();

                drop(server);
            });

            let timeout_start = Instant::now();
            let result = ClusterController::new(
                bootstrap_servers,
                ClusterControllerOptions {
                    connection_retires: 0,
                    ..Default::default()
                },
            )
            .await;

            let connection_delay = Instant::now().sub(timeout_start);
            debug!(?connection_delay);
            assert!(connection_delay.as_millis() < 5);

            assert!(result.is_err());
            assert!(matches!(
                result,
                Err(ClusterControllerCreationError::OutOfConnectionAttempts(1))
            ));
        }
    }
}
