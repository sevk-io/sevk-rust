use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum AudienceVisibility {
    #[default]
    Public,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContentStyle {
    Html,
    #[default]
    Markup,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum BroadcastLifecycleStatus {
    #[default]
    Draft,
    Scheduled,
    Starting,
    Sending,
    Sent,
    Failed,
    Cancelling,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum BroadcastTargetType {
    #[default]
    All,
    Audience,
    Topic,
    Segment,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AbTestMetric {
    #[default]
    OpenRate,
    ClickRate,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AbTestStatus {
    #[default]
    Scheduled,
    Testing,
    Evaluating,
    RollingOut,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogicalOperator {
    #[default]
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SegmentRuleOperator {
    #[default]
    Equals,
    NotEquals,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    IsEmpty,
    IsNotEmpty,
    In,
    NotIn,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum TlsPolicy {
    #[default]
    Optional,
    Require,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum DnsRecordType {
    #[default]
    CNAME,
    TXT,
    MX,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum EmailStatus {
    #[default]
    Pending,
    Scheduled,
    Sent,
    Delivered,
    Bounced,
    Opened,
    Complaint,
    Failed,
    Suppressed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum EmailType {
    #[default]
    Marketing,
    Transactional,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum InboundEmailStatus {
    #[default]
    Received,
    Processed,
    Bounced,
    Blocked,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum InboundBlocklistType {
    #[default]
    Email,
    Domain,
    Ip,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContactUnsubscribeActor {
    #[default]
    User,
    Api,
    Admin,
    Bounce,
    Complaint,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum ActivityAction {
    #[default]
    Requested,
    Sent,
    Delivered,
    Opened,
    Clicked,
    Bounced,
    Complaint,
    Failed,
    Subscribed,
    Unsubscribed,
    Delayed,
    Rejected,
}

impl ActivityAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Requested => "REQUESTED",
            Self::Sent => "SENT",
            Self::Delivered => "DELIVERED",
            Self::Opened => "OPENED",
            Self::Clicked => "CLICKED",
            Self::Bounced => "BOUNCED",
            Self::Complaint => "COMPLAINT",
            Self::Failed => "FAILED",
            Self::Subscribed => "SUBSCRIBED",
            Self::Unsubscribed => "UNSUBSCRIBED",
            Self::Delayed => "DELAYED",
            Self::Rejected => "REJECTED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum WebhookEventCategory {
    #[default]
    Email,
    Subscription,
    Broadcast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookEventType {
    #[serde(rename = "email.sent")]
    EmailSent,
    #[serde(rename = "email.delivered")]
    EmailDelivered,
    #[serde(rename = "email.opened")]
    EmailOpened,
    #[serde(rename = "email.clicked")]
    EmailClicked,
    #[serde(rename = "email.bounced")]
    EmailBounced,
    #[serde(rename = "email.complained")]
    EmailComplained,
    #[serde(rename = "email.rejected")]
    EmailRejected,
    #[serde(rename = "email.delayed")]
    EmailDelayed,
    #[serde(rename = "email.rendering_failed")]
    EmailRenderingFailed,
    #[serde(rename = "contact.subscribed")]
    ContactSubscribed,
    #[serde(rename = "contact.unsubscribed")]
    ContactUnsubscribed,
    #[serde(rename = "broadcast.started")]
    BroadcastStarted,
    #[serde(rename = "broadcast.finished")]
    BroadcastFinished,
    #[serde(rename = "email.received")]
    EmailReceived,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum BroadcastStatusWarningType {
    #[default]
    #[serde(rename = "FAILED_EMAILS")]
    FailedEmails,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ActivityCategory {
    #[serde(rename = "EMAIL")]
    Email,
    #[serde(rename = "SUBSCRIPTION")]
    Subscription,
    #[default]
    #[serde(rename = "all")]
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum ActivityType {
    #[default]
    Email,
    Subscription,
}

impl ActivityCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Email => "EMAIL",
            Self::Subscription => "SUBSCRIPTION",
            Self::All => "all",
        }
    }
}

impl BroadcastLifecycleStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "DRAFT",
            Self::Scheduled => "SCHEDULED",
            Self::Starting => "STARTING",
            Self::Sending => "SENDING",
            Self::Sent => "SENT",
            Self::Failed => "FAILED",
            Self::Cancelling => "CANCELLING",
            Self::Cancelled => "CANCELLED",
        }
    }
}

impl InboundEmailStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Received => "RECEIVED",
            Self::Processed => "PROCESSED",
            Self::Bounced => "BOUNCED",
            Self::Blocked => "BLOCKED",
            Self::Failed => "FAILED",
        }
    }
}

impl WebhookEventCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::Subscription => "subscription",
            Self::Broadcast => "broadcast",
        }
    }
}
