//! # Sevk SDK for Rust
//!
//! Official Rust SDK for Sevk - Email Marketing Platform
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
//!     println!("Total contacts: {}", contacts.total);
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

pub use client::{Client, SevkOptions};
pub use error::Error;
pub use markup::{generate_email_from_markup, EmailHeadSettings};

/// Main entry point for the Sevk SDK
pub struct Sevk {
    client: Client,
}

impl Sevk {
    /// Create a new Sevk client with the given API key
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(api_key, SevkOptions::default()),
        }
    }

    /// Create a new Sevk client with custom options
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

    /// Access the Events resource
    pub fn events(&self) -> resources::Events<'_> {
        resources::Events::new(&self.client)
    }

    /// Get project usage and limits
    pub async fn get_usage(&self) -> Result<Usage, Error> {
        self.client.get("/limits").await
    }
}

/// Project usage and limits data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Usage {
    pub id: String,
    pub name: String,
    #[serde(rename = "audienceLimit")]
    pub audience_limit: i64,
    #[serde(rename = "contactLimit")]
    pub contact_limit: i64,
    #[serde(rename = "broadcastLimit")]
    pub broadcast_limit: i64,
    #[serde(rename = "domainLimit")]
    pub domain_limit: i64,
    #[serde(rename = "storageLimit", deserialize_with = "deserialize_string_to_i64")]
    pub storage_limit: i64,
    #[serde(rename = "storageUsed", deserialize_with = "deserialize_string_to_i64")]
    pub storage_used: i64,
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub balance: f64,
    #[serde(rename = "emailPrice", deserialize_with = "deserialize_string_to_f64")]
    pub email_price: f64,
    #[serde(rename = "emailStats")]
    pub email_stats: EmailStatsData,
    #[serde(rename = "emailLimits")]
    pub email_limits: EmailLimitsData,
    #[serde(rename = "_count")]
    pub count: UsageCount,
}

fn deserialize_string_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;
    struct StringOrI64Visitor;
    impl<'de> de::Visitor<'de> for StringOrI64Visitor {
        type Value = i64;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or integer")
        }
        fn visit_i64<E: de::Error>(self, v: i64) -> Result<i64, E> { Ok(v) }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<i64, E> { Ok(v as i64) }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<i64, E> {
            v.parse().map_err(de::Error::custom)
        }
    }
    deserializer.deserialize_any(StringOrI64Visitor)
}

fn deserialize_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;
    struct StringOrF64Visitor;
    impl<'de> de::Visitor<'de> for StringOrF64Visitor {
        type Value = f64;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or number")
        }
        fn visit_f64<E: de::Error>(self, v: f64) -> Result<f64, E> { Ok(v) }
        fn visit_i64<E: de::Error>(self, v: i64) -> Result<f64, E> { Ok(v as f64) }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<f64, E> { Ok(v as f64) }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<f64, E> {
            v.parse().map_err(de::Error::custom)
        }
    }
    deserializer.deserialize_any(StringOrF64Visitor)
}

/// Email statistics data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailStatsData {
    pub total: i64,
    pub marketing: i64,
    pub transactional: i64,
}

/// Email limits data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailLimitsData {
    pub project: ProjectEmailLimits,
}

/// Project email limits
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectEmailLimits {
    pub monthly: EmailLimitEntry,
    pub rate: EmailLimitEntry,
    #[serde(rename = "freeEmails")]
    pub free_emails: EmailLimitEntry,
}

/// Email limit entry
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailLimitEntry {
    pub current: i64,
    pub limit: i64,
    pub remaining: i64,
}

/// Resource counts
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsageCount {
    pub audiences: i64,
    pub contacts: i64,
    pub broadcasts: i64,
    pub domains: i64,
}
