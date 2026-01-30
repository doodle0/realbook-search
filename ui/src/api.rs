use crate::models::{RealBookEntry, SearchResponse};

/// Base URL for the API
/// During development with Trunk, this will be proxied through localhost:8080
const API_BASE_URL: &str = "http://localhost:8080/api";

/// Error type for API operations
#[derive(Debug, Clone)]
pub struct ApiError {
    pub message: String,
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError {
            message: format!("Request failed: {}", err),
        }
    }
}

impl From<String> for ApiError {
    fn from(message: String) -> Self {
        ApiError { message }
    }
}

/// Search for Real Book entries
pub async fn search(
    query: Option<String>,
    volume: Option<u32>,
    page: Option<u32>,
) -> Result<SearchResponse, ApiError> {
    let mut url = format!("{}/search", API_BASE_URL);
    let mut params = vec![];

    if let Some(q) = query {
        if !q.is_empty() {
            params.push(format!("query={}", urlencoding::encode(&q)));
        }
    }
    if let Some(v) = volume {
        params.push(format!("volume={}", v));
    }
    if let Some(p) = page {
        params.push(format!("page={}", p));
    }

    if !params.is_empty() {
        url.push('?');
        url.push_str(&params.join("&"));
    }

    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        return Err(ApiError {
            message: format!("API returned status: {}", response.status()),
        });
    }

    let data = response.json::<SearchResponse>().await?;
    Ok(data)
}

/// Get a random Real Book entry
pub async fn get_random() -> Result<RealBookEntry, ApiError> {
    let url = format!("{}/random", API_BASE_URL);
    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        return Err(ApiError {
            message: format!("API returned status: {}", response.status()),
        });
    }

    let data = response.json::<RealBookEntry>().await?;
    Ok(data)
}

