//! Emails resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{SendEmailRequest, SendEmailResponse};

/// Emails API resource
pub struct Emails<'a> {
    client: &'a Client,
}

impl<'a> Emails<'a> {
    /// Create a new Emails resource
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Send an email
    pub async fn send(&self, data: SendEmailRequest) -> Result<SendEmailResponse, Error> {
        self.client.post("/emails", &data).await
    }
}
