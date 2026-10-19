use crate::client::Client;
use crate::error::Error;
use crate::types::{
    AddInboundBlocklistRequest, CreateInboundRouteRequest, DeleteResponse,
    InboundAttachmentDownload, InboundBlocklistEntry, InboundEmail, InboundListResponse,
    InboundQuota, InboundRoute, InboundSecurityConfig, InboundStats, ListInboundRequest,
    PaginatedList, PaginationParams, UpdateInboundQuotaRequest, UpdateInboundRouteRequest,
    UpdateInboundSecurityRequest,
};

/// Inbound API resource. Read incoming mail, manage per-project inbound
/// settings (routes, blocklist, quota, security), and download
/// attachments via short-lived URLs.
pub struct Inbound<'a> {
    client: &'a Client,
}

impl<'a> Inbound<'a> {
    /// Binds inbound email and settings operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves received email with optional pagination and filtering.
    pub async fn list(&self, opts: ListInboundRequest) -> Result<InboundListResponse, Error> {
        let query_params = opts.to_query_params();

        self.client.get_with_params("/inbound", &query_params).await
    }

    /// Retrieves a received email with parsed content and attachments.
    pub async fn get(&self, id: &str) -> Result<InboundEmail, Error> {
        self.client.get(&format!("/inbound/{id}")).await
    }

    /// Removes a received email and its stored attachments.
    pub async fn delete(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client.delete(&format!("/inbound/{id}")).await
    }

    /// Retrieves inbound processing statistics, optionally for one domain.
    pub async fn stats(&self, domain_id: Option<&str>) -> Result<InboundStats, Error> {
        match domain_id {
            Some(d) => {
                let params = vec![("domainId", d.to_string())];

                self.client.get_with_params("/inbound/stats", &params).await
            }
            None => self.client.get("/inbound/stats").await,
        }
    }

    /// Creates a short-lived URL for an inbound attachment.
    pub async fn download_attachment(
        &self,
        email_id: &str,
        attachment_id: &str,
    ) -> Result<InboundAttachmentDownload, Error> {
        self.client
            .get(&format!(
                "/inbound/{email_id}/attachments/{attachment_id}/download"
            ))
            .await
    }

    /// Retrieves rules used to store or forward matching inbound email.
    pub async fn list_routes(&self) -> Result<PaginatedList<InboundRoute>, Error> {
        self.list_routes_with_params(None).await
    }

    /// Retrieves routing rules using offset or cursor pagination.
    pub async fn list_routes_with_params(
        &self,
        params: Option<PaginationParams>,
    ) -> Result<PaginatedList<InboundRoute>, Error> {
        match params {
            Some(params) => {
                let query_params = params.to_query_params();

                self.client
                    .get_with_params("/inbound-settings/routes", &query_params)
                    .await
            }
            None => self.client.get("/inbound-settings/routes").await,
        }
    }

    /// Retrieves an inbound routing rule by ID.
    pub async fn get_route(&self, id: &str) -> Result<InboundRoute, Error> {
        self.client
            .get(&format!("/inbound-settings/routes/{id}"))
            .await
    }

    /// Adds an inbound routing rule.
    pub async fn create_route(
        &self,
        opts: CreateInboundRouteRequest,
    ) -> Result<InboundRoute, Error> {
        self.client.post("/inbound-settings/routes", &opts).await
    }

    /// Changes an inbound routing rule while preserving omitted fields.
    pub async fn update_route(
        &self,
        id: &str,
        opts: UpdateInboundRouteRequest,
    ) -> Result<InboundRoute, Error> {
        self.client
            .put(&format!("/inbound-settings/routes/{id}"), &opts)
            .await
    }

    /// Removes a routing rule without deleting stored email.
    pub async fn delete_route(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client
            .delete(&format!("/inbound-settings/routes/{id}"))
            .await
    }

    /// Retrieves addresses blocked from inbound processing.
    pub async fn list_blocklist(&self) -> Result<PaginatedList<InboundBlocklistEntry>, Error> {
        self.list_blocklist_with_params(None).await
    }

    /// Retrieves blocked addresses using offset or cursor pagination.
    pub async fn list_blocklist_with_params(
        &self,
        params: Option<PaginationParams>,
    ) -> Result<PaginatedList<InboundBlocklistEntry>, Error> {
        match params {
            Some(params) => {
                let query_params = params.to_query_params();

                self.client
                    .get_with_params("/inbound-settings/blocklist", &query_params)
                    .await
            }
            None => self.client.get("/inbound-settings/blocklist").await,
        }
    }

    /// Blocks an address from inbound processing.
    pub async fn add_blocklist_entry(
        &self,
        opts: AddInboundBlocklistRequest,
    ) -> Result<InboundBlocklistEntry, Error> {
        self.client.post("/inbound-settings/blocklist", &opts).await
    }

    /// Removes an address from the inbound blocklist.
    pub async fn remove_blocklist_entry(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client
            .delete(&format!("/inbound-settings/blocklist/{id}"))
            .await
    }

    /// Retrieves inbound quota settings and current usage counters.
    pub async fn get_quota(&self) -> Result<InboundQuota, Error> {
        self.client.get("/inbound-settings/quota").await
    }

    /// Changes the inbound monthly limit while preserving omitted fields.
    pub async fn update_quota(
        &self,
        opts: UpdateInboundQuotaRequest,
    ) -> Result<InboundQuota, Error> {
        self.client.put("/inbound-settings/quota", &opts).await
    }

    /// Clears a quota pause without resetting current-period usage.
    pub async fn reset_quota(&self) -> Result<InboundQuota, Error> {
        self.client
            .post("/inbound-settings/quota/reset", &serde_json::json!({}))
            .await
    }

    /// Retrieves effective inbound security settings for a domain.
    pub async fn get_security_config(
        &self,
        domain_id: &str,
    ) -> Result<InboundSecurityConfig, Error> {
        self.client
            .get(&format!("/inbound-settings/security/{domain_id}"))
            .await
    }

    /// Changes inbound security settings for a domain.
    pub async fn update_security_config(
        &self,
        domain_id: &str,
        opts: UpdateInboundSecurityRequest,
    ) -> Result<InboundSecurityConfig, Error> {
        self.client
            .put(&format!("/inbound-settings/security/{domain_id}"), &opts)
            .await
    }
}
