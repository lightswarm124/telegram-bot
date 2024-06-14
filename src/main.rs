use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use teloxide::dispatching::update_listeners::polling_default;
use teloxide::dispatching::HandlerExt;
use teloxide::types::Update;
use reqwest::Client;
use std::env;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    Start,
    Help,
    Price(String),
    Unknown(String),
}

async fn fetch_price(symbol: &str, client: &Client, api_key: &str) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?symbol={}",
        symbol
    );
    let response = client
        .get(&url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .await?;

    let data: serde_json::Value = response.json().await?;

    if let Some(price) = data["data"][symbol]["quote"]["USD"]["price"].as_f64() {
        Ok(format!("${:.2}", price))
    } else {
        Ok("Data not found".to_string())
    }
}

async fn handle_command(
    bot: Bot,
    message: Message,
    command: Command,
) -> ResponseResult<()> {
    let client = Client::new();
    let api_key = env::var("COINMARKETCAP_API_KEY").expect("COINMARKETCAP_API_KEY not set");

    match command {
        Command::Start => {
            bot.send_message(message.chat.id, "Welcome! Use /help to see available commands.")
                .await?;
        }
        Command::Help => {
            bot.send_message(message.chat.id, "/start - Start the bot\n/help - Show help\n/price <symbol> - Get price of a cryptocurrency (e.g., /price btc)")
                .await?;
        }
        Command::Price(symbol) => {
            let price = fetch_price(&symbol.to_uppercase(), &client, &api_key).await.unwrap_or("Error fetching data".to_string());
            bot.send_message(message.chat.id, format!("Price of {}: {}", symbol, price))
                .await?;
        }
        Command::Unknown(command) => {
            bot.send_message(message.chat.id, format!("Unknown command: {}", command))
                .await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    log::info!("Starting bot...");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().filter_command::<Command>().endpoint(handle_command));

    Dispatcher::builder(bot.clone(), handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            polling_default(bot).await,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
}
