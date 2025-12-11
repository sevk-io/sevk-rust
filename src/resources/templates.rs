//! Templates resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{
    CreateTemplateRequest, EmptyResponse, ListParams, PaginatedList, Template, UpdateTemplateRequest,
};

/// Templates API resource
pub struct Templates<'a> {
    client: &'a Client,
}

impl<'a> Templates<'a> {
    /// Create a new Templates resource
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all templates
    pub async fn list(
        &self,
        params: Option<ListParams>,
    ) -> Result<PaginatedList<Template>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();
        self.client
            .get_with_params("/templates", &query_params)
            .await
    }

    /// Get a template by ID
    pub async fn get(&self, id: &str) -> Result<Template, Error> {
        self.client.get(&format!("/templates/{}", id)).await
    }

    /// Create a new template
    pub async fn create(&self, data: CreateTemplateRequest) -> Result<Template, Error> {
        self.client.post("/templates", &data).await
    }

    /// Update a template
    pub async fn update(&self, id: &str, data: UpdateTemplateRequest) -> Result<Template, Error> {
        self.client.put(&format!("/templates/{}", id), &data).await
    }

    /// Delete a template
    pub async fn delete(&self, id: &str) -> Result<EmptyResponse, Error> {
        self.client.delete(&format!("/templates/{}", id)).await
    }

    /// Duplicate a template
    pub async fn duplicate(&self, id: &str) -> Result<Template, Error> {
        self.client
            .post(&format!("/templates/{}/duplicate", id), &serde_json::json!({}))
            .await
    }
}
