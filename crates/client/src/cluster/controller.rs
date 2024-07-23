use crate::{
    broker::controller::{BrokerController, BrokerStatus},
    cluster::{
        error::{ClusterControllerCreationError, ClusterControllerInitializationError},
        options::ClusterControllerOptions,
        send_api_request::send_api_request,
    },
};
use bytes::BytesMut;
use indexmap::IndexMap;
use kafka_connector_protocol::{
    api_versions_request::ApiVersionsRequest, api_versions_response::ApiVersionsResponseKeyKey,
    metadata_request::MetadataRequest, metadata_response::MetadataResponse,
    request_header::RequestHeader, ApiRequest,
};
use std::{
    fmt::Debug,
    net::{SocketAddr, ToSocketAddrs},
};
use tokio::{net::TcpStream, time::timeout};
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
        Err(ClusterControllerCreationError::OutOfRetryAttempts(
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

        let api_versions_response = send_api_request(
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
        Ok(send_api_request(
            &metadata_request,
            &mut buffer,
            metadata_request_version,
            &mut request_header,
            &mut connection,
            options.request_timeout,
        )
        .await?)
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

        #[test]
        fn errors_if_bootstrap_servers_is_empty() {
            unimplemented!()
        }
        #[test]
        fn connects_and_initializes_broker_clients_successfully() {
            unimplemented!()
        }

        #[test]
        fn connects_to_second_broker_if_first_one_fails() {
            unimplemented!()
        }

        #[test]
        fn retries_with_specified_amount_of_times_and_delay() {
            // with retry delay
            unimplemented!()
        }
        #[test]
        fn errors_if_could_not_connect_to_any_broker() {
            unimplemented!()
        }

        #[test]
        fn handles_timeout_on_connect_operation() {
            unimplemented!()
        }
        #[test]
        fn handles_rejection_of_tcp_connection() {
            unimplemented!()
        }
    }
}
