use std::collections::HashMap;

use crate::{
    model::{ApiCall, CallType, FieldData, FieldTypeWithPayload},
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
        let parsed_call = parse_call(call);
        match call_type {
            CallType::Request => grouped_call.requests.push(parsed_call),
            CallType::Response => grouped_call.responses.push(parsed_call),
        }
    }

    for definition in endpoints.values_mut() {
        mark_new_fields_as_optional(&mut definition.requests);
        mark_new_fields_as_optional(&mut definition.responses);
    }

    endpoints
}

fn mark_new_fields_as_optional(endpoint_definitions: &mut Vec<Vec<ApiStructDefinition>>) {
    let (historic_endpoint_definition, endpoint_definitions) =
        endpoint_definitions.split_first_mut().unwrap();
    assert_eq!(historic_endpoint_definition.get(0).unwrap().version, 0);
    let mut new_structs_base = vec![]; // first versions of structs not available in base(0) api call
    for struc in endpoint_definitions.iter_mut().flatten() {
        let base_struct = historic_endpoint_definition
            .iter()
            .chain(new_structs_base.iter())
            .find(|ac| ac.name == struc.name);
        match base_struct {
            None => new_structs_base.push(struc.clone()),
            Some(base_struct) => {
                for field in &mut struc.fields {
                    if base_struct
                        .fields
                        .iter()
                        .find(|x| x.name == field.name)
                        .is_none()
                    {
                        field.ty = format!("Optional<{}>", field.ty);
                    }
                }
            }
        }
    }
}

fn parse_call(api_call: ApiCall) -> Vec<ApiStructDefinition> {
    let mut definitions = Vec::new();

    let name = format!("{}{:?}", api_call.name, api_call.ty);
    let (fields, mut children_definitions) =
        parse_vec(api_call.fields, name.clone(), api_call.version);
    let definition = ApiStructDefinition {
        name,
        version: api_call.version,
        fields,
    };
    definitions.push(definition);
    definitions.append(&mut children_definitions);

    definitions
}

fn parse_vec(
    fields: Vec<FieldData>,
    prefix: String,
    api_version: i32,
) -> (Vec<StructField>, Vec<ApiStructDefinition>) {
    let mut children = Vec::new();
    let mut returned_fields = vec![];
    for field in fields {
        let (ty,is_vec) = match field.type_with_payload {
            FieldTypeWithPayload::Field(ty) => {
                (format!("{:?}", ty),false)
            }
            FieldTypeWithPayload::VecSimple(ty) => {
                (format!("Vec<{:?}>", ty),true)
            }
            FieldTypeWithPayload::VecStruct(ty) => {
                let struct_name = format!("{}{}", prefix, to_upper_case(field.name));
                let (fields, mut grandchildren) = parse_vec(ty, struct_name.clone(), api_version);
                children.push(ApiStructDefinition {
                    name: struct_name.clone(),
                    version: api_version,
                    fields,
                });
                children.append(&mut grandchildren);
                (format!("Vec<{}{}>", struct_name, api_version),true)
            }
        };
        returned_fields.push(StructField {
            name: field.name,
            ty,
            is_vec
        });
    }
    (returned_fields, children)
}

#[derive(Default, Debug)]
pub struct ApiEndpoint<'a> {
    pub requests: Vec<Vec<ApiStructDefinition<'a>>>,
    pub responses: Vec<Vec<ApiStructDefinition<'a>>>,
}

#[derive(Debug, Clone)]
pub struct ApiStructDefinition<'a> {
    pub name: String,
    pub version: i32,
    pub fields: Vec<StructField<'a>>,
}

#[derive(Debug, Clone)]
pub struct StructField<'a> {
    pub name: &'a str,
    pub ty: String,
    pub is_vec:bool,
}
