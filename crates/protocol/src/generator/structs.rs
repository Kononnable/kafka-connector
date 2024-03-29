use serde::{Deserialize, Deserializer};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ApiSpecType {
    Request,
    Response,
    Header,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ApiSpec {
    pub api_key: Option<u8>,
    #[serde(rename = "type")]
    pub type_: ApiSpecType,
    pub name: String,
    pub valid_versions: String,
    pub fields: Vec<ApiSpecField>,
    #[serde(default)]
    pub api_versions_comment: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ApiSpecFieldType {
    pub type_: ApiSpecFieldSubtype,
    pub is_array: bool,
}

#[derive(Clone, Debug)]
pub enum ApiSpecFieldSubtype {
    Bool,
    Bytes,
    Int8,
    Int16,
    Int32,
    Int64,
    String,
    SubObject(String),
    Map {
        name: String,
        keys: Vec<ApiSpecField>,
    },
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ApiSpecField {
    #[serde(deserialize_with = "sanitize_field_name")]
    pub name: String,
    #[serde(rename = "type", deserialize_with = "deserialize_api_spec_field_type")]
    pub type_: ApiSpecFieldType,
    pub versions: String,
    pub about: Option<String>,
    #[serde(default)]
    pub fields: Vec<ApiSpecField>,
    #[serde(default)]
    pub map_key: bool,
    pub nullable_versions: Option<String>,
    #[serde(default)]
    pub ignorable: bool,
    #[serde(default)]
    pub default: Option<String>,
}

fn deserialize_api_spec_field_type<'de, D>(
    deserializer: D,
) -> anyhow::Result<ApiSpecFieldType, D::Error>
where
    D: Deserializer<'de>,
{
    let mut buf = String::deserialize(deserializer)?;
    let is_array = buf.starts_with("[]");
    if is_array {
        buf = buf.split_off(2);
    }
    let subtype = match buf.as_str() {
        "bool" => Ok(ApiSpecFieldSubtype::Bool),
        "bytes" => Ok(ApiSpecFieldSubtype::Bytes),
        "int8" => Ok(ApiSpecFieldSubtype::Int8),
        "int16" => Ok(ApiSpecFieldSubtype::Int16),
        "int32" => Ok(ApiSpecFieldSubtype::Int32),
        "int64" => Ok(ApiSpecFieldSubtype::Int64),
        "string" => Ok(ApiSpecFieldSubtype::String),
        _ => Ok(ApiSpecFieldSubtype::SubObject(buf)),
    }?;
    Ok(ApiSpecFieldType {
        type_: subtype,
        is_array,
    })
}

fn sanitize_field_name<'de, D>(deserializer: D) -> anyhow::Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    match buf.as_str() {
        "Type" => Ok("r#Type".to_owned()),
        _ => Ok(buf),
    }
}
