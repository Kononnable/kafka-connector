use super::prelude::*;
pub type ProduceRequest = ProduceRequest0;
impl ApiCall for ProduceRequest0 {
    type Response = ProduceResponse0;
    fn get_min_supported_version() -> u16 {
        0
    }
    fn get_max_supported_version() -> u16 {
        8
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::Produce
    }
    fn get_first_error(response: &Self::Response) -> Option<ApiError> {
        {
            Self::Response::get_first_error(response)
        }
    }
    fn is_flexible_version(_version: u16) -> bool {
        false
    }
    fn serialize(&self, version: u16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 2),
            false => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                .serialize(buf, false, 1),
        }
        ToBytes::serialize(self, buf, Self::is_flexible_version(version), version);
    }
    fn deserialize_response(version: u16, buf: &mut Bytes) -> (i32, Self::Response) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse::deserialize(buf, false, 2).correlation,
            false => HeaderResponse::deserialize(buf, false, 1).correlation,
        };
        let response =
            Self::Response::deserialize(buf, Self::is_flexible_version(version), version);
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequest0 {
    #[min_version = 3]
    pub transactional_id: NullableString,
    #[min_version = 0]
    pub acks: Int16,
    #[min_version = 0]
    pub timeout: Int32,
    #[min_version = 0]
    pub topic_data: Vec<ProduceRequestTopicData0>,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicData0 {
    #[min_version = 0]
    pub topic: String,
    #[min_version = 0]
    pub data: Vec<ProduceRequestTopicDataData0>,
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicDataData0 {
    #[min_version = 0]
    pub partition: Int32,
    #[min_version = 0]
    pub record_set: Records,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponse0 {
    #[min_version = 0]
    pub responses: Vec<ProduceResponseResponses0>,
    #[min_version = 1]
    pub throttle_time_ms: Option<Int32>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponses0 {
    #[min_version = 0]
    pub topic: String,
    #[min_version = 0]
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses0>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses0 {
    #[min_version = 0]
    pub partition: Int32,
    #[min_version = 0]
    pub error_code: Int16,
    #[min_version = 0]
    pub base_offset: Int64,
    #[min_version = 2]
    pub log_append_time: Option<Int64>,
    #[min_version = 5]
    pub log_start_offset: Option<Int64>,
    #[min_version = 8]
    pub record_errors: Option<Vec<ProduceResponseResponsesPartitionResponsesRecordErrors0>>,
    #[min_version = 8]
    pub error_message: Option<NullableString>,
}
#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponsesRecordErrors0 {
    #[min_version = 8]
    pub batch_index: Option<Int32>,
    #[min_version = 8]
    pub batch_index_error_message: Option<NullableString>,
}

impl ProduceResponse0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.responses.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl ProduceResponseResponses0 {
    fn get_first_error(&self) -> Option<ApiError> {
        for item in self.partition_responses.iter() {
            if let Some(x) = item.get_first_error() {
                return Some(x);
            };
        }
        None
    }
}
impl ProduceResponseResponsesPartitionResponses0 {
    fn get_first_error(&self) -> Option<ApiError> {
        if let Some(vec) = self.record_errors.as_ref() {
            for item in vec {
                if let Some(x) = item.get_first_error() {
                    return Some(x);
                };
            }
        }
        None
    }
}
impl ProduceResponseResponsesPartitionResponsesRecordErrors0 {
    fn get_first_error(&self) -> Option<ApiError> {
        None
    }
}
