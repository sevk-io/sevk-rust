use super::{EmailStatus, EmailType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Email resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub id: String,
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(
        rename = "providerMessageId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub status: EmailStatus,
    #[serde(rename = "type")]
    pub email_type: EmailType,
    #[serde(rename = "contactId", default, skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(
        rename = "broadcastId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub broadcast_id: Option<String>,
    #[serde(
        rename = "broadcastVariantId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub broadcast_variant_id: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "openedAt", default, skip_serializing_if = "Option::is_none")]
    pub opened_at: Option<String>,
    #[serde(rename = "clickedAt", default, skip_serializing_if = "Option::is_none")]
    pub clicked_at: Option<String>,
    #[serde(
        rename = "unsubscribedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribed_at: Option<String>,
    #[serde(
        rename = "scheduledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_at: Option<String>,
    #[serde(
        rename = "cancelledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelled_at: Option<String>,
    #[serde(rename = "userAgent", default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Email attachment. Set `cid` for inline images referenced from the HTML body
/// via `<img src="cid:...">`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub filename: String,
    pub content: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
}

/// Email tracking options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmailTracking {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrList {
    Single(String),
    Multiple(Vec<String>),
}

impl Default for StringOrList {
    fn default() -> Self {
        Self::Single(String::new())
    }
}

impl From<String> for StringOrList {
    fn from(value: String) -> Self {
        Self::Single(value)
    }
}

impl From<&str> for StringOrList {
    fn from(value: &str) -> Self {
        Self::Single(value.to_string())
    }
}

impl From<Vec<String>> for StringOrList {
    fn from(value: Vec<String>) -> Self {
        Self::Multiple(value)
    }
}

/// Custom email headers
/// Send email request. The `to`, `reply_to`, `cc`, and `bcc` fields accept
/// either a single address (string) or an array of addresses.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendEmailRequest {
    pub from: String,
    pub to: StringOrList,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// sevk Markup body. The SDK renders this to HTML locally before sending,
    /// so it never appears on the wire (skipped during serialization).
    #[serde(skip)]
    pub markup: Option<String>,
    #[serde(rename = "reply_to", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<EmailAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<EmailTracking>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(rename = "scheduledAt", skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScheduledEmailRequest {
    #[serde(rename = "scheduledAt")]
    pub scheduled_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledEmailMutationResponse {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BulkEmailItem {
    pub from: String,
    pub to: String,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip)]
    pub markup: Option<String>,
    #[serde(rename = "reply_to", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<StringOrList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<EmailAttachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<EmailTracking>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleEmailResponse {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleEmailResponse {
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SendEmailResponse {
    Single(SingleEmailResponse),
    Multiple(MultipleEmailResponse),
}

impl<'de> Deserialize<'de> for SendEmailResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct WireResponse {
            id: Option<String>,
            ids: Option<Vec<String>>,
        }

        let response = WireResponse::deserialize(deserializer)?;
        match (response.id, response.ids) {
            (Some(id), None) => Ok(Self::Single(SingleEmailResponse { id })),
            (None, Some(ids)) => Ok(Self::Multiple(MultipleEmailResponse { ids })),
            _ => Err(serde::de::Error::custom(
                "expected exactly one of id or ids",
            )),
        }
    }
}

/// Bulk email error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEmailError {
    pub index: usize,
    pub email: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEmailRequest {
    pub emails: Vec<BulkEmailItem>,
}

/// Bulk email response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEmailResponse {
    pub success: u32,
    pub failed: u32,
    pub ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BulkEmailError>>,
}
