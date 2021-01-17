use crate::{transformer::{ApiStruct, GroupedApiCall, StructField}, utils::to_snake_case};

pub fn generate_content(api_call: GroupedApiCall) -> String {
    let use_statements = "use super::prelude::*;\n";
    let request_type_alias =
        generate_type_alias(api_call.requests.last().unwrap().first().unwrap());
    let response_type_alias =
        generate_type_alias(api_call.responses.last().unwrap().first().unwrap());

    let request_version_call = serialize_api_request(&api_call.requests);
    let response_version_call = deserialize_api_response(&api_call.responses);

    let structs = api_call
        .requests
        .iter()
        .flatten()
        .map(|x|genrate_struct(x,true))
        .chain(api_call.responses.iter().flatten()
        .map(|x|genrate_struct(x,false))
    )
        .fold("".to_string(), |acc, x| format!("{}\n{}", acc, x));
    let from_latest_impl = genetate_impl_from_latest(api_call.requests);
    let to_latest_impl = genetate_impl_to_latest(api_call.responses);
    format!(
        "{}\n{}{}{}{}{}\n{}\n{}",
        use_statements,
        request_type_alias,
        response_type_alias,
        request_version_call,
        response_version_call,
        structs,
        from_latest_impl,
        to_latest_impl
    )
}

fn serialize_api_request(requests: &Vec<Vec<ApiStruct>>) -> String {
    let main_struct = requests.first().unwrap().first().unwrap();
    let struct_name = &main_struct.name;
    let mut fn_def = format!("pub fn serialize_{}(data:{},version:i32, buf: &mut BytesMut) -> Result<(),Error> {{\n",to_snake_case(struct_name),struct_name);
    fn_def.push_str("    match version {\n");
    for version in 0..requests.len()-1{
        fn_def.push_str(&format!("        {} => ToBytes::serialize(&{}{}::try_from(data)?,buf),\n",version,struct_name,version));
    }
    fn_def.push_str("        _ => ToBytes::serialize(&data,buf),\n");
    fn_def.push_str("    }\n");
    fn_def.push_str("    Ok(())\n");
    fn_def.push_str("}\n");
    fn_def

}

fn deserialize_api_response(responses: &Vec<Vec<ApiStruct>>) -> String {
    let main_struct = responses.first().unwrap().first().unwrap();
    let struct_name = &main_struct.name;
    let mut fn_def = format!("pub fn deserialize_{}<T>(version:i32, buf: &mut T) -> Result<{},Error> where T: Iterator<Item=u8> {{\n",to_snake_case(struct_name),struct_name);
    fn_def.push_str("    Ok(match version {\n");
    for version in 0..responses.len()-1{
        fn_def.push_str(&format!("        {} =>  {}{}::deserialize(buf).try_into()?,\n",version,struct_name,version));
    }
    fn_def.push_str(&format!("        _ => {}::deserialize(buf),\n",struct_name));
    fn_def.push_str("    })\n");
    fn_def.push_str("}\n");
    fn_def
}

fn genrate_struct(api_call: &ApiStruct, is_request:bool) -> String {
    let struct_name = format!("{}{}", api_call.name, api_call.version);

    let derive_bytes = if is_request{
        "ToBytes"
    }else {
        "FromBytes"
    };
    let derive = format!("#[derive(Default,{})]\n",derive_bytes);
    let struct_name_with_version = format!("pub struct {} {{ \n", struct_name);
    let fields = api_call
        .fields
        .iter()
        .map(|x| generate_field(x))
        .fold("".to_string(), |acc, x| format!("{}{}", acc, x));

    format!("{}{}{}{}", derive, struct_name_with_version, fields, "}\n")
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
                impl_def.push_str(&format!(
                    "impl TryFrom<{}{}> for {}{}{{\n",
                    latest.name, latest.version, call.name, call.version
                ));
                impl_def.push_str("    type Error = Error;\n");
                impl_def.push_str(&format!(
                    "    fn try_from(latest:{}{}) -> Result<Self, Self::Error> {{\n",
                    latest.name, latest.version
                ));
                for field in &latest.fields {
                    if call.fields.iter().find(|x| x.name == field.name).is_none() {
                        impl_def
                            .push_str(&format!("        if latest.{}.is_some() {{\n", field.name));
                        impl_def.push_str(&format!(
                            "            Err(Error::OldKafkaVersion(\"{}\",{},\"{}\"))?\n",
                            call.name, call.version, field.name
                        ));
                        impl_def.push_str(&format!("        }}\n"));
                    }
                }
                impl_def.push_str(&format!("        Ok({}{}{{\n", call.name, call.version));
                for field in &call.fields {
                    if field.ty.starts_with(&call.name) || field.ty.starts_with(&format!("Optional<{}",call.name))  {
                        impl_def.push_str(&format!(
                            "            {}: latest.{}.try_into()?,\n",
                            field.name, field.name
                        ));
                    } else {
                        impl_def.push_str(&format!(
                            "            {}: latest.{},\n",
                            field.name, field.name
                        ));
                    }
                }
                impl_def.push_str(&format!("        }})\n"));
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
                impl_def.push_str(&format!(
                    "impl TryFrom<{}{}> for {}{}{{\n",
                    call.name, call.version, latest.name, latest.version
                ));
                impl_def.push_str("    type Error = Error;\n");
                impl_def.push_str(&format!(
                    "    fn try_from(older:{}{}) -> Result<Self, Self::Error> {{\n",
                    call.name, call.version
                ));
                impl_def.push_str(&format!("        Ok({}{}{{\n", latest.name, latest.version));
                for field in &call.fields {
                    if field.ty.starts_with(&call.name) || field.ty.starts_with(&format!("Optional<{}",call.name))  {
                        impl_def.push_str(&format!(
                            "            {}: older.{}.try_into()?,\n",
                            field.name, field.name
                        ));
                    } else {
                        impl_def.push_str(&format!(
                            "            {}: older.{},\n",
                            field.name, field.name
                        ));
                    }
                }
                impl_def.push_str(&format!(
                    "            ..{}{}::default()\n",
                    latest.name, latest.version
                ));
                impl_def.push_str(&format!("        }})\n"));
                impl_def.push_str(&format!("    }}\n"));
                impl_def.push_str(&format!("}}\n\n"));
            }
        }
    }
    impl_def
}