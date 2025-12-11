//! Contacts resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{
    Contact, CreateContactRequest, EmptyResponse, ListParams, PaginatedList, UpdateContactRequest,
};

/// Contacts API resource
pub struct Contacts<'a> {
    client: &'a Client,
}

impl<'a> Contacts<'a> {
    /// Create a new Contacts resource
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all contacts
    pub async fn list(&self, params: Option<ListParams>) -> Result<PaginatedList<Contact>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();
        self.client
            .get_with_params("/contacts", &query_params)
            .await
    }

    /// Get a contact by ID
    pub async fn get(&self, id: &str) -> Result<Contact, Error> {
        self.client.get(&format!("/contacts/{}", id)).await
    }

    /// Create a new contact
    pub async fn create(
        &self,
        email: &str,
        options: Option<CreateContactRequest>,
    ) -> Result<Contact, Error> {
        let mut request = options.unwrap_or_default();
        request.email = email.to_string();
        self.client.post("/contacts", &request).await
    }

    /// Update a contact
    pub async fn update(&self, id: &str, data: UpdateContactRequest) -> Result<Contact, Error> {
        self.client.put(&format!("/contacts/{}", id), &data).await
    }

    /// Delete a contact
    pub async fn delete(&self, id: &str) -> Result<EmptyResponse, Error> {
        self.client.delete(&format!("/contacts/{}", id)).await
    }
}
