use crate::{
    broker::controller::{BrokerController, BrokerControllerStatus},
    cluster::{error::ClusterControllerCreationError, options::ClusterControllerOptions},
};
use indexmap::IndexMap;
use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Mutex, RwLock};
use std::time::Instant;
use std::{fmt::Debug, sync::Arc};
use tokio::net::ToSocketAddrs;

use crate::broker::broker_metadata::BrokerMetadata;
use crate::{
    broker::connection::fetch_initial_broker_list_from_broker, cluster::error::ApiCallError,
};
use kafka_connector_protocol::api_versions_response::ApiVersionsResponseKey;
use kafka_connector_protocol::metadata_request::{MetadataRequest, MetadataRequestTopic};
use kafka_connector_protocol::metadata_response::{
    MetadataResponseTopic, MetadataResponseTopicKey,
};
use kafka_connector_protocol::{ApiRequest, ApiVersion, metadata_response::MetadataResponse};
use tracing::{debug, instrument};

/// Main entrypoint for communication with Kafka cluster.
pub struct ClusterController {
    broker_list: HashMap<i32, BrokerController>,
    options: Arc<ClusterControllerOptions>,
    topic_metadata_cache: RwLock<HashMap<String, MetadataResponseTopic>>,
    topic_metadata_refresh: Mutex<Instant>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ForceRefresh {
    Yes,
    No,
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
        let (supported_apis, metadata) =
            Self::fetch_initial_broker_list(bootstrap_servers, &options).await?;

        let topic_metadata_cache = metadata
            .topics
            .into_iter()
            .filter(|x| x.1.error_code == 0)
            .map(|(k, v)| (k.name, v))
            .collect();

        let broker_list = metadata
            .brokers
            .into_iter()
            .map(|(k, v)| {
                (
                    k.node_id,
                    BrokerController::new(v.into(), k.node_id, &options, supported_apis.clone()),
                )
            })
            .collect();
        let topic_metadata_refresh =
            Mutex::new(Instant::now().add(options.metadata_refresh_interval));
        Ok(Self {
            broker_list,
            options,
            topic_metadata_cache: RwLock::new(topic_metadata_cache),
            topic_metadata_refresh,
        })
    }

    /// Connect to first available bootstrap server and fetch kafka cluster broker list
    #[instrument(level = "debug", skip_all)]
    async fn fetch_initial_broker_list(
        bootstrap_servers: Vec<impl ToSocketAddrs + Debug>,
        options: &ClusterControllerOptions,
    ) -> Result<
        (IndexMap<i16, ApiVersionsResponseKey>, MetadataResponse),
        ClusterControllerCreationError,
    > {
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
                        return Ok((resp.1, resp.2));
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

    /// Returns metadata for known brokers with their statuses
    pub fn get_broker_status_list(&self) -> Vec<(i32, (BrokerMetadata, BrokerControllerStatus))> {
        self.broker_list
            .iter()
            .map(|(k, v)| (*k, (v.get_metadata().to_owned(), v.get_status())))
            .collect()
    }

    pub async fn make_api_call<R: ApiRequest, I: Into<Option<i32>>>(
        &self,
        broker_id: I,
        request: R,
        version: Option<ApiVersion>,
    ) -> Result<R::Response, ApiCallError> {
        // TODO: If broker_id not specified find first available(connected) broker, if no connected wait for connection with timeout (from config)
        // TODO: how to handle timeouts in such case; e.g. situation when machine lost internet connection (it must have been established earlier, otherwise Cluster Controller would not be created)
        let broker_id = broker_id.into().unwrap();
        let broker = self
            .broker_list
            .get(&broker_id)
            .ok_or(ApiCallError::BrokerNotFound(broker_id))?;
        // TODO: Refactor unwrap or else
        let version = if let Some(version) = version {
            version
        } else {
            // TODO: Extract to new method - clients code may want to know if specific fields(features) are supported or not
            let supported_apis = broker.supported_api_versions.read().expect("Poisoned lock");
            let broker_supported_versions = supported_apis
                .get(&R::get_api_key().0)
                .ok_or(ApiCallError::UnsupportedApi(R::get_api_key()))?;
            let max_supported = i16::min(
                broker_supported_versions.max_version,
                R::get_max_supported_version().0,
            );
            let min_supported = i16::max(
                broker_supported_versions.min_version,
                R::get_min_supported_version().0,
            );
            if min_supported > max_supported {
                return Err(ApiCallError::UnsupportedApi(R::get_api_key()));
            }
            ApiVersion(max_supported)
        };
        broker.make_api_call(version, request).await
    }

    // TODO: Tests
    // TODO: Extract Metadata cache as a separate struct(?)
    pub(crate) async fn get_topic_metadata<N: Into<String>>(
        &self,
        name: N,
        force_refresh: ForceRefresh,
    ) -> Result<MetadataResponseTopic, ApiCallError> {
        let topic_name = name.into();

        self.clear_metadata_cache_if_timeout_reached();

        if force_refresh == ForceRefresh::No {
            if let Some(value) = self.fetch_metadata_from_cache(&topic_name) {
                return Ok(value);
            }
        }

        let response = self
            .make_api_call(
                Some(1), // None, TODO: Change to None, once using any available broker is implemented in make_api_call
                MetadataRequest {
                    topics: Some(vec![MetadataRequestTopic {
                        name: topic_name.clone(),
                    }]),
                    allow_auto_topic_creation: false,
                },
                None,
            )
            .await?;

        let (MetadataResponseTopicKey { name: key }, metadata) = response
            .topics
            .into_iter()
            .next()
            // TODO: General (unknown) kafka error handling, or is expect ok - it will be common error in the whole code
            .expect("Unexpected Kafka Api Response format");
        assert_eq!(key, topic_name);
        assert_eq!(metadata.error_code, 0); // TODO: General (unknown) kafka error handling

        self.topic_metadata_cache
            .write()
            .unwrap_or_else(|poison| {
                self.topic_metadata_cache.clear_poison();
                let mut cache = poison.into_inner();
                cache.clear();
                cache
            })
            .insert(topic_name, metadata.clone());

        Ok(metadata)

        // TODO: node metadata - act if:
        // new broker added
        // broker removed
        // broker hostname/port/rack changed
        // + tests (?)
        // use same method in controller creation
    }

    fn fetch_metadata_from_cache(&self, topic_name: &str) -> Option<MetadataResponseTopic> {
        let cache = self.topic_metadata_cache.read().unwrap_or_else(|_| {
            self.topic_metadata_cache.clear_poison();
            let mut cache = self.topic_metadata_cache.write().unwrap();
            cache.clear();
            drop(cache);
            self.topic_metadata_cache.read().unwrap()
        });
        if let Some(metadata) = cache.get(topic_name) {
            return Some(metadata.to_owned());
        }
        None
    }

    fn clear_metadata_cache_if_timeout_reached(&self) {
        let mut refresh_timeout = self.topic_metadata_refresh.lock().unwrap_or_else(|poison| {
            self.topic_metadata_refresh.clear_poison();
            let mut lock = poison.into_inner();
            *lock = Instant::now();
            lock
        });
        if *refresh_timeout > Instant::now() {
            *refresh_timeout = Instant::now() + self.options.metadata_refresh_interval;
            self.topic_metadata_cache
                .write()
                .unwrap_or_else(|poison| {
                    self.topic_metadata_cache.clear_poison();
                    poison.into_inner()
                })
                .clear()
        }
        drop(refresh_timeout);
    }

    fn update_broker_metadata() {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod creating_and_initializing {
        use super::*;
        use bytes::BytesMut;
        use kafka_connector_protocol::{
            ApiKey, ApiRequest, ApiResponse,
            api_versions_response::{
                ApiVersionsResponse, ApiVersionsResponseKey, ApiVersionsResponseKeyKey,
            },
            metadata_request::MetadataRequest,
            metadata_response::{MetadataResponseBroker, MetadataResponseBrokerKey},
            request_header::RequestHeader,
            response_header::ResponseHeader,
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
            assert_eq!(result.unwrap().get_broker_status_list().len(), 1);
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
            let broker_list = result.unwrap().get_broker_status_list();
            assert_eq!(
                format!("{}:{}", broker_list[0].1.0.host, broker_list[0].1.0.port),
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
