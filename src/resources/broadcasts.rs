//! Broadcasts resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{Broadcast, ListParams, PaginatedList};

/// Broadcasts API resource
pub struct Broadcasts<'a> {
    client: &'a Client,
}

impl<'a> Broadcasts<'a> {
    /// Create a new Broadcasts resource
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all broadcasts
    pub async fn list(
        &self,
        params: Option<ListParams>,
    ) -> Result<PaginatedList<Broadcast>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();
        self.client
            .get_with_params("/broadcasts", &query_params)
            .await
    }

    /// Get a broadcast by ID
    pub async fn get(&self, id: &str) -> Result<Broadcast, Error> {
        self.client.get(&format!("/broadcasts/{}", id)).await
    }
}
