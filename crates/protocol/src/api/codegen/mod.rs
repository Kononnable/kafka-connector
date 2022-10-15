use std::{
    env, fs,
    io::Write,
    process::{Command, Stdio},
};

use convert_case::{Case, Casing};

use crate::api_numbers::ApiNumbers;

mod api_versions;

#[derive(Debug)]
struct ApiStruct {
    name: &'static str,
    key: ApiNumbers,
    min_flexible_version: u8,
    max_version: u8,
    request_fields: Vec<Field>,
    response_fields: Vec<Field>,
}

#[derive(Debug)]
struct Field {
    name: &'static str,
    type_: FieldType,
    is_array: bool,
    min_version: u8,
}

#[derive(Debug)]
enum FieldType {
    // Boolean,
    // Bytes,
    // Int8,
    Int16,
    Int32,
    // Int64,
    // Float64,
    String,
    TagBuffer,
    SubObject(Vec<Field>),
}

impl Field {
    pub fn new_tag_buffer(min_flexible_version: u8) -> Field {
        Field {
            name: "tag_buffer",
            type_: FieldType::TagBuffer,
            is_array: false,
            min_version: min_flexible_version,
        }
    }
}

impl FieldType {
    pub fn is_copyable(&self) -> bool {
        match self {
            FieldType::Int16 | FieldType::Int32 | FieldType::TagBuffer => true,
            FieldType::String | FieldType::SubObject(_) => false,
        }
    }
}
#[derive(Debug)]
enum StructType {
    Request,
    Response,
}

fn generate_code(api_call: &ApiStruct) -> String {
    let mut generated = String::new();
    generated.push_str("use super::super::prelude::*;\n\n");

    generate_struct(&mut generated, api_call, StructType::Request);

    generated.push_str(&format!(
        "impl<const V: u8> {}Request<V>{{\n",
        api_call.name
    ));
    for field in &api_call.request_fields {
        generated.push_str(&format!(
            "    pub fn with_{}(&mut self, {}:{:?}){{\n",
            field.name, field.name, field.type_
        ));
        if field.min_version > 0 {
            generated.push_str(&format!(
                "        debug_assert!(V < {}, \"Field not supported in this version of request\");\n",
                field.min_version
            ));
        }
        generated.push_str(&format!("        self.{} = {};\n", field.name, field.name));
        generated.push_str("    }\n\n");
    }
    generated.push_str("}\n\n");

    generated.push_str(&format!(
        "impl<const V: u8> ApiRequest for {}Request<V>{{\n",
        api_call.name
    ));
    generated.push_str(&format!(
        "    type Response = {}Response<V>;\n\n",
        api_call.name
    ));
    generated.push_str("    fn get_max_supported_version() -> u16 {\n");
    generated.push_str(&format!("        {}\n", api_call.max_version));
    generated.push_str("    }\n\n");
    generated.push_str("    fn get_api_key() -> ApiNumbers {\n");
    generated.push_str(&format!("        ApiNumbers::{:?}\n", api_call.key));
    generated.push_str("    }\n\n");
    generated.push_str("    fn serialize(&self, version: u16, bytes: &mut BytesMut, correlation_id: i32, client_id: &str) {\n");
    generated.push_str(&format!(
        "        let is_flexible = {} >= version;\n",
        api_call.min_flexible_version
    ));
    generated.push_str("        match is_flexible{\n");
    generated.push_str("            true => {\n");
    generated.push_str("                HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id).serialize(bytes,false,2);\n");
    generated.push_str("            },\n");
    generated.push_str("            false => {\n");
    generated.push_str("                HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id).serialize(bytes,false,1);\n");

    generated.push_str("            },\n");
    generated.push_str("        };\n");

    let mut sub_objects = vec![];
    for field in &api_call.request_fields {
        if field.min_version > 0 {
            generated.push_str(&format!("if version >= {} {{\n", field.min_version));
        }
        match &field.type_ {
            FieldType::SubObject(sub) => {
                sub_objects.push((field.name.to_case(Case::UpperCamel), sub));
                generated.push_str(&format!(
                    "self.{}.serialize(bytes,is_flexible,version);\n",
                    field.name
                ));
            }
            _ => {
                generated.push_str(&format!(
                    "self.{}.serialize(bytes,is_flexible);\n",
                    field.name
                ));
            }
        }
        if field.min_version > 0 {
            generated.push_str("}\n");
        }
    }
    generated.push_str("    }\n");
    generated.push_str("}\n\n");
    // TODO: Subobjects

    generate_struct(&mut generated, api_call, StructType::Response);

    generated.push_str(&format!(
        "impl<const V: u8> {}Response<V>{{\n",
        api_call.name
    ));
    for field in &api_call.response_fields {
        let mut field_type = match &field.type_ {
            FieldType::SubObject(_sub) => {
                format!(
                    "{}Response{}<V>",
                    api_call.name,
                    field.name.to_case(Case::UpperCamel)
                )
            }
            _ => {
                format!("{:?}", &field.type_)
            }
        };
        let mut ref_char = if field.type_.is_copyable() { "" } else { "&" };
        if field.is_array {
            field_type = format!("Vec<{}>", field_type);
            ref_char = "&";
        }
        generated.push_str(&format!(
            "    pub fn {}(&self)-> {}{}{{\n",
            field.name, ref_char, field_type
        ));
        if field.min_version > 0 {
            generated.push_str(&format!(
                "        debug_assert!(V < {}, \"Field not supported in this version of response\");\n",
                field.min_version
            ));
        }
        generated.push_str(&format!("        {}self.{}\n", ref_char, field.name));
        generated.push_str("    }\n\n");
    }
    generated.push_str("}\n\n");

    generated.push_str(&format!(
        "impl<const V: u8> ApiResponse for {}Response<V>{{\n",
        api_call.name
    ));
    generated.push_str("    fn deserialize(version: u16, bytes: &mut Bytes) -> (i32, Self) {\n");
    generated.push_str(&format!(
        "        let is_flexible = {} >= version;\n",
        api_call.min_flexible_version
    ));
    generated.push_str("        let correlation = match is_flexible {\n");
    generated.push_str(
        "            true=> HeaderResponse::deserialize(bytes, false, 2).correlation, \n",
    );
    generated.push_str(
        "            false=> HeaderResponse::deserialize(bytes, false, 1).correlation, \n",
    );
    generated.push_str("        };\n");

    let mut sub_objects = vec![];

    for field in &api_call.response_fields {
        generated.push_str(&format!("let {} = ", field.name));
        if field.min_version > 0 {
            generated.push_str(&format!("if version >= {} {{\n", field.min_version));
        }
        match &field.type_ {
            FieldType::SubObject(sub) => {
                sub_objects.push((field.name.to_case(Case::UpperCamel), sub));
                generated.push_str(&format!(
                    "Vec::<{}Response{}<V>>::deserialize(bytes,is_flexible,version)\n",
                    api_call.name,
                    field.name.to_case(Case::UpperCamel)
                ));
            }
            _ => {
                generated.push_str(&format!(
                    "{:?}::deserialize(bytes,is_flexible,version)\n",
                    field.type_
                ));
            }
        }
        if field.min_version > 0 {
            generated.push_str("} else { Default::default() }");
        }
        generated.push_str(";\n");
    }
    generated.push_str("(correlation,Self{\n");
    for field in &api_call.response_fields {
        generated.push_str(&format!("{},\n", field.name));
    }
    generated.push_str("})\n");

    generated.push_str("    }\n\n");
    generated.push_str("    fn get_general_error(&self) -> Option<ApiError> {\n");
    generated.push_str("        match self.error_code {\n");
    generated.push_str("            0 => None,\n");
    generated.push_str("            error_code => Some(ApiError::from(error_code)),\n");
    generated.push_str("        }\n");
    generated.push_str("    }\n");
    generated.push_str("}\n\n");

    while let Some((name, fields)) = sub_objects.pop() {
        generated.push_str(&format!(
            "impl<const V: u8> {}Response{}<V>{{\n",
            api_call.name, name
        ));
        for field in fields {
            let mut field_type = match &field.type_ {
                FieldType::SubObject(_sub) => {
                    format!(
                        "{}Response{}<V>",
                        api_call.name,
                        field.name.to_case(Case::UpperCamel)
                    )
                }
                _ => {
                    format!("{:?}", &field.type_)
                }
            };
            let mut ref_char = if field.type_.is_copyable() { "" } else { "&" };
            if field.is_array {
                field_type = format!("Vec<{}>", field_type);
                ref_char = "&";
            }
            generated.push_str(&format!(
                "    pub fn {}(&self)-> {}{}{{\n",
                field.name, ref_char, field_type
            ));
            if field.min_version > 0 {
                generated.push_str(&format!(
                    "        debug_assert!(V < {}, \"Field not supported in this version of response\");\n",
                    field.min_version
                ));
            }
            generated.push_str(&format!("        {}self.{}\n", ref_char, field.name));
            generated.push_str("    }\n\n");
        }
        generated.push_str("}\n\n");

        generated.push_str(&format!(
            "impl<const V: u8> FromBytes for {}Response{}<V> {{\n",
            api_call.name, name
        ));
        generated.push_str(
            "    fn deserialize(bytes: &mut Bytes, is_flexible: bool, version: u16 ) -> Self {\n",
        );
        for field in fields {
            generated.push_str(&format!("let {} = ", field.name));
            if field.min_version > 0 {
                generated.push_str(&format!("if version >= {} {{\n", field.min_version));
            }
            match &field.type_ {
                FieldType::SubObject(sub) => {
                    sub_objects.push((field.name.to_case(Case::UpperCamel), sub));
                    generated.push_str(&format!(
                        "Vec::<{}Response{}<V>>::deserialize(bytes,is_flexible,version)\n",
                        api_call.name,
                        field.name.to_case(Case::UpperCamel)
                    ));
                }
                _ => {
                    generated.push_str(&format!(
                        "{:?}::deserialize(bytes,is_flexible, version)\n",
                        field.type_
                    ));
                }
            }
            if field.min_version > 0 {
                generated.push_str("} else { Default::default() }");
            }
            generated.push_str(";\n");
        }
        generated.push_str("Self{\n");
        for field in fields {
            generated.push_str(&format!("{},\n", field.name));
        }
        generated.push_str("}\n");
        generated.push_str("    }\n");
        generated.push_str("}\n\n");
    }
    // TODO: builder pattern for request?
    generated
}

fn generate_struct(generated: &mut String, api_call: &ApiStruct, struct_type: StructType) {
    generated.push_str("#[derive(Debug, Default, Clone)]\n");
    generated.push_str(&format!(
        "pub struct {}{:?}<const V: u8> {{\n",
        api_call.name, struct_type
    ));

    let fields = match struct_type {
        StructType::Request => &api_call.request_fields,
        StructType::Response => &api_call.response_fields,
    };
    let mut sub_objects = vec![];

    for field in fields {
        let mut field_type = if let FieldType::SubObject(sub) = &field.type_ {
            let field_name = field.name.to_case(Case::UpperCamel);
            sub_objects.push((field_name.clone(), sub));
            format!("{}{:?}{}<V>", api_call.name, struct_type, field_name)
        } else {
            format!("{:?}", field.type_)
        };
        if field.is_array {
            field_type = format!("Vec<{}>", field_type);
        }
        generated.push_str(&format!("    {}: {},\n", field.name, field_type));
    }
    generated.push_str("}\n\n");

    while let Some((name, fields)) = sub_objects.pop() {
        generated.push_str("#[derive(Debug, Default, Clone)]\n");
        generated.push_str(&format!(
            "pub struct {}{:?}{}<const V: u8> {{\n",
            api_call.name, struct_type, name
        ));
        for field in fields {
            let mut field_type = if let FieldType::SubObject(sub) = &field.type_ {
                let field_name = field.name.to_case(Case::UpperCamel);
                sub_objects.push((field_name.clone(), sub));
                format!("{}{:?}{}", api_call.name, struct_type, field_name)
            } else {
                format!("{:?}", field.type_)
            };
            if field.is_array {
                field_type = format!("Vec<{}>", field_type);
            }
            generated.push_str(&format!("    {}: {},\n", field.name, field_type));
        }
        generated.push_str("}\n\n");
    }
}

fn generate_mod_file(api_calls: Vec<ApiStruct>) -> String {
    let mut generated = String::new();
    for api_call in &api_calls {
        generated.push_str(&format!(
            "pub mod {};\n",
            api_call.name.to_case(Case::Snake)
        ));
    }

    generated
}

fn format_code(generated: String) -> String {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(generated.as_bytes()).unwrap();
    drop(stdin);
    let output = child.wait_with_output().expect("Failed to read stdout");
    let formated = String::from_utf8(output.stdout).unwrap();
    assert!(!formated.is_empty());
    formated
}

#[test]
fn api_codegen() {
    const GENERATE_FILES: bool = true;
    const FILE_PATH_PREFIX: &str = "/src/api/generated/";

    let api_calls = vec![api_versions::get_api_call()];

    for api_struct in &api_calls {
        let generated = generate_code(api_struct);
        let generated = format_code(generated);

        let file_name = api_struct.name.to_case(convert_case::Case::Snake);

        let file_name = format!(
            "{}{}{}.rs",
            env::current_dir().unwrap().to_str().unwrap(),
            FILE_PATH_PREFIX,
            file_name
        );

        if GENERATE_FILES {
            fs::write(file_name, generated).unwrap();
        } else {
            let actual = fs::read_to_string(&file_name).unwrap();
            assert_eq!(
                generated, actual,
                "Difference in generated file found {}",
                file_name
            );
        }
    }

    let generated_mod = generate_mod_file(api_calls);
    let generated_mod = format_code(generated_mod);
    let mod_file_path = format!(
        "{}{}mod.rs",
        env::current_dir().unwrap().to_str().unwrap(),
        FILE_PATH_PREFIX
    );

    if GENERATE_FILES {
        fs::write(mod_file_path, generated_mod).unwrap();
    } else {
        let actual_mod_file = fs::read_to_string(&mod_file_path).unwrap();
        assert_eq!(
            generated_mod, actual_mod_file,
            "Difference in generated file found mod.rs",
        );
    }
}
