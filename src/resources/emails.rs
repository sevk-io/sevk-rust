use crate::client::Client;
use crate::error::Error;
use crate::markup;
use crate::types::{
    BulkEmailItem, BulkEmailRequest, BulkEmailResponse, Email, ScheduledEmailMutationResponse,
    SendEmailRequest, SendEmailResponse, UpdateScheduledEmailRequest,
};

/// Emails API resource
pub struct Emails<'a> {
    client: &'a Client,
}

/// If `markup` is set, render it to HTML locally and place the result in `html`
/// before the request is serialized.
fn apply_markup(req: &mut SendEmailRequest) -> Result<(), markup::RenderError> {
    if let Some(src) = req.markup.take() {
        if !src.is_empty() {
            req.html = Some(markup::render(&src)?);
        }
    }
    Ok(())
}

fn apply_bulk_markup(req: &mut BulkEmailItem) -> Result<(), markup::RenderError> {
    if let Some(src) = req.markup.take() {
        if !src.is_empty() {
            req.html = Some(markup::render(&src)?);
        }
    }
    Ok(())
}

/// Per-call options for send/send_bulk (e.g. idempotency key).
#[derive(Debug, Default, Clone)]
pub struct SendOptions {
    pub idempotency_key: Option<String>,
}

impl<'a> Emails<'a> {
    /// Binds transactional email operations to an API client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Queues one transactional email, rendering local markup when supplied.
    pub async fn send(&self, mut data: SendEmailRequest) -> Result<SendEmailResponse, Error> {
        apply_markup(&mut data)?;

        self.client.post("/emails", &data).await
    }

    /// Queues one email with optional retry idempotency.
    pub async fn send_with_options(
        &self,
        mut data: SendEmailRequest,
        options: &SendOptions,
    ) -> Result<SendEmailResponse, Error> {
        apply_markup(&mut data)?;
        match options.idempotency_key.as_deref() {
            Some(key) => {
                self.client
                    .post_with_headers("/emails", &data, &[("Idempotency-Key", key)])
                    .await
            }
            None => self.client.post("/emails", &data).await,
        }
    }

    /// Queues up to 100 emails and returns per-item results.
    pub async fn send_bulk(&self, mut data: BulkEmailRequest) -> Result<BulkEmailResponse, Error> {
        for email in &mut data.emails {
            apply_bulk_markup(email)?;
        }
        self.client.post("/emails/bulk", &data).await
    }

    /// Queues an email batch with optional retry idempotency.
    pub async fn send_bulk_with_options(
        &self,
        mut data: BulkEmailRequest,
        options: &SendOptions,
    ) -> Result<BulkEmailResponse, Error> {
        for email in &mut data.emails {
            apply_bulk_markup(email)?;
        }
        match options.idempotency_key.as_deref() {
            Some(key) => {
                self.client
                    .post_with_headers("/emails/bulk", &data, &[("Idempotency-Key", key)])
                    .await
            }
            None => self.client.post("/emails/bulk", &data).await,
        }
    }

    /// Retrieves a sent email and its current delivery state.
    pub async fn get(&self, id: &str) -> Result<Email, Error> {
        self.client.get(&format!("/emails/{id}")).await
    }

    /// moves an unattempted scheduled email to a new delivery time.
    pub async fn update(
        &self,
        id: &str,
        data: UpdateScheduledEmailRequest,
    ) -> Result<ScheduledEmailMutationResponse, Error> {
        self.client.patch(&format!("/emails/{id}"), &data).await
    }

    /// cancels an unattempted scheduled email.
    pub async fn cancel(&self, id: &str) -> Result<ScheduledEmailMutationResponse, Error> {
        self.client
            .post(&format!("/emails/{id}/cancel"), &serde_json::json!({}))
            .await
    }
}
