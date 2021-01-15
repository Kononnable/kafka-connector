use std::vec;

use nom::{IResult, bitvec::field, branch::alt, bytes::streaming::take_while1, character::{complete::{alpha1, alphanumeric1, char, crlf, digit1, space0, space1}, is_alphabetic, is_alphanumeric, is_digit, is_space}, multi::many1, sequence::{preceded, tuple}};
use nom::{
    bytes::complete::{tag},
    character::complete::line_ending,
    combinator::not,
};

const REQ: &str = r#"
  Fetch Response (Version: 0) => [responses] 
   responses => topic [partition_responses] 
      topic => STRING
      partition_responses => partition error_code high_watermark record_set 
       partition => INT32
       error_code => INT16
       high_watermark => INT64
       record_set => RECORDS
"#;

#[derive(Debug)]
pub struct ApiCall<'a> {
  name:&'a str,
  typ: CallType,
  version: &'a str,
  fields: Vec<FieldData<'a>>
}

#[derive(Debug)]
pub enum CallType {
    Request,
    Response,
}
fn parse_call_type(input: &str) -> IResult<&str, CallType> {
    let (input, call_type) = alpha1(input)?;
    let call_type = match call_type {
        "Request" => CallType::Request,
        "Response" => CallType::Response,
        _ => panic!("TODO:Change to custom error type"),
    };
    Ok((input, call_type))
}
#[derive(Debug)]
pub enum FieldDef<'a> {
    Field(&'a str),
    Vec(&'a str),
}
fn parse_field_list(input: &str) -> IResult<&str, Vec<FieldDef>> {
    // let square_bracket_start = tag("[");
    // let square_bracket_end = tag("]");
    // let vector = tuple((square_bracket_start, alphanumeric1, square_bracket_end));
    // let field = alt((alpha1, vector));

    let field = take_while1(|c:char| c.is_alphanumeric() || c=='[' || c == ']' || c == '_');
    let mut field_list = many1(preceded(space1, field));

    let (input, fields) = field_list(input)?;
    let fields: Vec<FieldDef> = fields
        .into_iter()
        .map(|x| {
            if x.starts_with("[") {
                FieldDef::Vec(&x[1..x.len() - 1])
            } else {
                FieldDef::Field(x)
            }
        })
        .collect();
    Ok((input, fields))
}
fn parse_version(input: &str) -> IResult<&str, &str> {
  let version_tag = tag("Version:");
  let bracket_start = char('(');
  let version_number = digit1;
  let bracket_end = tag(")");
  let (input, (_, _, _, version, _)) = tuple((
      bracket_start,
      version_tag,
      space1,
      version_number,
      bracket_end,
  ))(input)?;
  Ok((input, version))
}

#[derive(Debug)]
pub enum FieldData<'a> {
    Field(&'a str, FieldType),
    Vec(&'a str, Vec<FieldData<'a>>),
}
fn parse_field<'a>(input: &'a str, is_vec:bool, field_name:&'a str) -> IResult<&'a str, FieldData<'a>> {
  let ignored = take_while1(|c:char|c!='>');
  let (mut input, (_,_,fields,_,_)) = tuple((
      ignored,
      char('>'),
      parse_field_list,
      space0,
      line_ending
  ))(input)?;
  let fields:Vec<FieldDef> = fields;
  let ret_val = if is_vec{
    let mut childrens = vec![];
    for child in fields{
      let (is_vec,field_name) = match child {
        FieldDef::Field(name)=>{
          (false,name)
        },
        FieldDef::Vec(name)=>{
          (true,name)
        }
      };
      let (input2, parsed_child) = parse_field(input, is_vec, field_name)?;
      childrens.push(parsed_child);
      input=input2;
    }
    FieldData::Vec(field_name,childrens)
  }else{
    let field_type = match fields.first().unwrap(){
      FieldDef::Field(ty)=>match_field_type(ty),
      _ => panic!()
    };
    FieldData::Field(field_name,field_type)
  };
  Ok((input, ret_val))
}
#[derive(Debug)]
pub enum FieldType{
  Int16,
  Int32,
  Int64,
  String,
  Records,
}
fn match_field_type(ty:&str)->FieldType{
  match ty {
    "INT16"=>FieldType::Int16,
    "INT32"=>FieldType::Int32,
    "INT64"=>FieldType::Int64,
    "STRING"=>FieldType::String,
    "RECORDS"=>FieldType::Records,
    _ => panic!("Unknown field type: {}",ty)
  }
}
pub fn parse_api_call(input: &str) -> IResult<&str, ApiCall> {
    let pointer = tag("=>");

    // first_row
    let (mut input, (api_name, _, api_type, _, api_version, _, _, fields,_,_)) = tuple((
        alphanumeric1,
        space1,
        parse_call_type,
        space1,
        parse_version,
        space1,
        pointer,
        parse_field_list,
        space0,
        line_ending
    ))(input)?;
    println!("{} {:?} {} {:?}", api_name, api_type, api_version, fields);
    let mut api_call = ApiCall{
      name:api_name,
      typ:api_type,
      version:api_version,
      fields: vec![]
    };

    for field in fields {
      let (is_vec,field_name) = match field {
        FieldDef::Field(name)=>{
          (false,name)
        },
        FieldDef::Vec(name)=>{
          (true,name)
        }
      };
      let (input2, parsed_child) = parse_field(input, is_vec, field_name)?;
      input=input2;
      api_call.fields.push(parsed_child);
    }
    Ok((input,api_call))
}

// pub fn length_value2(input: &[u8]) -> IResult<&[u8],&[u8]> {

// // first implement the basic parsers
// let method = take_while1(is_alpha);
// let space = take_while1(|c| c == ' ');
// let url = take_while1(|c| c!= ' ');
// let is_version = |c| c >= b'0' && c <= b'9' || c == b'.';
// let http = tag("HTTP/");
// let version = take_while1(is_version);
// let line_ending = tag("\r\n");

// // combine http and version to extract the version string
// // preceded will return the result of the second parser
// // if both succeed
// let http_version = preceded(http, version);

// // combine all previous parsers in one function
// fn request_line(i: &[u8]) -> IResult<&[u8], Request> {

//   // tuple takes as argument a tuple of parsers and will return
//   // a tuple of their results
//   let (input, (method, _, url, _, version, _)) =
//     tuple((method, space, url, space, http_version, line_ending))(i)?;

//   Ok((input, Request { method, url, version }))
// }
