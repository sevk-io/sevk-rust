use crate::client::Client;
use crate::error::Error;
use crate::types::{
    CreateSegmentRequest, DeleteResponse, ListParams, PaginatedList, PreviewSegmentRequest,
    PreviewSegmentResponse, Segment, SegmentCalculateResponse, UpdateSegmentRequest,
};

/// Segments API resource
pub struct Segments<'a> {
    client: &'a Client,
}

impl<'a> Segments<'a> {
    /// Binds segment operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Retrieves segments belonging to an audience with optional filters.
    pub async fn list(
        &self,
        audience_id: &str,
        params: Option<ListParams>,
    ) -> Result<PaginatedList<Segment>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();

        self.client
            .get_with_params(&format!("/audiences/{audience_id}/segments"), &query_params)
            .await
    }

    /// Adds a saved contact segment under an audience.
    pub async fn create(
        &self,
        audience_id: &str,
        data: CreateSegmentRequest,
    ) -> Result<Segment, Error> {
        self.client
            .post(&format!("/audiences/{audience_id}/segments"), &data)
            .await
    }

    /// Evaluates segment rules and returns matches without saving them.
    pub async fn preview(
        &self,
        audience_id: &str,
        data: PreviewSegmentRequest,
    ) -> Result<PreviewSegmentResponse, Error> {
        self.client
            .post(&format!("/audiences/{audience_id}/segments/preview"), &data)
            .await
    }

    /// Retrieves a saved segment by ID.
    pub async fn get(&self, id: &str) -> Result<Segment, Error> {
        self.client.get(&format!("/segments/{id}")).await
    }

    /// Changes a segment while preserving omitted fields.
    pub async fn update(&self, id: &str, data: UpdateSegmentRequest) -> Result<Segment, Error> {
        self.client.put(&format!("/segments/{id}"), &data).await
    }

    /// Removes a segment without deleting any contacts.
    pub async fn delete(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client.delete(&format!("/segments/{id}")).await
    }

    /// Refreshes and returns the current matching contact count.
    pub async fn calculate(&self, segment_id: &str) -> Result<SegmentCalculateResponse, Error> {
        self.client
            .get(&format!("/segments/{segment_id}/calculate"))
            .await
    }
}
