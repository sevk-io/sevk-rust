use crate::client::Client;
use crate::error::Error;
use crate::types::{
    AddContactsToTopicResponse, Contact, CreateTopicRequest, DeleteResponse, ListParams,
    ListTopicContactsParams, PaginatedList, Topic, TopicAddContactsRequest, TopicMutationResponse,
    TopicRemoveContactResponse, UpdateTopicRequest,
};

/// Topics API resource
pub struct Topics<'a> {
    client: &'a Client,
}

impl<'a> Topics<'a> {
    /// Binds topic operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves topics belonging to an audience with optional filters.
    pub async fn list(
        &self,
        audience_id: &str,
        params: Option<ListParams>,
    ) -> Result<PaginatedList<Topic>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();

        self.client
            .get_with_params(&format!("/audiences/{audience_id}/topics"), &query_params)
            .await
    }

    /// Adds a subscription topic under an audience.
    pub async fn create(
        &self,
        audience_id: &str,
        data: CreateTopicRequest,
    ) -> Result<TopicMutationResponse, Error> {
        self.client
            .post(&format!("/audiences/{audience_id}/topics"), &data)
            .await
    }

    /// Retrieves a topic by ID.
    pub async fn get(&self, id: &str) -> Result<Topic, Error> {
        self.client.get(&format!("/topics/{id}")).await
    }

    /// Changes a topic while preserving omitted fields.
    pub async fn update(
        &self,
        id: &str,
        data: UpdateTopicRequest,
    ) -> Result<TopicMutationResponse, Error> {
        self.client.put(&format!("/topics/{id}"), &data).await
    }

    /// Removes a topic without deleting contacts or its parent audience.
    pub async fn delete(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client.delete(&format!("/topics/{id}")).await
    }

    /// Attaches contacts to a topic, ignoring existing memberships.
    pub async fn add_contacts(
        &self,
        topic_id: &str,
        contact_ids: Vec<String>,
    ) -> Result<AddContactsToTopicResponse, Error> {
        let request = TopicAddContactsRequest { contact_ids };

        self.client
            .post(&format!("/topics/{topic_id}/contacts"), &request)
            .await
    }

    /// Detaches a contact from a topic without deleting it.
    pub async fn remove_contact(
        &self,
        topic_id: &str,
        contact_id: &str,
    ) -> Result<TopicRemoveContactResponse, Error> {
        self.client
            .delete(&format!("/topics/{topic_id}/contacts/{contact_id}"))
            .await
    }

    /// Retrieves contacts attached to a topic with optional filters.
    pub async fn list_contacts(
        &self,
        topic_id: &str,
        params: Option<ListTopicContactsParams>,
    ) -> Result<PaginatedList<Contact>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();

        self.client
            .get_with_params(&format!("/topics/{topic_id}/contacts"), &query_params)
            .await
    }
}
