use super::super::{BroadcastTargetType, EmailStatus, EmailType};
use super::BroadcastSummary;
use serde::{Deserialize, Serialize};

/// Broadcast cancel response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastCancelResponse {
    pub broadcast: BroadcastSummary,
    #[serde(rename = "cancelledJobs")]
    pub cancelled_jobs: i64,
}

/// Compact contact reference embedded in BroadcastEmail. `id` is `None` when
/// the contact was deleted after the send.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BroadcastEmailContact {
    #[serde(default)]
    pub id: Option<String>,
    pub email: String,
}

/// Broadcast email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastEmail {
    pub id: String,
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    pub to: String,
    #[serde(default)]
    pub subject: String,
    pub status: EmailStatus,
    #[serde(rename = "type")]
    pub r#type: EmailType,
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
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(default)]
    pub contact: BroadcastEmailContact,
}

/// Broadcast cost estimate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastCostEstimate {
    #[serde(rename = "estimatedCost", default)]
    pub estimated_cost: f64,
    #[serde(rename = "emailPrice", default)]
    pub email_price: f64,
    #[serde(rename = "contactCount", default)]
    pub contact_count: i64,
    #[serde(rename = "currentBalance", default)]
    pub current_balance: f64,
    #[serde(rename = "hasSufficientBalance", default)]
    pub has_sufficient_balance: bool,
    #[serde(default)]
    pub shortfall: f64,
    #[serde(rename = "freeEmailsAvailable", default)]
    pub free_emails_available: i64,
    #[serde(rename = "freeEmailsToUse", default)]
    pub free_emails_to_use: i64,
    #[serde(rename = "paidEmailsCount", default)]
    pub paid_emails_count: i64,
}

/// Target used to resolve the current subscribed recipient count.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastRecipientEstimateRequest {
    #[serde(rename = "targetType")]
    pub target_type: BroadcastTargetType,
    #[serde(rename = "audienceId", skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
    #[serde(rename = "topicId", skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
    #[serde(rename = "segmentId", skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
}

/// Current subscribed recipient count for a broadcast target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastRecipientEstimate {
    pub count: i64,
}
