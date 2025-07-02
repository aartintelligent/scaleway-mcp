use reqwest::Client as HttpClient;
use reqwest::header::HeaderValue;
use serde::de::DeserializeOwned;

use crate::endpoint::Endpoint;
use crate::error::{Result, ScalewayError};

#[derive(Debug, Clone)]
pub struct ScalewayClient {
    http: HttpClient,
    base_url: String,
    secret_key: String,
}

impl ScalewayClient {
    pub fn new(secret_key: impl Into<String>, base_url: Option<String>) -> Self {
        Self {
            http: HttpClient::new(),
            base_url: base_url.unwrap_or_else(|| "https://api.scaleway.com".to_string()),
            secret_key: secret_key.into(),
        }
    }

    pub async fn execute<E: Endpoint>(&self, endpoint: E) -> Result<E::Response>
    where
        E::Response: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint.path());

        let mut request = self.http.request(endpoint.method(), &url);

        request = request.header(
            "X-Auth-Token",
            HeaderValue::from_str(&self.secret_key).unwrap(),
        );

        if !endpoint.query().is_empty() {
            request = request.query(&endpoint.query());
        }

        if let Some(body) = endpoint.body() {
            request = request.json(&body);
        }

        let response = request.send().await.map_err(ScalewayError::from)?;

        let response = response.error_for_status().map_err(ScalewayError::from)?;

        let parsed = response
            .json::<E::Response>()
            .await
            .map_err(ScalewayError::from)?;

        Ok(parsed)
    }
}
