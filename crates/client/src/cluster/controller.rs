use crate::broker::broker_metadata::BrokerMetadata;
use crate::cluster::error::ApiCallError;
use crate::{
    broker::controller::{BrokerController, BrokerControllerStatus},
    cluster::options::ClusterControllerOptions,
};
use indexmap::IndexMap;
use kafka_connector_protocol::metadata_request::{MetadataRequest, MetadataRequestTopic};
use kafka_connector_protocol::metadata_response::{
    MetadataResponseBroker, MetadataResponseBrokerKey, MetadataResponseTopic,
};
use kafka_connector_protocol::{ApiRequest, ApiVersion};
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use std::{fmt::Debug, sync::Arc};
use tokio::time::{sleep, timeout};
use tracing::{debug, instrument};

pub type BrokerList = Mutex<HashMap<i32, (BrokerController, Arc<Mutex<BrokerMetadata>>)>>;

/// Main entrypoint for communication with Kafka cluster.
pub struct ClusterController {
    broker_list: BrokerList,
    pub options: Arc<ClusterControllerOptions>,
    topic_metadata_cache: Mutex<HashMap<String, MetadataResponseTopic>>,
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
    pub async fn new(options: ClusterControllerOptions) -> ClusterController {
        assert!(
            !options.bootstrap_servers.is_empty(),
            "Kafka bootstrap servers not provided"
        );
        let options = Arc::new(options);

        let broker_list = Mutex::new(
            options
                .bootstrap_servers
                .iter()
                .enumerate()
                .map(|(enumerator, (host, port))| {
                    let fake_broker_id = -(enumerator as i32) - 1;
                    let metadata = Arc::new(Mutex::new(BrokerMetadata {
                        broker_id: fake_broker_id,
                        host: host.clone(),
                        port: *port as i32,
                        rack: None,
                    }));
                    (
                        fake_broker_id,
                        (
                            BrokerController::new(metadata.clone(), options.clone()),
                            metadata,
                        ),
                    )
                })
                .collect(),
        );

        let topic_metadata_refresh =
            Mutex::new(Instant::now().add(options.advanced.metadata_refresh_interval));

        let controller = ClusterController {
            broker_list,
            options,
            topic_metadata_cache: Mutex::new(HashMap::new()),
            topic_metadata_refresh,
        };

        // Get metadata for nodes after first broker connects, synchronize broker list
        while let Err(err) = controller
            .get_metadata(HashSet::new(), ForceRefresh::Yes)
            .await
        {
            debug!("Cluster initialization error: {err:?}")
        }
        controller
    }

    pub fn get_broker_list(&self) -> HashMap<i32, (BrokerMetadata, BrokerControllerStatus)> {
        self.broker_list
            .lock()
            .unwrap()
            .iter()
            .map(|(broker_id, (controller, metadata))| {
                (
                    *broker_id,
                    (metadata.lock().unwrap().clone(), controller.get_status()),
                )
            })
            .collect()
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn make_api_call<R: ApiRequest, I: Into<Option<i32>>>(
        &self,
        broker_id: I,
        request: R,
        version: Option<ApiVersion>,
    ) -> Result<R::Response, ApiCallError> {
        self.make_api_call_internal(broker_id.into(), request, version, true)
            .await
            .map(|response| response.unwrap())
    }
    #[instrument(level = "debug", skip_all)]
    pub async fn make_api_call_without_response<R: ApiRequest, I: Into<Option<i32>>>(
        &self,
        broker_id: I,
        request: R,
        version: Option<ApiVersion>,
    ) -> Result<(), ApiCallError> {
        self.make_api_call_internal(broker_id.into(), request, version, false)
            .await
            .map(|_| ())
    }

    async fn make_api_call_internal<R: ApiRequest>(
        &self,
        broker_id: Option<i32>,
        request: R,
        version: Option<ApiVersion>,
        wait_for_response: bool,
    ) -> Result<Option<R::Response>, ApiCallError> {
        let broker_id = if let Some(broker_id) = broker_id {
            timeout(self.options.request_timeout, async {
                loop {
                    if self
                        .broker_list
                        .lock()
                        .unwrap()
                        .get(&broker_id)
                        .unwrap()
                        .0
                        .get_status()
                        == BrokerControllerStatus::Connected
                    {
                        break broker_id;
                    }
                    sleep(Duration::from_millis(10)).await
                }
            })
            .await
            .map_err(|_| ApiCallError::TimeoutReached)?
        } else {
            self.get_any_connected_broker_id().await?
        };

        let api_call = {
            let broker_list = self.broker_list.lock().unwrap();
            let (broker, _) = broker_list
                .get(&broker_id)
                .ok_or(ApiCallError::BrokerNotFound(broker_id))?;

            let version = version
                .or_else(|| broker.get_max_supported_api_version::<R>())
                .ok_or(ApiCallError::UnsupportedApi(R::get_api_key()))?;

            broker.make_api_call(version, request, wait_for_response)
        };

        api_call.await
    }

    async fn get_any_connected_broker_id(&self) -> Result<i32, ApiCallError> {
        timeout(self.options.request_timeout, async {
            loop {
                let broker_id = self
                    .broker_list
                    .lock()
                    .unwrap()
                    .iter()
                    .find(|(_id, (controller, _))| {
                        controller.get_status() == BrokerControllerStatus::Connected
                    })
                    .map(|(id, _)| *id);
                if let Some(broker_id) = broker_id {
                    return Ok::<i32, ApiCallError>(broker_id);
                }
                sleep(Duration::from_millis(10)).await
            }
        })
        .await
        .map_err(|_| ApiCallError::TimeoutReached)?
    }

    pub async fn get_metadata(
        &self,
        topics: HashSet<String>,
        force_refresh: ForceRefresh,
    ) -> Result<HashMap<String, MetadataResponseTopic>, ApiCallError> {
        self.clear_metadata_cache_if_timeout_reached();

        if force_refresh == ForceRefresh::No
            && let Some(value) = self.fetch_metadata_from_cache(&topics)
        {
            return Ok(value);
        }

        let broker = self.get_any_connected_broker_id().await?;
        let api_version = self
            .broker_list
            .lock()
            .unwrap()
            .get(&broker)
            .unwrap()
            .0
            .get_max_supported_api_version::<MetadataRequest>()
            .unwrap();
        let response = self
            .make_api_call(
                broker,
                MetadataRequest {
                    topics: Some(
                        topics
                            .into_iter()
                            .map(|name| MetadataRequestTopic { name })
                            .collect(),
                    ),
                    allow_auto_topic_creation: api_version.0 < 4,
                },
                Some(api_version),
            )
            .await?;

        let ret_val = {
            let mut topic_cache = self.topic_metadata_cache.lock().unwrap();
            let mut ret_val = HashMap::new();
            for (key, metadata) in response.topics {
                if metadata.error_code.is_none() {
                    topic_cache.insert(key.name.clone(), metadata.clone());
                }
                ret_val.insert(key.name, metadata);
            }
            ret_val
        };

        self.sync_broker_metadata(response.brokers);
        Ok(ret_val)
    }

    fn sync_broker_metadata(
        &self,
        response: IndexMap<MetadataResponseBrokerKey, MetadataResponseBroker>,
    ) {
        let mut broker_list = self.broker_list.lock().unwrap();
        let response_keys = response
            .iter()
            .map(|(&MetadataResponseBrokerKey { node_id }, _)| node_id)
            .collect::<Vec<_>>();
        for (MetadataResponseBrokerKey { node_id }, MetadataResponseBroker { host, port, rack }) in
            response
        {
            if let Some((_, stored_metadata)) = broker_list.get_mut(&node_id) {
                let mut stored_metadata = stored_metadata.lock().unwrap();
                if stored_metadata.host == host && stored_metadata.port == port {
                    if stored_metadata.rack != rack {
                        stored_metadata.rack = rack;
                    }
                } else {
                    debug!(
                        ?node_id,
                        "Broker address change detected - migration to new address will happen on next network error"
                    );
                    *stored_metadata = BrokerMetadata {
                        broker_id: node_id,
                        host,
                        port,
                        rack,
                    };
                }
            } else if let Some(fake_broker_id) = broker_list
                .iter()
                .find(|&(&id, (_, metadata))| {
                    let metadata = metadata.lock().unwrap();
                    id < 0 && metadata.host == host && metadata.port == port
                })
                .map(|(&fake_broker_id, _)| fake_broker_id)
            {
                debug!(
                    "Connected to Kafka cluster. Assigning broker_id: {fake_broker_id} => {node_id}, rack: {rack:?}",
                );
                let (broker, metadata) = broker_list.remove(&fake_broker_id).unwrap();
                {
                    let mut metadata = metadata.lock().unwrap();
                    metadata.broker_id = node_id;
                    metadata.rack = rack;
                }
                broker_list.insert(node_id, (broker, metadata));
            } else {
                let metadata = Arc::new(Mutex::new(BrokerMetadata {
                    broker_id: node_id,
                    host,
                    port,
                    rack,
                }));
                let broker = BrokerController::new(metadata.clone(), self.options.clone());
                broker_list.insert(node_id, (broker, metadata));
            }
        }
        let removed_brokers = broker_list
            .keys()
            .filter(|&x| !response_keys.contains(x))
            .cloned()
            .collect::<Vec<_>>();
        for broker_id in removed_brokers {
            broker_list.remove(&broker_id);
        }
    }

    fn fetch_metadata_from_cache(
        &self,
        topic_names: &HashSet<String>,
    ) -> Option<HashMap<String, MetadataResponseTopic>> {
        let topic_cache = self.topic_metadata_cache.lock().unwrap();
        let mut results = HashMap::new();
        for name in topic_names {
            if let Some(metadata) = topic_cache.get(name) {
                results.insert(name.to_string(), metadata.to_owned());
            } else {
                return None;
            }
        }
        Some(results)
    }

    fn clear_metadata_cache_if_timeout_reached(&self) {
        let mut refresh_timeout = self.topic_metadata_refresh.lock().unwrap();
        if *refresh_timeout < Instant::now() {
            *refresh_timeout = Instant::now() + self.options.advanced.metadata_refresh_interval;
            self.topic_metadata_cache.lock().unwrap().clear()
        }
        drop(refresh_timeout);
    }

    /// Used only in UTs to test higher level structs without any network interaction.
    #[cfg(test)]
    pub(crate) fn fake_new() -> Arc<ClusterController> {
        Arc::new(ClusterController {
            broker_list: Default::default(),
            options: Arc::new(ClusterControllerOptions::default()),
            topic_metadata_cache: Mutex::new(HashMap::new()),
            topic_metadata_refresh: Mutex::new(Instant::now()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod creating_and_initializing {
        use super::*;
        use crate::test_utils::cluster_controller::initialize_as_single_broker_cluster;
        use tokio::net::TcpListener;

        #[test_log::test(tokio::test)]
        #[should_panic = "Kafka bootstrap servers not provided"]
        async fn errors_if_bootstrap_servers_is_empty() {
            ClusterController::new(Default::default()).await;
        }
        #[test_log::test(tokio::test)]
        async fn connects_and_initializes_broker_clients_successfully() {
            let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let local_addr = server.local_addr().unwrap();
            let bootstrap_servers = vec![(local_addr.ip().to_string(), local_addr.port())];

            tokio::spawn(async move {
                initialize_as_single_broker_cluster(&server).await;
            });

            let cluster = ClusterController::new(ClusterControllerOptions {
                bootstrap_servers,
                ..Default::default()
            })
            .await;

            assert_eq!(cluster.get_broker_list().len(), 1);
        }
    }

    mod broker_metadata_synchronization {
        use crate::broker::broker_metadata::BrokerMetadata;
        use crate::broker::controller::BrokerController;
        use crate::cluster::controller::ClusterController;
        use kafka_connector_protocol::metadata_response::{
            MetadataResponseBroker, MetadataResponseBrokerKey,
        };
        use std::ops::Add;
        use std::sync::{Arc, Mutex};
        use std::time::{Duration, Instant};

        async fn prepare(initial_metadata: &[Arc<Mutex<BrokerMetadata>>]) -> ClusterController {
            let broker_list = initial_metadata
                .iter()
                .map(|metadata| {
                    let broker_id = metadata.lock().unwrap().broker_id;
                    let controller = BrokerController::new(metadata.clone(), Default::default());
                    (broker_id, (controller, metadata.clone()))
                })
                .collect();

            ClusterController {
                broker_list: Mutex::new(broker_list),
                options: Arc::new(Default::default()),
                topic_metadata_cache: Mutex::new(Default::default()),
                topic_metadata_refresh: Mutex::new(Instant::now().add(Duration::from_secs(10))),
            }
        }

        #[test_log::test(tokio::test)]
        async fn broker_address_change() {
            let old_broker_metadata = vec![Arc::new(Mutex::new(BrokerMetadata {
                broker_id: 1,
                host: "127.0.0.2".to_string(),
                port: 1234,
                rack: None,
            }))];

            let cluster = prepare(&old_broker_metadata).await;

            cluster.sync_broker_metadata(
                [(
                    MetadataResponseBrokerKey { node_id: 1 },
                    MetadataResponseBroker {
                        host: "127.0.0.3".to_string(),
                        port: 1234,
                        rack: None,
                    },
                )]
                .into(),
            );

            let broker_list = cluster.broker_list.lock().unwrap();
            assert_eq!(broker_list.len(), 1);
            let new_broker_metadata = broker_list.get(&1).unwrap().1.lock().unwrap();
            assert_eq!(new_broker_metadata.broker_id, 1);
            assert_eq!(new_broker_metadata.host, "127.0.0.3");
            assert_eq!(new_broker_metadata.port, 1234);
            assert_eq!(new_broker_metadata.rack, None);

            assert!(
                Arc::strong_count(old_broker_metadata.first().unwrap()) > 1,
                "BrokerController wasn't reused"
            );
        }

        #[test_log::test(tokio::test)]
        async fn broker_rack_change() {
            let old_broker_metadata = vec![Arc::new(Mutex::new(BrokerMetadata {
                broker_id: 1,
                host: "127.0.0.2".to_string(),
                port: 1234,
                rack: None,
            }))];

            let cluster = prepare(&old_broker_metadata).await;

            cluster.sync_broker_metadata(
                [(
                    MetadataResponseBrokerKey { node_id: 1 },
                    MetadataResponseBroker {
                        host: "127.0.0.2".to_string(),
                        port: 1234,
                        rack: Some("rack".to_owned()),
                    },
                )]
                .into(),
            );

            let broker_list = cluster.broker_list.lock().unwrap();
            assert_eq!(broker_list.len(), 1);
            let new_broker_metadata = broker_list.get(&1).unwrap().1.lock().unwrap();
            assert_eq!(new_broker_metadata.broker_id, 1);
            assert_eq!(new_broker_metadata.host, "127.0.0.2");
            assert_eq!(new_broker_metadata.port, 1234);
            assert_eq!(new_broker_metadata.rack, Some("rack".to_owned()));

            assert!(
                Arc::strong_count(old_broker_metadata.first().unwrap()) > 1,
                "BrokerController wasn't reused"
            );
        }
        #[test_log::test(tokio::test)]
        async fn broker_added() {
            let old_broker_metadata = vec![Arc::new(Mutex::new(BrokerMetadata {
                broker_id: 1,
                host: "127.0.0.2".to_string(),
                port: 1234,
                rack: None,
            }))];

            let cluster = prepare(&old_broker_metadata).await;

            cluster.sync_broker_metadata(
                [
                    (
                        MetadataResponseBrokerKey { node_id: 1 },
                        MetadataResponseBroker {
                            host: "127.0.0.2".to_string(),
                            port: 1234,
                            rack: None,
                        },
                    ),
                    (
                        MetadataResponseBrokerKey { node_id: 2 },
                        MetadataResponseBroker {
                            host: "127.0.0.3".to_string(),
                            port: 1234,
                            rack: None,
                        },
                    ),
                ]
                .into(),
            );

            assert_eq!(cluster.broker_list.lock().unwrap().len(), 2);
        }

        #[test_log::test(tokio::test)]
        async fn broker_removed() {
            let old_broker_metadata = vec![
                Arc::new(Mutex::new(BrokerMetadata {
                    broker_id: 1,
                    host: "127.0.0.2".to_string(),
                    port: 1234,
                    rack: None,
                })),
                Arc::new(Mutex::new(BrokerMetadata {
                    broker_id: 2,
                    host: "127.0.0.3".to_string(),
                    port: 1234,
                    rack: None,
                })),
            ];

            let cluster = prepare(&old_broker_metadata).await;

            cluster.sync_broker_metadata(
                [(
                    MetadataResponseBrokerKey { node_id: 2 },
                    MetadataResponseBroker {
                        host: "127.0.0.3".to_string(),
                        port: 1234,
                        rack: None,
                    },
                )]
                .into(),
            );

            let broker_list = cluster.broker_list.lock().unwrap();
            assert_eq!(broker_list.len(), 1);
            let new_broker_metadata = broker_list.get(&2).unwrap().1.lock().unwrap();
            assert_eq!(new_broker_metadata.broker_id, 2);
            assert_eq!(new_broker_metadata.host, "127.0.0.3");
            assert_eq!(new_broker_metadata.port, 1234);
            assert_eq!(new_broker_metadata.rack, None);
        }

        #[test_log::test(tokio::test)]
        async fn cluster_initialization_fake_id_handling() {
            let old_broker_metadata = vec![Arc::new(Mutex::new(BrokerMetadata {
                broker_id: -2,
                host: "127.0.0.2".to_string(),
                port: 1234,
                rack: None,
            }))];

            let cluster = prepare(&old_broker_metadata).await;

            cluster.sync_broker_metadata(
                [(
                    MetadataResponseBrokerKey { node_id: 1 },
                    MetadataResponseBroker {
                        host: "127.0.0.2".to_string(),
                        port: 1234,
                        rack: None,
                    },
                )]
                .into(),
            );

            let broker_list = cluster.broker_list.lock().unwrap();
            assert_eq!(broker_list.len(), 1);
            let new_broker_metadata = broker_list.get(&1).unwrap().1.lock().unwrap();
            assert_eq!(new_broker_metadata.broker_id, 1);
            assert_eq!(new_broker_metadata.host, "127.0.0.2");
            assert_eq!(new_broker_metadata.port, 1234);
            assert_eq!(new_broker_metadata.rack, None);

            assert!(
                Arc::strong_count(old_broker_metadata.first().unwrap()) > 1,
                "BrokerController wasn't reused"
            );
        }
    }
}
