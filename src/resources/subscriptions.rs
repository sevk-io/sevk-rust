//! Subscriptions resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{EmptyResponse, SubscribeRequest, UnsubscribeRequest};

/// Subscriptions API resource
pub struct Subscriptions<'a> {
    client: &'a Client,
}

impl<'a> Subscriptions<'a> {
    /// Create a new Subscriptions resource
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Subscribe a contact to an audience
    pub async fn subscribe(&self, data: SubscribeRequest) -> Result<EmptyResponse, Error> {
        self.client.post("/subscriptions/subscribe", &data).await
    }

    /// Unsubscribe a contact
    pub async fn unsubscribe(&self, data: UnsubscribeRequest) -> Result<EmptyResponse, Error> {
        self.client.post("/subscriptions/unsubscribe", &data).await
    }
}
