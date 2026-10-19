use crate::client::Client;
use crate::error::Error;
use crate::types::{SubscribeRequest, SubscribeResponse, UnsubscribeRequest, UnsubscribeResponse};

/// Subscriptions API resource
pub struct Subscriptions<'a> {
    client: &'a Client,
}

impl<'a> Subscriptions<'a> {
    /// Binds subscription operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Opts in a contact and can attach it to an audience.
    pub async fn subscribe(&self, data: SubscribeRequest) -> Result<SubscribeResponse, Error> {
        self.client.post("/subscriptions/subscribe", &data).await
    }

    /// Globally opts out a contact and can attribute the action.
    pub async fn unsubscribe(
        &self,
        data: UnsubscribeRequest,
    ) -> Result<UnsubscribeResponse, Error> {
        self.client.post("/subscriptions/unsubscribe", &data).await
    }
}
