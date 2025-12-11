//! Audiences resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{
    AddContactsRequest, Audience, CreateAudienceRequest, EmptyResponse, ListParams, PaginatedList,
    UpdateAudienceRequest,
};

/// Audiences API resource
pub struct Audiences<'a> {
    client: &'a Client,
}

impl<'a> Audiences<'a> {
    /// Create a new Audiences resource
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all audiences
    pub async fn list(
        &self,
        params: Option<ListParams>,
    ) -> Result<PaginatedList<Audience>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();
        self.client
            .get_with_params("/audiences", &query_params)
            .await
    }

    /// Get an audience by ID
    pub async fn get(&self, id: &str) -> Result<Audience, Error> {
        self.client.get(&format!("/audiences/{}", id)).await
    }

    /// Create a new audience
    pub async fn create(&self, data: CreateAudienceRequest) -> Result<Audience, Error> {
        self.client.post("/audiences", &data).await
    }

    /// Update an audience
    pub async fn update(&self, id: &str, data: UpdateAudienceRequest) -> Result<Audience, Error> {
        self.client.put(&format!("/audiences/{}", id), &data).await
    }

    /// Delete an audience
    pub async fn delete(&self, id: &str) -> Result<EmptyResponse, Error> {
        self.client.delete(&format!("/audiences/{}", id)).await
    }

    /// Add contacts to an audience
    pub async fn add_contacts(
        &self,
        id: &str,
        contact_ids: Vec<String>,
    ) -> Result<EmptyResponse, Error> {
        let request = AddContactsRequest { contact_ids };
        self.client
            .post(&format!("/audiences/{}/contacts", id), &request)
            .await
    }
}
