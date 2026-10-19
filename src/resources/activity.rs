use crate::client::Client;
use crate::error::Error;
use crate::types::{ActivityItem, ActivityListParams, ActivityStats, PaginatedList};

/// Activity API resource
pub struct Activity<'a> {
    client: &'a Client,
}

impl<'a> Activity<'a> {
    /// Binds activity operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves project activity with pagination and optional filters.
    pub async fn list(
        &self,
        params: Option<ActivityListParams>,
    ) -> Result<PaginatedList<ActivityItem>, Error> {
        let params = params.unwrap_or_default();

        let query_params = params.to_query_params();

        self.client
            .get_with_params("/activity", &query_params)
            .await
    }

    /// Retrieves aggregate counts and rates for recent project activity.
    pub async fn stats(&self) -> Result<ActivityStats, Error> {
        self.client.get("/activity/stats").await
    }
}
