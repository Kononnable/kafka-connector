use std::sync::Arc;

use log::{debug, error, trace};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    oneshot::{self, Receiver},
};

use crate::{cluster::Cluster, producer::producer_loop::ProducerLoopSignal};

use self::{
    error::MessageSendError, options::ProducerOptions, producer_loop::ProducerLoop,
    record::ProducerRecord,
};

pub mod error;
pub mod options;
mod producer_loop;
pub mod record;

pub struct Producer {
    loop_signal_sender: UnboundedSender<ProducerLoopSignal>,
    cluster: Arc<Cluster>,
    options: Arc<ProducerOptions>,
}

impl Producer {
    /// Creates kafka producer and waits for it to connect to at least one broker
    pub async fn new(cluster: Arc<Cluster>, options: ProducerOptions) -> Self {
        let options = Arc::new(options);
        let (loop_signal_sender, loop_signal_receiver) = unbounded_channel();

        let producer_loop = ProducerLoop {
            cluster: cluster.inner.clone(),
            options: options.clone(),
        };
        tokio::spawn(producer_loop.producer_loop(loop_signal_receiver));
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

        self.loop_signal_sender
            .send(ProducerLoopSignal::SendBatch)
            .expect("Producer loop dead");

        receiver
    }
}

/// Completes when message is received and acknowledged by kafka broker.
/// This behavior depends on `acks` producer setting.
type SendFuture = Receiver<Result<(), (MessageSendError, ProducerRecord)>>;

impl Drop for Producer {
    fn drop(&mut self) {
        debug!("Producer is being dropped");
        let result = self.loop_signal_sender.send(ProducerLoopSignal::Shutdown);

        if result.is_err() {
            error!("Producer dropped when loop is already dead");
        }
    }
}
