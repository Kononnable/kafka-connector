use std::str::FromStr;

#[derive(Debug)]
pub struct ApiCall<'a> {
    pub name: &'a str,
    pub typ: CallType,
    pub version: i32,
    pub fields: Vec<FieldData<'a>>,
}

#[derive(Debug)]
pub enum FieldTypeWithPayload<'a> {
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

#[derive(Debug)]
pub enum FieldType {
    Boolean,
    Bytes,
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
}

impl FieldType {
    fn try_from_str(ty: &str) -> anyhow::Result<FieldType> {
        let ret_val = match ty {
            "BOOLEAN" => FieldType::Boolean,
            "BYTES" => FieldType::Bytes,
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
