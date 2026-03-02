use crate::clients::consumer::consumer_loop::ConsumerLoop;
use crate::clients::consumer::options::KafkaConsumerOptions;
use crate::clients::consumer::record::Record;
use crate::cluster::controller::ClusterController;
use crate::cluster::error::ClusterControllerCreationError;
use crate::cluster::options::ClusterControllerOptions;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::net::ToSocketAddrs;
use tokio::sync::mpsc;

pub struct KafkaConsumer {
    cluster: Arc<ClusterController>,
    options: KafkaConsumerOptions,
    record_channel: mpsc::Receiver<Record>,
    command_channel: mpsc::Sender<()>,
}

impl KafkaConsumer {
    pub async fn new(
        bootstrap_servers: Vec<impl ToSocketAddrs + Debug>,
        connection_options: ClusterControllerOptions,
        consumer_options: KafkaConsumerOptions,
    ) -> Result<KafkaConsumer, ClusterControllerCreationError> {
        let controller = ClusterController::new(bootstrap_servers, connection_options).await?;
        Ok(Self::from_cluster_controller(
            Arc::new(controller),
            consumer_options,
        ))
    }
    pub fn from_cluster_controller(
        controller: Arc<ClusterController>,
        consumer_options: KafkaConsumerOptions,
    ) -> KafkaConsumer {
        let (record_tx, record_rx) = mpsc::channel(1);
        let (command_tx, command_rx) = mpsc::channel(1);
        tokio::spawn(ConsumerLoop::start(
            controller.clone(),
            consumer_options.clone(),
            record_tx,
            command_rx,
        ));
        KafkaConsumer {
            cluster: controller,
            options: consumer_options,
            record_channel: record_rx,
            command_channel: command_tx,
        }
    }

    pub async fn recv(&mut self) -> Record {
        self.record_channel
            .recv()
            .await
            .expect("Consumer loop should be alive")
    }

    pub async fn try_recv(&mut self) -> Option<Record> {
        self.record_channel.try_recv().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::cluster_controller::initialize_as_single_broker_cluster;
    use std::time::Duration;
    use tokio::net::TcpListener;

    #[test_log::test(tokio::test)]
    async fn loop_closes_after_client_drops() {
        let server = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let bootstrap_servers = vec![server.local_addr().unwrap()];

        tokio::spawn(async move {
            initialize_as_single_broker_cluster(&server).await;
        });

        let cluster = Arc::new(
            ClusterController::new(bootstrap_servers, Default::default())
                .await
                .unwrap(),
        );

        let cluster_weak = Arc::downgrade(&cluster);

        let consumer = KafkaConsumer::from_cluster_controller(cluster, Default::default());
        drop(consumer);

        // Wait for loop to exit
        tokio::time::sleep(Duration::from_millis(50)).await;

        assert!(cluster_weak.upgrade().is_none());
    }
}
