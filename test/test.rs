#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_shodan_client() {
        let client = ShodanClient::new("INSERT_API_KEY");

        // test for host_info method
        let response = client.host_info("8.8.8.8").await;
        assert!(response.is_ok());

        // test for search method
        let search_response = client.search("apache").await;
        assert!(search_response.is_ok());

        // test for honeyscore method
        let honeyscore_response = client.honeyscore("8.8.8.8").await;
        assert!(honeyscore_response.is_ok());
    }
}
