use kafka_connector_protocol::api_error::ApiError;

/// Requests with those errors in responses can be just resent, other retriable errors might require some business logic to change the message
pub fn is_api_error_retriable(error: ApiError) -> bool {
    matches!(
        error,
        ApiError::CorruptMessage
            | ApiError::RequestTimedOut
            | ApiError::NetworkException
            | ApiError::CoordinatorLoadInProgress
            | ApiError::CoordinatorNotAvailable
            | ApiError::NotEnoughReplicas
            | ApiError::KafkaStorageError
            | ApiError::FetchSessionIdNotFound
            | ApiError::InvalidFetchSessionEpoch
            | ApiError::TopicDeletionDisabled
            | ApiError::OffsetNotAvailable
            | ApiError::PreferredLeaderNotAvailable
            | ApiError::EligibleLeadersNotAvailable
            | ApiError::ThrottlingQuotaExceeded
    )
}
