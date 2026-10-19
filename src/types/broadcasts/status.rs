use super::super::{BroadcastLifecycleStatus, BroadcastStatusWarningType};
use serde::{Deserialize, Serialize};

/// Broadcast analytics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BroadcastAnalytics {
    #[serde(rename = "totalEmails", default)]
    pub total_emails: i64,
    #[serde(rename = "sentEmails", default)]
    pub sent_emails: i64,
    #[serde(rename = "deliveredEmails", default)]
    pub delivered_emails: i64,
    #[serde(rename = "openedEmails", default)]
    pub opened_emails: i64,
    #[serde(rename = "clickedEmails", default)]
    pub clicked_emails: i64,
    #[serde(rename = "totalClicks", default)]
    pub total_clicks: i64,
    #[serde(rename = "bouncedEmails", default)]
    pub bounced_emails: i64,
    #[serde(rename = "failedEmails", default)]
    pub failed_emails: i64,
    #[serde(rename = "unsubscribedEmails", default)]
    pub unsubscribed_emails: i64,
    #[serde(rename = "deliveryRate", default)]
    pub delivery_rate: f64,
    #[serde(rename = "openRate", default)]
    pub open_rate: f64,
    #[serde(rename = "clickRate", default)]
    pub click_rate: f64,
    #[serde(rename = "bounceRate", default)]
    pub bounce_rate: f64,
    #[serde(rename = "unsubscribeRate", default)]
    pub unsubscribe_rate: f64,
}

/// Broadcast status warning
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BroadcastStatusWarning {
    #[serde(rename = "type", default)]
    pub warning_type: BroadcastStatusWarningType,
    #[serde(default)]
    pub message: String,
    #[serde(rename = "failedCount", default)]
    pub failed_count: i64,
}

/// Broadcast status progress (only present while SENDING)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BroadcastStatusProgress {
    #[serde(default)]
    pub percentage: i64,
    #[serde(rename = "emailsSent", default)]
    pub emails_sent: i64,
    #[serde(rename = "totalContacts", default)]
    pub total_contacts: i64,
    #[serde(rename = "pendingEmails", default)]
    pub pending_emails: i64,
    #[serde(
        rename = "estimatedTimeRemaining",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub estimated_time_remaining: Option<String>,
}

/// Broadcast status schedule (only present while SCHEDULED)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BroadcastStatusSchedule {
    #[serde(
        rename = "scheduledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_at: Option<String>,
    /// Milliseconds until the scheduled send.
    #[serde(rename = "timeUntilSend", default)]
    pub time_until_send: i64,
    #[serde(rename = "willSendIn", default)]
    pub will_send_in: String,
    #[serde(rename = "isPastDue", default)]
    pub is_past_due: bool,
}

/// Broadcast status completion (only present while SENT)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BroadcastStatusCompletion {
    #[serde(
        rename = "completedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_at: Option<String>,
    #[serde(rename = "totalEmailsSent", default)]
    pub total_emails_sent: i64,
    #[serde(rename = "successRate", default)]
    pub success_rate: i64,
}

/// Broadcast status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastStatus {
    pub id: String,
    pub name: String,
    pub status: BroadcastLifecycleStatus,
    #[serde(rename = "totalContacts", default)]
    pub total_contacts: i64,
    #[serde(rename = "emailsSent", default)]
    pub emails_sent: i64,
    #[serde(rename = "failedEmails", default)]
    pub failed_emails: i64,
    #[serde(rename = "pendingEmails", default)]
    pub pending_emails: i64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "sentAt", default, skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
    #[serde(
        rename = "completedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_at: Option<String>,
    #[serde(
        rename = "scheduledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_at: Option<String>,
    #[serde(
        rename = "sendingStartedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sending_started_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<String>,
    #[serde(rename = "stepLabel", default, skip_serializing_if = "Option::is_none")]
    pub step_label: Option<String>,
    #[serde(rename = "stepIndex", default, skip_serializing_if = "Option::is_none")]
    pub step_index: Option<i64>,
    #[serde(rename = "stepTotal", default, skip_serializing_if = "Option::is_none")]
    pub step_total: Option<i64>,
    #[serde(
        rename = "stepProgress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub step_progress: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning: Option<BroadcastStatusWarning>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<BroadcastStatusProgress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<BroadcastStatusSchedule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion: Option<BroadcastStatusCompletion>,
}
