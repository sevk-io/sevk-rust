use crate::markup::RenderError;
use std::collections::HashMap;
use thiserror::Error;

/// Error type for sevk SDK operations
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request failed
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
        code: Option<String>,
        /// Field-level validation errors (422), field name -> messages.
        fields: HashMap<String, Vec<String>>,
        /// Seconds to wait before retrying, from `Retry-After` on a 429.
        retry_after: Option<u64>,
        details: Option<serde_json::Value>,
        response_body: serde_json::Value,
    },

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

    /// Domain not verified
    #[error("Domain not verified: {0}")]
    DomainNotVerified(String),

    /// XML parsing error
    #[error("XML parsing error: {0}")]
    XmlParse(String),

    /// Markup exceeded a render limit
    #[error("{0}")]
    Markup(#[from] RenderError),
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
        match self {
            Error::Api { status: 422, .. } => true,
            Error::Api {
                status: 400,
                code,
                fields,
                ..
            } => code.as_deref() == Some("VALIDATION_ERROR") || !fields.is_empty(),
            _ => false,
        }
    }

    /// Check if this is a 429 Too Many Requests error
    pub fn is_rate_limited(&self) -> bool {
        matches!(self, Error::Api { status: 429, .. })
    }

    /// Check if this is a 5xx server error
    pub fn is_server_error(&self) -> bool {
        matches!(self, Error::Api { status, .. } if *status >= 500)
    }

    /// Field-level validation errors (field name -> messages), when present.
    pub fn fields(&self) -> Option<&HashMap<String, Vec<String>>> {
        match self {
            Error::Api { fields, .. } if !fields.is_empty() => Some(fields),
            _ => None,
        }
    }

    /// Seconds to wait before retrying, from `Retry-After` on a 429.
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            Error::Api { retry_after, .. } => *retry_after,
            _ => None,
        }
    }

    /// Machine-readable error code from the API envelope, when present.
    pub fn code(&self) -> Option<&str> {
        match self {
            Error::Api { code, .. } => code.as_deref(),
            Error::Json(_) => Some("SERIALIZATION_ERROR"),
            Error::Timeout(_) => Some("TIMEOUT"),
            Error::Request(error) if error.is_timeout() => Some("TIMEOUT"),
            Error::Request(_) => Some("NETWORK_ERROR"),
            _ => None,
        }
    }

    pub fn details(&self) -> Option<&serde_json::Value> {
        match self {
            Error::Api { details, .. } => details.as_ref(),
            _ => None,
        }
    }

    pub fn response_body(&self) -> Option<&serde_json::Value> {
        match self {
            Error::Api { response_body, .. } => Some(response_body),
            _ => None,
        }
    }

    /// Get the HTTP status code if available
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Error::Api { status, .. } => Some(*status),
            Error::NotFound(_) => Some(404),
            Error::InvalidApiKey => Some(401),
            Error::DomainNotVerified(_) => Some(403),
            _ => None,
        }
    }
}
