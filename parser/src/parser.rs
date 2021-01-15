use std::vec;

use nom::{bytes::complete::tag, character::complete::line_ending};
use nom::{
    bytes::streaming::take_while1,
    character::complete::{alpha1, alphanumeric1, char, digit1, space0, space1},
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::model::{ApiCall, CallType, FieldData, FieldType};

#[derive(Debug)]
pub enum FieldDef<'a> {
    Field(&'a str),
    Vec(&'a str),
}

pub fn parse_api_call(input: &str) -> IResult<&str, ApiCall> {
    let (mut input, (mut api_call, fields)) = parse_first_row(input)?;
    for field in fields {
        let (is_vec, field_name) = match field {
            FieldDef::Field(name) => (false, name),
            FieldDef::Vec(name) => (true, name),
        };
        let (input2, parsed_child) = parse_field(input, is_vec, field_name)?;
        input = input2;
        api_call.fields.push(parsed_child);
    }
    Ok((input, api_call))
}

fn parse_first_row(input: &str) -> IResult<&str, (ApiCall, Vec<FieldDef>)> {
    let pointer = tag("=>");

    let (input, (api_name, _, api_type, _, api_version, _, _, fields, _, _)) = tuple((
        alphanumeric1,
        space1,
        parse_call_type,
        space1,
        parse_version,
        space1,
        pointer,
        parse_field_list,
        space0,
        line_ending,
    ))(input)?;
    Ok((
        input,
        (
            ApiCall {
                name: api_name,
                typ: api_type,
                version: api_version,
                fields: vec![],
            },
            fields,
        ),
    ))
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

fn parse_field_list(input: &str) -> IResult<&str, Vec<FieldDef>> {
    let field = take_while1(|c: char| c.is_alphanumeric() || c == '[' || c == ']' || c == '_');
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

fn parse_field<'a>(
    input: &'a str,
    is_vec: bool,
    field_name: &'a str,
) -> IResult<&'a str, FieldData<'a>> {
    let ignored = take_while1(|c: char| c != '>');
    let (mut input, (_, _, fields, _, _)) =
        tuple((ignored, char('>'), parse_field_list, space0, line_ending))(input)?;
    let fields: Vec<FieldDef> = fields;
    let ret_val = if is_vec {
        let mut childrens = vec![];
        for child in fields {
            let (is_vec, field_name) = match child {
                FieldDef::Field(name) => (false, name),
                FieldDef::Vec(name) => (true, name),
            };
            let (input2, parsed_child) = parse_field(input, is_vec, field_name)?;
            childrens.push(parsed_child);
            input = input2;
        }
        FieldData::Vec(field_name, childrens)
    } else {
        let field_type = match fields.first().unwrap() {
            FieldDef::Field(ty) => FieldType::from_str(ty),
            _ => panic!(),
        };
        FieldData::Field(field_name, field_type)
    };
    Ok((input, ret_val))
}
