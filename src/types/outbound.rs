use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundQuota {
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "monthlyEmailLimit", default)]
    pub monthly_email_limit: Option<i64>,
    #[serde(rename = "adminMonthlyEmailLimit", default)]
    pub admin_monthly_email_limit: Option<i64>,
    #[serde(rename = "effectiveMonthlyEmailLimit", default)]
    pub effective_monthly_email_limit: Option<i64>,
    #[serde(rename = "monthlyEmailsUsed", default)]
    pub monthly_emails_used: i64,
    #[serde(rename = "limitEmailsUsed")]
    pub limit_emails_used: i64,
    #[serde(rename = "monthlyRemaining")]
    pub monthly_remaining: Option<i64>,
    #[serde(rename = "freeMonthlyEmails")]
    pub free_monthly_emails: Option<i64>,
    #[serde(rename = "freeEmailsUsed")]
    pub free_emails_used: i64,
    #[serde(rename = "rateLimit", default)]
    pub rate_limit: Option<i64>,
    #[serde(rename = "emailPrice")]
    pub email_price: f64,
    #[serde(rename = "resetAt")]
    pub reset_at: String,
    #[serde(rename = "pausedAt", default)]
    pub paused_at: Option<String>,
    #[serde(rename = "pausedReason", default)]
    pub paused_reason: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateOutboundQuotaRequest {
    #[serde(rename = "monthlyEmailLimit")]
    pub monthly_email_limit: Option<i64>,
}
