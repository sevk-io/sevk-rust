use super::{LogicalOperator, SegmentRuleOperator};
use serde::{Deserialize, Serialize};

/// Segment rule
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SegmentRule {
    pub field: String,
    pub operator: SegmentRuleOperator,
    pub value: serde_json::Value,
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(
        rename = "groupOperator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_operator: Option<LogicalOperator>,
}

/// Segment group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentGroup {
    pub id: String,
    pub operator: LogicalOperator,
}

/// Segment resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub id: String,
    pub name: String,
    pub rules: Vec<SegmentRule>,
    pub operator: LogicalOperator,
    #[serde(rename = "limitToTopics", default)]
    pub limit_to_topics: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<SegmentGroup>>,
    #[serde(rename = "audienceId")]
    pub audience_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "lastRunAt", default, skip_serializing_if = "Option::is_none")]
    pub last_run_at: Option<String>,
    #[serde(
        rename = "lastRunCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_run_count: Option<i64>,
}

/// Create segment request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSegmentRequest {
    pub name: String,
    pub rules: Vec<SegmentRule>,
    pub operator: LogicalOperator,
    #[serde(rename = "limitToTopics", skip_serializing_if = "Option::is_none")]
    pub limit_to_topics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<SegmentGroup>>,
}

/// Update segment request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSegmentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<SegmentRule>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<LogicalOperator>,
    #[serde(rename = "limitToTopics", skip_serializing_if = "Option::is_none")]
    pub limit_to_topics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<SegmentGroup>>,
}

/// Preview segment request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PreviewSegmentRequest {
    pub rules: Vec<SegmentRule>,
    pub operator: LogicalOperator,
    #[serde(rename = "limitToTopics", skip_serializing_if = "Option::is_none")]
    pub limit_to_topics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<SegmentGroup>>,
}

/// Contact row returned in a segment preview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewSegmentContact {
    pub id: String,
    pub email: String,
    pub subscribed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// Preview segment response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PreviewSegmentResponse {
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub contacts: Vec<PreviewSegmentContact>,
}

/// Segment calculate response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SegmentCalculateResponse {
    #[serde(default)]
    pub count: i64,
    #[serde(rename = "segmentId", default)]
    pub segment_id: String,
    #[serde(rename = "segmentName", default)]
    pub segment_name: String,
    #[serde(rename = "lastRunAt", default)]
    pub last_run_at: String,
}
