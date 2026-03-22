//! API Resources for the Sevk SDK

mod audiences;
mod broadcasts;
mod contacts;
mod domains;
mod emails;
mod events;
mod segments;
mod subscriptions;
mod templates;
mod topics;
mod webhooks;

pub use audiences::Audiences;
pub use broadcasts::Broadcasts;
pub use contacts::Contacts;
pub use domains::Domains;
pub use emails::Emails;
pub use events::Events;
pub use segments::Segments;
pub use subscriptions::Subscriptions;
pub use templates::Templates;
pub use topics::Topics;
pub use webhooks::Webhooks;
