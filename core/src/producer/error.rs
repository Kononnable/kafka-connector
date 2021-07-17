use thiserror::Error as DeriveError;

#[non_exhaustive]
#[derive(Debug, DeriveError)]
pub enum MessageSendError {
    #[error("The producer has been closed before message was sent")]
    ProducerClosed,
}
