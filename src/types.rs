//! Type definitions for the Sevk SDK

use serde::{Deserialize, Serialize};

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
    pub title: String,
    pub subject: Option<String>,
    pub content: Option<String>,
    pub status: String,
    #[serde(rename = "scheduledAt")]
    pub scheduled_at: Option<String>,
    #[serde(rename = "sentAt")]
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
    pub domains: Vec<Domain>,
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
}

/// Send email response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailResponse {
    pub id: String,
    pub message: Option<String>,
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
