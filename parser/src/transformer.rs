use std::collections::HashMap;

use crate::{
    model::{ApiCall, CallType, FieldData, FieldTypeWithPayload},
    utils::to_upper_case,
};

pub fn group_api_calls(api_calls: Vec<ApiCall>) -> HashMap<&str, GroupedApiCall> {
    let mut ret_val = HashMap::new();

    for call in api_calls {
        if !ret_val.contains_key(call.name) {
            ret_val.insert(call.name, GroupedApiCall::default());
        }
        let grouped_call = ret_val.get_mut(call.name).unwrap();
        let call_type = call.typ;
        let parsed_call = parse_call(call);
        match call_type {
            CallType::Request => grouped_call.requests.push(parsed_call),
            CallType::Response => grouped_call.responses.push(parsed_call),
        }
    }

    for api_call in &mut ret_val {
        transform_new_fields_to_optional(&mut api_call.1.requests);
        transform_new_fields_to_optional(&mut api_call.1.responses);
    }

    ret_val
}

fn transform_new_fields_to_optional(api_calls: &mut Vec<Vec<ApiStruct>>) {
    let (oldest_api_call, newer_api_calls) = api_calls.split_first_mut().unwrap();
    assert_eq!(oldest_api_call.get(0).unwrap().version, 0);
    for api_call in newer_api_calls {
        for struc in api_call {
            let base_struc = oldest_api_call.iter().find(|ac| ac.name == struc.name);
            match base_struc {
                None => {}
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
}

fn parse_call(grouped_call: ApiCall) -> Vec<ApiStruct> {
    let mut ret_val = Vec::new();

    let name = format!("{}{:?}", grouped_call.name, grouped_call.typ);
    let (fields, mut sub_structs_int) =
        parse_vec(grouped_call.fields, name.clone(), grouped_call.version);
    let val = ApiStruct {
        name,
        version: grouped_call.version,
        fields,
    };
    ret_val.push(val);
    ret_val.append(&mut sub_structs_int);

    ret_val
}

fn parse_vec(
    fields: Vec<FieldData>,
    prefix: String,
    api_version: i32,
) -> (Vec<StructField>, Vec<ApiStruct>) {
    let mut sub_structs = Vec::new();
    let mut returned_fields = vec![];
    for field in fields {
        let ty = match field.type_with_payload {
            FieldTypeWithPayload::Field(ty) => {
                format!("{:?}", ty)
            }
            FieldTypeWithPayload::VecSimple(ty) => {
                format!("Vec<{:?}>", ty)
            }
            FieldTypeWithPayload::VecStruct(ty) => {
                let struct_name = format!("{}{}", prefix, to_upper_case(field.name));
                let (fields, mut sub_structs_int) = parse_vec(ty, struct_name.clone(), api_version);
                let val = ApiStruct {
                    name: struct_name.clone(),
                    version: api_version,
                    fields,
                };
                sub_structs.push(val);
                sub_structs.append(&mut sub_structs_int);
                format!("{}{}", struct_name, api_version)
            }
        };
        returned_fields.push(StructField {
            name: field.name,
            ty,
        });
    }
    (returned_fields, sub_structs)
}

#[derive(Default, Debug)]
pub struct GroupedApiCall<'a> {
    pub requests: Vec<Vec<ApiStruct<'a>>>,
    pub responses: Vec<Vec<ApiStruct<'a>>>,
}

#[derive(Debug)]
pub struct ApiStruct<'a> {
    pub name: String,
    pub version: i32,
    pub fields: Vec<StructField<'a>>,
}

#[derive(Debug)]
pub struct StructField<'a> {
    pub name: &'a str,
    pub ty: String,
}
