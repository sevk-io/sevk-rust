//! Domains resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{DomainListParams, DomainsListResponse};

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
}
