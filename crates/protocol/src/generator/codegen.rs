use super::structs::{ApiSpec, ApiSpecField, ApiSpecFieldSubtype, ApiSpecFieldType, ApiSpecType};
use convert_case::{Case, Casing};
use std::collections::VecDeque;

pub fn generate_source(spec: &ApiSpec) -> (String, String) {
    let mut content = "use super::super::prelude::*;\n\n".to_owned();
    let mut sub_structs_partial = VecDeque::new();
    let mut sub_structs = Vec::new();
    if should_derive_default(&spec.fields) {
        content.push_str("#[derive(Clone, Debug, Default)]\n");
    } else {
        content.push_str("#[derive(Clone, Debug)]\n");
    }
    content.push_str(&format!("pub struct {} {{\n", spec.name));
    for field in &spec.fields {
        content.push_str(&get_field_definition(field, &mut sub_structs_partial));
    }
    content.push_str("}\n\n");

    while let Some((s_name, s_fields)) = sub_structs_partial.pop_front() {
        if should_derive_default(&s_fields) {
            content.push_str("#[derive(Clone, Debug, Default)]\n");
        } else {
            content.push_str("#[derive(Clone, Debug)]\n");
        }
        content.push_str(&format!("pub struct {} {{\n", s_name));
        for field in &s_fields {
            content.push_str(&get_field_definition(field, &mut sub_structs_partial));
        }
        content.push_str("}\n\n");
        sub_structs.push((s_name, s_fields));
    }

    match spec.type_ {
        ApiSpecType::Request => {
            let response_type = spec.name.replace("Request", "Response");
            content.push_str(&format!("impl ApiRequest for {}{{\n", spec.name));
            content.push_str(&format!(
                "    type Response = super::{}::{};\n\n",
                response_type.to_case(Case::Snake),
                response_type
            ));
            content.push_str(&get_api_key(&spec));
            content.push_str(&get_min_max_supported_version(&spec));
            content.push_str("    fn serialize(&self, version: i16, bytes: &mut BytesMut, header: &RequestHeader) -> Result<(),SerializationError>{\n");
            content.push_str(
                "        debug_assert!(header.request_api_key == Self::get_api_key());\n",
            );
            content.push_str("        debug_assert!(header.request_api_version == version);\n");
            content
                .push_str("        debug_assert!(version >= Self::get_min_supported_version());\n");
            content
                .push_str("        debug_assert!(version <= Self::get_max_supported_version());\n");
            content.push_str("        self.validate_fields(version)?;\n");
            content.push_str("        header.serialize(0, bytes)?;\n");
            for field in &spec.fields {
                content.push_str(&serialize_field(field));
            }
            content.push_str("    Ok(())\n");
            content.push_str("    }\n\n");
            content.push_str("}\n\n");

            content.push_str(&generate_validate_fields(&spec.name, &spec.fields));
            content.push_str(&impl_default_trait(&spec.name, &spec.fields));

            for (name, fields) in sub_structs {
                content.push_str(&format!("impl ToBytes for {name}{{\n"));
                content.push_str("    fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(),SerializationError> {\n");
                content.push_str("        self.validate_fields(version)?;\n");
                for field in &fields {
                    content.push_str(&serialize_field(field));
                }
                content.push_str("    Ok(())\n");
                content.push_str("    }\n\n");
                content.push_str("}\n\n");
                content.push_str(&generate_validate_fields(&name, &fields));
                content.push_str(&impl_default_trait(&name, &fields));
            }
        }
        ApiSpecType::Response => {
            content.push_str(&format!("impl ApiResponse for {}{{\n", spec.name));
            content.push_str(
                "    fn deserialize(version: i16, bytes: &mut Bytes) -> (ResponseHeader, Self) {\n",
            );
            content.push_str("        let header = ResponseHeader::deserialize(0, bytes);\n");
            for field in &spec.fields {
                content.push_str(&deserialize_field(field));
            }
            content.push_str(&format!("        (header, {} {{\n", spec.name));
            for field in &spec.fields {
                content.push_str(&format!(
                    "            {},\n",
                    field.name.to_case(Case::Snake)
                ));
            }
            content.push_str("        })\n\n");
            content.push_str("    }\n\n");
            content.push_str("}\n\n");
            content.push_str(&impl_default_trait(&spec.name, &spec.fields));

            for (name, fields) in sub_structs {
                content.push_str(&format!("impl  FromBytes for {name}{{\n"));
                content.push_str("    fn deserialize(version: i16, bytes: &mut Bytes) -> Self {\n");
                for field in &fields {
                    content.push_str(&deserialize_field(field));
                }
                content.push_str(&format!("        {} {{\n", name));
                for field in &fields {
                    content.push_str(&format!(
                        "            {},\n",
                        field.name.to_case(Case::Snake)
                    ));
                }
                content.push_str("        }\n\n");
                content.push_str("    }\n\n");
                content.push_str("}\n\n");
                content.push_str(&impl_default_trait(&name, &fields));
            }
        }
        ApiSpecType::Header => {
            content.push_str(&format!("impl {}{{\n", spec.name));
            match spec.name.as_str() {
                "RequestHeader" => {
                    content.push_str(&get_min_max_supported_version(&spec));
                    content.push_str(
                        "    pub fn serialize(&self, version: i16, bytes: &mut BytesMut) -> Result<(),SerializationError> {\n",
                    );
                    content.push_str(
                        "        debug_assert!(version >= Self::get_min_supported_version());\n",
                    );
                    content.push_str(
                        "        debug_assert!(version <= Self::get_max_supported_version());\n",
                    );
                    content.push_str("        self.validate_fields(version)?;\n");
                    for field in &spec.fields {
                        content.push_str(&serialize_field(field));
                    }
                    content.push_str("    Ok(())\n");
                    content.push_str("    }\n\n");
                    content.push_str("}\n\n");
                    content.push_str(&generate_validate_fields(&spec.name, &spec.fields));
                }
                "ResponseHeader" => {
                    content.push_str(
                        "    pub fn deserialize(version: i16, bytes: &mut Bytes) -> ResponseHeader {\n",
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
                }
                _ => panic!("Unknown header type"),
            }
            content.push_str(&impl_default_trait(&spec.name, &spec.fields));
        }
    }

    (spec.name.to_case(Case::Snake), content)
}

fn generate_validate_fields(struct_name: &str, fields: &Vec<ApiSpecField>) -> String {
    let mut content = "".to_owned();

    content.push_str(&format!("impl {}{{\n", struct_name));
    content.push_str(
        "    fn validate_fields(&self, _version: i16) -> Result<(),SerializationError>{\n",
    );
    for field in fields.iter().filter(|x| x.nullable_versions.is_some()) {
        let nullable_versions = field.nullable_versions.clone().unwrap();

        if field.versions.contains("-") {
            let mut split = nullable_versions.split("-");
            let min = split.next().unwrap().to_owned();
            let max = split.next().unwrap().to_owned();
            content.push_str(&format!(
                "    if self.{}.is_none() && !({min}..={max}).contains(_version){{\n",
                field.name.to_case(Case::Snake)
            ));
        } else if field.versions == "0+" {
            content.push_str(&format!(
                "        if self.{}.is_none() {{\n",
                field.name.to_case(Case::Snake)
            ));
        } else {
            let min = field.versions.replace("+", "");
            content.push_str(&format!(
                "        if self.{}.is_none() && !_version >= {min} {{\n",
                field.name.to_case(Case::Snake)
            ));
        };
        content.push_str(&format!(
            "        return Err(SerializationError::NullValue(\"{}\", _version, \"{}\"))\n",
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

    content.push_str("    fn get_api_key() -> i16 {\n");
    content.push_str(&format!("        {}\n", spec.api_key.unwrap()));
    content.push_str("    }\n\n");

    content
}

fn get_min_max_supported_version(spec: &ApiSpec) -> String {
    let mut content = "".to_owned();

    let min_supported_version = spec.valid_versions.split("-").next().unwrap();
    let max_supported_version = spec.valid_versions.split("-").last().unwrap();
    content.push_str("    fn get_min_supported_version() -> i16 {\n");
    content.push_str(&format!("        {}\n", min_supported_version));
    content.push_str("    }\n\n");
    content.push_str("    fn get_max_supported_version() -> i16 {\n");
    content.push_str(&format!("        {}\n", max_supported_version));
    content.push_str("    }\n\n");
    content
}

fn should_derive_default(fields: &Vec<ApiSpecField>) -> bool {
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
        "self.{}.serialize(version, bytes)?;",
        field.name.to_case(Case::Snake)
    );

    if field.versions.contains("-") {
        let mut split = field.versions.split("-");
        let min = split.next().unwrap().to_owned();
        let max = split.next().unwrap().to_owned();
        content.push_str(&format!(
            "        if ({min}..={max}).contains(&version) {{\n"
        ));
        content.push_str(&format!("            {serialize}\n"));
        content.push_str("        }\n");
    } else if field.versions == "0+" {
        content.push_str(&format!("        {serialize}\n"));
    } else {
        let min = field.versions.replace("+", "");
        content.push_str(&format!("        if version >= {min} {{\n"));
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

    if field.versions.contains("-") {
        let mut split = field.versions.split("-");
        let min = split.next().unwrap().to_owned();
        let max = split.next().unwrap().to_owned();
        content.push_str(&format!(
            "        let {} = if ({min}..={max}).contains(&version) {{\n",
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
        let min = field.versions.replace("+", "");
        content.push_str(&format!(
            "        let {} = if version >= {min} {{\n",
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
    field_type.replace("<", "::<")
}

fn get_field_definition(
    field: &ApiSpecField,
    sub_structs: &mut VecDeque<(String, Vec<ApiSpecField>)>,
) -> String {
    let mut content = "".to_owned();
    let field_type = get_field_type(field);
    if !field.fields.is_empty() {
        sub_structs.push_back((
            get_field_base_type(&field.type_.type_),
            field.fields.clone(),
        ));
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
    let ApiSpecFieldType { type_, is_array } = &field.type_;
    let mut field_type = get_field_base_type(type_);
    if *is_array {
        field_type = format!("Vec<{field_type}>");
    }
    if field.nullable_versions.is_some() {
        field_type = format!("Option<{field_type}>");
    }
    field_type
}

fn get_field_base_type(type_: &ApiSpecFieldSubtype) -> String {
    match type_ {
        ApiSpecFieldSubtype::Bool => "bool".to_owned(),
        ApiSpecFieldSubtype::Bytes => "Vec<u8>".to_owned(),
        ApiSpecFieldSubtype::Int16 => "i16".to_owned(),
        ApiSpecFieldSubtype::Int8 => "i8".to_owned(),
        ApiSpecFieldSubtype::Int32 => "i32".to_owned(),
        ApiSpecFieldSubtype::Int64 => "i64".to_owned(),
        ApiSpecFieldSubtype::String => "String".to_owned(),
        ApiSpecFieldSubtype::SubObject(sub) => sub.to_owned(),
    }
}
