//! Webhooks resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{
    CreateWebhookRequest, EmptyResponse, ItemsList, UpdateWebhookRequest,
    Webhook, WebhookTestResponse,
};

/// Webhooks API resource
pub struct Webhooks<'a> {
    client: &'a Client,
}

impl<'a> Webhooks<'a> {
    /// Create a new Webhooks resource
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all webhooks
    pub async fn list(&self) -> Result<ItemsList<Webhook>, Error> {
        self.client.get("/webhooks").await
    }

    /// Get a webhook by ID
    pub async fn get(&self, id: &str) -> Result<Webhook, Error> {
        self.client.get(&format!("/webhooks/{}", id)).await
    }

    /// Create a new webhook
    pub async fn create(&self, data: CreateWebhookRequest) -> Result<Webhook, Error> {
        self.client.post("/webhooks", &data).await
    }

    /// Update a webhook
    pub async fn update(
        &self,
        id: &str,
        data: UpdateWebhookRequest,
    ) -> Result<Webhook, Error> {
        self.client
            .put(&format!("/webhooks/{}", id), &data)
            .await
    }

    /// Delete a webhook
    pub async fn delete(&self, id: &str) -> Result<EmptyResponse, Error> {
        self.client.delete(&format!("/webhooks/{}", id)).await
    }

    /// Test a webhook
    pub async fn test(&self, id: &str) -> Result<WebhookTestResponse, Error> {
        self.client
            .post(&format!("/webhooks/{}/test", id), &serde_json::json!({}))
            .await
    }

    /// List available webhook event types
    pub async fn list_events(&self) -> Result<serde_json::Value, Error> {
        self.client.get("/webhooks/events").await
    }
}
