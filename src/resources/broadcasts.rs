use crate::client::Client;
use crate::error::Error;
use crate::types::{
    ActiveBroadcastListResponse, Broadcast, BroadcastAnalytics, BroadcastCancelResponse,
    BroadcastCostEstimate, BroadcastEmail, BroadcastRecipientEstimate,
    BroadcastRecipientEstimateRequest, BroadcastStatus, BroadcastSummary, BroadcastUpdate,
    BroadcastWithVariants, CreateBroadcastRequest, DeleteResponse, ListBroadcastEmailsParams,
    ListBroadcastsParams, PaginatedList, SendTestRequest, SendTestResponse, UpdateBroadcastRequest,
};

/// Broadcasts API resource
pub struct Broadcasts<'a> {
    client: &'a Client,
}

/// Per-call options for a retry-safe broadcast test send.
#[derive(Debug, Default, Clone)]
pub struct BroadcastTestOptions {
    pub idempotency_key: Option<String>,
}

impl<'a> Broadcasts<'a> {
    /// Binds broadcast operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves broadcasts with pagination, search, and status filters.
    pub async fn list(
        &self,
        params: Option<ListBroadcastsParams>,
    ) -> Result<PaginatedList<BroadcastSummary>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();

        self.client
            .get_with_params("/broadcasts", &query_params)
            .await
    }

    /// Retrieves a broadcast by ID with its current delivery state.
    pub async fn get(&self, id: &str) -> Result<Broadcast, Error> {
        self.client.get(&format!("/broadcasts/{id}")).await
    }

    /// Adds a draft or scheduled broadcast for a target group.
    pub async fn create(
        &self,
        data: CreateBroadcastRequest,
    ) -> Result<BroadcastWithVariants, Error> {
        self.client.post("/broadcasts", &data).await
    }

    /// Changes a broadcast while preserving omitted fields.
    pub async fn update(
        &self,
        id: &str,
        data: UpdateBroadcastRequest,
    ) -> Result<BroadcastUpdate, Error> {
        self.client.put(&format!("/broadcasts/{id}"), &data).await
    }

    /// Removes a broadcast that has not been sent.
    pub async fn delete(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client.delete(&format!("/broadcasts/{id}")).await
    }

    /// Queues a prepared broadcast for delivery.
    pub async fn send(&self, id: &str) -> Result<BroadcastSummary, Error> {
        self.client
            .post(&format!("/broadcasts/{id}/send"), &serde_json::json!({}))
            .await
    }

    /// Stops a broadcast when its state still allows cancellation.
    pub async fn cancel(&self, id: &str) -> Result<BroadcastCancelResponse, Error> {
        self.client
            .post(&format!("/broadcasts/{id}/cancel"), &serde_json::json!({}))
            .await
    }

    /// Delivers a broadcast to test recipients without starting the campaign.
    pub async fn test(&self, id: &str, emails: Vec<String>) -> Result<SendTestResponse, Error> {
        self.test_with_options(id, emails, &BroadcastTestOptions::default())
            .await
    }

    /// Delivers a test broadcast with optional retry idempotency.
    pub async fn test_with_options(
        &self,
        id: &str,
        emails: Vec<String>,
        options: &BroadcastTestOptions,
    ) -> Result<SendTestResponse, Error> {
        let request = SendTestRequest { emails };

        match options.idempotency_key.as_deref() {
            Some(key) => {
                self.client
                    .post_with_headers(
                        &format!("/broadcasts/{id}/test"),
                        &request,
                        &[("Idempotency-Key", key)],
                    )
                    .await
            }
            None => {
                self.client
                    .post(&format!("/broadcasts/{id}/test"), &request)
                    .await
            }
        }
    }

    /// Retrieves aggregate delivery and engagement metrics.
    pub async fn analytics(&self, id: &str) -> Result<BroadcastAnalytics, Error> {
        self.client
            .get(&format!("/broadcasts/{id}/analytics"))
            .await
    }

    /// Retrieves live scheduling and delivery progress.
    pub async fn status(&self, id: &str) -> Result<BroadcastStatus, Error> {
        self.client.get(&format!("/broadcasts/{id}/status")).await
    }

    /// Retrieves email deliveries created for a broadcast.
    pub async fn emails(
        &self,
        id: &str,
        params: Option<ListBroadcastEmailsParams>,
    ) -> Result<PaginatedList<BroadcastEmail>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();

        self.client
            .get_with_params(&format!("/broadcasts/{id}/emails"), &query_params)
            .await
    }

    /// Calculates delivery cost without sending the broadcast.
    pub async fn estimate_cost(&self, id: &str) -> Result<BroadcastCostEstimate, Error> {
        self.client
            .get(&format!("/broadcasts/{id}/estimate-cost"))
            .await
    }

    /// Resolves the subscribed recipient count for a broadcast target.
    pub async fn estimate_recipients(
        &self,
        request: &BroadcastRecipientEstimateRequest,
    ) -> Result<BroadcastRecipientEstimate, Error> {
        self.client
            .post("/broadcasts/estimate-recipients", request)
            .await
    }

    /// Retrieves broadcasts that are currently scheduled or processing.
    pub async fn active(&self) -> Result<ActiveBroadcastListResponse, Error> {
        self.client.get("/broadcasts/active").await
    }
}
