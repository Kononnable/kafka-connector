use super::structs::{ApiSpec, ApiSpecField, ApiSpecFieldSubtype, ApiSpecType};
use convert_case::{Case, Casing};
use std::collections::VecDeque;

pub fn generate_source(spec: ApiSpec) -> (String, String) {
    let mut content = "use super::super::prelude::*;\n\n".to_owned();
    let mut sub_structs_partial = VecDeque::new();
    let mut sub_structs = Vec::new();
    for line in &spec.api_versions_comment {
        content.push_str(&format!("/// {line}\n"));
    }
    if should_derive_default(&spec.fields) {
        content.push_str("#[derive(Clone, Debug, Default, PartialEq)]\n");
    } else {
        content.push_str("#[derive(Clone, Debug, PartialEq)]\n");
    }
    content.push_str(&format!("pub struct {} {{\n", spec.name));
    for field in &spec.fields {
        content.push_str(&get_field_definition(field, &mut sub_structs_partial));
    }
    content.push_str("}\n\n");

    while let Some(sub_struct) = sub_structs_partial.pop_front() {
        let default_derive = if should_derive_default(&sub_struct.fields) {
            ", Default"
        } else {
            ""
        };
        let map_key_derive = if sub_struct.is_map_key {
            ", Eq, Hash"
        } else {
            ""
        };
        content.push_str(&format!(
            "#[derive(Clone, Debug, PartialEq{}{})]\n",
            default_derive, map_key_derive
        ));
        content.push_str(&format!("pub struct {} {{\n", sub_struct.name));
        for field in &sub_struct.fields {
            content.push_str(&get_field_definition(field, &mut sub_structs_partial));
        }
        content.push_str("}\n\n");
        sub_structs.push(sub_struct);
    }

    match spec.type_ {
        ApiSpecType::Request | ApiSpecType::Response => {
            match spec.type_ {
                ApiSpecType::Request => {
                    let response_type = spec.name.replace("Request", "Response");
                    content.push_str(&format!("impl ApiRequest for {}{{\n", spec.name));
                    content.push_str(&format!(
                        "    type Response = super::{}::{};\n\n",
                        response_type.to_case(Case::Snake),
                        response_type
                    ));
                }
                ApiSpecType::Response => {
                    let request_type = spec.name.replace("Response", "Request");
                    content.push_str(&format!("impl ApiResponse for {}{{\n", spec.name));
                    content.push_str(&format!(
                        "    type Request = super::{}::{};\n\n",
                        request_type.to_case(Case::Snake),
                        request_type
                    ));
                }
                ApiSpecType::Header => {
                    unreachable!()
                }
            }

            content.push_str(&get_api_key(&spec));
            content.push_str(&get_min_max_supported_version(&spec));

            match spec.type_ {
                ApiSpecType::Request => {
                    content.push_str("    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) -> Result<(),SerializationError>{\n");
                }
                ApiSpecType::Response => {
                    content.push_str("    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) -> Result<(),SerializationError>{\n");
                }
                ApiSpecType::Header => {
                    unreachable!()
                }
            }
            content
                .push_str("        debug_assert!(version >= Self::get_min_supported_version());\n");
            content
                .push_str("        debug_assert!(version <= Self::get_max_supported_version());\n");
            content.push_str("        self.validate_fields(version)?;\n");
            for field in &spec.fields {
                content.push_str(&serialize_field(field));
            }
            content.push_str("    Ok(())\n");
            content.push_str("    }\n\n");

            let no_fields_marker = if spec.fields.is_empty() { "_" } else { "" };
            match spec.type_ {
                ApiSpecType::Request => {
                    content.push_str(
                        &format!("    fn deserialize({no_fields_marker}version: ApiVersion, {no_fields_marker}bytes: &mut BytesMut) -> Self {{\n"),
                    );
                    for field in &spec.fields {
                        content.push_str(&deserialize_field(field));
                    }
                    content.push_str(&format!("        {} {{\n", spec.name));
                    for field in &spec.fields {
                        content.push_str(&format!(
                            "            {},\n",
                            field.name.to_case(Case::Snake)
                        ));
                    }
                    content.push_str("        }\n\n");
                    content.push_str("    }\n\n");
                }
                ApiSpecType::Response => {
                    content.push_str(
                        &format!("    fn deserialize({no_fields_marker}version: ApiVersion, {no_fields_marker}bytes: &mut BytesMut) -> Self {{\n"),
                    );
                    for field in &spec.fields {
                        content.push_str(&deserialize_field(field));
                    }
                    content.push_str(&format!("        {} {{\n", spec.name));
                    for field in &spec.fields {
                        content.push_str(&format!(
                            "            {},\n",
                            field.name.to_case(Case::Snake)
                        ));
                    }
                    content.push_str("        }\n\n");
                    content.push_str("    }\n\n");
                }

                ApiSpecType::Header => {
                    unreachable!()
                }
            }

            content.push_str("}\n\n");

            content.push_str(&generate_validate_fields(&spec.name, &spec.fields));
            content.push_str(&impl_default_trait(&spec.name, &spec.fields));

            for substruct in sub_structs {
                content.push_str(&format!("impl ToBytes for {}{{\n", substruct.name));
                content.push_str("    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) -> Result<(),SerializationError> {\n");
                content.push_str("        self.validate_fields(version)?;\n");
                for field in &substruct.fields {
                    content.push_str(&serialize_field(field));
                }
                content.push_str("    Ok(())\n");
                content.push_str("    }\n\n");
                content.push_str("}\n\n");
                content.push_str(&generate_validate_fields(
                    &substruct.name,
                    &substruct.fields,
                ));

                content.push_str(&format!("impl  FromBytes for {}{{\n", substruct.name));
                content.push_str(
                    "    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {\n",
                );
                for field in &substruct.fields {
                    content.push_str(&deserialize_field(field));
                }
                content.push_str(&format!("        {} {{\n", substruct.name));
                for field in &substruct.fields {
                    content.push_str(&format!(
                        "            {},\n",
                        field.name.to_case(Case::Snake)
                    ));
                }
                content.push_str("        }\n\n");
                content.push_str("    }\n\n");
                content.push_str("}\n\n");

                content.push_str(&impl_default_trait(&substruct.name, &substruct.fields));
            }
        }
        ApiSpecType::Header => {
            content.push_str(&format!("impl {}{{\n", spec.name));
            content.push_str(&get_min_max_supported_version(&spec));
            content.push_str(
                        "    pub fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) -> Result<(),SerializationError> {\n",
                    );
            content
                .push_str("        debug_assert!(version >= Self::get_min_supported_version());\n");
            content
                .push_str("        debug_assert!(version <= Self::get_max_supported_version());\n");
            content.push_str("        self.validate_fields(version)?;\n");
            for field in &spec.fields {
                content.push_str(&serialize_field(field));
            }
            content.push_str("    Ok(())\n");
            content.push_str("    }\n\n");
            content.push_str(
                "    pub fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {\n",
            );
            for field in &spec.fields {
                content.push_str(&deserialize_field(field));
            }

            content.push_str(&format!("        {} {{\n", spec.name));
            for field in &spec.fields {
                content.push_str(&format!(
                    "            {},\n",
                    field.name.to_case(Case::Snake)
                ));
            }
            content.push_str("        }\n\n");
            content.push_str("    }\n\n");
            content.push_str("}\n\n");
            content.push_str(&generate_validate_fields(&spec.name, &spec.fields));
            content.push_str(&impl_default_trait(&spec.name, &spec.fields));
        }
    }

    (spec.name.to_case(Case::Snake), content)
}

fn generate_validate_fields(struct_name: &str, fields: &[ApiSpecField]) -> String {
    let mut content = "".to_owned();

    content.push_str(&format!("impl {}{{\n", struct_name));
    content.push_str(
        "    fn validate_fields(&self, _version: ApiVersion) -> Result<(),SerializationError>{\n",
    );
    for field in fields.iter().filter(|x| x.nullable_versions.is_some()) {
        let nullable_versions = field.nullable_versions.clone().unwrap();

        if field.versions.contains('-') {
            let mut split = nullable_versions.split('-');
            let min = split.next().unwrap().to_owned();
            let max = split.next().unwrap().to_owned();
            content.push_str(&format!(
                "    if self.{}.is_none() && !({min}..={max}).contains(&_version.0){{\n",
                field.name.to_case(Case::Snake)
            ));
            content.push_str(&format!(
                "        return Err(SerializationError::NullValue(\"{}\", *_version, \"{}\"))\n",
                field.name.to_case(Case::Snake),
                struct_name
            ));
            content.push_str("        }\n");
        } else {
            let min_null_version: u8 = nullable_versions.replace('+', "").parse().unwrap();
            let field_min_version: u8 = field.versions.replace('+', "").parse().unwrap();
            if field_min_version < min_null_version {
                content.push_str(&format!(
                    "        if self.{}.is_none() && !_version.0 < {min_null_version} {{\n",
                    field.name.to_case(Case::Snake)
                ));
                content.push_str(&format!(
                    "        return Err(SerializationError::NullValue(\"{}\", *_version, \"{}\"))\n",
                    field.name.to_case(Case::Snake),
                    struct_name
                ));
                content.push_str("        }\n");
            };
        }
    }
    for field in fields.iter().filter(|x| !x.ignorable) {
        let field_non_option_type = {
            let mut field_type = get_field_base_type(field);
            if field.type_.is_array {
                field_type = format!("Vec<{field_type}>");
            }
            field_type
        };
        let nullable_filter = if field.nullable_versions.is_some() {
            format!(
                "self.{}.is_some() && self.{} != Some({}::default())",
                field.name.to_case(Case::Snake),
                field.name.to_case(Case::Snake),
                apply_turbo_fish(field_non_option_type)
            )
        } else {
            format!(
                "self.{} != {}::default()",
                field.name.to_case(Case::Snake),
                apply_turbo_fish(field_non_option_type)
            )
        };
        if field.versions.contains('-') {
            let mut split = field.versions.split('-');
            let min = split.next().unwrap().to_owned();
            let max = split.next().unwrap().to_owned();
            content.push_str(&format!(
                "    if {nullable_filter} && !({min}..={max}).contains(&_version.0){{\n",
            ));
        } else if field.versions == "0+" {
            continue;
        } else {
            let min = field.versions.replace('+', "");
            content.push_str(&format!(
                "        if {nullable_filter} && _version >= ApiVersion({min}){{\n",
            ));
        };
        content.push_str(&format!(
            "        return Err(SerializationError::NonIgnorableFieldSet(\"{}\", *_version, \"{}\"))\n",
            field.name.to_case(Case::Snake),
            struct_name
        ));
        content.push_str("        }\n");
    }
    content.push_str("    Ok(())\n");
    content.push_str("    }\n\n");
    content.push_str("}\n\n");
    content
}

fn get_api_key(spec: &ApiSpec) -> String {
    let mut content = "".to_owned();

    content.push_str("    fn get_api_key() -> ApiKey {\n");
    content.push_str(&format!("        ApiKey({})\n", spec.api_key.unwrap()));
    content.push_str("    }\n\n");

    content
}

fn get_min_max_supported_version(spec: &ApiSpec) -> String {
    let mut content = "".to_owned();

    let min_supported_version = spec.valid_versions.split('-').next().unwrap();
    let max_supported_version = spec.valid_versions.split('-').last().unwrap();
    content.push_str("    fn get_min_supported_version() -> ApiVersion {\n");
    content.push_str(&format!("        ApiVersion({})\n", min_supported_version));
    content.push_str("    }\n\n");
    content.push_str("    fn get_max_supported_version() -> ApiVersion {\n");
    content.push_str(&format!("        ApiVersion({})\n", max_supported_version));
    content.push_str("    }\n\n");
    content
}

fn should_derive_default(fields: &[ApiSpecField]) -> bool {
    !fields
        .iter()
        .any(|x| match (&x.type_.type_, &x.type_.is_array, &x.default) {
            (_, _, None) => false,
            (ApiSpecFieldSubtype::Bool, false, Some(str)) if str == "false" => false,
            (ApiSpecFieldSubtype::Int8, false, Some(str)) if str == "0" => false,
            (ApiSpecFieldSubtype::Int16, false, Some(str)) if str == "0" => false,
            (ApiSpecFieldSubtype::Int32, false, Some(str)) if str == "0" => false,
            (ApiSpecFieldSubtype::Int64, false, Some(str)) if str == "0" => false,
            (_, _, Some(_)) => true,
        })
}

fn impl_default_trait(name: &str, fields: &Vec<ApiSpecField>) -> String {
    if should_derive_default(fields) {
        return "".to_owned();
    }
    let mut content = format!("impl Default for {name} {{\n",);
    content.push_str("fn default() -> Self {\n");
    content.push_str("    Self {\n");
    for field in fields {
        let default = field
            .default
            .clone()
            .unwrap_or("Default::default()".to_owned());
        content.push_str(&format!(
            "        {}: {},\n",
            field.name.to_case(Case::Snake),
            default
        ));
    }
    content.push_str("        }\n\n");
    content.push_str("    }\n\n");
    content.push_str("}\n\n");

    content
}

fn serialize_field(field: &ApiSpecField) -> String {
    let mut content = "".to_owned();
    let serialize = format!(
        "self.{}.serialize(version, _bytes)?;",
        field.name.to_case(Case::Snake)
    );

    if field.versions.contains('-') {
        let mut split = field.versions.split('-');
        let min = split.next().unwrap().to_owned();
        let max = split.next().unwrap().to_owned();
        content.push_str(&format!(
            "        if ({min}..={max}).contains(&version.0) {{\n"
        ));
        content.push_str(&format!("            {serialize}\n"));
        content.push_str("        }\n");
    } else if field.versions == "0+" {
        content.push_str(&format!("        {serialize}\n"));
    } else {
        let min = field.versions.replace('+', "");
        content.push_str(&format!("        if version >= ApiVersion({min}) {{\n"));
        content.push_str(&format!("            {serialize}\n"));
        content.push_str("        }\n");
    };
    content
}

fn deserialize_field(field: &ApiSpecField) -> String {
    let mut content = "".to_owned();
    let field_type = get_field_type(field);
    let deserialize = format!(
        "{}::deserialize(version, bytes)\n",
        apply_turbo_fish(field_type)
    );

    if field.versions.contains('-') {
        let mut split = field.versions.split('-');
        let min = split.next().unwrap().to_owned();
        let max = split.next().unwrap().to_owned();
        content.push_str(&format!(
            "        let {} = if ({min}..={max}).contains(&version.0) {{\n",
            field.name.to_case(Case::Snake)
        ));
        content.push_str(&format!("            {deserialize}\n"));
        content.push_str("        } else {\n");
        content.push_str("            Default::default()\n");
        content.push_str("        };\n");
    } else if field.versions == "0+" {
        content.push_str(&format!(
            "        let {} = {deserialize};\n",
            field.name.to_case(Case::Snake)
        ));
    } else {
        let min = field.versions.replace('+', "");
        content.push_str(&format!(
            "        let {} = if version >= ApiVersion({min}) {{\n",
            field.name.to_case(Case::Snake)
        ));
        content.push_str(&format!("            {deserialize}\n"));
        content.push_str("        } else {\n");
        content.push_str("            Default::default()\n");
        content.push_str("        };\n");
    };

    content
}

fn apply_turbo_fish(field_type: String) -> String {
    field_type.replace('<', "::<")
}

struct SubStructs {
    name: String,
    fields: Vec<ApiSpecField>,
    is_map_key: bool,
}

fn get_field_definition(field: &ApiSpecField, sub_structs: &mut VecDeque<SubStructs>) -> String {
    let mut content = "".to_owned();
    let field_type = get_field_type(field);
    match &field.type_.type_ {
        ApiSpecFieldSubtype::SubObject(name) => {
            sub_structs.push_back(SubStructs {
                name: name.to_owned(),
                fields: field.fields.clone(),
                is_map_key: false,
            });
        }
        ApiSpecFieldSubtype::Map { name, keys } => {
            let type_names = get_map_key_type_names(name, &field.fields);

            sub_structs.push_back(SubStructs {
                name: type_names.key_type,
                fields: keys.clone(),
                is_map_key: true,
            });

            if let Some(value_type) = type_names.value_type {
                sub_structs.push_back(SubStructs {
                    name: value_type,
                    fields: field.fields.clone(),
                    is_map_key: false,
                });
            }
        }
        _ => (),
    }

    if let Some(about) = &field.about {
        content.push_str(&format!("    /// {about}\n",));
    }
    content.push_str(&format!(
        "    pub {}: {field_type}, \n\n",
        field.name.to_case(Case::Snake)
    ));
    content
}

fn get_field_type(field: &ApiSpecField) -> String {
    let mut field_type = get_field_base_type(field);
    if field.type_.is_array {
        field_type = format!("Vec<{field_type}>");
    }
    if field.nullable_versions.is_some() {
        field_type = format!("Option<{field_type}>");
    }
    field_type
}

fn get_field_base_type(field: &ApiSpecField) -> String {
    match &field.type_.type_ {
        ApiSpecFieldSubtype::Bool => "bool".to_owned(),
        ApiSpecFieldSubtype::Bytes => "Vec<u8>".to_owned(),
        ApiSpecFieldSubtype::Int16 => "i16".to_owned(),
        ApiSpecFieldSubtype::Int8 => "i8".to_owned(),
        ApiSpecFieldSubtype::Int32 => "i32".to_owned(),
        ApiSpecFieldSubtype::Int64 => "i64".to_owned(),
        ApiSpecFieldSubtype::String => "String".to_owned(),
        ApiSpecFieldSubtype::SubObject(sub) => sub.to_owned(),
        ApiSpecFieldSubtype::Map { name, keys: _ } => {
            get_map_key_type_names(name, &field.fields).full_type
        }
    }
}

pub struct MapKeyTypeNames {
    pub full_type: String,
    pub key_type: String,
    pub value_type: Option<String>,
}

fn get_map_key_type_names(name: &str, subfields: &[ApiSpecField]) -> MapKeyTypeNames {
    if subfields.is_empty() {
        MapKeyTypeNames {
            full_type: format!("IndexSet<{name}>"),
            key_type: name.to_owned(),
            value_type: None,
        }
    } else {
        MapKeyTypeNames {
            full_type: format!("IndexMap<{name}Key,{name}>"),
            key_type: format!("{name}Key"),
            value_type: Some(name.to_owned()),
        }
    }
}
