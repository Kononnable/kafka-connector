use crate::{
    broker::controller::{BrokerController, BrokerControllerStatus},
    cluster::{error::ClusterControllerCreationError, options::ClusterControllerOptions},
};
use indexmap::IndexMap;
use std::{fmt::Debug, sync::Arc};
use tokio::net::ToSocketAddrs;
use tokio_stream as stream;
use tokio_stream::StreamExt;

use crate::{
    broker::connection::fetch_initial_broker_list_from_broker, cluster::error::ApiCallError,
};
use kafka_connector_protocol::{metadata_response::MetadataResponse, ApiRequest, ApiVersion};
use tracing::{debug, instrument};

/// Main entrypoint for communication with Kafka cluster.
pub struct ClusterController {
    broker_list: IndexMap<i32, BrokerController>,
    options: Arc<ClusterControllerOptions>,
}

impl ClusterController {
    /// Initializes communication with Kafka cluster.
    /// Will wait for successful connection with first available broker from `bootstrap_servers`.
    #[instrument(level = "debug")]
    pub async fn new(
        bootstrap_servers: Vec<impl ToSocketAddrs + Debug>,
        options: ClusterControllerOptions,
    ) -> Result<ClusterController, ClusterControllerCreationError> {
        let options = Arc::new(options);
        let metadata = Self::fetch_initial_broker_list(bootstrap_servers, &options).await?;

        let broker_list = metadata
            .brokers
            .iter()
            .map(|(k, v)| {
                (
                    k.node_id,
                    BrokerController::new(format!("{}:{}", v.host, v.port), &options, k.node_id),
                )
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
            for address in &bootstrap_servers {
                debug!(?address, "Connecting to kafka broker");
                match fetch_initial_broker_list_from_broker(options, address).await {
                    Ok(resp) => {
                        return Ok(resp.1);
                    }
                    Err(err) => {
                        debug!(?address, ?err, "Failed to connect to broker");
                        continue;
                    }
                }
            }
        }
        Err(ClusterControllerCreationError::OutOfConnectionAttempts(
            options.connection_retires as u16 + 1,
        ))
    }

    /// Returns known broker list with their statuses
    // TODO: Check if needed for 'production', or if rename is needed
    // TODO: should it be tested?
    pub async fn get_broker_list(&self) -> Vec<(i32, (String, BrokerControllerStatus))> {
        stream::iter(self.broker_list.iter())
            .then(|(k, v)| async { (*k, (v.get_address().to_owned(), v.get_status().await)) })
            .collect()
            .await
    }

    pub async fn make_api_call<R: ApiRequest>(
        &self,
        broker_id: i32,
        version: ApiVersion,
        request: R,
    ) -> Result<R::Response, ApiCallError> {
        self.broker_list
            .get(&broker_id)
            .ok_or(ApiCallError::BrokerNotFound(broker_id))?
            .make_api_call(version, request)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod creating_and_initializing {
        use super::*;
        use bytes::BytesMut;
        use kafka_connector_protocol::{
            api_versions_response::{
                ApiVersionsResponse, ApiVersionsResponseKey, ApiVersionsResponseKeyKey,
            },
            metadata_request::MetadataRequest,
            metadata_response::{MetadataResponseBroker, MetadataResponseBrokerKey},
            request_header::RequestHeader,
            response_header::ResponseHeader,
            ApiKey, ApiRequest, ApiResponse,
        };
        use std::{ops::Sub, time::Duration};
        use tokio::{
            io::{AsyncReadExt, AsyncWriteExt},
            net::{TcpListener, TcpStream},
            time::Instant,
        };

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

            let request_header = RequestHeader::deserialize(ApiVersion(0), &mut buffer);
            assert_eq!(
                ApiKey(request_header.request_api_key),
                R::Request::get_api_key()
            );
            let _ = R::Request::deserialize(
                ApiVersion(request_header.request_api_version),
                &mut buffer,
            );

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

        async fn initialize_as_single_broker_cluster(server: &TcpListener) {
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

        #[test_log::test(tokio::test)]
        async fn errors_if_bootstrap_servers_is_empty() {
            let timeout_start = Instant::now();
            let result = ClusterController::new(Vec::<String>::new(), Default::default()).await;
            let connection_delay = Instant::now().sub(timeout_start);
            debug!(?connection_delay);
            assert!(connection_delay.as_millis() < 5);

            assert!(matches!(
                result,
                Err(ClusterControllerCreationError::NoClusterAddressFound)
            ));
        }
        #[test_log::test(tokio::test)]
        async fn connects_and_initializes_broker_clients_successfully() {
            let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let bootstrap_servers = vec![server.local_addr().unwrap()];

            tokio::spawn(async move {
                initialize_as_single_broker_cluster(&server).await;
            });

            let result = ClusterController::new(bootstrap_servers, Default::default()).await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap().get_broker_list().await.len(), 1);
        }

        #[test_log::test(tokio::test)]
        async fn connects_first_available_broker() {
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
                    connection_timeout: Duration::from_millis(1000),
                    connection_retires: 0,
                    ..Default::default()
                },
            )
            .await;
            assert!(result.is_ok());
            assert_eq!(
                result.unwrap().get_broker_list().await[0].1 .0,
                bootstrap_servers[1].to_string()
            );
        }

        #[test_log::test(tokio::test(start_paused = true))]
        async fn retries_with_specified_amount_of_times_and_delay() {
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

        // Fails on windows with default settings - connecting to 255.255.255.0 results in NetworkUnreachable error
        #[cfg(not(target_family = "windows"))]
        #[test_log::test(tokio::test(start_paused = true))]
        async fn handles_timeout_during_connect_operation() {
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

        // Fails on Windows with default settings - connections on 127.0.0.2 are rejected
        #[cfg(not(target_family = "windows"))]
        #[test_log::test(tokio::test)]
        async fn handles_rejection_of_tcp_connection() {
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

        #[test_log::test(tokio::test(start_paused = true))]
        async fn handles_timeout_during_metadata_initialization() {
            let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let bootstrap_servers = vec![server.local_addr().unwrap()];

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

        #[test_log::test(tokio::test)]
        async fn errors_if_connection_closed_during_communication() {
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
