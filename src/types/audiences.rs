use super::AudienceVisibility;
use serde::{Deserialize, Serialize};

/// Audience returned by list/get, including aggregate counters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audience {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub visibility: AudienceVisibility,
    #[serde(rename = "projectId")]
    pub project_id: String,
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
    #[serde(
        rename = "topicsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub topics_count: Option<i64>,
    #[serde(
        rename = "segmentsCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub segments_count: Option<i64>,
}

/// Audience projection returned by create and update operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudienceMutationResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub visibility: AudienceVisibility,
    #[serde(rename = "projectId")]
    pub project_id: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<AudienceVisibility>,
}

/// Update audience request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateAudienceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<AudienceVisibility>,
}

/// Add contacts to audience request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddContactsRequest {
    #[serde(rename = "contactIds")]
    pub contact_ids: Vec<String>,
}
