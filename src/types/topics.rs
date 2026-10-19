use serde::{Deserialize, Serialize};

/// Topic returned by list/get, including aggregate counters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "audienceId")]
    pub audience_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(
        rename = "contactsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contacts_count: Option<i64>,
    #[serde(
        rename = "subscribedContactsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscribed_contacts_count: Option<i64>,
    #[serde(
        rename = "broadcastsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub broadcasts_count: Option<i64>,
}

/// Topic projection returned by create and update operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicMutationResponse {
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

/// Add contacts to topic request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicAddContactsRequest {
    #[serde(rename = "contactIds")]
    pub contact_ids: Vec<String>,
}
