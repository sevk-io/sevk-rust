use super::params::pagination_query;
use super::{
    BroadcastLifecycleStatus, EmailStatus, InboundBlocklistType, InboundEmailStatus, PaginatedList,
};
use serde::{Deserialize, Serialize};

/// Inbound email attachment metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundAttachment {
    pub id: String,
    pub filename: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "contentId")]
    pub content_id: Option<String>,
    pub size: i64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// Reply-link reference to a previously sent transactional email.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundReplyToEmailRef {
    pub id: String,
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EmailStatus>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(
        rename = "broadcastId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub broadcast_id: Option<String>,
    #[serde(rename = "contactId", default, skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
}

/// Reply-link reference to a broadcast.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundReplyToBroadcastRef {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<BroadcastLifecycleStatus>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// Lean outbound-email reference returned by inbound list operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundReplyEmailSummary {
    pub id: String,
    pub subject: Option<String>,
    pub to: Option<String>,
}

/// Lean broadcast reference returned by inbound list operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundReplyBroadcastSummary {
    pub id: String,
    pub name: String,
}

/// Exact inbound-email projection returned by list operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundEmailSummary {
    pub id: String,
    #[serde(rename = "messageId")]
    pub message_id: Option<String>,
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    #[serde(rename = "toAddress")]
    pub to_address: String,
    pub subject: Option<String>,
    pub status: InboundEmailStatus,
    #[serde(rename = "isSpam")]
    pub is_spam: bool,
    #[serde(rename = "isVirus")]
    pub is_virus: bool,
    #[serde(rename = "spamScore")]
    pub spam_score: Option<f64>,
    #[serde(rename = "rawSize")]
    pub raw_size: i64,
    #[serde(rename = "domainId")]
    pub domain_id: Option<String>,
    #[serde(rename = "replyToEmailId")]
    pub reply_to_email_id: Option<String>,
    #[serde(rename = "replyToBroadcastId")]
    pub reply_to_broadcast_id: Option<String>,
    #[serde(rename = "receivedAt")]
    pub received_at: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "replyToEmail")]
    pub reply_to_email: Option<InboundReplyEmailSummary>,
    #[serde(rename = "replyToBroadcast")]
    pub reply_to_broadcast: Option<InboundReplyBroadcastSummary>,
}

/// Inbound email resource.
/// Full inbound email returned by get.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundEmail {
    pub id: String,
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    #[serde(rename = "toAddress")]
    pub to_address: String,
    #[serde(rename = "ccAddresses", default)]
    pub cc_addresses: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    pub status: InboundEmailStatus,
    #[serde(rename = "isSpam", default)]
    pub is_spam: bool,
    #[serde(rename = "isVirus", default)]
    pub is_virus: bool,
    #[serde(rename = "spamScore", default, skip_serializing_if = "Option::is_none")]
    pub spam_score: Option<f64>,
    #[serde(rename = "rawSize", default)]
    pub raw_size: i64,
    #[serde(rename = "domainId", default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(rename = "receivedAt")]
    pub received_at: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "inReplyTo", default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<String>,
    #[serde(default)]
    pub references: Vec<String>,
    #[serde(rename = "bodyText", default, skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(rename = "bodyHtml", default, skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    /// Parsed message headers. Present only on get.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    /// Present only on get.
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Present only on get.
    #[serde(
        rename = "processedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub processed_at: Option<String>,
    /// Present only on get.
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub attachments: Vec<InboundAttachment>,
    #[serde(
        rename = "replyToEmailId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to_email_id: Option<String>,
    #[serde(
        rename = "replyToBroadcastId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to_broadcast_id: Option<String>,
    #[serde(
        rename = "replyToEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to_email: Option<InboundReplyToEmailRef>,
    #[serde(
        rename = "replyToBroadcast",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reply_to_broadcast: Option<InboundReplyToBroadcastRef>,
}

/// Per-status counts plus blocklist reject count for inbound mail.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InboundStats {
    pub total: i64,
    pub counts: InboundStatsCounts,
    pub received: i64,
    pub processed: i64,
    pub bounced: i64,
    pub blocked: i64,
    #[serde(rename = "blocklistRejected")]
    pub blocklist_rejected: i64,
    pub spam: i64,
    pub failed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InboundStatsCounts {
    #[serde(rename = "RECEIVED")]
    pub received: i64,
    #[serde(rename = "PROCESSED")]
    pub processed: i64,
    #[serde(rename = "BOUNCED")]
    pub bounced: i64,
    #[serde(rename = "BLOCKED")]
    pub blocked: i64,
    #[serde(rename = "FAILED")]
    pub failed: i64,
}

/// Inbound forward rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundRoute {
    pub id: String,
    #[serde(rename = "projectId", default)]
    pub project_id: String,
    pub name: String,
    pub pattern: String,
    pub target: String,
    pub priority: i32,
    pub enabled: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Create inbound route request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateInboundRouteRequest {
    pub name: String,
    pub pattern: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Update inbound route request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateInboundRouteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Inbound blocklist entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundBlocklistEntry {
    pub id: String,
    #[serde(rename = "projectId", default)]
    pub project_id: String,
    #[serde(rename = "type")]
    pub entry_type: InboundBlocklistType,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// Add inbound blocklist entry request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddInboundBlocklistRequest {
    #[serde(rename = "type")]
    pub entry_type: InboundBlocklistType,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Inbound quota state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundQuota {
    #[serde(rename = "projectId", default)]
    pub project_id: String,
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
    #[serde(rename = "updatedAt", default)]
    pub updated_at: String,
    #[serde(
        rename = "monthlyEmailLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub monthly_email_limit: Option<i64>,
    #[serde(rename = "monthlyEmailsUsed", default)]
    pub monthly_emails_used: i64,
    /// Always present; `None` means no free inbound tier.
    #[serde(rename = "freeMonthlyInboundEmails")]
    pub free_monthly_inbound_emails: Option<i64>,
    /// Free emails consumed during the current period.
    #[serde(rename = "freeEmailsUsed")]
    pub free_emails_used: i64,
    #[serde(rename = "inboundEmailPrice")]
    pub inbound_email_price: f64,
    #[serde(rename = "resetAt")]
    pub reset_at: String,
    #[serde(rename = "pausedAt", default, skip_serializing_if = "Option::is_none")]
    pub paused_at: Option<String>,
    #[serde(
        rename = "pausedReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub paused_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NullableI64 {
    Value(i64),
    Null,
}

/// Update inbound quota request. `None` omits the field; `Some(Null)` clears it.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateInboundQuotaRequest {
    #[serde(rename = "monthlyEmailLimit", skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "super::patch::deserialize_present")]
    pub monthly_email_limit: Option<NullableI64>,
}

/// Per-domain inbound security configuration. `id`, `created_at` and
/// `updated_at` are populated once a config has been saved for the domain;
/// until then the endpoint returns the platform default settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundSecurityConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "domainId")]
    pub domain_id: String,
    #[serde(rename = "dmarcStrict")]
    pub dmarc_strict: bool,
    #[serde(rename = "spfRequired")]
    pub spf_required: bool,
    #[serde(rename = "spamThreshold", deserialize_with = "Option::deserialize")]
    pub spam_threshold: Option<f64>,
    #[serde(rename = "catchAllEnabled")]
    pub catch_all_enabled: bool,
    #[serde(rename = "acceptedRecipients")]
    pub accepted_recipients: Vec<String>,
    #[serde(rename = "maxAttachmentSize")]
    pub max_attachment_size: i64,
    #[serde(rename = "allowedMimeTypes")]
    pub allowed_mime_types: Vec<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    pub platform: InboundSecurityPlatform,
}

/// Platform-level inbound security limits returned alongside a security config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundSecurityPlatform {
    #[serde(rename = "maxAttachmentSizeCap")]
    pub max_attachment_size_cap: i64,
}

/// Update inbound security request.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateInboundSecurityRequest {
    #[serde(rename = "dmarcStrict", skip_serializing_if = "Option::is_none")]
    pub dmarc_strict: Option<bool>,
    #[serde(rename = "spfRequired", skip_serializing_if = "Option::is_none")]
    pub spf_required: Option<bool>,
    #[serde(rename = "spamThreshold", skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "super::patch::deserialize_present")]
    pub spam_threshold: Option<Option<f64>>,
    #[serde(rename = "catchAllEnabled", skip_serializing_if = "Option::is_none")]
    pub catch_all_enabled: Option<bool>,
    #[serde(rename = "acceptedRecipients", skip_serializing_if = "Option::is_none")]
    pub accepted_recipients: Option<Vec<String>>,
    #[serde(rename = "maxAttachmentSize", skip_serializing_if = "Option::is_none")]
    pub max_attachment_size: Option<i64>,
    #[serde(rename = "allowedMimeTypes", skip_serializing_if = "Option::is_none")]
    pub allowed_mime_types: Option<Vec<String>>,
}

/// Inbound list request parameters. Pass either `page` (offset mode)
/// or `after` / `before` (cursor mode), not both.
#[derive(Debug, Clone, Default)]
pub struct ListInboundRequest {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    /// Forward cursor: id of the last item from the previous page. Mutually exclusive with `page`.
    pub after: Option<String>,
    /// Backward cursor: id of the first item from the next page. Walks results in reverse.
    /// Mutually exclusive with `page`.
    pub before: Option<String>,
    pub search: Option<String>,
    pub status: Option<InboundEmailStatus>,
    pub domain_id: Option<String>,
    pub broadcast_id: Option<String>,
    pub is_spam: Option<bool>,
    pub is_virus: Option<bool>,
}

impl ListInboundRequest {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = pagination_query(
            self.page,
            self.limit,
            self.after.as_deref(),
            self.before.as_deref(),
        );
        if let Some(ref search) = self.search {
            params.push(("search", search.clone()));
        }
        if let Some(ref status) = self.status {
            params.push(("status", status.as_str().to_string()));
        }
        if let Some(ref domain_id) = self.domain_id {
            params.push(("domainId", domain_id.clone()));
        }
        if let Some(ref broadcast_id) = self.broadcast_id {
            params.push(("broadcastId", broadcast_id.clone()));
        }
        if let Some(is_spam) = self.is_spam {
            params.push(("isSpam", is_spam.to_string()));
        }
        if let Some(is_virus) = self.is_virus {
            params.push(("isVirus", is_virus.to_string()));
        }
        params
    }
}

/// Inbound list response (paginated).
pub type InboundListResponse = PaginatedList<InboundEmailSummary>;

/// Inbound attachment download response (short-lived download URL).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundAttachmentDownload {
    pub url: String,
    pub filename: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}
