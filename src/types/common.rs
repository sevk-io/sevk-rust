use super::{ActiveBroadcast, Webhook, WebhookEventDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Simple items list response (non-paginated)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemsList<T> {
    pub items: Vec<T>,
}

/// Webhook list response, including the event catalog available to the project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookListResponse {
    pub items: Vec<Webhook>,
    #[serde(rename = "availableEvents")]
    pub available_events: HashMap<String, WebhookEventDefinition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(
        rename = "totalPages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_pages: Option<i64>,
    #[serde(rename = "hasMore", default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_cursor: Option<String>,
    #[serde(
        rename = "prevCursor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prev_cursor: Option<String>,
}

/// Active broadcasts response. This endpoint is capped and is not paginated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveBroadcastListResponse {
    pub items: Vec<ActiveBroadcast>,
    pub total: i64,
}

/// Paginated list response.
///
/// The shape varies by mode:
/// - Offset (`?page=`): `total`, `page`, `total_pages` are set.
/// - Cursor (default, `?after=`, or `?before=`): `has_more`, `next_cursor`, and `prev_cursor` are set.
///
/// `items` is always present.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedList<T> {
    pub items: Vec<T>,
    /// Offset mode: total rows matching the filter across all pages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Offset mode: current page number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Offset mode: total page count.
    #[serde(
        rename = "totalPages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_pages: Option<i64>,
    /// Cursor mode: true when at least one more page is available.
    #[serde(rename = "hasMore", default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// Cursor mode: pass back as `after` to fetch the next page. `None` once iteration ends.
    #[serde(
        rename = "nextCursor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_cursor: Option<String>,
    #[serde(
        rename = "prevCursor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prev_cursor: Option<String>,
}

/// Empty response for delete operations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmptyResponse {}

/// Standard delete response shape: { id, deleted: true }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResponse {
    pub id: String,
    pub deleted: bool,
}

/// Response from removing a contact from an audience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudienceRemoveContactResponse {
    #[serde(rename = "audienceId")]
    pub audience_id: String,
    #[serde(rename = "contactId")]
    pub contact_id: String,
    pub removed: bool,
}

/// Response from removing a contact from a topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicRemoveContactResponse {
    #[serde(rename = "topicId")]
    pub topic_id: String,
    #[serde(rename = "contactId")]
    pub contact_id: String,
    pub removed: bool,
}
