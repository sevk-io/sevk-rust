//! Emails resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{BulkEmailRequest, BulkEmailResponse, EmailDetail, SendEmailRequest, SendEmailResponse};

/// Emails API resource
pub struct Emails<'a> {
    client: &'a Client,
}

impl<'a> Emails<'a> {
    /// Create a new Emails resource
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Send an email with optional attachments
    pub async fn send(&self, data: SendEmailRequest) -> Result<SendEmailResponse, Error> {
        self.client.post("/emails", &data).await
    }

    /// Send multiple emails in bulk (max 100)
    pub async fn send_bulk(&self, data: BulkEmailRequest) -> Result<BulkEmailResponse, Error> {
        self.client.post("/emails/bulk", &data).await
    }

    /// Get email details by ID
    pub async fn get(&self, id: &str) -> Result<EmailDetail, Error> {
        self.client.get(&format!("/emails/{}", id)).await
    }
}
