use crate::client::Client;
use crate::error::Error;
use crate::types::{
    BulkUpdateContactsRequest, BulkUpdateContactsResponse, Contact, ContactActivityItem,
    CreateContactRequest, DeleteResponse, ImportContactsRequest, ImportContactsResponse,
    ListContactActivityParams, ListContactsParams, PaginatedList, UpdateContactRequest,
};

/// Contacts API resource
pub struct Contacts<'a> {
    client: &'a Client,
}

impl<'a> Contacts<'a> {
    /// Binds contact operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves contacts with pagination, search, and optional filters.
    pub async fn list(
        &self,
        params: Option<ListContactsParams>,
    ) -> Result<PaginatedList<Contact>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();

        self.client
            .get_with_params("/contacts", &query_params)
            .await
    }

    /// Retrieves a contact by ID.
    pub async fn get(&self, id: &str) -> Result<Contact, Error> {
        self.client.get(&format!("/contacts/{id}")).await
    }

    /// Adds a contact and optionally stores custom metadata.
    pub async fn create(
        &self,
        email: &str,
        options: Option<CreateContactRequest>,
    ) -> Result<Contact, Error> {
        let mut request = options.unwrap_or_default();
        request.email = email.to_string();

        self.client.post("/contacts", &request).await
    }

    /// Changes a contact while preserving omitted fields.
    pub async fn update(&self, id: &str, data: UpdateContactRequest) -> Result<Contact, Error> {
        self.client.put(&format!("/contacts/{id}"), &data).await
    }

    /// Removes a contact and its audience and topic memberships.
    pub async fn delete(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client.delete(&format!("/contacts/{id}")).await
    }

    /// Changes contacts by email and reports entries that were not found.
    pub async fn bulk_update(
        &self,
        updates: BulkUpdateContactsRequest,
    ) -> Result<BulkUpdateContactsResponse, Error> {
        self.client.put("/contacts/bulk-update", &updates).await
    }

    /// Adds contacts and can attach them to an audience or topic.
    pub async fn bulk_import(
        &self,
        params: ImportContactsRequest,
    ) -> Result<ImportContactsResponse, Error> {
        self.client.post("/contacts/import", &params).await
    }

    /// Retrieves delivery and engagement history for a contact.
    pub async fn activity(
        &self,
        id: &str,
        params: Option<ListContactActivityParams>,
    ) -> Result<PaginatedList<ContactActivityItem>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();

        self.client
            .get_with_params(&format!("/contacts/{id}/activity"), &query_params)
            .await
    }
}
