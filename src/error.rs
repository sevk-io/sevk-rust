//! Error types for the Sevk SDK

use thiserror::Error;

/// Error type for Sevk SDK operations
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request failed
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String },

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Request timeout
    #[error("Request timeout after {0}ms")]
    Timeout(u64),

    /// Invalid API key
    #[error("Invalid API key")]
    InvalidApiKey,

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Domain not verified
    #[error("Domain not verified: {0}")]
    DomainNotVerified(String),

    /// XML parsing error
    #[error("XML parsing error: {0}")]
    XmlParse(String),
}

impl Error {
    /// Check if this is a 404 Not Found error
    pub fn is_not_found(&self) -> bool {
        matches!(self, Error::NotFound(_) | Error::Api { status: 404, .. })
    }

    /// Check if this is an authentication error
    pub fn is_auth_error(&self) -> bool {
        matches!(self, Error::InvalidApiKey | Error::Api { status: 401, .. })
    }

    /// Check if this is a validation error
    pub fn is_validation_error(&self) -> bool {
        matches!(self, Error::Validation(_) | Error::Api { status: 400, .. })
    }

    /// Get the HTTP status code if available
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Error::Api { status, .. } => Some(*status),
            Error::NotFound(_) => Some(404),
            Error::InvalidApiKey => Some(401),
            Error::Validation(_) => Some(400),
            Error::DomainNotVerified(_) => Some(403),
            _ => None,
        }
    }
}
