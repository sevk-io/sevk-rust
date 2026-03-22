//! Type definitions for the Sevk SDK

use serde::{Deserialize, Serialize};

/// Simple items list response (non-paginated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemsList<T> {
    pub items: Vec<T>,
}

/// Paginated list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedList<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
}

/// Contact resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub email: String,
    #[serde(default)]
    pub subscribed: bool,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Create contact request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateContactRequest {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Update contact request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateContactRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Audience resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audience {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "usersCanSee")]
    pub users_can_see: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Create audience request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAudienceRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "usersCanSee", skip_serializing_if = "Option::is_none")]
    pub users_can_see: Option<String>,
}

/// Update audience request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateAudienceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "usersCanSee", skip_serializing_if = "Option::is_none")]
    pub users_can_see: Option<String>,
}

/// Add contacts to audience request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddContactsRequest {
    #[serde(rename = "contactIds")]
    pub contact_ids: Vec<String>,
}

/// Template resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub title: String,
    pub content: String,
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
}

/// Update template request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTemplateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Broadcast resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Broadcast {
    pub id: String,
    pub name: String,
    pub subject: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default, rename = "senderName")]
    pub sender_name: Option<String>,
    pub status: String,
    #[serde(default, rename = "scheduledAt")]
    pub scheduled_at: Option<String>,
    #[serde(default, rename = "sentAt")]
    pub sent_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Domain resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub id: String,
    pub domain: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Domains list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainsListResponse {
    pub items: Vec<Domain>,
}

/// Create domain request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDomainRequest {
    pub domain: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "senderName", skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// DNS record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
}

/// DNS records response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecordsResponse {
    pub items: Vec<DnsRecord>,
}

/// Domain verify response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainVerifyResponse {
    pub verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Email detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailDetail {
    pub id: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Topic resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "audienceId")]
    pub audience_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Create topic request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTopicRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Update topic request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTopicRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Segment rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentRule {
    pub field: String,
    pub operator: String,
    pub value: String,
}

/// Segment resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub id: String,
    pub name: String,
    pub rules: Vec<SegmentRule>,
    pub operator: String,
    #[serde(rename = "audienceId")]
    pub audience_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Create segment request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSegmentRequest {
    pub name: String,
    pub rules: Vec<SegmentRule>,
    pub operator: String,
}

/// Update segment request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSegmentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<SegmentRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
}

/// Subscribe request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
    #[serde(rename = "audienceId")]
    pub audience_id: String,
    #[serde(rename = "topicIds", skip_serializing_if = "Option::is_none")]
    pub topic_ids: Option<Vec<String>>,
}

/// Unsubscribe request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeRequest {
    pub email: String,
    #[serde(rename = "audienceId", skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
}

/// Email attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub filename: String,
    pub content: String,      // Base64 encoded
    #[serde(rename = "contentType")]
    pub content_type: String, // MIME type
}

/// Send email request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailRequest {
    pub to: String,
    pub subject: String,
    pub from: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<EmailAttachment>>,
}

/// Send email response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    pub message: Option<String>,
}

/// Bulk email error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEmailError {
    pub index: usize,
    pub email: String,
    pub error: String,
}

/// Bulk email request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEmailRequest {
    pub emails: Vec<SendEmailRequest>,
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

/// Bulk update contact entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUpdateContactEntry {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Bulk update contacts request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUpdateContactsRequest {
    pub contacts: Vec<BulkUpdateContactEntry>,
}

/// Bulk update contacts response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUpdateContactsResponse {
    #[serde(default)]
    pub updated: i64,
}

/// Import contact entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportContactEntry {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Import contacts request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportContactsRequest {
    pub contacts: Vec<ImportContactEntry>,
    #[serde(rename = "audienceId", skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
}

/// Import contacts response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportContactsResponse {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub errors: Vec<serde_json::Value>,
    #[serde(default)]
    pub contacts: Vec<Contact>,
}

/// Contact event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactEvent {
    pub id: String,
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<String>,
    #[serde(rename = "broadcastId", skip_serializing_if = "Option::is_none")]
    pub broadcast_id: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// Region (regions are returned as plain strings like "us-east-1")
pub type Region = String;

/// Create broadcast request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateBroadcastRequest {
    pub name: String,
    pub subject: String,
    pub body: String,
    #[serde(rename = "domainId")]
    pub domain_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "targetType", skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "audienceId", skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
    #[serde(rename = "topicId", skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
    #[serde(rename = "segmentId", skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    #[serde(rename = "senderName", skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[serde(rename = "senderEmail", skip_serializing_if = "Option::is_none")]
    pub sender_email: Option<String>,
    #[serde(rename = "scheduledAt", skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
}

/// Update broadcast request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateBroadcastRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "targetType", skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "audienceId", skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
    #[serde(rename = "topicId", skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
    #[serde(rename = "segmentId", skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    #[serde(rename = "senderName", skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[serde(rename = "domainId", skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(rename = "scheduledAt", skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
}

/// Send test email request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTestRequest {
    pub emails: Vec<String>,
}

/// Broadcast analytics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BroadcastAnalytics {
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub sent: i64,
    #[serde(default)]
    pub delivered: i64,
    #[serde(default)]
    pub opened: i64,
    #[serde(default)]
    pub clicked: i64,
    #[serde(default)]
    pub bounced: i64,
    #[serde(default)]
    pub complained: i64,
}

/// Segment calculate request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SegmentCalculateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<SegmentRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
}

/// Segment calculate response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SegmentCalculateResponse {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub contacts: Vec<Contact>,
}

/// Add contacts to topic request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicAddContactsRequest {
    #[serde(rename = "contactIds")]
    pub contact_ids: Vec<String>,
}

/// Update domain request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateDomainRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Broadcast status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastStatus {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub sent: i64,
    #[serde(default)]
    pub delivered: i64,
    #[serde(default)]
    pub failed: i64,
}

/// Broadcast email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastEmail {
    pub id: String,
    pub to: String,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
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

/// Webhook resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    pub id: String,
    pub url: String,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// Create webhook request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Update webhook request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateWebhookRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Webhook test response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookTestResponse {
    #[serde(default)]
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Webhook event type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Event resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<String>,
    #[serde(rename = "broadcastId", skip_serializing_if = "Option::is_none")]
    pub broadcast_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// Event list parameters
#[derive(Debug, Clone, Default)]
pub struct EventListParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub event_type: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

impl EventListParams {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(page) = self.page {
            params.push(("page", page.to_string()));
        }
        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(ref event_type) = self.event_type {
            params.push(("type", event_type.clone()));
        }
        if let Some(ref from) = self.from {
            params.push(("from", from.clone()));
        }
        if let Some(ref to) = self.to {
            params.push(("to", to.clone()));
        }
        params
    }
}

/// Event stats response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventStats {
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub sent: i64,
    #[serde(default)]
    pub delivered: i64,
    #[serde(default)]
    pub opened: i64,
    #[serde(default)]
    pub clicked: i64,
    #[serde(default)]
    pub bounced: i64,
    #[serde(default)]
    pub complained: i64,
}

/// Empty response for delete operations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmptyResponse {}

/// List parameters
#[derive(Debug, Clone, Default)]
pub struct ListParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
}

impl ListParams {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(page) = self.page {
            params.push(("page", page.to_string()));
        }
        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }
        if let Some(ref search) = self.search {
            params.push(("search", search.clone()));
        }
        params
    }
}

/// Domain list parameters
#[derive(Debug, Clone, Default)]
pub struct DomainListParams {
    pub verified: Option<bool>,
}

impl DomainListParams {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(verified) = self.verified {
            params.push(("verified", verified.to_string()));
        }
        params
    }
}
