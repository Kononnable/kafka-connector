use std::collections::HashMap;

use crate::{
    model::{ApiCall, CallType, FieldData, FieldType, FieldTypeWithPayload},
    utils::to_upper_case,
};

pub fn group_api_calls(api_calls: Vec<ApiCall>) -> HashMap<&str, ApiEndpoint> {
    let mut endpoints = HashMap::new();

    for call in api_calls {
        if !endpoints.contains_key(call.name) {
            endpoints.insert(call.name, ApiEndpoint::default());
        }
        let grouped_call = endpoints.get_mut(call.name).unwrap();
        let call_type = call.ty;
        let mut parsed_call = parse_call(call);
        change_reserved_keywords(&mut parsed_call);
        match call_type {
            CallType::Request => grouped_call.requests.push(parsed_call),
            CallType::Response => grouped_call.responses.push(parsed_call),
        }
    }

    endpoints
}

fn change_reserved_keywords(api_calls: &mut Vec<ApiStructDefinition>) {
    for api_call in api_calls {
        for field in &mut api_call.fields {
            if let "match" = field.name {
                field.name = "match_";
            }
        }
    }
}

fn parse_call(api_call: ApiCall) -> Vec<ApiStructDefinition> {
    let mut definitions = Vec::new();

    let name = format!("{}{:?}", api_call.name, api_call.ty);
    let (fields, mut children_definitions) =
        parse_vec(api_call.fields, name.clone(), api_call.version);
    let is_flexible_version = fields.iter().any(|x| x.is_compact_field);
    let definition = ApiStructDefinition {
        name,
        version: api_call.version,
        fields,
        is_flexible_version,
    };
    definitions.push(definition);
    definitions.append(&mut children_definitions);

    definitions
}

fn parse_vec(
    fields: Vec<FieldData>,
    prefix: String,
    api_version: i16,
) -> (Vec<StructField>, Vec<ApiStructDefinition>) {
    let mut children = Vec::new();
    let mut returned_fields = vec![];
    for field in fields {
        match field.type_with_payload {
            FieldTypeWithPayload::Field(ty) => {
                let (is_compact_field, typ) = process_flexible_version_fields(ty);
                returned_fields.push(StructField {
                    name: field.name,
                    ty: typ,
                    is_vec: false,
                    is_struct_field: false,
                    is_optional: false,
                    is_compact_field,
                });
            }
            FieldTypeWithPayload::VecSimple(ty) => {
                let (is_compact_field, typ) = process_flexible_version_fields(ty);
                returned_fields.push(StructField {
                    name: field.name,
                    ty: typ,
                    is_vec: true,
                    is_struct_field: false,
                    is_optional: false,
                    is_compact_field,
                });
            }
            FieldTypeWithPayload::VecStruct(ty) => {
                let struct_name = format!("{}{}", prefix, to_upper_case(field.name));
                let (fields, mut grandchildren) = parse_vec(ty, struct_name.clone(), api_version);
                let is_flexible_version = fields.iter().any(|x| x.is_compact_field);
                children.push(ApiStructDefinition {
                    name: struct_name.clone(),
                    version: api_version,
                    fields,
                    is_flexible_version,
                });
                children.append(&mut grandchildren);
                returned_fields.push(StructField {
                    name: field.name,
                    ty: struct_name,
                    is_vec: true,
                    is_struct_field: true,
                    is_optional: false,
                    is_compact_field: false,
                });
            }
            FieldTypeWithPayload::TagBuffer => {
                returned_fields.push(StructField {
                    name: field.name,
                    ty: "TagBuffer".to_owned(),
                    is_vec: false,
                    is_struct_field: false,
                    is_optional: false,
                    is_compact_field: true,
                });
            }
        };
    }
    (returned_fields, children)
}

fn process_flexible_version_fields(field_type: FieldType) -> (bool, String) {
    let field_type = format!("{:?}", field_type).replace("CompactBytes", "CompactKafkaBytes");
    if field_type.contains("Compact") {
        (true, field_type.replacen("Compact", "", 1))
    } else {
        (false, field_type)
    }
}

#[derive(Default, Debug)]
pub struct ApiEndpoint<'a> {
    pub requests: Vec<Vec<ApiStructDefinition<'a>>>,
    pub responses: Vec<Vec<ApiStructDefinition<'a>>>,
}

#[derive(Debug, Clone)]
pub struct ApiStructDefinition<'a> {
    pub name: String,
    pub version: i16,
    pub fields: Vec<StructField<'a>>,
    pub is_flexible_version: bool,
}

#[derive(Debug, Clone)]
pub struct StructField<'a> {
    pub name: &'a str,
    pub ty: String,
    pub is_vec: bool,
    pub is_struct_field: bool,
    pub is_optional: bool,
    pub is_compact_field: bool,
}
