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
    Vec(&'a str, Vec<FieldData<'a>>),
}
#[derive(Debug)]
pub enum CallType {
    Request,
    Response,
}

#[derive(Debug)]
pub enum FieldType {
    Int16,
    Int32,
    Int64,
    String,
    Records,
}

impl FieldType {
    pub fn from_str(ty: &str) -> FieldType {
        match ty {
            "INT16" => FieldType::Int16,
            "INT32" => FieldType::Int32,
            "INT64" => FieldType::Int64,
            "STRING" => FieldType::String,
            "RECORDS" => FieldType::Records,
            _ => panic!("Unknown field type: {}", ty),
        }
    }
}
