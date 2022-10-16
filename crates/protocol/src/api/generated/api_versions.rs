use super::super::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct ApiVersionsRequest<const V: u8> {
    client_software_name: String,
    client_software_version: String,
    tag_buffer: TagBuffer,
}

impl<const V: u8> ApiVersionsRequest<V> {
    pub fn with_client_software_name(&mut self, client_software_name: String) {
        debug_assert!(V < 3, "Field not supported in this version of request");
        self.client_software_name = client_software_name;
    }

    pub fn with_client_software_version(&mut self, client_software_version: String) {
        debug_assert!(V < 3, "Field not supported in this version of request");
        self.client_software_version = client_software_version;
    }

    pub fn with_tag_buffer(&mut self, tag_buffer: TagBuffer) {
        debug_assert!(V < 3, "Field not supported in this version of request");
        self.tag_buffer = tag_buffer;
    }
}

impl<const V: u8> ApiRequest for ApiVersionsRequest<V> {
    type Response = ApiVersionsResponse<V>;

    fn get_max_supported_version() -> u16 {
        3
    }

    fn get_api_key() -> ApiNumbers {
        ApiNumbers::ApiVersions
    }

    fn serialize(&self, version: u16, bytes: &mut BytesMut, correlation_id: i32, client_id: &str) {
        let is_flexible = 3 >= version;
        match is_flexible {
            true => {
                HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                    .serialize(bytes, false, 2);
            }
            false => {
                HeaderRequest::new(Self::get_api_key(), version, correlation_id, client_id)
                    .serialize(bytes, false, 1);
            }
        };
        if version >= 3 {
            self.client_software_name
                .serialize(bytes, is_flexible, version);
        }
        if version >= 3 {
            self.client_software_version
                .serialize(bytes, is_flexible, version);
        }
        if version >= 3 {
            self.tag_buffer.serialize(bytes, is_flexible, version);
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ApiVersionsResponse<const V: u8> {
    error_code: Int16,
    api_keys: Vec<ApiVersionsResponseApiKeys<V>>,
    throttle_time_ms: Int32,
    tag_buffer: TagBuffer,
}

#[derive(Debug, Default, Clone)]
pub struct ApiVersionsResponseApiKeys<const V: u8> {
    api_key: Int16,
    min_version: Int16,
    max_version: Int16,
    tag_buffer: TagBuffer,
}

impl<const V: u8> ApiVersionsResponse<V> {
    pub fn error_code(&self) -> Int16 {
        self.error_code
    }

    pub fn api_keys(&self) -> &Vec<ApiVersionsResponseApiKeys<V>> {
        &self.api_keys
    }

    pub fn throttle_time_ms(&self) -> Int32 {
        debug_assert!(V < 1, "Field not supported in this version of response");
        self.throttle_time_ms
    }

    pub fn tag_buffer(&self) -> TagBuffer {
        debug_assert!(V < 3, "Field not supported in this version of response");
        self.tag_buffer
    }
}

impl<const V: u8> ApiResponse for ApiVersionsResponse<V> {
    fn deserialize(version: u16, bytes: &mut Bytes) -> (i32, Self) {
        let is_flexible = 3 >= version;
        let correlation = match is_flexible {
            true => HeaderResponse::deserialize(bytes, false, 2).correlation,
            false => HeaderResponse::deserialize(bytes, false, 1).correlation,
        };
        let error_code = Int16::deserialize(bytes, is_flexible, version);
        let api_keys =
            Vec::<ApiVersionsResponseApiKeys<V>>::deserialize(bytes, is_flexible, version);
        let throttle_time_ms = if version >= 1 {
            Int32::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        let tag_buffer = if version >= 3 {
            TagBuffer::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        (
            correlation,
            Self {
                error_code,
                api_keys,
                throttle_time_ms,
                tag_buffer,
            },
        )
    }

    fn get_general_error(&self) -> Option<ApiError> {
        match self.error_code {
            0 => None,
            error_code => Some(ApiError::from(error_code)),
        }
    }
}

impl<const V: u8> ApiVersionsResponseApiKeys<V> {
    pub fn api_key(&self) -> Int16 {
        self.api_key
    }

    pub fn min_version(&self) -> Int16 {
        self.min_version
    }

    pub fn max_version(&self) -> Int16 {
        self.max_version
    }

    pub fn tag_buffer(&self) -> TagBuffer {
        debug_assert!(V < 3, "Field not supported in this version of response");
        self.tag_buffer
    }
}

impl<const V: u8> FromBytes for ApiVersionsResponseApiKeys<V> {
    fn deserialize(bytes: &mut Bytes, is_flexible: bool, version: u16) -> Self {
        let api_key = Int16::deserialize(bytes, is_flexible, version);
        let min_version = Int16::deserialize(bytes, is_flexible, version);
        let max_version = Int16::deserialize(bytes, is_flexible, version);
        let tag_buffer = if version >= 3 {
            TagBuffer::deserialize(bytes, is_flexible, version)
        } else {
            Default::default()
        };
        Self {
            api_key,
            min_version,
            max_version,
            tag_buffer,
        }
    }
}
