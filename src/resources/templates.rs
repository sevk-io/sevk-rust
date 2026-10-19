use crate::client::Client;
use crate::error::Error;
use crate::types::{
    CreateTemplateRequest, DeleteResponse, ListParams, PaginatedList, Template,
    UpdateTemplateRequest,
};

/// Templates API resource
pub struct Templates<'a> {
    client: &'a Client,
}

impl<'a> Templates<'a> {
    /// Binds template operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves reusable email templates with pagination and search.
    pub async fn list(&self, params: Option<ListParams>) -> Result<PaginatedList<Template>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();

        self.client
            .get_with_params("/templates", &query_params)
            .await
    }

    /// Retrieves a template by ID.
    pub async fn get(&self, id: &str) -> Result<Template, Error> {
        self.client.get(&format!("/templates/{id}")).await
    }

    /// Adds a reusable email template.
    pub async fn create(&self, data: CreateTemplateRequest) -> Result<Template, Error> {
        self.client.post("/templates", &data).await
    }

    /// Changes a template while preserving omitted fields.
    pub async fn update(&self, id: &str, data: UpdateTemplateRequest) -> Result<Template, Error> {
        self.client.put(&format!("/templates/{id}"), &data).await
    }

    /// Removes a template without affecting email already sent from it.
    pub async fn delete(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client.delete(&format!("/templates/{id}")).await
    }

    /// Creates an independent template copy for further editing.
    pub async fn duplicate(&self, id: &str) -> Result<Template, Error> {
        self.client
            .post(
                &format!("/templates/{id}/duplicate"),
                &serde_json::json!({}),
            )
            .await
    }
}
