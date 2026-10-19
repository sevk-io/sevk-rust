use super::ContentStyle;
use serde::{Deserialize, Serialize};

/// Template resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub title: String,
    pub content: String,
    pub style: ContentStyle,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Create template request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTemplateRequest {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ContentStyle>,
}

/// Update template request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTemplateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ContentStyle>,
}
