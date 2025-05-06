use shodan_rust::{get_host_info, search_shodan, get_honeyscore};
use serde_json::Value;

#[tokio::main]
async fn main() {
    let api_key = "YOUR_API_KEY"; // add Shodan API Key
    
    match get_host_info(api_key, "8.8.8.8").await {
        Ok(response) => println!("Host information: {:#}", response),
        Err(e) => eprintln!("Error fetching host information: {}", e),
    }

    match search_shodan(api_key, "apache").await {
        Ok(response) => println!("Search Results: {:#}", response),
        Err(e) => eprintln!("Error searching on Shodan: {}", e),
    }

    match get_honeyscore(api_key, "8.8.8.8").await {
        Ok(score) => println!("Honeyscore: {}", score),
        Err(e) => eprintln!("Error fetching Honeyscore: {}", e),
    }
}
