//! # sevk SDK for Rust
//!
//! Official Rust SDK for the sevk email platform.
//!
//! ## Example
//!
//! ```rust,no_run
//! use sevk::Sevk;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), sevk::Error> {
//!     let client = Sevk::new("sevk_your_api_key");
//!
//!     // List contacts
//!     let contacts = client.contacts().list(None).await?;
//!     println!("Contacts on this page: {}", contacts.items.len());
//!
//!     // Create a contact
//!     let contact = client.contacts().create("user@example.com", None).await?;
//!     println!("Created contact: {}", contact.id);
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod markup;
pub mod resources;
pub mod types;
pub mod webhook_verifier;

pub use client::{Client, SevkOptions};
pub use error::Error;
pub use markup::{render, EmailHeadSettings, RenderError};
pub use types::*;
pub use webhook_verifier::{WebhookVerificationError, WebhookVerifier};

/// Main entry point for the sevk SDK
pub struct Sevk {
    client: Client,
}

impl Sevk {
    /// Create a new sevk client with the given API key
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(api_key, SevkOptions::default()),
        }
    }

    /// Create a new sevk client with custom options
    pub fn with_options(api_key: &str, options: SevkOptions) -> Self {
        Self {
            client: Client::new(api_key, options),
        }
    }

    /// Access the Contacts resource
    pub fn contacts(&self) -> resources::Contacts<'_> {
        resources::Contacts::new(&self.client)
    }

    /// Access the Audiences resource
    pub fn audiences(&self) -> resources::Audiences<'_> {
        resources::Audiences::new(&self.client)
    }

    /// Access the Templates resource
    pub fn templates(&self) -> resources::Templates<'_> {
        resources::Templates::new(&self.client)
    }

    /// Access the Broadcasts resource
    pub fn broadcasts(&self) -> resources::Broadcasts<'_> {
        resources::Broadcasts::new(&self.client)
    }

    /// Access the Domains resource
    pub fn domains(&self) -> resources::Domains<'_> {
        resources::Domains::new(&self.client)
    }

    /// Access the Topics resource
    pub fn topics(&self) -> resources::Topics<'_> {
        resources::Topics::new(&self.client)
    }

    /// Access the Segments resource
    pub fn segments(&self) -> resources::Segments<'_> {
        resources::Segments::new(&self.client)
    }

    /// Access the Subscriptions resource
    pub fn subscriptions(&self) -> resources::Subscriptions<'_> {
        resources::Subscriptions::new(&self.client)
    }

    /// Access the Emails resource
    pub fn emails(&self) -> resources::Emails<'_> {
        resources::Emails::new(&self.client)
    }

    /// Access the Webhooks resource
    pub fn webhooks(&self) -> resources::Webhooks<'_> {
        resources::Webhooks::new(&self.client)
    }

    /// Access the Activity resource
    pub fn activity(&self) -> resources::Activity<'_> {
        resources::Activity::new(&self.client)
    }

    /// Access the Inbound resource
    pub fn inbound(&self) -> resources::Inbound<'_> {
        resources::Inbound::new(&self.client)
    }

    /// Access the Outbound resource
    pub fn outbound(&self) -> resources::Outbound<'_> {
        resources::Outbound::new(&self.client)
    }

    /// Access the Usage resource
    pub fn usage(&self) -> resources::Usage<'_> {
        resources::Usage::new(&self.client)
    }
}
