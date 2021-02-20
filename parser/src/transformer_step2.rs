use crate::transformer_step1::{ApiEndpoint, ApiStructDefinition};

pub fn transform(api_name: &str, endpoint: ApiEndpoint) -> File {
    let mut ret_val = File {
        file_name: api_name.to_owned(),
        api_calls: vec![],
    };

    let mut calls = endpoint
        .requests
        .into_iter()
        .zip(endpoint.responses.into_iter());
    let (first_req, first_resp) = calls.next().unwrap();

    let mut current_requests = vec![first_req];
    let mut current_responses = vec![first_resp];

    let mut api_def_number = 0;
    for (reqs, resps) in calls {
        let req_compatible = current_requests
            .last()
            .unwrap()
            .iter()
            .all(|x| reqs.iter().any(|y| y.name == x.name));
        let resp_compatible = current_responses
            .last()
            .unwrap()
            .iter()
            .all(|x| resps.iter().any(|y| y.name == x.name));
        if req_compatible && resp_compatible {
            current_requests.push(reqs);
            current_responses.push(resps);
        } else {
            ret_val.api_calls.push(ApiCall2 {
                api_name: api_name.to_owned(),
                api_struct_version: api_def_number,
                max_supported_version: current_requests.last().unwrap().first().unwrap().version,
                min_flexible_versions: get_min_flexible_version(current_requests.as_ref()),
                min_supported_version: current_requests.first().unwrap().first().unwrap().version,
                request_name: current_requests
                    .first()
                    .unwrap()
                    .first()
                    .unwrap()
                    .name
                    .clone(),
                response_name: current_responses
                    .first()
                    .unwrap()
                    .first()
                    .unwrap()
                    .name
                    .clone(),
                req_structs: transform_structs(current_requests, api_def_number, true),
                resp_structs: transform_structs(current_responses, api_def_number, false),
            });
            api_def_number += 1;
            current_requests = vec![reqs];
            current_responses = vec![resps];
        }
    }
    ret_val.api_calls.push(ApiCall2 {
        api_name: api_name.to_owned(),
        api_struct_version: api_def_number,
        max_supported_version: current_requests.last().unwrap().first().unwrap().version,
        min_flexible_versions: get_min_flexible_version(current_requests.as_ref()),
        min_supported_version: current_requests.first().unwrap().first().unwrap().version,
        request_name: current_requests
            .first()
            .unwrap()
            .first()
            .unwrap()
            .name
            .clone(),
        response_name: current_responses
            .first()
            .unwrap()
            .first()
            .unwrap()
            .name
            .clone(),
        req_structs: transform_structs(current_requests, api_def_number, true),
        resp_structs: transform_structs(current_responses, api_def_number, false),
    });

    ret_val
}

fn transform_structs(
    current: Vec<Vec<ApiStructDefinition>>,
    api_struct_version: u8,
    is_request: bool,
) -> Vec<ApiStruct2> {
    let mut ret_val: Vec<_> = current
        .iter()
        .last()
        .unwrap()
        .iter()
        .map(|def| ApiStruct2 {
            name: def.name.clone(),
            fields: def
                .fields
                .iter()
                .map(|field| ApiField2 {
                    is_vec: field.is_vec,
                    min_version: current
                        .iter()
                        .flatten()
                        .filter(|x| {
                            x.name == def.name && x.fields.iter().any(|y| y.name == field.name)
                        })
                        .min_by_key(|y| y.version)
                        .unwrap()
                        .version,
                    name: field.name.to_owned(),
                    ty: field.ty.clone(),
                    is_optional: false,
                    is_struct_field: field.is_struct_field,
                })
                .collect(),
        })
        .collect();

    let min_version = current.first().unwrap().first().unwrap().version;
    for struc in ret_val.iter_mut() {
        for field in struc.fields.iter_mut() {
            if field.is_struct_field {
                field.ty = format!("{}{}", field.ty, api_struct_version)
            }
            if field.is_vec {
                field.ty = format!("Vec<{}>", field.ty);
            }
            if field.min_version != min_version && !is_request {
                field.ty = format!("Option<{}>", field.ty);
                field.is_optional = true;
            }
        }
    }
    ret_val
}

fn get_min_flexible_version(current_requests: &[Vec<ApiStructDefinition>]) -> Option<i16> {
    current_requests
        .iter()
        .flatten()
        .filter(|x| x.fields.iter().any(|y| y.is_compact_field))
        .min_by_key(|x| x.version)
        .map(|x| x.version)
}

pub struct File {
    pub file_name: String,
    pub api_calls: Vec<ApiCall2>,
}
pub struct ApiCall2 {
    pub api_name: String,
    pub api_struct_version: u8,
    pub request_name: String,
    pub response_name: String,
    pub min_supported_version: i16,
    pub max_supported_version: i16,
    pub min_flexible_versions: Option<i16>,
    pub req_structs: Vec<ApiStruct2>,
    pub resp_structs: Vec<ApiStruct2>,
}

pub struct ApiStruct2 {
    pub name: String,
    pub fields: Vec<ApiField2>,
}
pub struct ApiField2 {
    pub name: String,
    pub ty: String,
    pub min_version: i16,
    pub is_vec: bool,
    pub is_struct_field: bool,
    pub is_optional: bool,
}
