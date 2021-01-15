#[derive(Debug)]
pub struct ApiCall<'a> {
    pub name: &'a str,
    pub typ: CallType,
    pub version: &'a str,
    pub fields: Vec<FieldData<'a>>,
}

#[derive(Debug)]
pub enum FieldData<'a> {
    Field(&'a str, FieldType),
    VecSimple(&'a str, FieldType),
    VecStruct(&'a str, Vec<FieldData<'a>>),
}
#[derive(Debug)]
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

    fn try_from_str(ty:&str)-> anyhow::Result<FieldType>{
        let ret_val = match ty {
            "BOOLEAN"=> FieldType::Boolean,
            "BYTES"=> FieldType::Bytes,
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
            _ => Err(anyhow::anyhow!("Unknown field type {}",ty))?
        };
        Ok(ret_val)
    }

    pub fn is_common_type(ty:&str)-> bool{
        FieldType::try_from_str(ty).is_ok()
    }

    pub fn from_str(ty: &str) -> FieldType {
        match FieldType::try_from_str(ty) {
            Ok(ty)=>ty,
            _ => panic!("Unknown field type: {}", ty),
        }
    }
}
