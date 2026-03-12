use crate::common::{KAFKA_TEST_BROKER_1_RACK, KAFKA_TEST_BROKER_2_RACK, KAFKA_TEST_BROKER_3_RACK};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use testcontainers::core::wait::LogWaitStrategy;
use testcontainers::core::{ContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, ContainerRequest, GenericImage, ImageExt};

pub struct SingleNodeCluster {
    pub broker_1: Arc<ContainerAsync<GenericImage>>,
}
impl SingleNodeCluster {
    pub async fn new() -> SingleNodeCluster {
        let broker_1 = kafka_container(1, 1).start().await.unwrap();
        SingleNodeCluster {
            broker_1: Arc::new(broker_1),
        }
    }
}

impl Drop for SingleNodeCluster {
    fn drop(&mut self) {
        thread::scope(|s| {
            s.spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async move {
                        self.broker_1.stop_with_timeout(Some(0)).await.unwrap();
                        tokio::time::sleep(Duration::from_millis(1000)).await;
                    });
            });
        });
    }
}

pub struct ThreeNodeCluster {
    pub broker_1: Arc<ContainerAsync<GenericImage>>,
    pub broker_2: Arc<ContainerAsync<GenericImage>>,
    pub broker_3: Arc<ContainerAsync<GenericImage>>,
}
impl ThreeNodeCluster {
    pub async fn new() -> ThreeNodeCluster {
        let broker_1 = kafka_container(1, 3).start();
        let broker_2 = kafka_container(2, 3).start();
        let broker_3 = kafka_container(3, 3).start();
        let (broker_1, broker_2, broker_3) =
            tokio::try_join!(broker_1, broker_2, broker_3).unwrap();
        ThreeNodeCluster {
            broker_1: Arc::new(broker_1),
            broker_2: Arc::new(broker_2),
            broker_3: Arc::new(broker_3),
        }
    }
}

impl Drop for ThreeNodeCluster {
    fn drop(&mut self) {
        thread::scope(|s| {
            s.spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async move {
                        self.broker_1.stop_with_timeout(Some(0)).await.unwrap();
                        self.broker_2.stop_with_timeout(Some(0)).await.unwrap();
                        self.broker_3.stop_with_timeout(Some(0)).await.unwrap();
                        tokio::time::sleep(Duration::from_millis(1000)).await;
                    });
            });
        });
    }
}

fn kafka_container(node_id: u16, nodes_in_cluster: u32) -> ContainerRequest<GenericImage> {
    let image = "apache/kafka-native";
    let tag = "4.2.0";

    let quorum_voters = (1..=nodes_in_cluster)
        .map(|id| format!("{id}@broker-{id}:9093"))
        .collect::<Vec<_>>()
        .join(",");

    let rack = match node_id {
        1 => KAFKA_TEST_BROKER_1_RACK,
        2 => KAFKA_TEST_BROKER_2_RACK,
        3 => KAFKA_TEST_BROKER_3_RACK,
        _ => unimplemented!(),
    };

    let mut image = GenericImage::new(image, tag)
        .with_wait_for(WaitFor::Log(LogWaitStrategy::stdout(
            "Kafka Server started",
        )))
        .with_container_name(format!("broker-{node_id}"))
        .with_env_var("KAFKA_NODE_ID", format!("{node_id}"))
        .with_env_var("KAFKA_PROCESS_ROLES", "broker,controller")
        .with_env_var(
            "KAFKA_LISTENERS",
            "PLAINTEXT://:9094,PLAINTEXT_HOST://:9092,CONTROLLER://:9093",
        )
        .with_env_var(
            "KAFKA_ADVERTISED_LISTENERS",
            format!("PLAINTEXT://broker-{node_id}:9094,PLAINTEXT_HOST://localhost:{node_id}9092"),
        )
        .with_env_var("KAFKA_INTER_BROKER_LISTENER_NAME", "PLAINTEXT")
        .with_env_var("KAFKA_CONTROLLER_LISTENER_NAMES", "CONTROLLER")
        .with_env_var(
            "KAFKA_LISTENER_SECURITY_PROTOCOL_MAP",
            "CONTROLLER:PLAINTEXT,PLAINTEXT:PLAINTEXT,PLAINTEXT_HOST:PLAINTEXT",
        )
        .with_env_var("KAFKA_CONTROLLER_QUORUM_VOTERS", quorum_voters)
        .with_env_var("KAFKA_GROUP_INITIAL_REBALANCE_DELAY_MS", "0")
        .with_mapped_port(node_id * 10_000 + 9_092, ContainerPort::Tcp(9092))
        .with_network("kafka-cluster");
    if let Some(rack) = rack {
        image = image.with_env_var("KAFKA_BROKER_RACK", rack);
    }
    image
}
