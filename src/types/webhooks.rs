use super::{WebhookEventCategory, WebhookEventType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotateWebhookResponse {
    pub id: String,
    pub secret: String,
}

/// Webhook resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    pub id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    pub url: String,
    #[serde(default)]
    pub events: Vec<WebhookEventType>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// Masked placeholder (`••••••••`) outside the one-time create response.
    pub secret: Option<String>,
    #[serde(rename = "failureCount", default)]
    pub failure_count: i64,
    #[serde(
        rename = "lastFailureAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_failure_at: Option<String>,
    #[serde(
        rename = "lastDeliveryAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_delivery_at: Option<String>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// One-time webhook creation response with a usable `whsec_` signing secret.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedWebhook {
    pub id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    pub url: String,
    #[serde(default)]
    pub events: Vec<WebhookEventType>,
    #[serde(default)]
    pub enabled: bool,
    pub secret: String,
    #[serde(rename = "failureCount", default)]
    pub failure_count: i64,
    #[serde(rename = "lastFailureAt", default)]
    pub last_failure_at: Option<String>,
    #[serde(rename = "lastDeliveryAt", default)]
    pub last_delivery_at: Option<String>,
    #[serde(rename = "disabledAt", default)]
    pub disabled_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Create webhook request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub events: Vec<WebhookEventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Update webhook request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateWebhookRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<WebhookEventType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Webhook test response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookTestResponse {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub status: i64,
    #[serde(rename = "statusText", default)]
    pub status_text: String,
    #[serde(default)]
    pub message: String,
}

/// Webhook event definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEventDefinition {
    pub name: String,
    pub description: String,
    pub category: WebhookEventCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEventsResponse {
    pub items: Vec<WebhookEventType>,
    pub events: HashMap<String, WebhookEventDefinition>,
}
