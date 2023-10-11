use dotenv::dotenv;
use hmac::{Hmac, Mac, NewMac};
use reqwest::{self};
use serde::Deserialize;
use serde_json::json;
use sha2::Sha256;
use std::collections::HashMap;
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct ApiResponse {
    data: Data,
}

#[derive(Deserialize)]
struct Data {
    poolData: Vec<PoolData>,
}

#[derive(Deserialize)]
struct PoolData {
    id: String,
    coins: Vec<Coin>,
}

async fn test_balance() -> Result<(), reqwest::Error> {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let url = "https://api.prod.mountainprotocol.com/v1/balance";

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let string_to_sign = format!(
        "{}:{}:{}:{}:{}:{}:{}",
        api_key, timestamp, "api.prod.mountainprotocol.com", "GET", "/v1/balance", "", ""
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

async fn test_withdraw() -> Result<(), reqwest::Error> {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let url = "https://api.beta.mountainprotocol.com/v1/withdraw/submit";

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Adjust the payload as per actual requirements
    let payload = json!({
        "amount": 10,
        "toAddress": "0x76a3C63e1996De61c39ca38fD9c84a133B2c41Be",
        "currency": "USDC",
    });

    // Serialize the payload to a JSON string
    let body = serde_json::to_string(&payload).unwrap_or_default();

    let string_to_sign = format!(
        "{}:{}:{}:{}:{}:{}:{}",
        api_key,
        timestamp,
        "api.beta.mountainprotocol.com",
        "POST",
        "/v1/withdraw/submit",
        "",
        body
    );

    let mut mac =
        Hmac::<Sha256>::new_varkey(secret_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(string_to_sign.as_bytes());
    let result = mac.finalize();
    let signature = base64::encode(result.into_bytes());

    let client = reqwest::Client::new();

    println!("Sending withdraw request...");
    let response = client
        .post(url)
        .header("X-API-Key", api_key)
        .header("X-Timestamp", timestamp.to_string())
        .header("Authorization", format!("HMAC-SHA256 {}", signature))
        .json(&payload)
        .timeout(Duration::from_secs(10)) // Setting a timeout of 10 seconds
        .send()
        .await?;
    println!("Received withdraw response");

    println!("Body:\n{}", response.text().await?);

    Ok(())
}

async fn test_transaction_detail() -> Result<(), reqwest::Error> {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let transaction_id = "TiapVIN-SFrRhAWsDBq8JA"; // Replace with actual transaction ID
    let url = format!(
        "https://api.beta.mountainprotocol.com/v1/transaction/detail/{}",
        transaction_id
    );

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let string_to_sign = format!(
        "{}:{}:{}:{}:{}:{}:{}",
        api_key,
        timestamp,
        "api.beta.mountainprotocol.com",
        "GET",
        format!("/v1/transaction/detail/{}", transaction_id),
        "",
        ""
    );

    let mut mac =
        Hmac::<Sha256>::new_varkey(secret_key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(string_to_sign.as_bytes());
    let result = mac.finalize();
    let signature = base64::encode(result.into_bytes());

    let client = reqwest::Client::new();

    println!("Sending transaction detail request...");
    let response = client
        .get(url)
        .header("X-API-Key", api_key)
        .header("X-Timestamp", timestamp.to_string())
        .header("Authorization", format!("HMAC-SHA256 {}", signature))
        .timeout(Duration::from_secs(10)) // Setting a timeout of 10 seconds
        .send()
        .await?;
    println!("Received transaction detail response");

    println!("Status: {}", response.status());
    println!("Headers:\n{:?}", response.headers());
    println!("Body:\n{}", response.text().await?);

    Ok(())
}

#[derive(Deserialize)]
struct Coin {
    symbol: String,
    usdPrice: Option<f64>,
}
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().ok();
    // let url = "https://api.curve.fi/api/getPools/ethereum/factory-crvusd";
    // let response: ApiResponse = reqwest::get(url).await?.json().await?;

    // let mut rates: HashMap<String, f64> = HashMap::new();

    // for pool in response.data.poolData {
    //     if pool.id == "factory-crvusd-0" || pool.id == "factory-crvusd-23" {
    //         for coin in &pool.coins {
    //             rates.insert(coin.symbol.clone(), coin.usdPrice.unwrap_or(0.0));
    //         }
    //     }
    // }

    // // 1. Convert 1000 USDM to crvUSD
    // let usdm_to_crvusd_rate =
    //     rates.get("USDM").unwrap_or(&0.0) / rates.get("crvUSD").unwrap_or(&1.0);
    // let crvusd_amount_before_fee = 1000.0 * usdm_to_crvusd_rate;
    // let crvusd_amount = crvusd_amount_before_fee * (1.0 - 0.000005);
    // println!("usdm_to_crvusd_rate: {}", crvusd_amount);

    // // 2. Convert the resulting crvUSD to USDC
    // let crvusd_to_usdc_rate =
    //     rates.get("crvUSD").unwrap_or(&0.0) / rates.get("USDC").unwrap_or(&1.0);
    // println!("crvusd_to_usdc_rate: {}", crvusd_to_usdc_rate);

    // let usdc_amount = crvusd_amount * crvusd_to_usdc_rate;

    // println!("usdc_amount after swapping: {}", usdc_amount);

    // if usdc_amount > 1000.0 {
    //     println!(
    //         "Arbitrage opportunity found! 1000 USDM can be converted to {} USDC",
    //         usdc_amount
    //     );
    // } else {
    //     println!(
    //         "No arbitrage opportunity. 1000 USDM will be converted to {} USDC",
    //         usdc_amount
    //     );
    // }
    test_balance().await;

    Ok(())
}
