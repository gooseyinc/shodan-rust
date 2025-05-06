use reqwest::{Client, Error};
use serde_json::Value;

const BASE_URL: &str = "https://api.shodan.io";

// Shodan API Structure
#[derive(Debug)]
pub struct ShodanClient {
    api_key: String,
    client: Client,
}

impl ShodanClient {
    // create new client
    pub fn new(api_key: String) -> ShodanClient {
        ShodanClient {
            api_key,
            client: Client::new(),
        }
    }

    // example GET
    pub async fn request(&self, endpoint: &str) -> Result<Value, Error> {
        let url = format!("{}/{}?key={}", BASE_URL, endpoint, self.api_key);
        let response = self.client.get(&url).send().await?.json::<Value>().await?;
        Ok(response)
    }

    // example use endpoint
    pub async fn host_info(&self, ip_address: &str) -> Result<Value, Error> {
        let endpoint = format!("shodan/host/{}", ip_address);
        self.request(&endpoint).await
    }

    // example use endpoint
    pub async fn search(&self, query: &str) -> Result<Value, Error> {
        let endpoint = format!("shodan/host/search?query={}", query);
        self.request(&endpoint).await
    }

    // example use honeyscore
    pub async fn honeyscore(&self, ip_address: &str) -> Result<Value, Error> {
        let endpoint = format!("labs/honeyscore/{}", ip_address);
        self.request(&endpoint).await
    }
}

pub async fn get_host_info(api_key: &str, ip_address: &str) -> Result<Value, Error> {
    let client = ShodanClient::new(api_key.to_string());
    client.host_info(ip_address).await
}

pub async fn search_shodan(api_key: &str, query: &str) -> Result<Value, Error> {
    let client = ShodanClient::new(api_key.to_string());
    client.search(query).await
}

pub async fn get_honeyscore(api_key: &str, ip_address: &str) -> Result<Value, Error> {
    let client = ShodanClient::new(api_key.to_string());
    client.honeyscore(ip_address).await
}