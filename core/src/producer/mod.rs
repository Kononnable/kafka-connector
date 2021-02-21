use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::kafka_client::KafkaClient;

use self::{error::SendError, options::ProducerOptions};

pub mod error;
pub mod options;

pub struct Producer {}

impl Producer {
    pub fn new(_kafka_client: KafkaClient, _options: ProducerOptions) -> Self {
        todo!()
    }
    pub fn send(&mut self, _record: Record<'_>) -> SendResultFuture {
        todo!()
    }
}

// TODO: Allow some simple types to be passed here directly - like str
pub struct Record<'a> {
    pub topic: &'a str,
    pub payload: &'a [u8],
    pub partition: Option<i32>,
    pub key: &'a [u8],
    pub timestamp: Option<i64>,
    pub headers: Option<&'a HashMap<String, Vec<u8>>>,
}

pub struct SendResultFuture {}
impl Future for SendResultFuture {
    type Output = Result<(), SendError>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}
