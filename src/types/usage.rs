use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FreeEmailsBucket {
    #[serde(default)]
    pub current: i64,
    #[serde(default)]
    pub limit: Option<i64>,
    #[serde(default)]
    pub remaining: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InboundUsage {
    #[serde(rename = "emailsCount", default)]
    pub emails_count: i64,
    #[serde(rename = "monthlyEmailLimit", default)]
    pub monthly_email_limit: Option<i64>,
    #[serde(rename = "monthlyEmailsUsed", default)]
    pub monthly_emails_used: i64,
    #[serde(rename = "monthlyRemaining", default)]
    pub monthly_remaining: Option<i64>,
    #[serde(rename = "freeEmails")]
    pub free_emails: FreeEmailsBucket,
    #[serde(rename = "resetAt", default)]
    pub reset_at: Option<String>,
    #[serde(rename = "pausedAt", default)]
    pub paused_at: Option<String>,
    #[serde(rename = "pausedReason", default)]
    pub paused_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutboundUsage {
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
    #[serde(rename = "monthlyRemaining", default)]
    pub monthly_remaining: Option<i64>,
    #[serde(rename = "rateLimit", default)]
    pub rate_limit: Option<i64>,
    #[serde(rename = "freeEmails")]
    pub free_emails: FreeEmailsBucket,
    #[serde(rename = "resetAt", default)]
    pub reset_at: Option<String>,
    #[serde(rename = "pausedAt", default)]
    pub paused_at: Option<String>,
    #[serde(rename = "pausedReason", default)]
    pub paused_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUsage {
    pub id: String,
    pub name: String,
    #[serde(rename = "audienceLimit", default)]
    pub audience_limit: Option<i64>,
    #[serde(rename = "contactLimit", default)]
    pub contact_limit: Option<i64>,
    #[serde(rename = "broadcastLimit", default)]
    pub broadcast_limit: Option<i64>,
    #[serde(rename = "domainLimit", default)]
    pub domain_limit: Option<i64>,
    #[serde(rename = "webhookLimit", default)]
    pub webhook_limit: Option<i64>,
    #[serde(rename = "storageLimit", default)]
    pub storage_limit: Option<String>,
    #[serde(rename = "storageUsed", default)]
    pub storage_used: String,
    #[serde(default)]
    pub balance: Option<String>,
    #[serde(rename = "outboundEmailPrice", default)]
    pub outbound_email_price: Option<String>,
    #[serde(rename = "inboundEmailPrice", default)]
    pub inbound_email_price: Option<String>,
    #[serde(rename = "monthlyOutboundEmailLimit", default)]
    pub monthly_outbound_email_limit: Option<i64>,
    #[serde(rename = "outboundRateLimit", default)]
    pub outbound_rate_limit: Option<i64>,
    #[serde(rename = "freeMonthlyOutboundEmails")]
    pub free_monthly_outbound_emails: Option<i64>,
    #[serde(rename = "freeMonthlyInboundEmails")]
    pub free_monthly_inbound_emails: Option<i64>,
    #[serde(rename = "audiencesCount", default)]
    pub audiences_count: i64,
    #[serde(rename = "contactsCount", default)]
    pub contacts_count: i64,
    #[serde(rename = "broadcastsCount", default)]
    pub broadcasts_count: i64,
    #[serde(rename = "monthlyBroadcastsUsed", default)]
    pub monthly_broadcasts_used: i64,
    #[serde(rename = "domainsCount", default)]
    pub domains_count: i64,
    #[serde(rename = "webhooksCount", default)]
    pub webhooks_count: i64,
    #[serde(rename = "inboundEmailsCount", default)]
    pub inbound_emails_count: i64,
    #[serde(default)]
    pub inbound: InboundUsage,
    #[serde(default)]
    pub outbound: OutboundUsage,
}
