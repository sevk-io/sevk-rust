use super::params::pagination_query;
use super::{ActivityAction, ActivityCategory, ActivityType};
use serde::{Deserialize, Serialize};

/// Activity item resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityItem {
    pub id: String,
    #[serde(rename = "eventType")]
    pub event_type: ActivityType,
    pub action: ActivityAction,
    pub description: Option<String>,
    #[serde(rename = "emailId", default)]
    pub email_id: Option<String>,
    #[serde(rename = "emailSubject", default)]
    pub email_subject: Option<String>,
    #[serde(rename = "contactId", default)]
    pub contact_id: Option<String>,
    #[serde(rename = "contactEmail", default)]
    pub contact_email: Option<String>,
    #[serde(rename = "broadcastId", default)]
    pub broadcast_id: Option<String>,
    #[serde(rename = "broadcastName", default)]
    pub broadcast_name: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(rename = "userAgent", default)]
    pub user_agent: Option<String>,
    #[serde(default)]
    pub metadata: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// Activity list parameters.
///
/// Pass either `page` (offset mode) or `after` / `before` (cursor mode), not both.
/// See the Pagination doc for the response shape per mode.
#[derive(Debug, Clone, Default)]
pub struct ActivityListParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    /// Forward cursor: id of the last item from the previous page. Mutually exclusive with `page`.
    pub after: Option<String>,
    /// Backward cursor: id of the first item from the next page. Walks results in reverse.
    /// Mutually exclusive with `page`.
    pub before: Option<String>,
    pub category: Option<ActivityCategory>,
    pub event_type: Option<Vec<ActivityAction>>,
    pub days: Option<i64>,
    pub search: Option<String>,
}

impl ActivityListParams {
    pub fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = pagination_query(
            self.page,
            self.limit,
            self.after.as_deref(),
            self.before.as_deref(),
        );
        if let Some(ref category) = self.category {
            params.push(("category", category.as_str().to_string()));
        }
        if let Some(ref event_type) = self.event_type {
            params.push((
                "type",
                event_type
                    .iter()
                    .map(ActivityAction::as_str)
                    .collect::<Vec<_>>()
                    .join(","),
            ));
        }
        if let Some(days) = self.days {
            params.push(("days", days.to_string()));
        }
        if let Some(ref search) = self.search {
            params.push(("search", search.clone()));
        }
        params
    }
}

/// Activity stats response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActivityStatsRates {
    #[serde(default)]
    pub open: f64,
    #[serde(default)]
    pub click: f64,
    #[serde(default)]
    pub bounce: f64,
    #[serde(default)]
    pub delivery: f64,
}

/// Activity stats response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActivityStats {
    #[serde(rename = "totalEmails", default)]
    pub total_emails: i64,
    #[serde(rename = "sentEmails", default)]
    pub sent_emails: i64,
    #[serde(rename = "deliveredEmails", default)]
    pub delivered_emails: i64,
    #[serde(rename = "openedEmails", default)]
    pub opened_emails: i64,
    #[serde(rename = "clickedEmails", default)]
    pub clicked_emails: i64,
    #[serde(rename = "bouncedEmails", default)]
    pub bounced_emails: i64,
    #[serde(rename = "marketingEmails", default)]
    pub marketing_emails: i64,
    #[serde(rename = "transactionalEmails", default)]
    pub transactional_emails: i64,
    #[serde(default)]
    pub rates: ActivityStatsRates,
}
