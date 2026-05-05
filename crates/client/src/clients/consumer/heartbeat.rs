use crate::clients::consumer::consumer_loop::ConsumerLoopType;
use crate::cluster::controller::ClusterController;
use crate::cluster::error::ApiCallError;
use derivative::Derivative;
use futures::future::Either;
use kafka_connector_protocol::heartbeat_request::HeartbeatRequest;
use kafka_connector_protocol::heartbeat_response::HeartbeatResponse;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Sleep;

type HeartbeatCall = dyn Future<Output = Result<HeartbeatResponse, ApiCallError>> + Send;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Heartbeat {
    heartbeat_duration: Duration,
    #[derivative(Debug = "ignore")]
    heartbeat_future: Either<Pin<Box<Sleep>>, Pin<Box<HeartbeatCall>>>,
}
impl Heartbeat {
    pub fn new(heartbeat_duration: Duration) -> Heartbeat {
        Heartbeat {
            heartbeat_duration,
            heartbeat_future: Either::Left(Box::pin(tokio::time::sleep(heartbeat_duration))),
        }
    }
    pub async fn heartbeat(
        &mut self,
        controller: &Arc<ClusterController>,
        parent: &ConsumerLoopType,
    ) -> Option<Result<HeartbeatResponse, ApiCallError>> {
        let ConsumerLoopType::Group {
            group_id,
            generation_id,
            member_id,
            coordinator,
            ..
        } = parent
        else {
            panic!("Group type is only supported for autocommit");
        };
        // TODO docs: cancel safety
        match &mut self.heartbeat_future {
            Either::Left(sleep_fut) => {
                sleep_fut.await;
                let request = HeartbeatRequest {
                    group_id: group_id.clone(),
                    generationid: *generation_id,
                    member_id: member_id.clone(),
                };
                let controller = controller.clone();
                let group_coordinator = *coordinator;
                self.heartbeat_future = Either::Right(Box::pin(async move {
                    controller
                        .make_api_call(group_coordinator, request, None)
                        .await
                }));
                None
            }
            Either::Right(heartbeat_fut) => {
                let resp = heartbeat_fut.await; // order is important - cancel safe
                self.heartbeat_future =
                    Either::Left(Box::pin(tokio::time::sleep(self.heartbeat_duration)));
                Some(resp)
            }
        }
    }
}
