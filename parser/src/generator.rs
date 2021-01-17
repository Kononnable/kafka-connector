use crate::transformer::{ApiStruct, GroupedApiCall, StructField};

pub fn generate_content(api_call: GroupedApiCall) -> String {
    let use_statements = "use super::prelude::*;\n";
    let request_type_alias =
        generate_type_alias(api_call.requests.last().unwrap().first().unwrap());
    let response_type_alias =
        generate_type_alias(api_call.responses.last().unwrap().first().unwrap());

    let structs = api_call
        .requests
        .iter()
        .chain(api_call.responses.iter())
        .flatten()
        .map(genrate_struct)
        .fold("".to_string(), |acc, x| format!("{}\n{}", acc, x));
    let from_latest_impl = genetate_impl_from_latest(api_call.requests);
    let to_latest_impl = genetate_impl_to_latest(api_call.responses);
    format!(
        "{}\n{}{}{}\n{}\n{}",
        use_statements,
        request_type_alias, response_type_alias, structs, from_latest_impl, to_latest_impl
    )
}

fn genrate_struct(api_call: &ApiStruct) -> String {
    let struct_name = format!("{}{}", api_call.name, api_call.version);


    let derive = "#[derive(Default,FromBytes,ToBytes)]\n";
    let struct_name_with_version = format!("pub struct {} {{ \n", struct_name);
    let fields = api_call
        .fields
        .iter()
        .map(|x| generate_field(x))
        .fold("".to_string(), |acc, x| format!("{}{}", acc, x));

    format!("{}{}{}{}",derive, struct_name_with_version, fields, "}\n")
}

fn generate_field(field: &StructField) -> String {
    format!("    pub {}: {},\n", field.name, field.ty)
}

fn generate_type_alias(struc: &ApiStruct) -> String {
    format!(
        "pub type {} = {}{};\n",
        struc.name, struc.name, struc.version
    )
}

fn genetate_impl_from_latest(api_calls: Vec<Vec<ApiStruct>>) -> String {
    let mut impl_def = "".to_owned();
    let (latest, older_calls) = api_calls.split_last().unwrap();
    for call in older_calls.iter().flatten() {
        let latest = latest.iter().find(|x| x.name == call.name);
        match latest {
            None => {}
            Some(latest) => {
                impl_def.push_str(&format!("impl From<{}{}> for {}{}{{\n", latest.name,latest.version, call.name, call.version));
                impl_def.push_str(&format!("    fn from(latest:{}{}) -> {}{}{{\n",latest.name,latest.version,call.name,call.version));
                impl_def.push_str(&format!("        {}{}{{\n",call.name,call.version));
                for field in &call.fields{
                    impl_def.push_str(&format!("            {}: latest.{}.into(),\n",field.name,field.name ));
                }
                impl_def.push_str(&format!("        }}\n"));
                impl_def.push_str(&format!("    }}\n"));
                impl_def.push_str(&format!("}}\n\n"));
            }
        }
    }
    impl_def
}

fn genetate_impl_to_latest(api_calls: Vec<Vec<ApiStruct>>) -> String {
    let mut impl_def = "".to_owned();
    let (latest, older_calls) = api_calls.split_last().unwrap();
    for call in older_calls.iter().flatten() {
        let latest = latest.iter().find(|x| x.name == call.name);
        match latest {
            None => {}
            Some(latest) => {
                impl_def.push_str(&format!("impl From<{}{}> for {}{}{{\n", call.name,call.version, latest.name, latest.version));
                impl_def.push_str(&format!("    fn from(older:{}{}) -> {}{}{{\n",call.name,call.version,latest.name,latest.version));
                impl_def.push_str(&format!("        {}{}{{\n",latest.name,latest.version));
                for field in &call.fields{
                    impl_def.push_str(&format!("            {}: older.{}.into(),\n",field.name,field.name ));
                }
                impl_def.push_str(&format!("            ..{}{}::default()\n",latest.name,latest.version ));
                impl_def.push_str(&format!("        }}\n"));
                impl_def.push_str(&format!("    }}\n"));
                impl_def.push_str(&format!("}}\n\n"));
            }
        }
    }
    impl_def
}
