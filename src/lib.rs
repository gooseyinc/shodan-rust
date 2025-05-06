use reqwest::{Client, Error};
use serde_json::Value;

const BASE_URL: &str = "https://api.shodan.io";

/// A minimal async Shodan API client using `reqwest` and `tokio`
#[derive(Debug, Clone)]
pub struct ShodanClient {
    api_key: String,
    client: Client,
}

impl ShodanClient {
    /// Create a new ShodanClient with the given API key.
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    /// Internal request handler for GET requests.
    async fn request(&self, endpoint: &str) -> Result<Value, Error> {
        let url = format!("{BASE_URL}/{endpoint}?key={}", self.api_key);
        let response = self.client.get(&url).send().await?;
        let json = response.json::<Value>().await?;
        Ok(json)
    }

    /// Get information about a specific IP address from Shodan.
    pub async fn host_info(&self, ip_address: &str) -> Result<Value, Error> {
        let endpoint = format!("shodan/host/{}", ip_address);
        self.request(&endpoint).await
    }

    /// Search Shodan using a search query (e.g. "nginx", "port:22 country:US", etc).
    pub async fn search(&self, query: &str) -> Result<Value, Error> {
        let endpoint = format!("shodan/host/search?query={}", query);
        self.request(&endpoint).await
    }

    /// Get honeyscore of a host (requires Shodan Labs access).
    pub async fn honeyscore(&self, ip_address: &str) -> Result<Value, Error> {
        let endpoint = format!("labs/honeyscore/{}", ip_address);
        self.request(&endpoint).await
    }
}

/// Get host info using a one-shot function interface.
pub async fn get_host_info(api_key: &str, ip_address: &str) -> Result<Value, Error> {
    ShodanClient::new(api_key.to_string())
        .host_info(ip_address)
        .await
}

/// Perform a Shodan search using a one-shot function interface.
pub async fn search_shodan(api_key: &str, query: &str) -> Result<Value, Error> {
    ShodanClient::new(api_key.to_string())
        .search(query)
        .await
}

/// Get the honeyscore of a host using a one-shot function interface.
pub async fn get_honeyscore(api_key: &str, ip_address: &str) -> Result<Value, Error> {
    ShodanClient::new(api_key.to_string())
        .honeyscore(ip_address)
        .await
}
