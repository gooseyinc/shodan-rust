use shodan::ShodanClient;

#[tokio::main]
async fn main() {
    let api_key = "YOUR_API_KEY".to_string(); // Add Shodan API key
    let client = ShodanClient::new(api_key);

    match client.host_info("8.8.8.8").await {
        Ok(response) => println!("Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
