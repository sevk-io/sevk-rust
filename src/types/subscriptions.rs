use serde::{Deserialize, Serialize};

/// Subscribe request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscribeRequest {
    pub email: String,
    #[serde(rename = "audienceId", skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Map<String, serde_json::Value>>,
}

/// Unsubscribe request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnsubscribeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "contactId", skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(rename = "broadcastId", skip_serializing_if = "Option::is_none")]
    pub broadcast_id: Option<String>,
}

/// Contact returned from a subscribe call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeResponseContact {
    pub id: String,
    pub email: String,
    pub subscribed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Map<String, serde_json::Value>>,
}

/// Audience returned from a subscribe call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeResponseAudience {
    pub id: String,
    pub name: String,
}

/// Subscribe response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeResponse {
    pub contact: SubscribeResponseContact,
    #[serde(default)]
    pub audience: Option<SubscribeResponseAudience>,
    #[serde(
        rename = "alreadySubscribed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub already_subscribed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Contact returned from an unsubscribe call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeResponseContact {
    pub id: String,
    pub email: String,
    pub subscribed: bool,
}

/// Unsubscribe response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeResponse {
    pub contact: UnsubscribeResponseContact,
    #[serde(
        rename = "alreadyUnsubscribed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub already_unsubscribed: Option<bool>,
}
