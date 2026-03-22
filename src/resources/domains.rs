//! Domains resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{
    CreateDomainRequest, DnsRecordsResponse, Domain, DomainListParams, DomainsListResponse,
    EmptyResponse, ItemsList, UpdateDomainRequest,
};

/// Domains API resource
pub struct Domains<'a> {
    client: &'a Client,
}

impl<'a> Domains<'a> {
    /// Create a new Domains resource
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all domains
    pub async fn list(&self, params: Option<DomainListParams>) -> Result<DomainsListResponse, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();
        self.client
            .get_with_params("/domains", &query_params)
            .await
    }

    /// Get a domain by ID
    pub async fn get(&self, id: &str) -> Result<Domain, Error> {
        self.client.get(&format!("/domains/{}", id)).await
    }

    /// Create a new domain
    pub async fn create(&self, params: CreateDomainRequest) -> Result<Domain, Error> {
        self.client.post("/domains", &params).await
    }

    /// Update a domain by ID
    pub async fn update(&self, id: &str, data: UpdateDomainRequest) -> Result<Domain, Error> {
        self.client
            .put(&format!("/domains/{}", id), &data)
            .await
    }

    /// Delete a domain by ID
    pub async fn delete(&self, id: &str) -> Result<EmptyResponse, Error> {
        self.client.delete(&format!("/domains/{}", id)).await
    }

    /// Trigger domain verification
    pub async fn verify(&self, id: &str) -> Result<Domain, Error> {
        self.client
            .post(&format!("/domains/{}/verify", id), &serde_json::json!({}))
            .await
    }

    /// Get DNS records for a domain
    pub async fn get_dns_records(&self, id: &str) -> Result<DnsRecordsResponse, Error> {
        self.client
            .get(&format!("/domains/{}/dns-records", id))
            .await
    }

    /// Get available regions
    pub async fn get_regions(&self) -> Result<ItemsList<String>, Error> {
        self.client.get("/domains/regions").await
    }
}
