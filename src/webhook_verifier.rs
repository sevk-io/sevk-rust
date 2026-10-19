//! Verifier for sevk webhook deliveries.
//!
//! sevk uses the Svix-compatible signature scheme (`svix-id`, `svix-timestamp`,
//! `svix-signature` headers), so this helper or the official `svix` crate can
//! be used. sevk issues canonical `whsec_` base64 secrets that can be passed
//! directly to either verifier.

use base64::{engine::general_purpose::STANDARD, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use subtle::ConstantTimeEq;
use thiserror::Error;

const TOLERANCE_SECONDS: i64 = 5 * 60;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Error)]
pub enum WebhookVerificationError {
    #[error("webhook secret is required")]
    EmptySecret,
    #[error("invalid base64 secret")]
    InvalidSecret,
    #[error("missing required svix headers")]
    MissingHeaders,
    #[error("invalid timestamp header")]
    InvalidTimestamp,
    #[error("timestamp outside tolerance window")]
    TimestampOutsideTolerance,
    #[error("signature mismatch")]
    SignatureMismatch,
    #[error("invalid payload: {0}")]
    InvalidPayload(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct WebhookVerifier {
    key: Vec<u8>,
}

impl WebhookVerifier {
    /// Creates a verifier from a webhook endpoint's signing secret.
    pub fn new(secret: &str) -> Result<Self, WebhookVerificationError> {
        if secret.is_empty() {
            return Err(WebhookVerificationError::EmptySecret);
        }

        let key = if let Some(rest) = secret.strip_prefix("whsec_") {
            let decoded = STANDARD
                .decode(rest)
                .map_err(|_| WebhookVerificationError::InvalidSecret)?;
            if decoded.is_empty() {
                return Err(WebhookVerificationError::InvalidSecret);
            }
            decoded
        } else {
            secret.as_bytes().to_vec()
        };

        Ok(Self { key })
    }

    /// Verify a webhook delivery and return the parsed JSON payload.
    pub fn verify(
        &self,
        payload: &str,
        headers: &HashMap<String, String>,
    ) -> Result<serde_json::Value, WebhookVerificationError> {
        let msg_id = header(headers, "svix-id").ok_or(WebhookVerificationError::MissingHeaders)?;
        let msg_timestamp =
            header(headers, "svix-timestamp").ok_or(WebhookVerificationError::MissingHeaders)?;
        let msg_signature =
            header(headers, "svix-signature").ok_or(WebhookVerificationError::MissingHeaders)?;

        if msg_id.is_empty() {
            return Err(WebhookVerificationError::MissingHeaders);
        }

        if msg_timestamp.is_empty() || !msg_timestamp.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(WebhookVerificationError::InvalidTimestamp);
        }
        let timestamp: i64 = msg_timestamp
            .parse()
            .map_err(|_| WebhookVerificationError::InvalidTimestamp)?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| WebhookVerificationError::InvalidTimestamp)?
            .as_secs() as i64;

        if timestamp < now - TOLERANCE_SECONDS || timestamp > now + TOLERANCE_SECONDS {
            return Err(WebhookVerificationError::TimestampOutsideTolerance);
        }

        let to_sign = format!("{msg_id}.{msg_timestamp}.{payload}");
        let mut mac = HmacSha256::new_from_slice(&self.key)
            .map_err(|_| WebhookVerificationError::InvalidSecret)?;
        mac.update(to_sign.as_bytes());
        let signature_bytes = mac.finalize().into_bytes();
        let expected = format!("v1,{}", STANDARD.encode(signature_bytes));
        let expected_bytes = expected.as_bytes();

        for candidate in msg_signature.split(' ') {
            let candidate_bytes = candidate.as_bytes();
            if candidate_bytes.len() == expected_bytes.len()
                && candidate_bytes.ct_eq(expected_bytes).into()
            {
                return Ok(serde_json::from_str(payload)?);
            }
        }

        Err(WebhookVerificationError::SignatureMismatch)
    }
}

fn header<'a>(headers: &'a HashMap<String, String>, name: &str) -> Option<&'a str> {
    headers.iter().find_map(|(key, value)| {
        let lower = key.to_ascii_lowercase();
        let normalized = lower
            .strip_prefix("http_")
            .unwrap_or(&lower)
            .replace('_', "-");
        (normalized == name).then_some(value.as_str())
    })
}
