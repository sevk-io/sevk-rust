//! Events resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{Event, EventListParams, EventStats, PaginatedList};

/// Events API resource
pub struct Events<'a> {
    client: &'a Client,
}

impl<'a> Events<'a> {
    /// Create a new Events resource
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List events with optional filters
    pub async fn list(
        &self,
        params: Option<EventListParams>,
    ) -> Result<PaginatedList<Event>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();
        self.client
            .get_with_params("/events", &query_params)
            .await
    }

    /// Get event stats
    pub async fn stats(&self) -> Result<EventStats, Error> {
        self.client.get("/events/stats").await
    }
}
