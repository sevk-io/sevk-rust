use super::{ActivityAction, ContactUnsubscribeActor};
use serde::{Deserialize, Serialize};

/// Contact resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub email: String,
    #[serde(default)]
    pub subscribed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
    #[serde(rename = "updatedAt", default)]
    pub updated_at: String,
    #[serde(rename = "projectId", default)]
    pub project_id: String,
    #[serde(
        rename = "lastEventAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_event_at: Option<String>,
    #[serde(
        rename = "unsubscribedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribed_at: Option<String>,
    #[serde(
        rename = "unsubscribedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unsubscribed_by: Option<ContactUnsubscribeActor>,
    #[serde(
        rename = "resubscriptionLocked",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resubscription_locked: Option<bool>,
    #[serde(
        rename = "bounceCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bounce_count: Option<i64>,
    #[serde(
        rename = "lastBouncedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_bounced_at: Option<String>,
}

/// Slim broadcast reference attached to contact activity items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityBroadcastRef {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
}

/// Slim email reference attached to contact activity items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEmailRef {
    pub id: String,
    #[serde(default)]
    pub subject: Option<String>,
}

/// Contact activity entry with joined email/broadcast refs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactActivityItem {
    pub id: String,
    pub action: ActivityAction,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "userAgent", default)]
    pub user_agent: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    #[serde(default)]
    pub broadcast: Option<ActivityBroadcastRef>,
    #[serde(default)]
    pub email: Option<ActivityEmailRef>,
}

/// Create contact request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateContactRequest {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Map<String, serde_json::Value>>,
}

/// Update contact request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContactDataUpdate {
    Object(serde_json::Map<String, serde_json::Value>),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateContactRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "super::patch::deserialize_present")]
    pub data: Option<ContactDataUpdate>,
}

/// Slim contact row returned when adding contacts to an audience or topic (newest first).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddedContact {
    pub id: String,
    pub email: String,
    pub subscribed: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// Response from adding contacts to an audience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddContactsToAudienceResponse {
    pub added: i64,
    #[serde(default)]
    pub contacts: Vec<AddedContact>,
}

/// Response from adding contacts to a topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddContactsToTopicResponse {
    pub added: i64,
    #[serde(default)]
    pub contacts: Vec<AddedContact>,
}

/// Bulk update contact entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUpdateContactEntry {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, deserialize_with = "super::patch::deserialize_present")]
    pub data: Option<ContactDataUpdate>,
}

/// Bulk update contacts request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUpdateContactsRequest {
    pub contacts: Vec<BulkUpdateContactEntry>,
}

/// Bulk update contacts response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BulkUpdateContactsResponse {
    #[serde(default)]
    pub updated: i64,
    #[serde(rename = "notFound", default)]
    pub not_found: i64,
    #[serde(default)]
    pub errors: Vec<String>,
    #[serde(rename = "updatedEmails", default)]
    pub updated_emails: Vec<String>,
    #[serde(rename = "notFoundEmails", default)]
    pub not_found_emails: Vec<String>,
}

/// Import contact entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportContactEntry {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Map<String, serde_json::Value>>,
}

/// Import contacts request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportContactsRequest {
    pub contacts: Vec<ImportContactEntry>,
    #[serde(rename = "audienceId", skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
    #[serde(rename = "topicId", skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

/// Compact contact returned from an import call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedContact {
    pub id: String,
    pub email: String,
    #[serde(default)]
    pub subscribed: bool,
}

/// Import contacts response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportContactsResponse {
    #[serde(default)]
    pub created: i64,
    #[serde(default)]
    pub errors: Vec<String>,
    #[serde(default)]
    pub contacts: Vec<ImportedContact>,
}
