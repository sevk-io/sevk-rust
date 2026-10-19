use crate::client::Client;
use crate::error::Error;
use crate::types::ProjectUsage;

/// Usage API resource. Read the project's plan caps, balance, outbound
/// price, current resource counts, and email send-limit stats.
pub struct Usage<'a> {
    client: &'a Client,
}

impl<'a> Usage<'a> {
    /// Binds usage operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves current limits, resource counts, and email usage.
    pub async fn get(&self) -> Result<ProjectUsage, Error> {
        self.client.get("/limits").await
    }
}
