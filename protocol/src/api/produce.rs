use super::prelude::*;

pub type ProduceRequest = ProduceRequest8;
pub type ProduceResponse = ProduceResponse8;
impl ApiCall for ProduceRequest {
    type Response = ProduceResponse;
    fn get_min_supported_version() -> i16 {
        0
    }
    fn get_max_supported_version() -> i16 {
        8
    }
    fn get_api_key() -> ApiNumbers {
        ApiNumbers::Produce
    }
    fn is_flexible_version(version: i16) -> bool {
        match version {
            0 => false,
            1 => false,
            2 => false,
            3 => false,
            4 => false,
            5 => false,
            6 => false,
            7 => false,
            8 => false,
            _ => false,
        }
    }
    fn serialize(self, version: i16, buf: &mut BytesMut, correlation_id: i32, client_id: &str) {
        match Self::is_flexible_version(version) {
            true => HeaderRequest2::new(
                ProduceRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
            false => HeaderRequest1::new(
                ProduceRequest::get_api_key(),
                version,
                correlation_id,
                client_id,
            )
            .serialize(buf, false),
        }
        match version {
            0 => ToBytes::serialize(
                &ProduceRequest0::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            1 => ToBytes::serialize(
                &ProduceRequest1::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            2 => ToBytes::serialize(
                &ProduceRequest2::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            3 => ToBytes::serialize(
                &ProduceRequest3::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            4 => ToBytes::serialize(
                &ProduceRequest4::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            5 => ToBytes::serialize(
                &ProduceRequest5::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            6 => ToBytes::serialize(
                &ProduceRequest6::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            7 => ToBytes::serialize(
                &ProduceRequest7::from(self),
                buf,
                Self::is_flexible_version(version),
            ),
            8 => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
            _ => ToBytes::serialize(&self, buf, Self::is_flexible_version(version)),
        }
    }
    fn deserialize_response(version: i16, buf: &mut Bytes) -> (i32, ProduceResponse) {
        let correlation = match Self::is_flexible_version(version) {
            true => HeaderResponse2::deserialize(buf, false).correlation,
            false => HeaderResponse::deserialize(buf, false).correlation,
        };
        let response = match version {
            0 => ProduceResponse0::deserialize(buf, Self::is_flexible_version(version)).into(),
            1 => ProduceResponse1::deserialize(buf, Self::is_flexible_version(version)).into(),
            2 => ProduceResponse2::deserialize(buf, Self::is_flexible_version(version)).into(),
            3 => ProduceResponse3::deserialize(buf, Self::is_flexible_version(version)).into(),
            4 => ProduceResponse4::deserialize(buf, Self::is_flexible_version(version)).into(),
            5 => ProduceResponse5::deserialize(buf, Self::is_flexible_version(version)).into(),
            6 => ProduceResponse6::deserialize(buf, Self::is_flexible_version(version)).into(),
            7 => ProduceResponse7::deserialize(buf, Self::is_flexible_version(version)).into(),
            8 => ProduceResponse::deserialize(buf, Self::is_flexible_version(version)),
            _ => ProduceResponse::deserialize(buf, Self::is_flexible_version(version)),
        };
        (correlation, response)
    }
}
#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequest0 {
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicData0 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData0>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicDataData0 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequest1 {
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicData1 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData1>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicDataData1 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequest2 {
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicData2 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData2>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicDataData2 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequest3 {
    pub transactional_id: NullableString,
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicData3 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData3>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicDataData3 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequest4 {
    pub transactional_id: NullableString,
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData4>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicData4 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData4>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicDataData4 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequest5 {
    pub transactional_id: NullableString,
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData5>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicData5 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData5>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicDataData5 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequest6 {
    pub transactional_id: NullableString,
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData6>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicData6 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData6>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicDataData6 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequest7 {
    pub transactional_id: NullableString,
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData7>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicData7 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData7>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicDataData7 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequest8 {
    pub transactional_id: NullableString,
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData8>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicData8 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData8>,
}

#[derive(Default, Debug, Clone, ToBytes)]
pub struct ProduceRequestTopicDataData8 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponse0 {
    pub responses: Vec<ProduceResponseResponses0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponses0 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses0>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses0 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponse1 {
    pub responses: Vec<ProduceResponseResponses1>,
    pub throttle_time_ms: Option<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponses1 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses1>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses1 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponse2 {
    pub responses: Vec<ProduceResponseResponses2>,
    pub throttle_time_ms: Option<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponses2 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses2>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses2 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Option<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponse3 {
    pub responses: Vec<ProduceResponseResponses3>,
    pub throttle_time_ms: Option<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponses3 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses3>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses3 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Option<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponse4 {
    pub responses: Vec<ProduceResponseResponses4>,
    pub throttle_time_ms: Option<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponses4 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses4>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses4 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Option<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponse5 {
    pub responses: Vec<ProduceResponseResponses5>,
    pub throttle_time_ms: Option<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponses5 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses5>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses5 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Option<Int64>,
    pub log_start_offset: Option<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponse6 {
    pub responses: Vec<ProduceResponseResponses6>,
    pub throttle_time_ms: Option<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponses6 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses6>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses6 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Option<Int64>,
    pub log_start_offset: Option<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponse7 {
    pub responses: Vec<ProduceResponseResponses7>,
    pub throttle_time_ms: Option<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponses7 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses7>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses7 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Option<Int64>,
    pub log_start_offset: Option<Int64>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponse8 {
    pub responses: Vec<ProduceResponseResponses8>,
    pub throttle_time_ms: Option<Int32>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponses8 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses8>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses8 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Option<Int64>,
    pub log_start_offset: Option<Int64>,
    pub record_errors: Option<Vec<ProduceResponseResponsesPartitionResponsesRecordErrors8>>,
    pub error_message: Option<NullableString>,
}

#[derive(Default, Debug, Clone, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponsesRecordErrors8 {
    pub batch_index: Int32,
    pub batch_index_error_message: NullableString,
}

impl From<ProduceRequest8> for ProduceRequest0 {
    fn from(latest: ProduceRequest8) -> ProduceRequest0 {
        log::debug!("Using old api format - ProduceRequest0, ignoring field transactional_id");
        ProduceRequest0 {
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<ProduceRequestTopicData8> for ProduceRequestTopicData0 {
    fn from(latest: ProduceRequestTopicData8) -> ProduceRequestTopicData0 {
        ProduceRequestTopicData0 {
            topic: latest.topic,
            data: latest.data.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData0 {
    fn from(latest: ProduceRequestTopicDataData8) -> ProduceRequestTopicDataData0 {
        ProduceRequestTopicDataData0 {
            partition: latest.partition,
            record_set: latest.record_set,
        }
    }
}

impl From<ProduceRequest8> for ProduceRequest1 {
    fn from(latest: ProduceRequest8) -> ProduceRequest1 {
        log::debug!("Using old api format - ProduceRequest1, ignoring field transactional_id");
        ProduceRequest1 {
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<ProduceRequestTopicData8> for ProduceRequestTopicData1 {
    fn from(latest: ProduceRequestTopicData8) -> ProduceRequestTopicData1 {
        ProduceRequestTopicData1 {
            topic: latest.topic,
            data: latest.data.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData1 {
    fn from(latest: ProduceRequestTopicDataData8) -> ProduceRequestTopicDataData1 {
        ProduceRequestTopicDataData1 {
            partition: latest.partition,
            record_set: latest.record_set,
        }
    }
}

impl From<ProduceRequest8> for ProduceRequest2 {
    fn from(latest: ProduceRequest8) -> ProduceRequest2 {
        log::debug!("Using old api format - ProduceRequest2, ignoring field transactional_id");
        ProduceRequest2 {
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<ProduceRequestTopicData8> for ProduceRequestTopicData2 {
    fn from(latest: ProduceRequestTopicData8) -> ProduceRequestTopicData2 {
        ProduceRequestTopicData2 {
            topic: latest.topic,
            data: latest.data.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData2 {
    fn from(latest: ProduceRequestTopicDataData8) -> ProduceRequestTopicDataData2 {
        ProduceRequestTopicDataData2 {
            partition: latest.partition,
            record_set: latest.record_set,
        }
    }
}

impl From<ProduceRequest8> for ProduceRequest3 {
    fn from(latest: ProduceRequest8) -> ProduceRequest3 {
        ProduceRequest3 {
            transactional_id: latest.transactional_id,
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<ProduceRequestTopicData8> for ProduceRequestTopicData3 {
    fn from(latest: ProduceRequestTopicData8) -> ProduceRequestTopicData3 {
        ProduceRequestTopicData3 {
            topic: latest.topic,
            data: latest.data.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData3 {
    fn from(latest: ProduceRequestTopicDataData8) -> ProduceRequestTopicDataData3 {
        ProduceRequestTopicDataData3 {
            partition: latest.partition,
            record_set: latest.record_set,
        }
    }
}

impl From<ProduceRequest8> for ProduceRequest4 {
    fn from(latest: ProduceRequest8) -> ProduceRequest4 {
        ProduceRequest4 {
            transactional_id: latest.transactional_id,
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<ProduceRequestTopicData8> for ProduceRequestTopicData4 {
    fn from(latest: ProduceRequestTopicData8) -> ProduceRequestTopicData4 {
        ProduceRequestTopicData4 {
            topic: latest.topic,
            data: latest.data.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData4 {
    fn from(latest: ProduceRequestTopicDataData8) -> ProduceRequestTopicDataData4 {
        ProduceRequestTopicDataData4 {
            partition: latest.partition,
            record_set: latest.record_set,
        }
    }
}

impl From<ProduceRequest8> for ProduceRequest5 {
    fn from(latest: ProduceRequest8) -> ProduceRequest5 {
        ProduceRequest5 {
            transactional_id: latest.transactional_id,
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<ProduceRequestTopicData8> for ProduceRequestTopicData5 {
    fn from(latest: ProduceRequestTopicData8) -> ProduceRequestTopicData5 {
        ProduceRequestTopicData5 {
            topic: latest.topic,
            data: latest.data.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData5 {
    fn from(latest: ProduceRequestTopicDataData8) -> ProduceRequestTopicDataData5 {
        ProduceRequestTopicDataData5 {
            partition: latest.partition,
            record_set: latest.record_set,
        }
    }
}

impl From<ProduceRequest8> for ProduceRequest6 {
    fn from(latest: ProduceRequest8) -> ProduceRequest6 {
        ProduceRequest6 {
            transactional_id: latest.transactional_id,
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<ProduceRequestTopicData8> for ProduceRequestTopicData6 {
    fn from(latest: ProduceRequestTopicData8) -> ProduceRequestTopicData6 {
        ProduceRequestTopicData6 {
            topic: latest.topic,
            data: latest.data.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData6 {
    fn from(latest: ProduceRequestTopicDataData8) -> ProduceRequestTopicDataData6 {
        ProduceRequestTopicDataData6 {
            partition: latest.partition,
            record_set: latest.record_set,
        }
    }
}

impl From<ProduceRequest8> for ProduceRequest7 {
    fn from(latest: ProduceRequest8) -> ProduceRequest7 {
        ProduceRequest7 {
            transactional_id: latest.transactional_id,
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|ele| ele.into())
                .collect(),
        }
    }
}

impl From<ProduceRequestTopicData8> for ProduceRequestTopicData7 {
    fn from(latest: ProduceRequestTopicData8) -> ProduceRequestTopicData7 {
        ProduceRequestTopicData7 {
            topic: latest.topic,
            data: latest.data.into_iter().map(|ele| ele.into()).collect(),
        }
    }
}

impl From<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData7 {
    fn from(latest: ProduceRequestTopicDataData8) -> ProduceRequestTopicDataData7 {
        ProduceRequestTopicDataData7 {
            partition: latest.partition,
            record_set: latest.record_set,
        }
    }
}

impl From<ProduceResponse0> for ProduceResponse8 {
    fn from(older: ProduceResponse0) -> Self {
        ProduceResponse8 {
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            ..ProduceResponse8::default()
        }
    }
}

impl From<ProduceResponseResponses0> for ProduceResponseResponses8 {
    fn from(older: ProduceResponseResponses0) -> Self {
        ProduceResponseResponses8 {
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<ProduceResponseResponsesPartitionResponses0>
    for ProduceResponseResponsesPartitionResponses8
{
    fn from(older: ProduceResponseResponsesPartitionResponses0) -> Self {
        ProduceResponseResponsesPartitionResponses8 {
            partition: older.partition,
            error_code: older.error_code,
            base_offset: older.base_offset,
            ..ProduceResponseResponsesPartitionResponses8::default()
        }
    }
}

impl From<ProduceResponse1> for ProduceResponse8 {
    fn from(older: ProduceResponse1) -> Self {
        ProduceResponse8 {
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}

impl From<ProduceResponseResponses1> for ProduceResponseResponses8 {
    fn from(older: ProduceResponseResponses1) -> Self {
        ProduceResponseResponses8 {
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<ProduceResponseResponsesPartitionResponses1>
    for ProduceResponseResponsesPartitionResponses8
{
    fn from(older: ProduceResponseResponsesPartitionResponses1) -> Self {
        ProduceResponseResponsesPartitionResponses8 {
            partition: older.partition,
            error_code: older.error_code,
            base_offset: older.base_offset,
            ..ProduceResponseResponsesPartitionResponses8::default()
        }
    }
}

impl From<ProduceResponse2> for ProduceResponse8 {
    fn from(older: ProduceResponse2) -> Self {
        ProduceResponse8 {
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}

impl From<ProduceResponseResponses2> for ProduceResponseResponses8 {
    fn from(older: ProduceResponseResponses2) -> Self {
        ProduceResponseResponses8 {
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<ProduceResponseResponsesPartitionResponses2>
    for ProduceResponseResponsesPartitionResponses8
{
    fn from(older: ProduceResponseResponsesPartitionResponses2) -> Self {
        ProduceResponseResponsesPartitionResponses8 {
            partition: older.partition,
            error_code: older.error_code,
            base_offset: older.base_offset,
            log_append_time: older.log_append_time,
            ..ProduceResponseResponsesPartitionResponses8::default()
        }
    }
}

impl From<ProduceResponse3> for ProduceResponse8 {
    fn from(older: ProduceResponse3) -> Self {
        ProduceResponse8 {
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}

impl From<ProduceResponseResponses3> for ProduceResponseResponses8 {
    fn from(older: ProduceResponseResponses3) -> Self {
        ProduceResponseResponses8 {
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<ProduceResponseResponsesPartitionResponses3>
    for ProduceResponseResponsesPartitionResponses8
{
    fn from(older: ProduceResponseResponsesPartitionResponses3) -> Self {
        ProduceResponseResponsesPartitionResponses8 {
            partition: older.partition,
            error_code: older.error_code,
            base_offset: older.base_offset,
            log_append_time: older.log_append_time,
            ..ProduceResponseResponsesPartitionResponses8::default()
        }
    }
}

impl From<ProduceResponse4> for ProduceResponse8 {
    fn from(older: ProduceResponse4) -> Self {
        ProduceResponse8 {
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}

impl From<ProduceResponseResponses4> for ProduceResponseResponses8 {
    fn from(older: ProduceResponseResponses4) -> Self {
        ProduceResponseResponses8 {
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<ProduceResponseResponsesPartitionResponses4>
    for ProduceResponseResponsesPartitionResponses8
{
    fn from(older: ProduceResponseResponsesPartitionResponses4) -> Self {
        ProduceResponseResponsesPartitionResponses8 {
            partition: older.partition,
            error_code: older.error_code,
            base_offset: older.base_offset,
            log_append_time: older.log_append_time,
            ..ProduceResponseResponsesPartitionResponses8::default()
        }
    }
}

impl From<ProduceResponse5> for ProduceResponse8 {
    fn from(older: ProduceResponse5) -> Self {
        ProduceResponse8 {
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}

impl From<ProduceResponseResponses5> for ProduceResponseResponses8 {
    fn from(older: ProduceResponseResponses5) -> Self {
        ProduceResponseResponses8 {
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<ProduceResponseResponsesPartitionResponses5>
    for ProduceResponseResponsesPartitionResponses8
{
    fn from(older: ProduceResponseResponsesPartitionResponses5) -> Self {
        ProduceResponseResponsesPartitionResponses8 {
            partition: older.partition,
            error_code: older.error_code,
            base_offset: older.base_offset,
            log_append_time: older.log_append_time,
            log_start_offset: older.log_start_offset,
            ..ProduceResponseResponsesPartitionResponses8::default()
        }
    }
}

impl From<ProduceResponse6> for ProduceResponse8 {
    fn from(older: ProduceResponse6) -> Self {
        ProduceResponse8 {
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}

impl From<ProduceResponseResponses6> for ProduceResponseResponses8 {
    fn from(older: ProduceResponseResponses6) -> Self {
        ProduceResponseResponses8 {
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<ProduceResponseResponsesPartitionResponses6>
    for ProduceResponseResponsesPartitionResponses8
{
    fn from(older: ProduceResponseResponsesPartitionResponses6) -> Self {
        ProduceResponseResponsesPartitionResponses8 {
            partition: older.partition,
            error_code: older.error_code,
            base_offset: older.base_offset,
            log_append_time: older.log_append_time,
            log_start_offset: older.log_start_offset,
            ..ProduceResponseResponsesPartitionResponses8::default()
        }
    }
}

impl From<ProduceResponse7> for ProduceResponse8 {
    fn from(older: ProduceResponse7) -> Self {
        ProduceResponse8 {
            responses: older.responses.into_iter().map(|el| el.into()).collect(),
            throttle_time_ms: older.throttle_time_ms,
        }
    }
}

impl From<ProduceResponseResponses7> for ProduceResponseResponses8 {
    fn from(older: ProduceResponseResponses7) -> Self {
        ProduceResponseResponses8 {
            topic: older.topic,
            partition_responses: older
                .partition_responses
                .into_iter()
                .map(|el| el.into())
                .collect(),
        }
    }
}

impl From<ProduceResponseResponsesPartitionResponses7>
    for ProduceResponseResponsesPartitionResponses8
{
    fn from(older: ProduceResponseResponsesPartitionResponses7) -> Self {
        ProduceResponseResponsesPartitionResponses8 {
            partition: older.partition,
            error_code: older.error_code,
            base_offset: older.base_offset,
            log_append_time: older.log_append_time,
            log_start_offset: older.log_start_offset,
            ..ProduceResponseResponsesPartitionResponses8::default()
        }
    }
}
