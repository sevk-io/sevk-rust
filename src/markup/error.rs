use thiserror::Error;

/// A render limit that the markup exceeded
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("{0}")]
pub struct RenderError(pub(super) String);
