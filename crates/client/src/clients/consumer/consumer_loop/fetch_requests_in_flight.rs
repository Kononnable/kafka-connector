use crate::cluster::error::ApiCallError;
use crate::protocol_consts::Broker;
use futures::future::{SelectAll, select_all};
use kafka_connector_protocol::fetch_response::FetchResponse;
use std::collections::HashSet;
use std::ops::Not;
use std::pin::Pin;

pub(super) type FetchApiCall =
    Pin<Box<dyn Future<Output = (Broker, Result<FetchResponse, ApiCallError>)> + Send>>;
pub(super) type FetchApiCalls = Pin<Box<SelectAll<FetchApiCall>>>;

pub(super) struct FetchRequestsInFlight {
    brokers: HashSet<Broker>,
    brokers_with_active_requests: HashSet<Broker>,
    futures: Option<FetchApiCalls>,
}
impl FetchRequestsInFlight {
    // TODO: add some form of persistent iterator, so consuming messages from different brokers is fair
    pub fn new(
        // only brokers used in consumer(that have partitions that consumer is consuming from)
        broker_list: HashSet<Broker>,
    ) -> FetchRequestsInFlight {
        FetchRequestsInFlight {
            brokers: broker_list.clone(),
            brokers_with_active_requests: HashSet::new(),
            futures: None,
        }
    }
    pub fn get_brokers_to_send_requests_to(&self) -> Vec<Broker> {
        self.brokers
            .iter()
            .filter(|x| !self.brokers_with_active_requests.contains(*x))
            .cloned()
            .collect()
    }
    pub fn send_requests_to_brokers(&mut self, requests: Vec<(Broker, FetchApiCall)>) {
        let mut futures = self
            .futures
            .take()
            .map(|x| Pin::into_inner(x).into_inner())
            .unwrap_or_default();
        for (broker_id, future) in requests {
            self.brokers_with_active_requests.insert(broker_id);
            futures.push(future);
        }
        self.futures = Some(Box::pin(select_all(futures)));
    }

    pub async fn future(
        &mut self,
    ) -> (
        (Broker, Result<FetchResponse, ApiCallError>),
        usize,
        Vec<FetchApiCall>,
    ) {
        if let Some(api_call) = &mut self.futures {
            api_call.await
        } else {
            std::future::pending().await
        }
    }
    pub fn process_fetch_response(&mut self, broker_ids: Vec<Broker>, futures: Vec<FetchApiCall>) {
        for broker_id in broker_ids {
            self.brokers_with_active_requests.remove(&broker_id);
        }
        self.futures = futures
            .is_empty()
            .not()
            .then(|| Box::pin(select_all(futures)));
    }
}
