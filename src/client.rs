//! HTTP client for the Sevk API

use crate::error::Error;
use reqwest::{Client as HttpClient, Method, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

/// Options for configuring the Sevk client
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
}

impl Client {
    /// Create a new client with the given API key and options
    pub fn new(api_key: &str, options: SevkOptions) -> Self {
        let http_client = HttpClient::builder()
            .timeout(Duration::from_millis(options.timeout))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key: api_key.to_string(),
            base_url: options.base_url,
            http_client,
        }
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T, Error> {
        let status = response.status().as_u16();

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(Error::Api {
                status,
                message: text,
            });
        }

        // Handle empty responses (e.g. 204 No Content)
        if status == 204 {
            let empty_json = serde_json::from_str("{}").unwrap();
            return Ok(empty_json);
        }

        let text = response.text().await?;
        serde_json::from_str(&text).map_err(Error::Json)
    }

    async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<impl Serialize>,
    ) -> Result<T, Error> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .http_client
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json");

        // Add body if present
        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Make a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        self.request::<T>(Method::GET, path, None::<()>).await
    }

    /// Make a GET request with query parameters
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

        self.request::<T>(Method::GET, &format!("{}{}", path, query_string), None::<()>)
            .await
    }

    /// Make a POST request
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        self.request(Method::POST, path, Some(body)).await
    }

    /// Make a PUT request
    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, Error> {
        self.request(Method::PUT, path, Some(body)).await
    }

    /// Make a DELETE request
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, Error> {
        self.request::<T>(Method::DELETE, path, None::<()>).await
    }
}

// URL encoding helper
mod urlencoding {
    pub fn encode(s: &str) -> String {
        let mut result = String::new();
        for c in s.chars() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
                ' ' => result.push_str("%20"),
                _ => {
                    for byte in c.to_string().as_bytes() {
                        result.push_str(&format!("%{:02X}", byte));
                    }
                }
            }
        }
        result
    }
}
