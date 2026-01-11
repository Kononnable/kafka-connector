use super::structs::{ApiSpec, ApiSpecField, ApiSpecFieldSubtype, ApiSpecType};
use convert_case::{Case, Casing};
use std::collections::VecDeque;

fn generate_substuct(
    content: &mut String,
    sub_struct: &SubStruct,
    is_map_key: bool,
    mut sub_structs_partial: &mut VecDeque<SubStruct>,
) {
    let default_derive = if should_derive_default(&sub_struct.fields) {
        ", Default"
    } else {
        ""
    };
    let map_key_derive = if is_map_key { ", Eq, Hash" } else { "" };
    content.push_str(&format!(
        "#[derive(Clone, Debug, PartialEq{}{})]\n",
        default_derive, map_key_derive
    ));
    content.push_str(&format!("pub struct {} {{\n", sub_struct.name));
    for field in &sub_struct.fields {
        content.push_str(&get_field_definition(field, &mut sub_structs_partial));
    }
    content.push_str("}\n\n");
}

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
        match &sub_struct.subtype {
            SubStructType::SubObject => {
                generate_substuct(&mut content, &sub_struct, false, &mut sub_structs_partial)
            }
            SubStructType::IndexSet => {
                generate_substuct(&mut content, &sub_struct, true, &mut sub_structs_partial)
            }
            SubStructType::IndexMap {
                fields_key,
                fields_value,
                key_type_name,
                value_type_name,
            } => {
                generate_substuct(
                    &mut content,
                    &SubStruct {
                        name: key_type_name.clone(),
                        fields: fields_key.clone(),
                        subtype: SubStructType::SubObject,
                    },
                    true,
                    &mut sub_structs_partial,
                );
                generate_substuct(
                    &mut content,
                    &SubStruct {
                        name: value_type_name.clone(),
                        fields: fields_value.clone(),
                        subtype: SubStructType::SubObject,
                    },
                    false,
                    &mut sub_structs_partial,
                );
            }
        }
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
                if let SubStructType::IndexMap {
                    fields_key,
                    fields_value,
                    key_type_name,
                    value_type_name,
                } = substruct.subtype
                {
                    content.push_str(&format!(
                        "impl ToBytes for IndexMap<{key_type_name},{value_type_name}>{{\n"
                    ));
                    content.push_str(
                        "    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {\n",
                    );
                    content.push_str("        _bytes.put_i32(self.len() as i32);\n");
                    content.push_str("        for (key, value) in self {\n");

                    for field in &substruct.fields {
                        let field_serialization = serialize_field(field);
                        if fields_key.iter().any(|x| x.name == field.name) {
                            content.push_str(&field_serialization.replace("self.", "    key."));
                        } else {
                            content.push_str(&field_serialization.replace("self.", "    value."));
                        }
                    }
                    content.push_str("        }\n\n");
                    content.push_str("    }\n\n");
                    content.push_str("}\n\n");
                    content.push_str(&generate_validate_fields(&key_type_name, &fields_key));
                    content.push_str(&generate_validate_fields(&value_type_name, &fields_value));

                    content.push_str(&format!(
                        "impl  FromBytes for IndexMap<{key_type_name},{value_type_name}>{{\n"
                    ));
                    content.push_str(
                        "    fn deserialize(version: ApiVersion, bytes: &mut BytesMut) -> Self {\n",
                    );
                    content.push_str(
                        "        let cap: i32 = FromBytes::deserialize(version, bytes);\n",
                    );
                    content
                        .push_str("        let mut ret = IndexMap::with_capacity(cap as usize);\n");
                    content.push_str("        for _ in 0..cap {\n");
                    for field in &substruct.fields {
                        content.push_str(&"            ");
                        content.push_str(&deserialize_field(field));
                    }

                    content.push_str(&format!("            let key = {} {{\n", key_type_name));
                    for field in &fields_key {
                        content.push_str(&format!(
                            "            {},\n",
                            field.name.to_case(Case::Snake)
                        ));
                    }
                    content.push_str("            };\n");
                    content.push_str(&format!("            let value = {} {{\n", value_type_name));
                    for field in &fields_value {
                        content.push_str(&format!(
                            "            {},\n",
                            field.name.to_case(Case::Snake)
                        ));
                    }
                    content.push_str("            };\n");
                    content.push_str("            ret.insert(key, value);\n");

                    content.push_str("        }\n\n");
                    content.push_str("    ret\n\n");
                    content.push_str("    }\n\n");
                    content.push_str("}\n\n");

                    content.push_str(&impl_default_trait(&key_type_name, &fields_key));
                    content.push_str(&impl_default_trait(&value_type_name, &fields_value));
                } else {
                    content.push_str(&format!("impl ToBytes for {}{{\n", substruct.name));
                    content.push_str(
                        "    fn serialize(&self, version: ApiVersion, _bytes: &mut BytesMut) {\n",
                    );
                    for field in &substruct.fields {
                        content.push_str(&serialize_field(field));
                    }
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
    let mut content = format!("impl {}{{\n", struct_name);
    content.push_str(
        "    fn validate_fields(&self, _version: ApiVersion) -> Result<(),SerializationError>{\n",
    );

    for field in fields.iter() {
        let flatten = if field.nullable_versions.is_some() {
            ".flatten()".to_owned()
        } else {
            "".to_owned()
        };
        match &field.type_.type_ {
            ApiSpecFieldSubtype::Map { .. } => {
                content.push_str(&format!(
                    "      for item in self.{}.iter(){} {{\n",
                    field.name.to_case(Case::Snake),
                    flatten
                ));
                if field.fields.iter().all(|x| x.map_key) {
                    content.push_str(&"        item.validate_fields(_version)?;\n");
                } else {
                    content.push_str(&"        item.0.validate_fields(_version)?;\n");
                    content.push_str(&"        item.1.validate_fields(_version)?;\n");
                }
                content.push_str("      }\n");
            }
            ApiSpecFieldSubtype::SubObject(_) => {
                if field.type_.is_array {
                    content.push_str(&format!(
                        "      for item in self.{}.iter(){} {{\n",
                        field.name.to_case(Case::Snake),
                        flatten
                    ));
                    content.push_str(&"        item.validate_fields(_version)?;\n");
                    content.push_str("      }\n");
                } else {
                    content.push_str(&format!(
                        "      self.{}.validate_fields(_version)?;\n",
                        field.name.to_case(Case::Snake)
                    ));
                }
            }
            _ => {}
        }
    }
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
        let default_value = field.default.clone().unwrap_or(format!(
            "{}::default()",
            apply_turbo_fish(field_non_option_type)
        ));
        let nullable_filter = if field.nullable_versions.is_some() {
            format!("self.{}.is_some()", field.name.to_case(Case::Snake))
        } else {
            match default_value.as_str() {
                "true" => format!("!self.{}", field.name.to_case(Case::Snake)),
                "false" => format!("self.{}", field.name.to_case(Case::Snake)),
                _ => format!(
                    "self.{} != {}",
                    field.name.to_case(Case::Snake),
                    default_value
                ),
            }
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
                "        if {nullable_filter} && _version.0 < {min}{{\n",
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
        "self.{}.serialize(version, _bytes);",
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

struct SubStruct {
    name: String,
    fields: Vec<ApiSpecField>,
    subtype: SubStructType,
}

enum SubStructType {
    SubObject,
    IndexSet,
    /// Two struct underneath, SubStruct->fields contains all fields in a proper order (needed for serialization),
    IndexMap {
        fields_key: Vec<ApiSpecField>,
        fields_value: Vec<ApiSpecField>,
        key_type_name: String,
        value_type_name: String,
    },
}

fn get_field_definition(field: &ApiSpecField, sub_structs: &mut VecDeque<SubStruct>) -> String {
    let mut content = "".to_owned();
    let field_type = get_field_type(field);
    match &field.type_.type_ {
        ApiSpecFieldSubtype::SubObject(name) => {
            sub_structs.push_back(SubStruct {
                name: name.to_owned(),
                fields: field.fields.clone(),
                subtype: SubStructType::SubObject,
            });
        }
        ApiSpecFieldSubtype::Map { name, keys } => {
            let is_indexmap = field.fields.len() != keys.len();
            let type_names = get_map_key_type_names(name, is_indexmap);

            if let Some(value_type) = type_names.value_type {
                let (keys, values): (Vec<_>, Vec<_>) =
                    field.fields.clone().into_iter().partition(|z| z.map_key);

                sub_structs.push_back(SubStruct {
                    name: "// TODO: IndexMap".to_owned(),
                    fields: field.fields.clone(),
                    subtype: SubStructType::IndexMap {
                        fields_key: keys,
                        fields_value: values,
                        key_type_name: type_names.key_type,
                        value_type_name: value_type,
                    },
                });
            } else {
                sub_structs.push_back(SubStruct {
                    name: type_names.key_type,
                    fields: keys.clone(),
                    subtype: SubStructType::IndexSet,
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
        ApiSpecFieldSubtype::Map { name, keys } => {
            let is_indexmap = field.fields.len() != keys.len();
            get_map_key_type_names(name, is_indexmap).full_type
        }
    }
}

pub struct MapKeyTypeNames {
    pub full_type: String,
    pub key_type: String,
    pub value_type: Option<String>,
}

fn get_map_key_type_names(name: &str, is_indexmap: bool) -> MapKeyTypeNames {
    if is_indexmap {
        MapKeyTypeNames {
            full_type: format!("IndexMap<{name}Key,{name}>"),
            key_type: format!("{name}Key"),
            value_type: Some(name.to_owned()),
        }
    } else {
        MapKeyTypeNames {
            full_type: format!("IndexSet<{name}>"),
            key_type: name.to_owned(),
            value_type: None,
        }
    }
}
