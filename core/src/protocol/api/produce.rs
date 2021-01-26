use super::prelude::*;

pub type ProduceRequest = ProduceRequest8;
pub type ProduceResponse = ProduceResponse8;
pub fn serialize_produce_request(
    data: ProduceRequest,
    version: i32,
    buf: &mut BytesMut,
) -> Result<(), Error> {
    match version {
        0 => ToBytes::serialize(&ProduceRequest0::try_from(data)?, buf),
        1 => ToBytes::serialize(&ProduceRequest1::try_from(data)?, buf),
        2 => ToBytes::serialize(&ProduceRequest2::try_from(data)?, buf),
        3 => ToBytes::serialize(&ProduceRequest3::try_from(data)?, buf),
        4 => ToBytes::serialize(&ProduceRequest4::try_from(data)?, buf),
        5 => ToBytes::serialize(&ProduceRequest5::try_from(data)?, buf),
        6 => ToBytes::serialize(&ProduceRequest6::try_from(data)?, buf),
        7 => ToBytes::serialize(&ProduceRequest7::try_from(data)?, buf),
        8 => ToBytes::serialize(&data, buf),
        _ => ToBytes::serialize(&data, buf),
    }
    Ok(())
}
pub fn deserialize_produce_response(version: i32, buf: &mut Bytes) -> ProduceResponse {
    match version {
        0 => ProduceResponse0::deserialize(buf).into(),
        1 => ProduceResponse1::deserialize(buf).into(),
        2 => ProduceResponse2::deserialize(buf).into(),
        3 => ProduceResponse3::deserialize(buf).into(),
        4 => ProduceResponse4::deserialize(buf).into(),
        5 => ProduceResponse5::deserialize(buf).into(),
        6 => ProduceResponse6::deserialize(buf).into(),
        7 => ProduceResponse7::deserialize(buf).into(),
        8 => ProduceResponse::deserialize(buf),
        _ => ProduceResponse::deserialize(buf),
    }
}

#[derive(Default, ToBytes)]
pub struct ProduceRequest0 {
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData0>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicData0 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData0>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicDataData0 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequest1 {
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData1>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicData1 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData1>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicDataData1 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequest2 {
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData2>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicData2 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData2>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicDataData2 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequest3 {
    pub transactional_id: Optional<NullableString>,
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData3>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicData3 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData3>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicDataData3 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequest4 {
    pub transactional_id: Optional<NullableString>,
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData4>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicData4 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData4>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicDataData4 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequest5 {
    pub transactional_id: Optional<NullableString>,
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData5>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicData5 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData5>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicDataData5 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequest6 {
    pub transactional_id: Optional<NullableString>,
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData6>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicData6 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData6>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicDataData6 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequest7 {
    pub transactional_id: Optional<NullableString>,
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData7>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicData7 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData7>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicDataData7 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequest8 {
    pub transactional_id: Optional<NullableString>,
    pub acks: Int16,
    pub timeout: Int32,
    pub topic_data: Vec<ProduceRequestTopicData8>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicData8 {
    pub topic: String,
    pub data: Vec<ProduceRequestTopicDataData8>,
}

#[derive(Default, ToBytes)]
pub struct ProduceRequestTopicDataData8 {
    pub partition: Int32,
    pub record_set: Records,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponse0 {
    pub responses: Vec<ProduceResponseResponses0>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponses0 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses0>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses0 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponse1 {
    pub responses: Vec<ProduceResponseResponses1>,
    pub throttle_time_ms: Optional<Int32>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponses1 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses1>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses1 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponse2 {
    pub responses: Vec<ProduceResponseResponses2>,
    pub throttle_time_ms: Optional<Int32>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponses2 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses2>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses2 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Optional<Int64>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponse3 {
    pub responses: Vec<ProduceResponseResponses3>,
    pub throttle_time_ms: Optional<Int32>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponses3 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses3>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses3 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Optional<Int64>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponse4 {
    pub responses: Vec<ProduceResponseResponses4>,
    pub throttle_time_ms: Optional<Int32>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponses4 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses4>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses4 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Optional<Int64>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponse5 {
    pub responses: Vec<ProduceResponseResponses5>,
    pub throttle_time_ms: Optional<Int32>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponses5 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses5>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses5 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponse6 {
    pub responses: Vec<ProduceResponseResponses6>,
    pub throttle_time_ms: Optional<Int32>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponses6 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses6>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses6 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponse7 {
    pub responses: Vec<ProduceResponseResponses7>,
    pub throttle_time_ms: Optional<Int32>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponses7 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses7>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses7 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponse8 {
    pub responses: Vec<ProduceResponseResponses8>,
    pub throttle_time_ms: Optional<Int32>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponses8 {
    pub topic: String,
    pub partition_responses: Vec<ProduceResponseResponsesPartitionResponses8>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponses8 {
    pub partition: Int32,
    pub error_code: Int16,
    pub base_offset: Int64,
    pub log_append_time: Optional<Int64>,
    pub log_start_offset: Optional<Int64>,
    pub record_errors: Optional<Vec<ProduceResponseResponsesPartitionResponsesRecordErrors8>>,
    pub error_message: Optional<NullableString>,
}

#[derive(Default, FromBytes)]
pub struct ProduceResponseResponsesPartitionResponsesRecordErrors8 {
    pub batch_index: Int32,
    pub batch_index_error_message: NullableString,
}

impl TryFrom<ProduceRequest8> for ProduceRequest0 {
    type Error = Error;
    fn try_from(latest: ProduceRequest8) -> Result<Self, Self::Error> {
        if latest.transactional_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "ProduceRequest",
                0,
                "transactional_id",
            ));
        }
        Ok(ProduceRequest0 {
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicData8> for ProduceRequestTopicData0 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicData0 {
            topic: latest.topic,
            data: latest
                .data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData0 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicDataData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicDataData0 {
            partition: latest.partition,
            record_set: latest.record_set,
        })
    }
}

impl TryFrom<ProduceRequest8> for ProduceRequest1 {
    type Error = Error;
    fn try_from(latest: ProduceRequest8) -> Result<Self, Self::Error> {
        if latest.transactional_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "ProduceRequest",
                1,
                "transactional_id",
            ));
        }
        Ok(ProduceRequest1 {
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicData8> for ProduceRequestTopicData1 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicData1 {
            topic: latest.topic,
            data: latest
                .data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData1 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicDataData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicDataData1 {
            partition: latest.partition,
            record_set: latest.record_set,
        })
    }
}

impl TryFrom<ProduceRequest8> for ProduceRequest2 {
    type Error = Error;
    fn try_from(latest: ProduceRequest8) -> Result<Self, Self::Error> {
        if latest.transactional_id.is_some() {
            return Err(Error::OldKafkaVersion(
                "ProduceRequest",
                2,
                "transactional_id",
            ));
        }
        Ok(ProduceRequest2 {
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicData8> for ProduceRequestTopicData2 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicData2 {
            topic: latest.topic,
            data: latest
                .data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData2 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicDataData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicDataData2 {
            partition: latest.partition,
            record_set: latest.record_set,
        })
    }
}

impl TryFrom<ProduceRequest8> for ProduceRequest3 {
    type Error = Error;
    fn try_from(latest: ProduceRequest8) -> Result<Self, Self::Error> {
        Ok(ProduceRequest3 {
            transactional_id: latest.transactional_id,
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicData8> for ProduceRequestTopicData3 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicData3 {
            topic: latest.topic,
            data: latest
                .data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData3 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicDataData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicDataData3 {
            partition: latest.partition,
            record_set: latest.record_set,
        })
    }
}

impl TryFrom<ProduceRequest8> for ProduceRequest4 {
    type Error = Error;
    fn try_from(latest: ProduceRequest8) -> Result<Self, Self::Error> {
        Ok(ProduceRequest4 {
            transactional_id: latest.transactional_id,
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicData8> for ProduceRequestTopicData4 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicData4 {
            topic: latest.topic,
            data: latest
                .data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData4 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicDataData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicDataData4 {
            partition: latest.partition,
            record_set: latest.record_set,
        })
    }
}

impl TryFrom<ProduceRequest8> for ProduceRequest5 {
    type Error = Error;
    fn try_from(latest: ProduceRequest8) -> Result<Self, Self::Error> {
        Ok(ProduceRequest5 {
            transactional_id: latest.transactional_id,
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicData8> for ProduceRequestTopicData5 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicData5 {
            topic: latest.topic,
            data: latest
                .data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData5 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicDataData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicDataData5 {
            partition: latest.partition,
            record_set: latest.record_set,
        })
    }
}

impl TryFrom<ProduceRequest8> for ProduceRequest6 {
    type Error = Error;
    fn try_from(latest: ProduceRequest8) -> Result<Self, Self::Error> {
        Ok(ProduceRequest6 {
            transactional_id: latest.transactional_id,
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicData8> for ProduceRequestTopicData6 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicData6 {
            topic: latest.topic,
            data: latest
                .data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData6 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicDataData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicDataData6 {
            partition: latest.partition,
            record_set: latest.record_set,
        })
    }
}

impl TryFrom<ProduceRequest8> for ProduceRequest7 {
    type Error = Error;
    fn try_from(latest: ProduceRequest8) -> Result<Self, Self::Error> {
        Ok(ProduceRequest7 {
            transactional_id: latest.transactional_id,
            acks: latest.acks,
            timeout: latest.timeout,
            topic_data: latest
                .topic_data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicData8> for ProduceRequestTopicData7 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicData7 {
            topic: latest.topic,
            data: latest
                .data
                .into_iter()
                .map(|el| el.try_into())
                .collect::<Result<_, Error>>()?,
        })
    }
}

impl TryFrom<ProduceRequestTopicDataData8> for ProduceRequestTopicDataData7 {
    type Error = Error;
    fn try_from(latest: ProduceRequestTopicDataData8) -> Result<Self, Self::Error> {
        Ok(ProduceRequestTopicDataData7 {
            partition: latest.partition,
            record_set: latest.record_set,
        })
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
