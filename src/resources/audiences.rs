use crate::client::Client;
use crate::error::Error;
use crate::types::{
    AddContactsRequest, AddContactsToAudienceResponse, Audience, AudienceMutationResponse,
    AudienceRemoveContactResponse, Contact, CreateAudienceRequest, DeleteResponse,
    ListAudienceContactsParams, ListParams, PaginatedList, UpdateAudienceRequest,
};

/// Audiences API resource
pub struct Audiences<'a> {
    client: &'a Client,
}

impl<'a> Audiences<'a> {
    /// Binds audience operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves audiences with optional pagination and search.
    pub async fn list(&self, params: Option<ListParams>) -> Result<PaginatedList<Audience>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();

        self.client
            .get_with_params("/audiences", &query_params)
            .await
    }

    /// Retrieves an audience by ID.
    pub async fn get(&self, id: &str) -> Result<Audience, Error> {
        self.client.get(&format!("/audiences/{id}")).await
    }

    /// Adds an audience for organizing contacts, topics, and segments.
    pub async fn create(
        &self,
        data: CreateAudienceRequest,
    ) -> Result<AudienceMutationResponse, Error> {
        self.client.post("/audiences", &data).await
    }

    /// Changes an audience while preserving omitted fields.
    pub async fn update(
        &self,
        id: &str,
        data: UpdateAudienceRequest,
    ) -> Result<AudienceMutationResponse, Error> {
        self.client.put(&format!("/audiences/{id}"), &data).await
    }

    /// Removes an audience without deleting its contacts.
    pub async fn delete(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client.delete(&format!("/audiences/{id}")).await
    }

    /// Attaches contacts to an audience, ignoring existing memberships.
    pub async fn add_contacts(
        &self,
        id: &str,
        contact_ids: Vec<String>,
    ) -> Result<AddContactsToAudienceResponse, Error> {
        let request = AddContactsRequest { contact_ids };

        self.client
            .post(&format!("/audiences/{id}/contacts"), &request)
            .await
    }

    /// Detaches a contact from an audience without deleting it.
    pub async fn remove_contact(
        &self,
        audience_id: &str,
        contact_id: &str,
    ) -> Result<AudienceRemoveContactResponse, Error> {
        self.client
            .delete(&format!("/audiences/{audience_id}/contacts/{contact_id}"))
            .await
    }

    /// Retrieves contacts attached to an audience with optional filters.
    pub async fn list_contacts(
        &self,
        audience_id: &str,
        params: Option<ListAudienceContactsParams>,
    ) -> Result<PaginatedList<Contact>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();

        self.client
            .get_with_params(&format!("/audiences/{audience_id}/contacts"), &query_params)
            .await
    }
}
