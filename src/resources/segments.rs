//! Segments resource

use crate::client::Client;
use crate::error::Error;
use crate::types::{
    CreateSegmentRequest, EmptyResponse, ListParams, PaginatedList, Segment,
    SegmentCalculateRequest, SegmentCalculateResponse, UpdateSegmentRequest,
};

/// Segments API resource
pub struct Segments<'a> {
    client: &'a Client,
}

impl<'a> Segments<'a> {
    /// Create a new Segments resource
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all segments for an audience
    pub async fn list(
        &self,
        audience_id: &str,
        params: Option<ListParams>,
    ) -> Result<PaginatedList<Segment>, Error> {
        let params = params.unwrap_or_default();
        let query_params = params.to_query_params();
        self.client
            .get_with_params(&format!("/audiences/{}/segments", audience_id), &query_params)
            .await
    }

    /// Get a segment by ID
    pub async fn get(&self, audience_id: &str, id: &str) -> Result<Segment, Error> {
        self.client
            .get(&format!("/audiences/{}/segments/{}", audience_id, id))
            .await
    }

    /// Create a new segment
    pub async fn create(&self, audience_id: &str, data: CreateSegmentRequest) -> Result<Segment, Error> {
        self.client
            .post(&format!("/audiences/{}/segments", audience_id), &data)
            .await
    }

    /// Update a segment
    pub async fn update(
        &self,
        audience_id: &str,
        id: &str,
        data: UpdateSegmentRequest,
    ) -> Result<Segment, Error> {
        self.client
            .put(&format!("/audiences/{}/segments/{}", audience_id, id), &data)
            .await
    }

    /// Delete a segment
    pub async fn delete(&self, audience_id: &str, id: &str) -> Result<EmptyResponse, Error> {
        self.client
            .delete(&format!("/audiences/{}/segments/{}", audience_id, id))
            .await
    }

    /// Calculate a segment's matching contacts
    pub async fn calculate(
        &self,
        audience_id: &str,
        segment_id: &str,
    ) -> Result<SegmentCalculateResponse, Error> {
        self.client
            .get(&format!(
                "/audiences/{}/segments/{}/calculate",
                audience_id, segment_id
            ))
            .await
    }

    /// Preview how many contacts match given segment rules before saving
    pub async fn preview(
        &self,
        audience_id: &str,
        data: SegmentCalculateRequest,
    ) -> Result<SegmentCalculateResponse, Error> {
        self.client
            .post(
                &format!("/audiences/{}/segments/preview", audience_id),
                &data,
            )
            .await
    }
}
