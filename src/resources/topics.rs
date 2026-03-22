//! Topics resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{
    Contact, CreateTopicRequest, EmptyResponse, ListParams, PaginatedList, Topic,
    TopicAddContactsRequest, UpdateTopicRequest,
};

/// Topics API resource
pub struct Topics<'a> {
    client: &'a Client,
}

impl<'a> Topics<'a> {
    /// Create a new Topics resource
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all topics for an audience
    pub async fn list(
        &self,
        audience_id: &str,
        params: Option<ListParams>,
    ) -> Result<PaginatedList<Topic>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();
        self.client
            .get_with_params(&format!("/audiences/{}/topics", audience_id), &query_params)
            .await
    }

    /// Get a topic by ID
    pub async fn get(&self, audience_id: &str, id: &str) -> Result<Topic, Error> {
        self.client
            .get(&format!("/audiences/{}/topics/{}", audience_id, id))
            .await
    }

    /// Create a new topic
    pub async fn create(&self, audience_id: &str, data: CreateTopicRequest) -> Result<Topic, Error> {
        self.client
            .post(&format!("/audiences/{}/topics", audience_id), &data)
            .await
    }

    /// Update a topic
    pub async fn update(
        &self,
        audience_id: &str,
        id: &str,
        data: UpdateTopicRequest,
    ) -> Result<Topic, Error> {
        self.client
            .put(&format!("/audiences/{}/topics/{}", audience_id, id), &data)
            .await
    }

    /// Delete a topic
    pub async fn delete(&self, audience_id: &str, id: &str) -> Result<EmptyResponse, Error> {
        self.client
            .delete(&format!("/audiences/{}/topics/{}", audience_id, id))
            .await
    }

    /// Add contacts to a topic
    pub async fn add_contacts(
        &self,
        audience_id: &str,
        topic_id: &str,
        contact_ids: Vec<String>,
    ) -> Result<EmptyResponse, Error> {
        let request = TopicAddContactsRequest { contact_ids };
        self.client
            .post(
                &format!("/audiences/{}/topics/{}/contacts", audience_id, topic_id),
                &request,
            )
            .await
    }

    /// Remove a contact from a topic
    pub async fn remove_contact(
        &self,
        audience_id: &str,
        topic_id: &str,
        contact_id: &str,
    ) -> Result<EmptyResponse, Error> {
        self.client
            .delete(&format!(
                "/audiences/{}/topics/{}/contacts/{}",
                audience_id, topic_id, contact_id
            ))
            .await
    }

    /// List contacts for a topic
    pub async fn list_contacts(
        &self,
        audience_id: &str,
        topic_id: &str,
        params: Option<ListParams>,
    ) -> Result<PaginatedList<Contact>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();
        self.client
            .get_with_params(
                &format!("/audiences/{}/topics/{}/contacts", audience_id, topic_id),
                &query_params,
            )
            .await
    }
}
