use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use thiserror::Error;

pub mod anime;
pub mod character;
pub mod clubs;
pub mod common;
use common::enums;
use common::response;
use common::structs;
use common::utils;
pub mod genre;
pub mod magazines;
pub mod manga;
pub mod people;
pub mod producer;
pub mod random;
pub mod recommendations;
pub mod review;
pub mod schedule;
pub mod seasons;
pub mod top;
pub mod users;
pub mod watch;
const API_BASE_URL: &str = "https://api.jikan.moe/v4";

#[derive(Error, Debug)]
pub enum JikanError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("API response parsing failed: {0}")]
    ParseError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Resource not found")]
    NotFound,
}

pub struct JikanClient {
    client: Client,
    api_base_url: Option<String>,
}

impl JikanClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_base_url: None,
        }
    }

    pub fn set_api_base_url(mut self, api_base_url: String) -> Self {
        self.api_base_url = Some(api_base_url);
        self
    }

    fn get_api_base_url(&self) -> String {
        match &self.api_base_url {
            Some(url) => url.to_string(),
            None => API_BASE_URL.to_string(),
        }
    }

    pub async fn get<T>(&self, path: &str) -> Result<T, JikanError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}{}", self.get_api_base_url(), path);
        let response = self.client.get(&url).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let text = response.text().await?;
                serde_json::from_str(&text).map_err(|e| JikanError::ParseError(e.to_string()))
            }
            reqwest::StatusCode::TOO_MANY_REQUESTS => Err(JikanError::RateLimitExceeded),
            reqwest::StatusCode::NOT_FOUND => Err(JikanError::NotFound),
            status => Err(JikanError::BadRequest(format!(
                "Unexpected status code: {}",
                status
            ))),
        }
    }
}

impl Default for JikanClient {
    fn default() -> Self {
        Self::new()
    }
}

pub fn format_search_query(query: String) -> String {
    query
        .to_lowercase()
        .chars()
        .map(|c| match c {
            ' ' => '-',
            c if c.is_alphanumeric() => c,
            _ => ' ',
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}
