use super::super::{BroadcastTargetType, ContentStyle};
use super::{AbTestConfigInput, VariantInput};
use serde::{Deserialize, Serialize};

/// Region (regions are returned as plain strings like "us-east-1")
pub type Region = String;

/// Create broadcast request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBroadcastRequest {
    pub name: String,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ContentStyle>,
    #[serde(rename = "targetType", skip_serializing_if = "Option::is_none")]
    pub target_type: Option<BroadcastTargetType>,
    #[serde(rename = "audienceId", skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
    #[serde(rename = "topicId", skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
    #[serde(rename = "segmentId", skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    #[serde(rename = "senderName")]
    pub sender_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(rename = "replyToInbound", skip_serializing_if = "Option::is_none")]
    pub reply_to_inbound: Option<bool>,
    #[serde(rename = "domainId")]
    pub domain_id: String,
    #[serde(rename = "scheduledAt", skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
    #[serde(
        rename = "includeListUnsubscribeHeader",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_list_unsubscribe_header: Option<bool>,
    #[serde(rename = "openTracking", skip_serializing_if = "Option::is_none")]
    pub open_tracking: Option<bool>,
    #[serde(rename = "clickTracking", skip_serializing_if = "Option::is_none")]
    pub click_tracking: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<VariantInput>>,
    #[serde(rename = "abTest", default, skip_serializing_if = "Option::is_none")]
    pub ab_test: Option<AbTestConfigInput>,
}

/// Update broadcast request.
///
/// `audience_id`, `topic_id`, `segment_id`, `reply_to`, `scheduled_at`,
/// `open_tracking`, `click_tracking`, `variants` and `ab_test` are
/// double-`Option` fields: `None` omits the field, `Some(None)` sends an
/// explicit JSON null. Null clears reply-to/schedule and resets tracking; null
/// target IDs, like non-null target IDs, require `target_type`. Variants and
/// A/B config can be updated independently, but clearing variants cannot be
/// combined with a non-null A/B configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateBroadcastRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ContentStyle>,
    #[serde(rename = "targetType", skip_serializing_if = "Option::is_none")]
    pub target_type: Option<BroadcastTargetType>,
    #[serde(
        rename = "audienceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "super::super::patch::deserialize_present")]
    pub audience_id: Option<Option<String>>,
    #[serde(rename = "topicId", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "super::super::patch::deserialize_present")]
    pub topic_id: Option<Option<String>>,
    #[serde(rename = "segmentId", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "super::super::patch::deserialize_present")]
    pub segment_id: Option<Option<String>>,
    #[serde(rename = "senderName", skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "replyTo", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "super::super::patch::deserialize_present")]
    pub reply_to: Option<Option<String>>,
    #[serde(rename = "replyToInbound", skip_serializing_if = "Option::is_none")]
    pub reply_to_inbound: Option<bool>,
    #[serde(rename = "domainId", skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(
        rename = "scheduledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "super::super::patch::deserialize_present")]
    pub scheduled_at: Option<Option<String>>,
    #[serde(
        rename = "includeListUnsubscribeHeader",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_list_unsubscribe_header: Option<bool>,
    #[serde(
        rename = "openTracking",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "super::super::patch::deserialize_present")]
    pub open_tracking: Option<Option<bool>>,
    #[serde(
        rename = "clickTracking",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[serde(deserialize_with = "super::super::patch::deserialize_present")]
    pub click_tracking: Option<Option<bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "super::super::patch::deserialize_present")]
    pub variants: Option<Option<Vec<VariantInput>>>,
    #[serde(rename = "abTest", default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "super::super::patch::deserialize_present")]
    pub ab_test: Option<Option<AbTestConfigInput>>,
}

/// Send test email request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTestRequest {
    pub emails: Vec<String>,
}

/// Send test broadcast response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendTestResponse {
    #[serde(rename = "testEmails", default)]
    pub test_emails: i64,
    #[serde(default)]
    pub message: String,
}
