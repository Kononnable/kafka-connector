use super::prelude::*;
use crate::api_numbers::ApiNumbers;

pub(super) fn get_api_call() -> ApiStruct {
    let min_flexible_version = 9;

    let topics_request = [
        Field {
            name: "name",
            type_: FieldType::String,
            is_array: false,
            min_version: 0,
        },
        Field::new_tag_buffer(min_flexible_version),
    ]
    .into();

    let brokers_response = [
        Field {
            name: "node_id",
            type_: FieldType::Int32,
            is_array: false,
            min_version: 0,
        },
        Field {
            name: "host",
            type_: FieldType::String,
            is_array: false,
            min_version: 0,
        },
        Field {
            name: "port",
            type_: FieldType::Int32,
            is_array: false,
            min_version: 0,
        },
        Field {
            name: "rack",
            type_: FieldType::NullableString,
            is_array: false,
            min_version: 1,
        },
        Field::new_tag_buffer(min_flexible_version),
    ]
    .into();

    let topics_partitions_response = [
        Field {
            name: "error_code",
            type_: FieldType::Int16,
            is_array: false,
            min_version: 0,
        },
        Field {
            name: "partition_index",
            type_: FieldType::Int32,
            is_array: false,
            min_version: 0,
        },
        Field {
            name: "leader_id",
            type_: FieldType::Int32,
            is_array: false,
            min_version: 0,
        },
        Field {
            name: "leader_epoch",
            type_: FieldType::Int32,
            is_array: false,
            min_version: 7,
        },
        Field {
            name: "replica_nodes",
            type_: FieldType::Int32,
            is_array: true,
            min_version: 0,
        },
        Field {
            name: "isr_nodes",
            type_: FieldType::Int32,
            is_array: true,
            min_version: 0,
        },
        Field {
            name: "offline_replicas",
            type_: FieldType::Int32,
            is_array: true,
            min_version: 5,
        },
        Field::new_tag_buffer(min_flexible_version),
    ]
    .into();

    let topics_response = [
        Field {
            name: "error_code",
            type_: FieldType::Int16,
            is_array: false,
            min_version: 0,
        },
        Field {
            name: "name",
            type_: FieldType::String,
            is_array: false,
            min_version: 0,
        },
        Field {
            name: "is_internal",
            type_: FieldType::Boolean,
            is_array: false,
            min_version: 1,
        },
        Field {
            name: "partitions",
            type_: FieldType::SubObject(topics_partitions_response),
            is_array: true,
            min_version: 0,
        },
        Field {
            name: "topic_authorized_operations",
            type_: FieldType::Int32,
            is_array: false,
            min_version: 8,
        },
        Field::new_tag_buffer(min_flexible_version),
    ]
    .into();

    ApiStruct {
        name: "Metadata",
        key: ApiNumbers::Metadata,
        min_flexible_version: 9,
        max_version: 12,
        request_fields: vec![
            Field {
                name: "topics",
                type_: FieldType::SubObject(topics_request),
                is_array: false,
                min_version: 0,
            },
            Field {
                name: "allow_auto_topic_creation",
                type_: FieldType::Boolean,
                is_array: false,
                min_version: 4,
            },
            Field {
                name: "include_cluster_authorized_operations",
                type_: FieldType::Boolean,
                is_array: false,
                min_version: 8,
            },
            Field {
                name: "include_topic_authorized_operations",
                type_: FieldType::Boolean,
                is_array: false,
                min_version: 8,
            },
            Field::new_tag_buffer(min_flexible_version),
        ],
        response_fields: [
            Field {
                name: "throttle_time_ms",
                type_: FieldType::Int32,
                is_array: false,
                min_version: 3,
            },
            Field {
                name: "brokers",
                type_: FieldType::SubObject(brokers_response),
                is_array: true,
                min_version: 0,
            },
            Field {
                name: "cluster_id",
                type_: FieldType::NullableString,
                is_array: false,
                min_version: 2,
            },
            Field {
                name: "controller_id",
                type_: FieldType::Int32,
                is_array: false,
                min_version: 1,
            },
            Field {
                name: "topics",
                type_: FieldType::SubObject(topics_response),
                is_array: true,
                min_version: 0,
            },
            Field {
                name: "cluster_authorized_operations",
                type_: FieldType::Int32,
                is_array: false,
                min_version: 8,
            },
            Field::new_tag_buffer(min_flexible_version),
        ]
        .into(),
    }
}
