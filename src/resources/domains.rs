use crate::client::Client;
use crate::error::Error;
use crate::types::{
    CreateDomainRequest, DeleteResponse, DnsRecordsResponse, DomainListParams, DomainRead,
    DomainsListResponse, InboundVerifyResult, ItemsList, UpdateDomainRequest,
};

/// Domains API resource
pub struct Domains<'a> {
    client: &'a Client,
}

impl<'a> Domains<'a> {
    /// Binds domain operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves sending domains, optionally filtered by verification state.
    pub async fn list(
        &self,
        params: Option<DomainListParams>,
    ) -> Result<DomainsListResponse, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();

        self.client.get_with_params("/domains", &query_params).await
    }

    /// Retrieves a domain and its current verification state.
    pub async fn get(&self, id: &str) -> Result<DomainRead, Error> {
        self.client.get(&format!("/domains/{id}")).await
    }

    /// registers a domain; dns material requires `domain:dns-read`.
    pub async fn create(&self, params: CreateDomainRequest) -> Result<DomainRead, Error> {
        self.client.post("/domains", &params).await
    }

    /// Changes domain sending, tracking, TLS, or inbound settings.
    pub async fn update(&self, id: &str, data: UpdateDomainRequest) -> Result<DomainRead, Error> {
        self.client.put(&format!("/domains/{id}"), &data).await
    }

    /// Removes a domain after the API confirms it is safe to remove.
    pub async fn delete(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client.delete(&format!("/domains/{id}")).await
    }

    /// Rechecks sending DNS records and returns the updated state.
    pub async fn verify(&self, id: &str) -> Result<DomainRead, Error> {
        self.client
            .post(&format!("/domains/{id}/verify"), &serde_json::json!({}))
            .await
    }

    /// Checks inbound MX records and returns diagnostic details.
    pub async fn verify_inbound(&self, id: &str) -> Result<InboundVerifyResult, Error> {
        self.client
            .post(
                &format!("/domains/{id}/inbound/verify"),
                &serde_json::json!({}),
            )
            .await
    }

    /// Retrieves records required for sending and inbound mail.
    pub async fn dns_records(&self, id: &str) -> Result<DnsRecordsResponse, Error> {
        self.client.get(&format!("/domains/{id}/dns-records")).await
    }

    /// Retrieves region identifiers accepted when registering a domain.
    pub async fn regions(&self) -> Result<ItemsList<String>, Error> {
        self.client.get("/domains/regions").await
    }
}
