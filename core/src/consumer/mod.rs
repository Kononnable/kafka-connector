use std::collections::HashMap;

use self::{
    error::{AckError, MessageError, SubscribeError},
    options::ConsumerOptions,
};

use async_stream::try_stream;
use futures_util::stream::Stream;

pub mod error;
pub mod options;

pub struct Consumer {}

impl Consumer {
    pub fn new(_options: ConsumerOptions) -> Self {
        todo!()
    }
    pub fn stream(
        &mut self,
    ) -> Result<impl Stream<Item = Result<Message, MessageError>>, SubscribeError> {
        // TODO:
        Ok(try_stream! {
            yield Message{
                headers: Default::default(),
                key: Default::default(),
                offset: Default::default(),
                partition: Default::default(),
                payload: Default::default(),
                timestamp: Default::default(),
                topic: Default::default(),
            };
        })
    }
    /// e.g. before rebalance - store offset etc.
    pub fn on_partition_revoked() {
        todo!()
    }
    /// e.g. after rebalance - seek offset if needed etc.
    pub fn on_partition_assigned() {
        todo!()
    }
}

pub struct Message {
    // TODO: Change to zero-copy solution
    pub topic: String,
    pub key: Vec<u8>,
    pub payload: Vec<u8>,
    pub partition: i32,
    pub offset: i64,
    pub timestamp: i64,
    pub headers: HashMap<String, Vec<u8>>,
}
impl Message {
    pub fn ack(&self) -> Result<(), AckError> {
        todo!()
    }
}
