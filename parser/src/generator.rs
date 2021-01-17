
use crate::{
    transformer::{ApiStruct, GroupedApiCall, StructField},
};

pub fn generate_content(api_call: GroupedApiCall) -> String {

    let request_type_alias = generate_type_alias(api_call.requests.last().unwrap().first().unwrap());
    let response_type_alias = generate_type_alias(api_call.responses.last().unwrap().first().unwrap());

    let structs = api_call
        .requests
        .into_iter()
        .chain(api_call.responses.into_iter())
        .flatten()
        .map(genrate_struct)
        .fold("".to_string(), |acc, x| format!("{} \n{}", acc, x));

    format!("{}{}\n{}]",request_type_alias,response_type_alias,structs)
}

fn genrate_struct(api_call: ApiStruct) -> String {
    let struct_name = format!("{}{}", api_call.name, api_call.version);
      
    let struct_name_with_version = format!("pub struct {} {{ \n", struct_name);
    let fields = api_call
        .fields
        .iter()
        .map(|x| generate_field(x))
        .fold("".to_string(), |acc, x| format!("{}{}", acc, x));

    format!("{}{}{}", struct_name_with_version, fields, "}\n")
}

fn generate_field(field: &StructField) -> String {
    format!("    pub {}: {},\n", field.name, field.ty)
}

fn generate_type_alias(struc:&ApiStruct)->String{
    format!("pub type {} = {}{};\n",struc.name,struc.name,struc.version)
}