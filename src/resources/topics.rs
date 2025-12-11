//! Topics resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{
    CreateTopicRequest, EmptyResponse, ListParams, PaginatedList, Topic, UpdateTopicRequest,
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
}
