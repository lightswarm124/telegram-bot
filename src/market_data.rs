use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct ApiResponse {
    status: Status,
    data: std::collections::HashMap<String, Cryptocurrency>,
}

#[derive(Deserialize)]
struct Status {
    error_code: u32,
    error_message: Option<String>,
    elapsed: u32,
    credit_count: u32,
}

#[derive(Deserialize)]
struct Cryptocurrency {
    id: u32,
    name: String,
    symbol: String,
    slug: String,
    cmc_rank: u32,
    quote: std::collections::HashMap<String, Quote>,
}

#[derive(Deserialize)]
struct Quote {
    price: f64,
    volume_24h: f64,
    percent_change_1h: f64,
    percent_change_24h: f64,
    percent_change_7d: f64,
    market_cap: f64,
    last_updated: String,
}

pub async fn fetch_market_data(coin: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("COINMARKETCAP_API_KEY")?;
    let client = Client::new();
    let url = format!("https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={}", coin);

    let res = client.get(&url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    if let Some(data) = res.data.get(coin) {
        if let Some(quote) = data.quote.get("USD") {
            return Ok(format!("${:.2}", quote.price));
        }
    }

    Ok("Data not found".to_string())
}
