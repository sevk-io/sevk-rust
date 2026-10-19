use crate::client::Client;
use crate::error::Error;
use crate::types::{OutboundQuota, UpdateOutboundQuotaRequest};

/// Outbound API resource. Read and adjust the project's monthly outbound
/// sending cap, and pause / resume transactional sending.
///
/// Pause affects `POST /emails`, `POST /emails/bulk`, and SMTP relay.
/// Broadcasts run on a separate balance check and are NOT affected.
pub struct Outbound<'a> {
    client: &'a Client,
}

impl<'a> Outbound<'a> {
    /// Binds outbound quota operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves outbound quota settings and current usage counters.
    pub async fn get_quota(&self) -> Result<OutboundQuota, Error> {
        self.client.get("/outbound-settings/quota").await
    }

    /// Changes the cap and clears a cap-exhausted pause when possible.
    pub async fn update_quota(
        &self,
        opts: UpdateOutboundQuotaRequest,
    ) -> Result<OutboundQuota, Error> {
        self.client.put("/outbound-settings/quota", &opts).await
    }

    /// Clears an outbound pause while preserving usage and the cap.
    pub async fn reset_quota(&self) -> Result<OutboundQuota, Error> {
        self.client
            .post("/outbound-settings/quota/reset", &serde_json::json!({}))
            .await
    }

    /// Resets custom-limit usage while preserving the cap and monthly usage.
    pub async fn reset_limit(&self) -> Result<OutboundQuota, Error> {
        self.client
            .post(
                "/outbound-settings/quota/reset-limit",
                &serde_json::json!({}),
            )
            .await
    }

    /// Stops transactional sending until `reset_quota` is called.
    pub async fn pause_quota(&self) -> Result<OutboundQuota, Error> {
        self.client
            .post("/outbound-settings/quota/pause", &serde_json::json!({}))
            .await
    }
}
