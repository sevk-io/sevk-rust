use crate::error::Error;
use reqwest::{Client as HttpClient, Method, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Render a JSON value as a plain string (strings without surrounding quotes).
fn json_value_to_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

fn parse_api_error(response_status: u16, retry_after: Option<u64>, text: String) -> Error {
    let parsed: Option<serde_json::Value> = serde_json::from_str(&text).ok();
    let payload = parsed
        .as_ref()
        .and_then(|value| value.get("error").filter(|nested| nested.is_object()))
        .or(parsed.as_ref());
    let message = payload
        .and_then(|value| value.get("message"))
        .and_then(|value| value.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| {
            if text.is_empty() {
                format!("HTTP {response_status}")
            } else {
                text.clone()
            }
        });
    let status = response_status;
    let code = payload
        .and_then(|value| value.get("code"))
        .and_then(|value| value.as_str())
        .unwrap_or("UNKNOWN_ERROR")
        .to_string();
    let fields = payload
        .and_then(|value| value.get("fields"))
        .and_then(|value| value.as_object())
        .map(|object| {
            object
                .iter()
                .map(|(key, value)| {
                    let values = match value {
                        serde_json::Value::Array(items) => {
                            items.iter().map(json_value_to_string).collect()
                        }
                        _ => vec![json_value_to_string(value)],
                    };
                    (key.clone(), values)
                })
                .collect()
        })
        .unwrap_or_default();
    let details = payload.and_then(|value| value.get("details")).cloned();
    let response_body = parsed.unwrap_or(serde_json::Value::String(text));

    Error::Api {
        status,
        message,
        code: Some(code),
        fields,
        retry_after,
        details,
        response_body,
    }
}

fn invalid_success_response(status: u16, response_body: serde_json::Value) -> Error {
    Error::Api {
        status,
        message: "Expected a JSON object response body but received empty or malformed JSON"
            .to_string(),
        code: Some("INVALID_RESPONSE".to_string()),
        fields: HashMap::new(),
        retry_after: None,
        details: None,
        response_body,
    }
}

fn parse_retry_after(raw: &str) -> Option<u64> {
    let value = raw.trim();
    if value.is_empty() || !value.chars().all(|character| character.is_ascii_digit()) {
        return None;
    }

    value
        .parse::<u64>()
        .ok()
        .filter(|seconds| *seconds <= i32::MAX as u64)
}

/// Options for configuring the sevk client
#[derive(Debug, Clone)]
pub struct SevkOptions {
    /// Base URL for the API
    pub base_url: String,
    /// Request timeout in milliseconds
    pub timeout: u64,
}

impl Default for SevkOptions {
    fn default() -> Self {
        Self {
            base_url: "https://api.sevk.io".to_string(),
            timeout: 30000,
        }
    }
}

/// HTTP client for making API requests
#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    base_url: String,
    http_client: HttpClient,
    timeout_ms: u64,
}

impl Client {
    /// Creates the authenticated HTTP client used by all resource handles.
    pub fn new(api_key: &str, options: SevkOptions) -> Self {
        let http_client = HttpClient::builder()
            .user_agent(concat!("sevk-rust/", env!("CARGO_PKG_VERSION")))
            .timeout(Duration::from_millis(options.timeout))
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key: api_key.to_string(),
            base_url: options.base_url.trim_end_matches('/').to_string(),
            http_client,
            timeout_ms: options.timeout,
        }
    }

    fn transport_error(&self, error: reqwest::Error) -> Error {
        if error.is_timeout() {
            Error::Timeout(self.timeout_ms)
        } else {
            Error::Request(error)
        }
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T, Error> {
        let status = response.status().as_u16();

        if !response.status().is_success() {
            let retry_after = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|value| value.to_str().ok())
                .and_then(parse_retry_after);
            let text = response
                .text()
                .await
                .map_err(|error| self.transport_error(error))?;
            return Err(parse_api_error(status, retry_after, text));
        }

        let text = response
            .text()
            .await
            .map_err(|error| self.transport_error(error))?;
        let response_body = if status == 204 && text.trim().is_empty() {
            serde_json::json!({})
        } else {
            match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(value) if value.is_object() => value,
                _ => {
                    return Err(invalid_success_response(
                        status,
                        serde_json::Value::String(text),
                    ))
                }
            }
        };
        serde_json::from_value(response_body.clone())
            .map_err(|_| invalid_success_response(status, response_body))
    }

    async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<impl Serialize>,
    ) -> Result<T, Error> {
        self.request_with_headers(method, path, body, &[]).await
    }

    async fn request_with_headers<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<impl Serialize>,
        extra_headers: &[(&str, &str)],
    ) -> Result<T, Error> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .http_client
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json");

        for (k, v) in extra_headers {
            request = request.header(*k, *v);
        }

        if let Some(body) = body {
            request = request.body(serde_json::to_vec(&body)?);
        }

        let response = request
            .send()
            .await
            .map_err(|error| self.transport_error(error))?;
        self.handle_response(response).await
    }

    /// Sends an authenticated GET request and decodes its JSON response.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        self.request::<T>(Method::GET, path, None::<()>).await
    }

    /// Sends an authenticated GET request with serialized query parameters.
    pub async fn get_with_params<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, String)],
    ) -> Result<T, Error> {
        let query_string = if params.is_empty() {
            String::new()
        } else {
            let pairs: Vec<String> = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                .collect();
            format!("?{}", pairs.join("&"))
        };

        self.request::<T>(Method::GET, &format!("{path}{query_string}"), None::<()>)
            .await
    }

    /// Sends a JSON POST request and decodes its response.
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        self.request(Method::POST, path, Some(body)).await
    }

    /// Sends a JSON POST request with additional headers for retry control.
    pub async fn post_with_headers<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
        headers: &[(&str, &str)],
    ) -> Result<T, Error> {
        self.request_with_headers(Method::POST, path, Some(body), headers)
            .await
    }

    /// Sends a JSON PUT request and decodes its response.
    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        self.request(Method::PUT, path, Some(body)).await
    }

    /// sends a json patch request and decodes its response.
    pub async fn patch<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        self.request(Method::PATCH, path, Some(body)).await
    }

    /// Sends an authenticated DELETE request and decodes its response.
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        self.request::<T>(Method::DELETE, path, None::<()>).await
    }
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        let mut result = String::new();
        for c in s.chars() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
                ' ' => result.push_str("%20"),
                _ => {
                    for byte in c.to_string().as_bytes() {
                        result.push_str(&format!("%{byte:02X}"));
                    }
                }
            }
        }
        result
    }
}
