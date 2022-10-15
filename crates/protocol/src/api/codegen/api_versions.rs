use super::{ApiStruct, Field, FieldType};
use crate::api_numbers::ApiNumbers;

pub(super) fn get_api_call() -> ApiStruct {
    let min_flexible_version = 3;

    let api_keys = vec![
        Field {
            name: "api_key",
            type_: FieldType::Int16,
            is_array: false,
            min_version: 0,
        },
        Field {
            name: "min_version",
            type_: FieldType::Int16,
            is_array: false,
            min_version: 0,
        },
        Field {
            name: "max_version",
            type_: FieldType::Int16,
            is_array: false,
            min_version: 0,
        },
        Field::new_tag_buffer(min_flexible_version),
    ];

    ApiStruct {
        name: "ApiVersions",
        key: ApiNumbers::ApiVersions,
        min_flexible_version,
        max_version: 3,
        request_fields: vec![
            Field {
                name: "client_software_name",
                type_: FieldType::String,
                is_array: false,
                min_version: 3,
            },
            Field {
                name: "client_software_version",
                type_: FieldType::String,
                is_array: false,
                min_version: 3,
            },
            Field::new_tag_buffer(min_flexible_version),
        ],
        response_fields: vec![
            Field {
                name: "error_code",
                type_: FieldType::Int16,
                is_array: false,
                min_version: 0,
            },
            Field {
                name: "api_keys",
                type_: FieldType::SubObject(api_keys),
                is_array: true,
                min_version: 0,
            },
            Field {
                name: "throttle_time_ms",
                type_: FieldType::Int32,
                is_array: false,
                min_version: 1,
            },
            Field::new_tag_buffer(min_flexible_version),
        ],
    }
}
