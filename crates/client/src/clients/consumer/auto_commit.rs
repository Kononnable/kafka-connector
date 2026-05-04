use crate::clients::consumer::consumer_loop::ConsumerLoopType;
use crate::cluster::controller::ClusterController;
use crate::cluster::error::ApiCallError;
use derivative::Derivative;
use futures::future::Either;
use kafka_connector_protocol::offset_commit_request::OffsetCommitRequest;
use kafka_connector_protocol::offset_commit_response::OffsetCommitResponse;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Sleep;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct AutoCommit {
    offset_to_commit: HashMap<(String, i32), i64>,
    offset_in_current_offset_commit_request: Option<HashMap<(String, i32), i64>>,
    auto_commit_duration: Duration,
    #[derivative(Debug = "ignore")]
    auto_commit_future: Either<
        Pin<Box<Sleep>>,
        Pin<Box<dyn Future<Output = Result<OffsetCommitResponse, ApiCallError>> + Send>>,
    >,
}
impl AutoCommit {
    pub fn new(auto_commit_duration: Duration) -> AutoCommit {
        AutoCommit {
            offset_to_commit: HashMap::new(),
            offset_in_current_offset_commit_request: None,
            auto_commit_duration,
            auto_commit_future: Either::Left(Box::pin(tokio::time::sleep(auto_commit_duration))),
        }
    }
    pub async fn autocommit(
        &mut self,
        controller: &Arc<ClusterController>,
        parent: &ConsumerLoopType,
    ) -> Option<Result<OffsetCommitResponse, ApiCallError>> {
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
        match &mut self.auto_commit_future {
            Either::Left(sleep_fut) => {
                sleep_fut.await;
                let request = OffsetCommitRequest {
                    group_id: group_id.clone(),
                    generation_id: *generation_id,
                    member_id: member_id.clone(),
                    retention_time_ms: 0, // TODO:
                    topics: vec![],       // TODO:
                };
                let controller = controller.clone();
                let coordinator = *coordinator;
                self.auto_commit_future = Either::Right(Box::pin(async move {
                    controller.make_api_call(coordinator, request, None).await
                }));
                None
            }
            Either::Right(autocommit_fut) => {
                let resp = autocommit_fut.await; // order is important - cancel safe
                self.auto_commit_future =
                    Either::Left(Box::pin(tokio::time::sleep(self.auto_commit_duration)));
                Some(resp)
            }
        }
    }
    pub fn mark_commit_as_procesed(&mut self, topic: String, partition: i32, offset: i64) {
        self.offset_to_commit.insert((topic, partition), offset);
    }
    pub async fn force_store_current_offsets(&mut self) {
        // TODO: wait for current request(if one is in progress)
        // TODO: send new request if there are new offsets to commit and wait for it
    }
}
