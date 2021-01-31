use std::str::FromStr;

#[derive(Debug)]
pub struct ApiCall<'a> {
    pub name: &'a str,
    pub ty: CallType,
    pub version: i32,
    pub fields: Vec<FieldData<'a>>,
}

#[derive(Debug)]
pub enum FieldTypeWithPayload<'a> {
    TagBuffer,
    Field(FieldType),
    VecSimple(FieldType),
    VecStruct(Vec<FieldData<'a>>),
}

#[derive(Debug)]
pub struct FieldData<'a> {
    pub name: &'a str,
    pub type_with_payload: FieldTypeWithPayload<'a>,
}

#[derive(Debug, Clone, Copy)]
pub enum CallType {
    Request,
    Response,
}

impl FromStr for CallType {
    type Err = String;

    fn from_str(call_type: &str) -> Result<Self, Self::Err> {
        match call_type {
            "Request" => Ok(CallType::Request),
            "Response" => Ok(CallType::Response),
            _ => Err(format!("Unknown field type: {}", call_type)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FieldType {
    Boolean,
    KafkaBytes,
    Int8,
    Int16,
    Int32,
    Int64,
    Float64,
    String,
    Records,
    NullableString,
    CompactString,
    CompactRecords,
    CompactNullableString,
    CompactBytes,
    TagBuffer,
}

impl FieldType {
    fn try_from_str(ty: &str) -> anyhow::Result<FieldType> {
        let ret_val = match ty {
            "BOOLEAN" => FieldType::Boolean,
            "BYTES" => FieldType::KafkaBytes,
            "INT8" => FieldType::Int8,
            "INT16" => FieldType::Int16,
            "INT32" => FieldType::Int32,
            "INT64" => FieldType::Int64,
            "FLOAT64" => FieldType::Float64,
            "STRING" => FieldType::String,
            "RECORDS" => FieldType::Records,
            "NULLABLE_STRING" => FieldType::NullableString,
            "COMPACT_STRING" => FieldType::CompactString,
            "COMPACT_RECORDS" => FieldType::CompactRecords,
            "COMPACT_NULLABLE_STRING" => FieldType::CompactNullableString,
            "COMPACT_BYTES" => FieldType::CompactBytes,
            _ => return Err(anyhow::anyhow!("Unknown field type {}", ty)),
        };
        Ok(ret_val)
    }

    pub fn is_common_type(ty: &str) -> bool {
        FieldType::try_from_str(ty).is_ok()
    }
    pub fn is_simple_type(&self) -> bool {
        match self {
            FieldType::Int8
            | FieldType::Int16
            | FieldType::Int32
            | FieldType::Int64
            | FieldType::Float64
            | FieldType::String
            | FieldType::Boolean => true,
            // TODO:
            FieldType::Records
            | FieldType::NullableString
            | FieldType::CompactRecords
            | FieldType::TagBuffer => true,

            FieldType::KafkaBytes
            | FieldType::CompactBytes
            | FieldType::CompactString
            | FieldType::CompactNullableString => false,
        }
    }
}

impl FromStr for FieldType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match FieldType::try_from_str(s) {
            Ok(ty) => Ok(ty),
            _ => Err(format!("Unknown field type: {}", s)),
        }
    }
}
