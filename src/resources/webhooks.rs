use crate::client::Client;
use crate::error::Error;
use crate::types::{
    CreateWebhookRequest, CreatedWebhook, DeleteResponse, PaginationParams, RotateWebhookResponse,
    UpdateWebhookRequest, Webhook, WebhookEventsResponse, WebhookListResponse, WebhookTestResponse,
};

/// Webhooks API resource
pub struct Webhooks<'a> {
    client: &'a Client,
}

impl<'a> Webhooks<'a> {
    pub async fn rotate(&self, id: &str) -> Result<RotateWebhookResponse, Error> {
        self.client
            .post(&format!("/webhooks/{id}/rotate"), &serde_json::json!({}))
            .await
    }

    /// Binds webhook operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves configured webhooks and their recent delivery health.
    pub async fn list(
        &self,
        params: Option<PaginationParams>,
    ) -> Result<WebhookListResponse, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();

        self.client
            .get_with_params("/webhooks", &query_params)
            .await
    }

    /// Retrieves a webhook and its recent delivery state.
    pub async fn get(&self, id: &str) -> Result<Webhook, Error> {
        self.client.get(&format!("/webhooks/{id}")).await
    }

    /// Adds a webhook and returns its usable `whsec_` signing secret exactly once.
    pub async fn create(&self, data: CreateWebhookRequest) -> Result<CreatedWebhook, Error> {
        self.client.post("/webhooks", &data).await
    }

    /// Changes a webhook while preserving omitted fields.
    pub async fn update(&self, id: &str, data: UpdateWebhookRequest) -> Result<Webhook, Error> {
        self.client.put(&format!("/webhooks/{id}"), &data).await
    }

    /// Removes a webhook and stops future deliveries to its URL.
    pub async fn delete(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client.delete(&format!("/webhooks/{id}")).await
    }

    /// Sends a test event and returns the delivery result.
    pub async fn test(&self, id: &str) -> Result<WebhookTestResponse, Error> {
        self.client
            .post(&format!("/webhooks/{id}/test"), &serde_json::json!({}))
            .await
    }

    /// Retrieves event names that can be assigned to a webhook.
    pub async fn events(&self) -> Result<WebhookEventsResponse, Error> {
        self.client.get("/webhooks/events").await
    }
}
