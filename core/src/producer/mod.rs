use std::sync::Arc;

use log::{debug, error, trace};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    oneshot::{self, Receiver},
};

use crate::{cluster::Cluster, producer::producer_loop::ProducerLoopSignal};

use self::{options::ProducerOptions, record::ProducerRecord};

pub mod options;
mod producer_loop;
pub mod record;

pub struct Producer {
    loop_signal_sender: UnboundedSender<ProducerLoopSignal>,
    cluster: Arc<Cluster>,
    options: ProducerOptions,
}

impl Producer {
    /// Creates kafka producer and waits for it to connect to at least one broker
    pub async fn new(cluster: Arc<Cluster>, options: ProducerOptions) -> Self {
        let (loop_signal_sender, loop_signal_receiver) = unbounded_channel();

        tokio::spawn(producer_loop::producer_loop(
            loop_signal_receiver,
            cluster.inner.clone(),
        ));
        Producer {
            cluster,
            options,
            loop_signal_sender,
        }
    }
    pub async fn send(&mut self, record: ProducerRecord) -> SendFuture {
        trace!("Send start");

        let (sender, receiver) = oneshot::channel();
        self.loop_signal_sender
            .send(ProducerLoopSignal::SendMessage(record, sender))
            .expect("Producer loop dead");
        receiver
    }
}

type SendFuture = Receiver<()>;

impl Drop for Producer {
    fn drop(&mut self) {
        debug!("Producer is being dropped");
        let result = self.loop_signal_sender.send(ProducerLoopSignal::Shutdown);

        if result.is_err() {
            error!("Producer dropped when loop is already dead");
        }
    }
}
