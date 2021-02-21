use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum MessageError {
    #[error("TODO")]
    Todo {},
}

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum SubscribeError {}

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum AckError {}
