//! Broadcasts resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{
    Broadcast, BroadcastAnalytics, BroadcastCostEstimate, BroadcastEmail, BroadcastStatus,
    CreateBroadcastRequest, EmptyResponse, ItemsList, ListParams, PaginatedList, SendTestRequest,
    UpdateBroadcastRequest,
};

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

    /// Create a new broadcast
    pub async fn create(&self, data: CreateBroadcastRequest) -> Result<Broadcast, Error> {
        self.client.post("/broadcasts", &data).await
    }

    /// Update a broadcast
    pub async fn update(
        &self,
        id: &str,
        data: UpdateBroadcastRequest,
    ) -> Result<Broadcast, Error> {
        self.client
            .put(&format!("/broadcasts/{}", id), &data)
            .await
    }

    /// Delete a broadcast
    pub async fn delete(&self, id: &str) -> Result<EmptyResponse, Error> {
        self.client.delete(&format!("/broadcasts/{}", id)).await
    }

    /// Send a broadcast
    pub async fn send(&self, id: &str) -> Result<Broadcast, Error> {
        self.client
            .post(&format!("/broadcasts/{}/send", id), &serde_json::json!({}))
            .await
    }

    /// Cancel a scheduled broadcast
    pub async fn cancel(&self, id: &str) -> Result<Broadcast, Error> {
        self.client
            .post(
                &format!("/broadcasts/{}/cancel", id),
                &serde_json::json!({}),
            )
            .await
    }

    /// Send a test email for a broadcast
    pub async fn send_test(
        &self,
        id: &str,
        emails: Vec<String>,
    ) -> Result<EmptyResponse, Error> {
        let request = SendTestRequest { emails };
        self.client
            .post(&format!("/broadcasts/{}/test", id), &request)
            .await
    }

    /// Get broadcast analytics
    pub async fn get_analytics(&self, id: &str) -> Result<BroadcastAnalytics, Error> {
        self.client
            .get(&format!("/broadcasts/{}/analytics", id))
            .await
    }

    /// Get broadcast status
    pub async fn get_status(&self, id: &str) -> Result<BroadcastStatus, Error> {
        self.client
            .get(&format!("/broadcasts/{}/status", id))
            .await
    }

    /// Get broadcast emails
    pub async fn get_emails(
        &self,
        id: &str,
        params: Option<ListParams>,
    ) -> Result<PaginatedList<BroadcastEmail>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();
        self.client
            .get_with_params(&format!("/broadcasts/{}/emails", id), &query_params)
            .await
    }

    /// Estimate broadcast cost
    pub async fn estimate_cost(&self, id: &str) -> Result<BroadcastCostEstimate, Error> {
        self.client
            .get(&format!("/broadcasts/{}/estimate-cost", id))
            .await
    }

    /// List active broadcasts
    pub async fn list_active(&self) -> Result<ItemsList<Broadcast>, Error> {
        self.client.get("/broadcasts/active").await
    }
}
