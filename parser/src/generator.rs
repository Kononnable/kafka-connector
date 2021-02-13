use crate::transformer::{ApiEndpoint, ApiStructDefinition, StructField};

pub fn generate_content(api_call: ApiEndpoint, api_name: &str) -> String {
    let use_statements = "use super::prelude::*;\n";
    let request_type_alias =
        generate_type_alias(api_call.requests.last().unwrap().first().unwrap());
    let response_type_alias =
        generate_type_alias(api_call.responses.last().unwrap().first().unwrap());

    let api_struct = generate_api_struct(&api_call, api_name);

    let structs = api_call
        .requests
        .iter()
        .flatten()
        .map(|x| genrate_struct(x, true))
        .chain(
            api_call
                .responses
                .iter()
                .flatten()
                .map(|x| genrate_struct(x, false)),
        )
        .fold("".to_string(), |acc, x| format!("{}\n{}", acc, x));
    let from_latest_impl = genetate_impl_from_latest(api_call.requests);
    let to_latest_impl = genetate_impl_to_latest(api_call.responses);
    format!(
        "{}\n{}{}{}{}\n{}\n{}",
        use_statements,
        request_type_alias,
        response_type_alias,
        api_struct,
        structs,
        from_latest_impl,
        to_latest_impl
    )
}

fn generate_api_struct(api_call: &ApiEndpoint, api_name: &str) -> String {
    let mut def = format!("impl ApiCall for {}Request {{\n", api_name);
    def.push_str(&format!("type Response = {}Response;\n", api_name));
    let min_version = 0;
    let max_version = api_call.requests.len() - 1;
    def.push_str(&format!(
        "fn get_min_supported_version()->i16{{\n{}\n}}\n",
        min_version
    ));
    def.push_str(&format!(
        "fn get_max_supported_version()->i16{{\n{}\n}}\n",
        max_version
    ));
    def.push_str(&format!(
        "fn get_api_key()->ApiNumbers{{\nApiNumbers::{}\n}}\n",
        api_name
    ));
    def.push_str("fn is_flexible_version(version: i16) -> bool {\n match version{\n");
    for (version, struct_definition) in api_call.responses.iter().enumerate() {
        def.push_str(&format!(
            "        {} =>  {},\n",
            version,
            struct_definition.first().unwrap().is_flexible_version
        ));
    }
    def.push_str(&format!(
        "        _ =>  {},\n",
        api_call
            .responses
            .last()
            .unwrap()
            .first()
            .unwrap()
            .is_flexible_version
    ));
    def.push_str("}\n}\n");

    let request_version_call = serialize_api_request(&api_call.requests);
    let response_version_call = deserialize_api_response(&api_call.responses);
    def.push_str(&format!(
        "{}{}}}",
        request_version_call, response_version_call
    ));
    def
}

fn serialize_api_request(requests: &[Vec<ApiStructDefinition>]) -> String {
    let main_struct = requests.first().unwrap().first().unwrap();
    let struct_name = &main_struct.name;
    let mut fn_def =
        "fn serialize(self,version:i16, buf: &mut BytesMut,correlation_id: i32,client_id: &str){\n"
            .to_owned();

    fn_def.push_str("match Self::is_flexible_version(version) {\n");
    fn_def.push_str(&format!("true => HeaderRequest2::new({}::get_api_key(), version, correlation_id, client_id).serialize(buf, false),\n",struct_name));
    fn_def.push_str(&format!("false => HeaderRequest1::new({}::get_api_key(), version, correlation_id, client_id).serialize(buf, false),\n",struct_name));
    fn_def.push_str("}\n");

    fn_def.push_str("    match version {\n");

    for version in 0..requests.len() - 1 {
        fn_def.push_str(&format!(
            "        {} => ToBytes::serialize(&{}{}::from(self),buf,Self::is_flexible_version(version)),\n",
            version,
            struct_name,
            version,
        ));
    }
    fn_def.push_str(&format!(
        "        {} => ToBytes::serialize(&self,buf, Self::is_flexible_version(version)),\n",
        requests.len() - 1,
    ));
    fn_def.push_str(
        "        _ => ToBytes::serialize(&self,buf, Self::is_flexible_version(version)),\n",
    );
    fn_def.push_str("    }\n");
    fn_def.push_str("}\n");
    fn_def
}

fn deserialize_api_response(responses: &[Vec<ApiStructDefinition>]) -> String {
    let main_struct = responses.first().unwrap().first().unwrap();
    let struct_name = &main_struct.name;
    let mut fn_def = format!(
        "fn deserialize_response(version:i16, buf: &mut Bytes) -> (i32,{}) {{\n",
        struct_name
    );

    fn_def.push_str("let correlation = match Self::is_flexible_version(version) {\n");
    fn_def.push_str("true => HeaderResponse2::deserialize(buf, false).correlation,\n");
    fn_def.push_str("false => HeaderResponse::deserialize(buf, false).correlation,\n");
    fn_def.push_str("};\n");

    fn_def.push_str("let response = match version {\n");
    for version in 0..responses.len() - 1 {
        fn_def.push_str(&format!(
            "        {} =>  {}{}::deserialize(buf, Self::is_flexible_version(version)).into(),\n",
            version, struct_name, version,
        ));
    }
    fn_def.push_str(&format!(
        "        {} => {}::deserialize(buf, Self::is_flexible_version(version)),\n",
        responses.len() - 1,
        struct_name,
    ));
    fn_def.push_str(&format!(
        "        _ => {}::deserialize(buf, Self::is_flexible_version(version)),\n",
        struct_name
    ));
    fn_def.push_str("    };\n");
    fn_def.push_str("(correlation, response)\n");
    fn_def.push_str("}\n");
    fn_def
}

fn genrate_struct(api_call: &ApiStructDefinition, is_request: bool) -> String {
    let struct_name = format!("{}{}", api_call.name, api_call.version);

    let derive_bytes = if is_request { "ToBytes" } else { "FromBytes" };
    let derive = format!("#[derive(Default,Debug,Clone,{})]\n", derive_bytes);
    let struct_name_with_version = format!("pub struct {} {{ \n", struct_name);
    let fields = api_call
        .fields
        .iter()
        .map(generate_field)
        .fold("".to_string(), |acc, x| format!("{}{}", acc, x));

    format!("{}{}{}{}", derive, struct_name_with_version, fields, "}\n")
}

fn generate_field(field: &StructField) -> String {
    format!("    pub {}: {},\n", field.name, field.ty)
}

fn generate_type_alias(struc: &ApiStructDefinition) -> String {
    format!(
        "pub type {} = {}{};\n",
        struc.name, struc.name, struc.version
    )
}

fn genetate_impl_from_latest(api_calls: Vec<Vec<ApiStructDefinition>>) -> String {
    let mut impl_def = "".to_owned();
    let (latest, older_calls) = api_calls.split_last().unwrap();
    for call in older_calls.iter().flatten() {
        let latest = latest.iter().find(|x| x.name == call.name);
        match latest {
            None => {}
            Some(latest) => {
                impl_def.push_str(&format!(
                    "impl From<{}{}> for {}{}{{\n",
                    latest.name, latest.version, call.name, call.version
                ));
                let latest_str = if call.fields.is_empty() {
                    "_latest"
                } else {
                    "latest"
                };
                impl_def.push_str(&format!(
                    "    fn from({}:{}{}) -> {}{} {{\n",
                    latest_str, latest.name, latest.version, call.name, call.version
                ));
                for field in &latest.fields {
                    if call.fields.iter().find(|x| x.name == field.name).is_none()
                        && !field.is_compact_field
                    {
                        impl_def.push_str(&format!(
                            "            log::debug!(\"Using old api format - {}{}, ignoring field {}\");\n",
                            call.name, call.version, field.name
                        ));
                    }
                }
                impl_def.push_str(&format!("        {}{}{{\n", call.name, call.version));
                for field in &call.fields {
                    let latest_field = latest
                        .fields
                        .iter()
                        .find(|latest_field| latest_field.name == field.name);
                    if latest_field.is_none() {
                        continue;
                    }
                    let latest_field = latest_field.unwrap();
                    let conversion = if field.ty != latest_field.ty {
                        let mut conversion = if latest_field.is_simple_type && field.is_vec {
                            ".into_iter().collect()"
                        } else if latest_field.is_simple_type && !field.is_vec {
                            ""
                        } else if !latest_field.is_simple_type && field.is_vec && !field.is_optional
                        {
                            ".into_iter().map(|ele|ele.into()).collect()"
                        } else if !latest_field.is_simple_type && field.is_vec && field.is_optional
                        {
                            ".map(|val|val.into_iter().map(|el|el.into()).collect())"
                        } else {
                            ".into()"
                        }
                        .to_owned();
                        if field.is_optional && (field.is_simple_type || !field.is_vec) {
                            conversion = format!(".map(|val|val{})", conversion);
                        }
                        conversion
                    } else {
                        "".to_owned()
                    };

                    impl_def.push_str(&format!(
                        "            {}: latest.{}{},\n",
                        field.name, field.name, conversion
                    ));
                }

                let cond = |call_field: &StructField| {
                    latest
                        .fields
                        .iter()
                        .any(|latest_field| call_field.name == latest_field.name)
                };
                if !call.fields.iter().all(cond) {
                    impl_def.push_str(&format!(
                        "            ..{}{}::default()\n",
                        call.name, call.version
                    ));
                }

                impl_def.push_str(&"        }\n");
                impl_def.push_str(&"    }\n");
                impl_def.push_str(&"}\n\n");
            }
        }
    }
    impl_def
}

fn genetate_impl_to_latest(api_calls: Vec<Vec<ApiStructDefinition>>) -> String {
    let mut impl_def = "".to_owned();
    let (latest, older_calls) = api_calls.split_last().unwrap();
    for call in older_calls.iter().flatten() {
        let latest = latest.iter().find(|x| x.name == call.name);
        match latest {
            None => {}
            Some(latest) => {
                impl_def.push_str(&format!(
                    "impl From<{}{}> for {}{}{{\n",
                    call.name, call.version, latest.name, latest.version
                ));
                impl_def.push_str(&format!(
                    "    fn from(older:{}{}) -> Self {{\n",
                    call.name, call.version
                ));
                impl_def.push_str(&format!("        {}{}{{\n", latest.name, latest.version));
                for field in &call.fields {
                    let latest_field = latest
                        .fields
                        .iter()
                        .find(|latest_field| latest_field.name == field.name);
                    if latest_field.is_none() {
                        continue;
                    }
                    let conversion = if field.ty != latest_field.unwrap().ty {
                        let mut conversion = if field.is_simple_type && field.is_vec {
                            ".into_iter().collect()"
                        } else if field.is_simple_type
                            && !field.is_vec
                            && !latest_field.unwrap().is_easily_convertable
                        {
                            ""
                        } else if !field.is_simple_type && field.is_vec {
                            ".into_iter().map(|el|el.into()).collect()"
                        } else {
                            ".into()"
                        }
                        .to_owned();
                        if field.is_optional {
                            conversion = format!(".map(|val|val{})", conversion);
                        }
                        conversion
                    } else {
                        "".to_owned()
                    };
                    impl_def.push_str(&format!(
                        "            {}: older.{}{},\n",
                        field.name, field.name, conversion
                    ));
                }
                if !latest
                    .fields
                    .iter()
                    .all(|latest_field| call.fields.iter().any(|y| latest_field.name == y.name))
                {
                    impl_def.push_str(&format!(
                        "            ..{}{}::default()\n",
                        latest.name, latest.version
                    ));
                }
                impl_def.push_str(&"        }\n");
                impl_def.push_str(&"    }\n");
                impl_def.push_str(&"}\n\n");
            }
        }
    }
    impl_def
}
