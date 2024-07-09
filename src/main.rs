use reqwest::Client;
use serde_json::Value;
use std::error::Error;
mod utils;

async fn search_address(api_url: &str, query: &str) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let res = client.get(api_url).query(&[("q", query)]).send().await?;
    let json = res.json::<Value>().await?;
    Ok(json)
}

async fn search_zipcode(api_url: &str, zipcode: &str) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let res = client.get(api_url).query(&[("zip", zipcode)]).send().await?;
    let json = res.json::<Value>().await?;
    Ok(json)
}

#[tokio::main]
async fn main() {
    let api_url = "https://api.example.com/search";

    match search_address(api_url, "sample address").await {
        Ok(response) => println!("Address search result: {:?}", response),
        Err(e) => eprintln!("Address search failed: {}", e),
    }

    match search_zipcode(api_url, "12345").await {
        Ok(response) => println!("Zipcode search result: {:?}", response),
        Err(e) => eprintln!("Zipcode search failed: {}", e),
    }
}