use std::{str::FromStr, vec};

use anyhow::Result;
use nom::{bytes::complete::tag, character::complete::line_ending, multi::many0};
use nom::{
    bytes::streaming::take_while1,
    character::complete::{alpha1, alphanumeric1, char, digit1, space0, space1},
    sequence::{preceded, tuple},
    IResult,
};

use crate::model::{ApiCall, CallType, FieldData, FieldType, FieldTypeWithPayload};

#[derive(Debug, PartialEq)]
pub enum FieldTy {
    Field,
    Vec,
}

#[derive(Debug)]
pub struct FieldDef<'a> {
    ty: FieldTy,
    name: &'a str,
}

pub fn parse_api_call(input: &str) -> IResult<&str, ApiCall> {
    let (mut input, (mut api_call, fields)) = parse_first_row(input)?;
    for field in fields {
        let field_name = field.name;
        let is_vec = field.ty == FieldTy::Vec;
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
                ty: api_type,
                version: api_version,
                fields: vec![],
            },
            fields,
        ),
    ))
}

fn parse_call_type(input: &str) -> IResult<&str, CallType> {
    let (input, call_type) = alpha1(input)?;
    let call_type = CallType::from_str(call_type).unwrap();
    Ok((input, call_type))
}

fn parse_version(input: &str) -> IResult<&str, i32> {
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
    Ok((input, version.parse::<i32>().unwrap()))
}

fn parse_field_list(input: &str) -> IResult<&str, Vec<FieldDef>> {
    let field = take_while1(|c: char| c.is_alphanumeric() || c == '[' || c == ']' || c == '_');
    let mut field_list = many0(preceded(space1, field));

    let (input, fields) = field_list(input)?;
    let fields: Vec<FieldDef> = fields
        .into_iter()
        .map(|field| {
            if field.starts_with('[') {
                FieldDef {
                    ty: FieldTy::Vec,
                    name: &field[1..field.len() - 1],
                }
            } else {
                FieldDef {
                    ty: FieldTy::Field,
                    name: field,
                }
            }
        })
        .filter(|field| field.name != "TAG_BUFFER")
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

    let type_with_payload = match is_vec {
        true => {
            if fields.len() == 1 && FieldType::is_common_type(fields.first().unwrap().name) {
                let ty = FieldType::from_str(fields.first().unwrap().name).unwrap();
                FieldTypeWithPayload::VecSimple(ty)
            } else {
                let childrens = fields
                    .iter()
                    .map(|child| {
                        let field_name = child.name;
                        let is_vec = child.ty == FieldTy::Vec;

                        let (input2, parsed_child) = parse_field(input, is_vec, field_name)?;
                        input = input2;

                        Ok(parsed_child)
                    })
                    .collect::<Result<Vec<FieldData>, _>>()?;
                FieldTypeWithPayload::VecStruct(childrens)
            }
        }
        false => {
            let ty = FieldType::from_str(fields.first().unwrap().name).unwrap();
            FieldTypeWithPayload::Field(ty)
        }
    };
    Ok((
        input,
        FieldData {
            name: field_name,
            type_with_payload,
        },
    ))
}
