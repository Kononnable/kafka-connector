use kafka_connector_client::cluster::controller::ClusterController;
use kafka_connector_protocol::create_topics_request::{CreatableTopic, CreateTopicsRequest};
use kafka_connector_protocol::delete_topics_request::DeleteTopicsRequest;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;

const TIMEOUT_MS: i32 = 1_000;
pub struct TestTopic {
    name: String,
    cluster: Arc<ClusterController>,
    is_deleted: bool,
}
impl TestTopic {
    pub async fn new<S: Into<String>>(
        cluster: Arc<ClusterController>,
        base_name: S,
        options: Option<CreatableTopic>,
    ) -> TestTopic {
        let name = format!(
            "{}_{}",
            base_name.into(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        let mut options = options.unwrap_or(CreatableTopic {
            name: name.clone(),
            num_partitions: 1,
            replication_factor: 1,
            assignments: Default::default(),
            configs: Default::default(),
        });
        options.name = name.clone();
        debug!("Creating test topic {name}");
        let resp = cluster
            .make_api_call(
                1,
                CreateTopicsRequest {
                    topics: vec![options],
                    timeout_ms: TIMEOUT_MS,
                    validate_only: false,
                },
                None,
            )
            .await
            .unwrap();
        assert_eq!(resp.topics.first().unwrap().1.error_message, None);
        assert!(resp.topics.first().unwrap().1.error_code.is_none());
        TestTopic {
            name,
            cluster,
            is_deleted: false,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub async fn delete(mut self) {
        debug!("Deleting test topic {}", self.name);
        assert!(!self.is_deleted);
        let resp = self
            .cluster
            .make_api_call(
                1,
                DeleteTopicsRequest {
                    topic_names: vec![self.name.clone()],
                    timeout_ms: TIMEOUT_MS,
                },
                None,
            )
            .await
            .unwrap();
        assert!(resp.responses.first().unwrap().error_code.is_none());
        self.is_deleted = true;
    }
}
impl Drop for TestTopic {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            assert!(
                self.is_deleted,
                "Need to manually call TestTopic::delete() for topic {}",
                self.name
            )
        }
    }
}
