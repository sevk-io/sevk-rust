use super::{DnsRecordType, PaginatedList, TlsPolicy};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// non-dns domain metadata returned by list operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainSummary {
    #[serde(rename = "inboundPrefix")]
    pub inbound_prefix: String,
    pub id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    pub domain: String,
    pub email: String,
    #[serde(default, rename = "senderName")]
    pub sender_name: Option<String>,
    pub region: String,
    pub verified: bool,
    #[serde(rename = "clickTracking")]
    pub click_tracking: bool,
    #[serde(rename = "openTracking")]
    pub open_tracking: bool,
    #[serde(rename = "tlsPolicy", default, skip_serializing_if = "Option::is_none")]
    pub tls_policy: Option<TlsPolicy>,
    #[serde(
        rename = "inboundEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_enabled: Option<bool>,
    #[serde(
        rename = "inboundVerified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_verified: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// domain metadata returned by `get`. dns material is present only when the
/// caller has `domain:dns-read`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainRead {
    #[serde(rename = "inboundPrefix")]
    pub inbound_prefix: String,
    pub id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    pub domain: String,
    pub email: String,
    #[serde(default, rename = "senderName")]
    pub sender_name: Option<String>,
    pub region: String,
    pub verified: bool,
    #[serde(
        rename = "verificationToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_token: Option<String>,
    #[serde(
        rename = "dkimTokens",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dkim_tokens: Option<Vec<String>>,
    #[serde(
        rename = "dnsRecords",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_records: Option<Vec<DnsRecord>>,
    #[serde(rename = "clickTracking")]
    pub click_tracking: bool,
    #[serde(rename = "openTracking")]
    pub open_tracking: bool,
    #[serde(rename = "tlsPolicy", default, skip_serializing_if = "Option::is_none")]
    pub tls_policy: Option<TlsPolicy>,
    #[serde(
        rename = "inboundEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_enabled: Option<bool>,
    #[serde(
        rename = "inboundVerified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_verified: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

/// A single MX record resolved when verifying an inbound domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundMxRecord {
    /// MX target hostname.
    pub exchange: String,
    /// MX preference value.
    pub priority: i32,
}

/// capability-aware domain record plus diagnostics returned by inbound verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundVerifyResult {
    #[serde(flatten)]
    pub domain: DomainRead,
    #[serde(rename = "inboundMxResolved", default)]
    pub inbound_mx_resolved: Vec<InboundMxRecord>,
    #[serde(rename = "inboundExpectedTarget", default)]
    pub inbound_expected_target: String,
}

impl Deref for InboundVerifyResult {
    type Target = DomainRead;
    fn deref(&self) -> &Self::Target {
        &self.domain
    }
}

pub type DomainsListResponse = PaginatedList<DomainSummary>;

/// Create domain request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDomainRequest {
    pub domain: String,
    pub email: String,
    #[serde(rename = "senderName", skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// DNS record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    #[serde(rename = "type")]
    pub record_type: DnsRecordType,
    pub name: String,
    pub fqdn: String,
    pub value: String,
}

/// DNS records response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecordsResponse {
    pub items: Vec<DnsRecord>,
}

/// Update domain request
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateDomainRequest {
    #[serde(rename = "inboundPrefix", skip_serializing_if = "Option::is_none")]
    pub inbound_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "senderName", skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[serde(rename = "clickTracking", skip_serializing_if = "Option::is_none")]
    pub click_tracking: Option<bool>,
    #[serde(rename = "openTracking", skip_serializing_if = "Option::is_none")]
    pub open_tracking: Option<bool>,
    #[serde(rename = "tlsPolicy", skip_serializing_if = "Option::is_none")]
    pub tls_policy: Option<TlsPolicy>,
    #[serde(rename = "inboundEnabled", skip_serializing_if = "Option::is_none")]
    pub inbound_enabled: Option<bool>,
}
