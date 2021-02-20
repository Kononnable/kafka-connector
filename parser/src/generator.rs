use std::iter;

use crate::transformer_step2::{ApiCall2, ApiField2, File};

pub fn generate_content(file: File) -> String {
    let mut ret_val = "use super::prelude::*;\n".to_owned();
    ret_val.push_str(&format!(
        "pub type {}Request = {}Request{};\n",
        file.file_name,
        file.file_name,
        file.api_calls.last().unwrap().api_struct_version
    ));

    for api_call in file.api_calls {
        let api_struct = generate_api_struct(&api_call);
        let structs = generate_struct(&api_call);
        let get_first_error = generate_get_first_error(&api_call);
        ret_val.push_str(&format!("{}\n{}\n{}", api_struct, structs, get_first_error));
    }
    ret_val
}

fn generate_api_struct(api_call: &ApiCall2) -> String {
    let mut def = format!(
        "impl ApiCall for {}Request{} {{\n",
        api_call.api_name, api_call.api_struct_version
    );
    def.push_str(&format!(
        "type Response = {}Response{};\n",
        api_call.api_name, api_call.api_struct_version
    ));
    def.push_str(&format!(
        "fn get_min_supported_version()->u16{{\n{}\n}}\n",
        api_call.min_supported_version
    ));
    def.push_str(&format!(
        "fn get_max_supported_version()->u16{{\n{}\n}}\n",
        api_call.max_supported_version
    ));
    def.push_str(&format!(
        "fn get_api_key()->ApiNumbers{{\nApiNumbers::{}\n}}\n",
        api_call.api_name
    ));
    def.push_str("fn get_first_error(response: &Self::Response) -> Option<ApiError>{{\nSelf::Response::get_first_error(response)\n}}\n");
    if api_call.min_flexible_versions.is_none() {
        def.push_str("fn is_flexible_version(_version: u16) -> bool {\n false \n}");
    } else if api_call.min_supported_version == api_call.min_flexible_versions.unwrap() {
        def.push_str("fn is_flexible_version(_version: u16) -> bool {\n true \n}");
    } else {
        def.push_str(&format!(
            "fn is_flexible_version(version: u16) -> bool {{\n version >= {} }}",
            api_call.min_flexible_versions.unwrap()
        ));
    }

    let request_version_call = serialize_api_request();
    let response_version_call = deserialize_api_response();
    def.push_str(&format!(
        "{}{}}}",
        request_version_call, response_version_call
    ));
    def
}

fn serialize_api_request() -> String {
    let mut fn_def =
        "fn serialize(&self,version:u16, buf: &mut BytesMut,correlation_id: i32,client_id: &str){\n"
            .to_owned();

    fn_def.push_str("match Self::is_flexible_version(version) {\n");
    fn_def.push_str(&"true => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id).serialize(buf, false,2),\n");
    fn_def.push_str(&"false => HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id).serialize(buf, false,1),\n");
    fn_def.push_str("}\n");
    fn_def.push_str(
        "        ToBytes::serialize(self,buf,Self::is_flexible_version(version),version);\n",
    );

    fn_def.push_str("}\n");
    fn_def
}

fn deserialize_api_response() -> String {
    let mut fn_def =
        "fn deserialize_response(version:u16, buf: &mut Bytes) -> (i32,Self::Response) {\n"
            .to_owned();

    fn_def.push_str("let correlation = match Self::is_flexible_version(version) {\n");
    fn_def.push_str("true => HeaderResponse::deserialize(buf, false,2).correlation,\n");
    fn_def.push_str("false => HeaderResponse::deserialize(buf, false,1).correlation,\n");
    fn_def.push_str("};\n");

    fn_def.push_str("        let response = Self::Response::deserialize(buf, Self::is_flexible_version(version),version);\n");

    fn_def.push_str("(correlation, response)\n");
    fn_def.push_str("}\n");
    fn_def
}

fn generate_struct(api_call: &ApiCall2) -> String {
    let mut ret_val = "".to_owned();
    for (struc, is_request) in api_call
        .req_structs
        .iter()
        .zip(iter::repeat(true))
        .chain(api_call.resp_structs.iter().zip(iter::repeat(false)))
    {
        let derive_bytes = if is_request { "ToBytes" } else { "FromBytes" };
        let derive = format!("#[derive(Default,Debug,Clone,{})]\n", derive_bytes);
        let struct_name_with_version = format!(
            "pub struct {}{} {{ \n",
            struc.name, api_call.api_struct_version
        );
        let fields = struc
            .fields
            .iter()
            .map(generate_field)
            .fold("".to_string(), |acc, x| format!("{}{}", acc, x));
        ret_val.push_str(&format!(
            "{}{}{}{}",
            derive, struct_name_with_version, fields, "}\n"
        ));
    }
    ret_val
}

fn generate_field(field: &ApiField2) -> String {
    format!(
        "    #[min_version = {}]\n    pub {}: {},\n",
        field.min_version, field.name, field.ty
    )
}

fn generate_get_first_error(api_call: &ApiCall2) -> String {
    let mut impl_def = "".to_owned();
    for structs in api_call.resp_structs.iter() {
        impl_def.push_str(&format!(
            "impl {}{} {{\nfn get_first_error(&self) -> Option<ApiError>{{\n",
            structs.name, api_call.api_struct_version
        ));
        if structs
            .fields
            .iter()
            .any(|x| x.name == "error_code" && x.ty == "i16")
        {
            impl_def.push_str(&"    if self.error_code !=0 {\n");
            impl_def.push_str(&"        return self.error_code;\n");
            impl_def.push_str(&"    }\n");
        } else if structs
            .fields
            .iter()
            .any(|x| x.name == "error_code" && x.ty == "Option<i16>")
        {
            impl_def
                .push_str(&"    if self.error_code.is_some() && self.error_code.unwrap() !=0 {\n");
            impl_def.push_str(&"        return self.error_code.unwrap();\n");
            impl_def.push_str(&"    }\n");
        }
        for field in structs
            .fields
            .iter()
            .filter(|x| x.is_vec && x.is_struct_field)
        {
            if field.is_optional {
                impl_def.push_str(&format!(
                    "if let Some(vec) = self.{}.as_ref(){{\nfor item in vec {{\n",
                    field.name
                ));
            } else {
                impl_def.push_str(&format!("for item in self.{}.iter() {{\n", field.name));
            }
            impl_def.push_str(
                &("        if let Some(x) = item.get_first_error(){\nreturn Some(x);\n};\n"),
            );
            if field.is_optional {
                impl_def.push_str(&"    }\n}\n");
            } else {
                impl_def.push_str(&"    }\n");
            }
        }
        impl_def.push_str(&"None\n}\n}");
    }
    impl_def
}
