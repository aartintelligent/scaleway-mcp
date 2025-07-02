use rmcp::model::{ErrorCode, ErrorData};
use std::borrow::Cow;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScalewayError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Serialization or deserialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("URL or endpoint error: {0}")]
    EndpointError(String),
    #[error("Unexpected error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, ScalewayError>;

impl From<ScalewayError> for ErrorData {
    fn from(e: ScalewayError) -> Self {
        ErrorData {
            code: ErrorCode::INTERNAL_ERROR,
            message: Cow::from(e.to_string()),
            data: None,
        }
    }
}
