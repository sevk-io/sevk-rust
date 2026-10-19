use super::super::{
    AbTestMetric, AbTestStatus, BroadcastLifecycleStatus, BroadcastTargetType, ContentStyle,
};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// Fields guaranteed on every persisted broadcast projection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastBase {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub style: ContentStyle,
    pub status: BroadcastLifecycleStatus,
    #[serde(rename = "targetType", default)]
    pub target_type: BroadcastTargetType,
    #[serde(
        rename = "audienceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audience_id: Option<String>,
    #[serde(rename = "topicId", default, skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
    #[serde(rename = "segmentId", default, skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(
        rename = "senderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sender_name: Option<String>,
    #[serde(rename = "replyTo", default, skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(
        rename = "replyToInbound",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to_inbound: Option<bool>,
    #[serde(rename = "domainId", default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(
        rename = "scheduledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_at: Option<String>,
    #[serde(rename = "sentAt", default, skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
    #[serde(
        rename = "completedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_at: Option<String>,
    #[serde(rename = "projectId", default)]
    pub project_id: String,
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
    #[serde(rename = "updatedAt", default)]
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivered: Option<String>,
    #[serde(
        rename = "includeListUnsubscribeHeader",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_list_unsubscribe_header: Option<bool>,
    #[serde(
        rename = "openTracking",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub open_tracking: Option<bool>,
    #[serde(
        rename = "clickTracking",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub click_tracking: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<BroadcastAudienceRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<BroadcastTopicRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment: Option<BroadcastSegmentRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<BroadcastDomainRef>,
}

/// Broadcast projection returned by list, send, and cancel operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastSummary {
    #[serde(flatten)]
    pub base: BroadcastBase,
    #[serde(rename = "totalContacts", default)]
    pub total_contacts: i64,
    #[serde(rename = "sentCount", default)]
    pub sent_count: i64,
    #[serde(rename = "failedCount", default)]
    pub failed_count: i64,
    #[serde(rename = "deliveredCount", default)]
    pub delivered_count: i64,
    #[serde(rename = "openedCount", default)]
    pub opened_count: i64,
    #[serde(rename = "clickedCount", default)]
    pub clicked_count: i64,
    #[serde(rename = "bouncedCount", default)]
    pub bounced_count: i64,
    #[serde(rename = "complainedCount", default)]
    pub complained_count: i64,
    #[serde(rename = "unsubscribedCount", default)]
    pub unsubscribed_count: i64,
    #[serde(rename = "estimatedCost")]
    pub estimated_cost: Option<String>,
    #[serde(rename = "actualCost")]
    pub actual_cost: Option<String>,
}

impl Deref for BroadcastSummary {
    type Target = BroadcastBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Broadcast projection returned by create operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastWithVariants {
    #[serde(flatten)]
    pub base: BroadcastBase,
    pub variants: Vec<BroadcastVariant>,
    #[serde(rename = "abTest")]
    pub ab_test: Option<BroadcastAbTest>,
}

impl Deref for BroadcastWithVariants {
    type Target = BroadcastBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Broadcast projection returned by updates. Experiment fields are emitted
/// only when the mutation touched experiment state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastUpdate {
    #[serde(flatten)]
    pub base: BroadcastBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<BroadcastVariant>>,
    #[serde(rename = "abTest", default, skip_serializing_if = "Option::is_none")]
    pub ab_test: Option<BroadcastAbTest>,
}

impl Deref for BroadcastUpdate {
    type Target = BroadcastBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// Complete broadcast projection returned by get operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Broadcast {
    #[serde(flatten)]
    pub summary: BroadcastSummary,
    #[serde(default)]
    pub variants: Vec<BroadcastVariant>,
    #[serde(rename = "abTest")]
    pub ab_test: Option<BroadcastAbTest>,
}

impl Deref for Broadcast {
    type Target = BroadcastSummary;
    fn deref(&self) -> &Self::Target {
        &self.summary
    }
}

/// Snapshot of the target audience attached to a broadcast.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastAudienceRef {
    pub id: String,
    pub name: String,
    #[serde(rename = "contactsCount", default)]
    pub contacts_count: i64,
    #[serde(rename = "subscribedContactsCount", default)]
    pub subscribed_contacts_count: i64,
    #[serde(rename = "broadcastsCount", default)]
    pub broadcasts_count: i64,
    #[serde(rename = "topicsCount", default)]
    pub topics_count: i64,
    #[serde(rename = "segmentsCount", default)]
    pub segments_count: i64,
}

/// Snapshot of the target topic attached to a broadcast.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastTopicRef {
    pub id: String,
    pub name: String,
    #[serde(rename = "contactsCount", default)]
    pub contacts_count: i64,
    #[serde(rename = "subscribedContactsCount", default)]
    pub subscribed_contacts_count: i64,
    #[serde(rename = "broadcastsCount", default)]
    pub broadcasts_count: i64,
}

/// Snapshot of the target segment attached to a broadcast.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastSegmentRef {
    pub id: String,
    pub name: String,
}

/// Snapshot of the sending domain attached to a broadcast.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastDomainRef {
    pub id: String,
    pub domain: String,
    #[serde(
        rename = "senderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sender_name: Option<String>,
    pub verified: bool,
}

/// A/B test variant on a broadcast.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastVariant {
    pub id: String,
    #[serde(rename = "broadcastId", default)]
    pub broadcast_id: String,
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
    #[serde(rename = "updatedAt", default)]
    pub updated_at: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "fromName", default, skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    #[serde(rename = "totalContacts", default)]
    pub total_contacts: i64,
    #[serde(rename = "sentCount", default)]
    pub sent_count: i64,
    #[serde(rename = "failedCount", default)]
    pub failed_count: i64,
    #[serde(rename = "deliveredCount", default)]
    pub delivered_count: i64,
    #[serde(rename = "openedCount", default)]
    pub opened_count: i64,
    #[serde(rename = "clickedCount", default)]
    pub clicked_count: i64,
    #[serde(rename = "bouncedCount", default)]
    pub bounced_count: i64,
    #[serde(rename = "complainedCount", default)]
    pub complained_count: i64,
    #[serde(rename = "unsubscribedCount", default)]
    pub unsubscribed_count: i64,
    #[serde(rename = "isWinner", default)]
    pub is_winner: bool,
}

/// A/B test configuration and runtime state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastAbTest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "broadcastId", default)]
    pub broadcast_id: String,
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
    #[serde(rename = "updatedAt", default)]
    pub updated_at: String,
    #[serde(rename = "testPercent")]
    pub test_percent: i32,
    #[serde(
        rename = "evalAfterMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub eval_after_minutes: Option<i32>,
    #[serde(
        rename = "winnerMetric",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub winner_metric: Option<AbTestMetric>,
    pub status: AbTestStatus,
    #[serde(
        rename = "scheduledEvalAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_eval_at: Option<String>,
    #[serde(
        rename = "winnerVariantId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub winner_variant_id: Option<String>,
    #[serde(
        rename = "evaluatedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub evaluated_at: Option<String>,
}

/// Lean broadcast item returned by `GET /broadcasts/active`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveBroadcast {
    pub id: String,
    pub name: String,
    pub status: BroadcastLifecycleStatus,
    #[serde(rename = "sentCount")]
    pub sent_count: i64,
    #[serde(rename = "failedCount")]
    pub failed_count: i64,
    #[serde(rename = "totalContacts")]
    pub total_contacts: i64,
    #[serde(
        rename = "scheduledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_at: Option<String>,
    #[serde(rename = "abTest", default, skip_serializing_if = "Option::is_none")]
    pub ab_test: Option<ActiveBroadcastAbTest>,
}

/// Lean A/B test runtime state embedded in `ActiveBroadcast`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveBroadcastAbTest {
    pub status: AbTestStatus,
    #[serde(
        rename = "scheduledEvalAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_eval_at: Option<String>,
    #[serde(rename = "testPercent")]
    pub test_percent: i64,
}

/// Per-variant override sent on create/update.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VariantInput {
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "fromName", default, skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
}

/// A/B test config sent on create/update.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbTestConfigInput {
    #[serde(rename = "testPercent")]
    pub test_percent: i32,
    #[serde(rename = "evalAfterMinutes")]
    pub eval_after_minutes: i32,
    #[serde(
        rename = "winnerMetric",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub winner_metric: Option<AbTestMetric>,
}
