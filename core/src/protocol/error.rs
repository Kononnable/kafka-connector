use thiserror::Error as DeriveError;

#[derive(Debug, DeriveError)]
pub enum Error {
    #[error("Connected kafka broker doesn't support requested API version. API name: {0} API version: {1} Api field: {2}")]
    OldKafkaVersion(&'static str, i32, &'static str),
}
