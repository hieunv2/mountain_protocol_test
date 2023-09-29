use dotenv::dotenv;
use hmac::{Hmac, Mac, NewMac};
use reqwest;
use sha2::Sha256;
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv().ok(); // Load environment variables from .env file

    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let url = "https://api.beta.mountainprotocol.com/v1/balance";

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let string_to_sign = format!(
        "{}:{}:{}:{}:{}:{}:{}",
        api_key, timestamp, "api.beta.mountainprotocol.com", "GET", "/v1/balance", "", ""
    );

    let mut mac =
        Hmac::<Sha256>::new_varkey(secret_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(string_to_sign.as_bytes());
    let result = mac.finalize();
    let signature = base64::encode(result.into_bytes());

    let client = reqwest::Client::new();

    println!("Sending request...");
    let response = client
        .get(url)
        .header("X-API-Key", api_key)
        .header("X-Timestamp", timestamp.to_string())
        .header("Authorization", format!("HMAC-SHA256 {}", signature))
        .timeout(Duration::from_secs(30)) // Setting a timeout of 10 seconds
        .send()
        .await?;
    println!("Received response");

    println!("Body:\n{}", response.text().await?);

    Ok(())
}
